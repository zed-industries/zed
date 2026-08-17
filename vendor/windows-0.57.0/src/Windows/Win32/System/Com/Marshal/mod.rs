#[inline]
pub unsafe fn BSTR_UserFree(param0: *const u32, param1: *const windows_core::BSTR) {
    windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserFree(param0 : *const u32, param1 : *const core::mem::MaybeUninit < windows_core::BSTR >));
    BSTR_UserFree(param0, core::mem::transmute(param1))
}
#[inline]
pub unsafe fn BSTR_UserFree64(param0: *const u32, param1: *const windows_core::BSTR) {
    windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserFree64(param0 : *const u32, param1 : *const core::mem::MaybeUninit < windows_core::BSTR >));
    BSTR_UserFree64(param0, core::mem::transmute(param1))
}
#[inline]
pub unsafe fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const windows_core::BSTR) -> *mut u8 {
    windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const core::mem::MaybeUninit < windows_core::BSTR >) -> *mut u8);
    BSTR_UserMarshal(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const windows_core::BSTR) -> *mut u8 {
    windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const core::mem::MaybeUninit < windows_core::BSTR >) -> *mut u8);
    BSTR_UserMarshal64(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const windows_core::BSTR) -> u32 {
    windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserSize(param0 : *const u32, param1 : u32, param2 : *const core::mem::MaybeUninit < windows_core::BSTR >) -> u32);
    BSTR_UserSize(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const windows_core::BSTR) -> u32 {
    windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserSize64(param0 : *const u32, param1 : u32, param2 : *const core::mem::MaybeUninit < windows_core::BSTR >) -> u32);
    BSTR_UserSize64(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut windows_core::BSTR) -> *mut u8 {
    windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> *mut u8);
    BSTR_UserUnmarshal(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut windows_core::BSTR) -> *mut u8 {
    windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> *mut u8);
    BSTR_UserUnmarshal64(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16) {
    windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserFree(param0 : *const u32, param1 : *const u16));
    CLIPFORMAT_UserFree(param0, param1)
}
#[inline]
pub unsafe fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16) {
    windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserFree64(param0 : *const u32, param1 : *const u16));
    CLIPFORMAT_UserFree64(param0, param1)
}
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const u16) -> *mut u8);
    CLIPFORMAT_UserMarshal(param0, param1, param2)
}
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const u16) -> *mut u8);
    CLIPFORMAT_UserMarshal64(param0, param1, param2)
}
#[inline]
pub unsafe fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserSize(param0 : *const u32, param1 : u32, param2 : *const u16) -> u32);
    CLIPFORMAT_UserSize(param0, param1, param2)
}
#[inline]
pub unsafe fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserSize64(param0 : *const u32, param1 : u32, param2 : *const u16) -> u32);
    CLIPFORMAT_UserSize64(param0, param1, param2)
}
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut u16) -> *mut u8);
    CLIPFORMAT_UserUnmarshal(param0, param1, param2)
}
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut u16) -> *mut u8);
    CLIPFORMAT_UserUnmarshal64(param0, param1, param2)
}
#[inline]
pub unsafe fn CoGetMarshalSizeMax<P0>(pulsize: *mut u32, riid: *const windows_core::GUID, punk: P0, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoGetMarshalSizeMax(pulsize : *mut u32, riid : *const windows_core::GUID, punk : * mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32) -> windows_core::HRESULT);
    CoGetMarshalSizeMax(pulsize, riid, punk.param().abi(), dwdestcontext, core::mem::transmute(pvdestcontext.unwrap_or(std::ptr::null())), mshlflags).ok()
}
#[inline]
pub unsafe fn CoGetStandardMarshal<P0>(riid: *const windows_core::GUID, punk: P0, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::Result<IMarshal>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoGetStandardMarshal(riid : *const windows_core::GUID, punk : * mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32, ppmarshal : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoGetStandardMarshal(riid, punk.param().abi(), dwdestcontext, core::mem::transmute(pvdestcontext.unwrap_or(std::ptr::null())), mshlflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoGetStdMarshalEx<P0>(punkouter: P0, smexflags: u32) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoGetStdMarshalEx(punkouter : * mut core::ffi::c_void, smexflags : u32, ppunkinner : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoGetStdMarshalEx(punkouter.param().abi(), smexflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoMarshalHresult<P0>(pstm: P0, hresult: windows_core::HRESULT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IStream>,
{
    windows_targets::link!("ole32.dll" "system" fn CoMarshalHresult(pstm : * mut core::ffi::c_void, hresult : windows_core::HRESULT) -> windows_core::HRESULT);
    CoMarshalHresult(pstm.param().abi(), hresult).ok()
}
#[inline]
pub unsafe fn CoMarshalInterThreadInterfaceInStream<P0>(riid: *const windows_core::GUID, punk: P0) -> windows_core::Result<super::IStream>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoMarshalInterThreadInterfaceInStream(riid : *const windows_core::GUID, punk : * mut core::ffi::c_void, ppstm : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoMarshalInterThreadInterfaceInStream(riid, punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoMarshalInterface<P0, P1>(pstm: P0, riid: *const windows_core::GUID, punk: P1, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IStream>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoMarshalInterface(pstm : * mut core::ffi::c_void, riid : *const windows_core::GUID, punk : * mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32) -> windows_core::HRESULT);
    CoMarshalInterface(pstm.param().abi(), riid, punk.param().abi(), dwdestcontext, core::mem::transmute(pvdestcontext.unwrap_or(std::ptr::null())), mshlflags).ok()
}
#[inline]
pub unsafe fn CoReleaseMarshalData<P0>(pstm: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IStream>,
{
    windows_targets::link!("ole32.dll" "system" fn CoReleaseMarshalData(pstm : * mut core::ffi::c_void) -> windows_core::HRESULT);
    CoReleaseMarshalData(pstm.param().abi()).ok()
}
#[inline]
pub unsafe fn CoUnmarshalHresult<P0>(pstm: P0) -> windows_core::Result<windows_core::HRESULT>
where
    P0: windows_core::Param<super::IStream>,
{
    windows_targets::link!("ole32.dll" "system" fn CoUnmarshalHresult(pstm : * mut core::ffi::c_void, phresult : *mut windows_core::HRESULT) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoUnmarshalHresult(pstm.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoUnmarshalInterface<P0, T>(pstm: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::IStream>,
    T: windows_core::Interface,
{
    windows_targets::link!("ole32.dll" "system" fn CoUnmarshalInterface(pstm : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CoUnmarshalInterface(pstm.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL) {
    windows_targets::link!("ole32.dll" "system" fn HACCEL_UserFree(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL));
    HACCEL_UserFree(param0, param1)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL) {
    windows_targets::link!("ole32.dll" "system" fn HACCEL_UserFree64(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL));
    HACCEL_UserFree64(param0, param1)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HACCEL_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL) -> *mut u8);
    HACCEL_UserMarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HACCEL_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL) -> *mut u8);
    HACCEL_UserMarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HACCEL_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL) -> u32);
    HACCEL_UserSize(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HACCEL_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL) -> u32);
    HACCEL_UserSize64(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HACCEL_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HACCEL) -> *mut u8);
    HACCEL_UserUnmarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HACCEL_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HACCEL) -> *mut u8);
    HACCEL_UserUnmarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP) {
    windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserFree(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HBITMAP));
    HBITMAP_UserFree(param0, param1)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP) {
    windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserFree64(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HBITMAP));
    HBITMAP_UserFree64(param0, param1)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HBITMAP) -> *mut u8);
    HBITMAP_UserMarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HBITMAP) -> *mut u8);
    HBITMAP_UserMarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HBITMAP) -> u32);
    HBITMAP_UserSize(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HBITMAP) -> u32);
    HBITMAP_UserSize64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HBITMAP) -> *mut u8);
    HBITMAP_UserUnmarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HBITMAP) -> *mut u8);
    HBITMAP_UserUnmarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC) {
    windows_targets::link!("ole32.dll" "system" fn HDC_UserFree(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HDC));
    HDC_UserFree(param0, param1)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC) {
    windows_targets::link!("ole32.dll" "system" fn HDC_UserFree64(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HDC));
    HDC_UserFree64(param0, param1)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HDC_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HDC) -> *mut u8);
    HDC_UserMarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HDC_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HDC) -> *mut u8);
    HDC_UserMarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HDC_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HDC) -> u32);
    HDC_UserSize(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HDC_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HDC) -> u32);
    HDC_UserSize64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HDC_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HDC) -> *mut u8);
    HDC_UserUnmarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HDC_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HDC) -> *mut u8);
    HDC_UserUnmarshal64(param0, param1, param2)
}
#[inline]
pub unsafe fn HGLOBAL_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HGLOBAL) {
    windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserFree(param0 : *const u32, param1 : *const super::super::super::Foundation:: HGLOBAL));
    HGLOBAL_UserFree(param0, param1)
}
#[inline]
pub unsafe fn HGLOBAL_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HGLOBAL) {
    windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserFree64(param0 : *const u32, param1 : *const super::super::super::Foundation:: HGLOBAL));
    HGLOBAL_UserFree64(param0, param1)
}
#[inline]
pub unsafe fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HGLOBAL) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Foundation:: HGLOBAL) -> *mut u8);
    HGLOBAL_UserMarshal(param0, param1, param2)
}
#[inline]
pub unsafe fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HGLOBAL) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Foundation:: HGLOBAL) -> *mut u8);
    HGLOBAL_UserMarshal64(param0, param1, param2)
}
#[inline]
pub unsafe fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HGLOBAL) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Foundation:: HGLOBAL) -> u32);
    HGLOBAL_UserSize(param0, param1, param2)
}
#[inline]
pub unsafe fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HGLOBAL) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Foundation:: HGLOBAL) -> u32);
    HGLOBAL_UserSize64(param0, param1, param2)
}
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HGLOBAL) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Foundation:: HGLOBAL) -> *mut u8);
    HGLOBAL_UserUnmarshal(param0, param1, param2)
}
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HGLOBAL) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Foundation:: HGLOBAL) -> *mut u8);
    HGLOBAL_UserUnmarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON) {
    windows_targets::link!("ole32.dll" "system" fn HICON_UserFree(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HICON));
    HICON_UserFree(param0, param1)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON) {
    windows_targets::link!("ole32.dll" "system" fn HICON_UserFree64(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HICON));
    HICON_UserFree64(param0, param1)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HICON_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON) -> *mut u8);
    HICON_UserMarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HICON_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON) -> *mut u8);
    HICON_UserMarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HICON_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON) -> u32);
    HICON_UserSize(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HICON_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON) -> u32);
    HICON_UserSize64(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HICON_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HICON) -> *mut u8);
    HICON_UserUnmarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HICON_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HICON) -> *mut u8);
    HICON_UserUnmarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU) {
    windows_targets::link!("ole32.dll" "system" fn HMENU_UserFree(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HMENU));
    HMENU_UserFree(param0, param1)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU) {
    windows_targets::link!("ole32.dll" "system" fn HMENU_UserFree64(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HMENU));
    HMENU_UserFree64(param0, param1)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HMENU_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU) -> *mut u8);
    HMENU_UserMarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HMENU_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU) -> *mut u8);
    HMENU_UserMarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HMENU_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU) -> u32);
    HMENU_UserSize(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HMENU_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU) -> u32);
    HMENU_UserSize64(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HMENU_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HMENU) -> *mut u8);
    HMENU_UserUnmarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HMENU_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HMENU) -> *mut u8);
    HMENU_UserUnmarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE) {
    windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserFree(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HPALETTE));
    HPALETTE_UserFree(param0, param1)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE) {
    windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserFree64(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HPALETTE));
    HPALETTE_UserFree64(param0, param1)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HPALETTE) -> *mut u8);
    HPALETTE_UserMarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HPALETTE) -> *mut u8);
    HPALETTE_UserMarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HPALETTE) -> u32);
    HPALETTE_UserSize(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HPALETTE) -> u32);
    HPALETTE_UserSize64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HPALETTE) -> *mut u8);
    HPALETTE_UserUnmarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HPALETTE) -> *mut u8);
    HPALETTE_UserUnmarshal64(param0, param1, param2)
}
#[inline]
pub unsafe fn HWND_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HWND) {
    windows_targets::link!("ole32.dll" "system" fn HWND_UserFree(param0 : *const u32, param1 : *const super::super::super::Foundation:: HWND));
    HWND_UserFree(param0, param1)
}
#[inline]
pub unsafe fn HWND_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HWND) {
    windows_targets::link!("ole32.dll" "system" fn HWND_UserFree64(param0 : *const u32, param1 : *const super::super::super::Foundation:: HWND));
    HWND_UserFree64(param0, param1)
}
#[inline]
pub unsafe fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HWND_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Foundation:: HWND) -> *mut u8);
    HWND_UserMarshal(param0, param1, param2)
}
#[inline]
pub unsafe fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HWND_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Foundation:: HWND) -> *mut u8);
    HWND_UserMarshal64(param0, param1, param2)
}
#[inline]
pub unsafe fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HWND_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Foundation:: HWND) -> u32);
    HWND_UserSize(param0, param1, param2)
}
#[inline]
pub unsafe fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HWND_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Foundation:: HWND) -> u32);
    HWND_UserSize64(param0, param1, param2)
}
#[inline]
pub unsafe fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HWND_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Foundation:: HWND) -> *mut u8);
    HWND_UserUnmarshal(param0, param1, param2)
}
#[inline]
pub unsafe fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HWND_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Foundation:: HWND) -> *mut u8);
    HWND_UserUnmarshal64(param0, param1, param2)
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserFree(param0 : *const u32, param1 : *const *const super:: SAFEARRAY));
    LPSAFEARRAY_UserFree(param0, param1)
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserFree64(param0 : *const u32, param1 : *const *const super:: SAFEARRAY));
    LPSAFEARRAY_UserFree64(param0, param1)
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const *const super:: SAFEARRAY) -> *mut u8);
    LPSAFEARRAY_UserMarshal(param0, param1, param2)
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const *const super:: SAFEARRAY) -> *mut u8);
    LPSAFEARRAY_UserMarshal64(param0, param1, param2)
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserSize(param0 : *const u32, param1 : u32, param2 : *const *const super:: SAFEARRAY) -> u32);
    LPSAFEARRAY_UserSize(param0, param1, param2)
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserSize64(param0 : *const u32, param1 : u32, param2 : *const *const super:: SAFEARRAY) -> u32);
    LPSAFEARRAY_UserSize64(param0, param1, param2)
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut *mut super:: SAFEARRAY) -> *mut u8);
    LPSAFEARRAY_UserUnmarshal(param0, param1, param2)
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut *mut super:: SAFEARRAY) -> *mut u8);
    LPSAFEARRAY_UserUnmarshal64(param0, param1, param2)
}
#[inline]
pub unsafe fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16) {
    windows_targets::link!("ole32.dll" "system" fn SNB_UserFree(param0 : *const u32, param1 : *const *const *const u16));
    SNB_UserFree(param0, param1)
}
#[inline]
pub unsafe fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16) {
    windows_targets::link!("ole32.dll" "system" fn SNB_UserFree64(param0 : *const u32, param1 : *const *const *const u16));
    SNB_UserFree64(param0, param1)
}
#[inline]
pub unsafe fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn SNB_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const *const *const u16) -> *mut u8);
    SNB_UserMarshal(param0, param1, param2)
}
#[inline]
pub unsafe fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn SNB_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const *const *const u16) -> *mut u8);
    SNB_UserMarshal64(param0, param1, param2)
}
#[inline]
pub unsafe fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn SNB_UserSize(param0 : *const u32, param1 : u32, param2 : *const *const *const u16) -> u32);
    SNB_UserSize(param0, param1, param2)
}
#[inline]
pub unsafe fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn SNB_UserSize64(param0 : *const u32, param1 : u32, param2 : *const *const *const u16) -> u32);
    SNB_UserSize64(param0, param1, param2)
}
#[inline]
pub unsafe fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn SNB_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut *mut *mut u16) -> *mut u8);
    SNB_UserUnmarshal(param0, param1, param2)
}
#[inline]
pub unsafe fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn SNB_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut *mut *mut u16) -> *mut u8);
    SNB_UserUnmarshal64(param0, param1, param2)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::STGMEDIUM) {
    windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserFree(param0 : *const u32, param1 : *const super:: STGMEDIUM));
    STGMEDIUM_UserFree(param0, param1)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::STGMEDIUM) {
    windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserFree64(param0 : *const u32, param1 : *const super:: STGMEDIUM));
    STGMEDIUM_UserFree64(param0, param1)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super:: STGMEDIUM) -> *mut u8);
    STGMEDIUM_UserMarshal(param0, param1, param2)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super:: STGMEDIUM) -> *mut u8);
    STGMEDIUM_UserMarshal64(param0, param1, param2)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserSize(param0 : *const u32, param1 : u32, param2 : *const super:: STGMEDIUM) -> u32);
    STGMEDIUM_UserSize(param0, param1, param2)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super:: STGMEDIUM) -> u32);
    STGMEDIUM_UserSize64(param0, param1, param2)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super:: STGMEDIUM) -> *mut u8);
    STGMEDIUM_UserUnmarshal(param0, param1, param2)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super:: STGMEDIUM) -> *mut u8);
    STGMEDIUM_UserUnmarshal64(param0, param1, param2)
}
windows_core::imp::define_interface!(IMarshal, IMarshal_Vtbl, 0x00000003_0000_0000_c000_000000000046);
impl core::ops::Deref for IMarshal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMarshal, windows_core::IUnknown);
impl IMarshal {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const windows_core::GUID, pv: Option<*const core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUnmarshalClass)(windows_core::Interface::as_raw(self), riid, core::mem::transmute(pv.unwrap_or(std::ptr::null())), dwdestcontext, core::mem::transmute(pvdestcontext.unwrap_or(std::ptr::null())), mshlflags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const windows_core::GUID, pv: Option<*const core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMarshalSizeMax)(windows_core::Interface::as_raw(self), riid, core::mem::transmute(pv.unwrap_or(std::ptr::null())), dwdestcontext, core::mem::transmute(pvdestcontext.unwrap_or(std::ptr::null())), mshlflags, &mut result__).map(|| result__)
    }
    pub unsafe fn MarshalInterface<P0>(&self, pstm: P0, riid: *const windows_core::GUID, pv: Option<*const core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStream>,
    {
        (windows_core::Interface::vtable(self).MarshalInterface)(windows_core::Interface::as_raw(self), pstm.param().abi(), riid, core::mem::transmute(pv.unwrap_or(std::ptr::null())), dwdestcontext, core::mem::transmute(pvdestcontext.unwrap_or(std::ptr::null())), mshlflags).ok()
    }
    pub unsafe fn UnmarshalInterface<P0>(&self, pstm: P0, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStream>,
    {
        (windows_core::Interface::vtable(self).UnmarshalInterface)(windows_core::Interface::as_raw(self), pstm.param().abi(), riid, ppv).ok()
    }
    pub unsafe fn ReleaseMarshalData<P0>(&self, pstm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStream>,
    {
        (windows_core::Interface::vtable(self).ReleaseMarshalData)(windows_core::Interface::as_raw(self), pstm.param().abi()).ok()
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisconnectObject)(windows_core::Interface::as_raw(self), dwreserved).ok()
    }
}
#[repr(C)]
pub struct IMarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUnmarshalClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetMarshalSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub MarshalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnmarshalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMarshalData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMarshal2, IMarshal2_Vtbl, 0x000001cf_0000_0000_c000_000000000046);
impl core::ops::Deref for IMarshal2 {
    type Target = IMarshal;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMarshal2, windows_core::IUnknown, IMarshal);
impl IMarshal2 {}
#[repr(C)]
pub struct IMarshal2_Vtbl {
    pub base__: IMarshal_Vtbl,
}
windows_core::imp::define_interface!(IMarshalingStream, IMarshalingStream_Vtbl, 0xd8f2f5e6_6102_4863_9f26_389a4676efde);
impl core::ops::Deref for IMarshalingStream {
    type Target = super::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMarshalingStream, windows_core::IUnknown, super::ISequentialStream, super::IStream);
impl IMarshalingStream {
    pub unsafe fn GetMarshalingContextAttribute(&self, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMarshalingContextAttribute)(windows_core::Interface::as_raw(self), attribute, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMarshalingStream_Vtbl {
    pub base__: super::IStream_Vtbl,
    pub GetMarshalingContextAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, super::CO_MARSHALING_CONTEXT_ATTRIBUTES, *mut usize) -> windows_core::HRESULT,
}
pub const SMEXF_HANDLER: STDMSHLFLAGS = STDMSHLFLAGS(2i32);
pub const SMEXF_SERVER: STDMSHLFLAGS = STDMSHLFLAGS(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STDMSHLFLAGS(pub i32);
impl windows_core::TypeKind for STDMSHLFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STDMSHLFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STDMSHLFLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
