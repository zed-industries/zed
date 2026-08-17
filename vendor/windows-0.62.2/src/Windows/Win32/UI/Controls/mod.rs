#[cfg(feature = "Win32_UI_Controls_Dialogs")]
pub mod Dialogs;
#[cfg(feature = "Win32_UI_Controls_RichEdit")]
pub mod RichEdit;
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn BeginBufferedAnimation(hwnd: super::super::Foundation::HWND, hdctarget: super::super::Graphics::Gdi::HDC, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: Option<*const BP_PAINTPARAMS>, panimationparams: *const BP_ANIMATIONPARAMS, phdcfrom: *mut super::super::Graphics::Gdi::HDC, phdcto: *mut super::super::Graphics::Gdi::HDC) -> isize {
    windows_core::link!("uxtheme.dll" "system" fn BeginBufferedAnimation(hwnd : super::super::Foundation:: HWND, hdctarget : super::super::Graphics::Gdi:: HDC, prctarget : *const super::super::Foundation:: RECT, dwformat : BP_BUFFERFORMAT, ppaintparams : *const BP_PAINTPARAMS, panimationparams : *const BP_ANIMATIONPARAMS, phdcfrom : *mut super::super::Graphics::Gdi:: HDC, phdcto : *mut super::super::Graphics::Gdi:: HDC) -> isize);
    unsafe { BeginBufferedAnimation(hwnd, hdctarget, prctarget, dwformat, ppaintparams.unwrap_or(core::mem::zeroed()) as _, panimationparams, phdcfrom as _, phdcto as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn BeginBufferedPaint(hdctarget: super::super::Graphics::Gdi::HDC, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: Option<*const BP_PAINTPARAMS>, phdc: *mut super::super::Graphics::Gdi::HDC) -> isize {
    windows_core::link!("uxtheme.dll" "system" fn BeginBufferedPaint(hdctarget : super::super::Graphics::Gdi:: HDC, prctarget : *const super::super::Foundation:: RECT, dwformat : BP_BUFFERFORMAT, ppaintparams : *const BP_PAINTPARAMS, phdc : *mut super::super::Graphics::Gdi:: HDC) -> isize);
    unsafe { BeginBufferedPaint(hdctarget, prctarget, dwformat, ppaintparams.unwrap_or(core::mem::zeroed()) as _, phdc as _) }
}
#[inline]
pub unsafe fn BeginPanningFeedback(hwnd: super::super::Foundation::HWND) -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn BeginPanningFeedback(hwnd : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { BeginPanningFeedback(hwnd) }
}
#[inline]
pub unsafe fn BufferedPaintClear(hbufferedpaint: isize, prc: Option<*const super::super::Foundation::RECT>) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn BufferedPaintClear(hbufferedpaint : isize, prc : *const super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe { BufferedPaintClear(hbufferedpaint, prc.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn BufferedPaintInit() -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn BufferedPaintInit() -> windows_core::HRESULT);
    unsafe { BufferedPaintInit().ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn BufferedPaintRenderAnimation(hwnd: super::super::Foundation::HWND, hdctarget: super::super::Graphics::Gdi::HDC) -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn BufferedPaintRenderAnimation(hwnd : super::super::Foundation:: HWND, hdctarget : super::super::Graphics::Gdi:: HDC) -> windows_core::BOOL);
    unsafe { BufferedPaintRenderAnimation(hwnd, hdctarget) }
}
#[inline]
pub unsafe fn BufferedPaintSetAlpha(hbufferedpaint: isize, prc: Option<*const super::super::Foundation::RECT>, alpha: u8) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn BufferedPaintSetAlpha(hbufferedpaint : isize, prc : *const super::super::Foundation:: RECT, alpha : u8) -> windows_core::HRESULT);
    unsafe { BufferedPaintSetAlpha(hbufferedpaint, prc.unwrap_or(core::mem::zeroed()) as _, alpha).ok() }
}
#[inline]
pub unsafe fn BufferedPaintStopAllAnimations(hwnd: super::super::Foundation::HWND) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn BufferedPaintStopAllAnimations(hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    unsafe { BufferedPaintStopAllAnimations(hwnd).ok() }
}
#[inline]
pub unsafe fn BufferedPaintUnInit() -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn BufferedPaintUnInit() -> windows_core::HRESULT);
    unsafe { BufferedPaintUnInit().ok() }
}
#[inline]
pub unsafe fn CheckDlgButton(hdlg: super::super::Foundation::HWND, nidbutton: i32, ucheck: DLG_BUTTON_CHECK_STATE) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn CheckDlgButton(hdlg : super::super::Foundation:: HWND, nidbutton : i32, ucheck : DLG_BUTTON_CHECK_STATE) -> windows_core::BOOL);
    unsafe { CheckDlgButton(hdlg, nidbutton, ucheck).ok() }
}
#[inline]
pub unsafe fn CheckRadioButton(hdlg: super::super::Foundation::HWND, nidfirstbutton: i32, nidlastbutton: i32, nidcheckbutton: i32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn CheckRadioButton(hdlg : super::super::Foundation:: HWND, nidfirstbutton : i32, nidlastbutton : i32, nidcheckbutton : i32) -> windows_core::BOOL);
    unsafe { CheckRadioButton(hdlg, nidfirstbutton, nidlastbutton, nidcheckbutton).ok() }
}
#[inline]
pub unsafe fn CloseThemeData(htheme: HTHEME) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn CloseThemeData(htheme : HTHEME) -> windows_core::HRESULT);
    unsafe { CloseThemeData(htheme).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateMappedBitmap(hinstance: super::super::Foundation::HINSTANCE, idbitmap: isize, wflags: u32, lpcolormap: Option<*const COLORMAP>, inummaps: i32) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
    windows_core::link!("comctl32.dll" "system" fn CreateMappedBitmap(hinstance : super::super::Foundation:: HINSTANCE, idbitmap : isize, wflags : u32, lpcolormap : *const COLORMAP, inummaps : i32) -> super::super::Graphics::Gdi:: HBITMAP);
    let result__ = unsafe { CreateMappedBitmap(hinstance, idbitmap, wflags, lpcolormap.unwrap_or(core::mem::zeroed()) as _, inummaps) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn CreatePropertySheetPageA(constpropsheetpagepointer: *mut PROPSHEETPAGEA) -> HPROPSHEETPAGE {
    windows_core::link!("comctl32.dll" "system" fn CreatePropertySheetPageA(constpropsheetpagepointer : *mut PROPSHEETPAGEA) -> HPROPSHEETPAGE);
    unsafe { CreatePropertySheetPageA(constpropsheetpagepointer as _) }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn CreatePropertySheetPageW(constpropsheetpagepointer: *mut PROPSHEETPAGEW) -> HPROPSHEETPAGE {
    windows_core::link!("comctl32.dll" "system" fn CreatePropertySheetPageW(constpropsheetpagepointer : *mut PROPSHEETPAGEW) -> HPROPSHEETPAGE);
    unsafe { CreatePropertySheetPageW(constpropsheetpagepointer as _) }
}
#[inline]
pub unsafe fn CreateStatusWindowA<P1>(style: i32, lpsztext: P1, hwndparent: super::super::Foundation::HWND, wid: u32) -> windows_core::Result<super::super::Foundation::HWND>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn CreateStatusWindowA(style : i32, lpsztext : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND, wid : u32) -> super::super::Foundation:: HWND);
    let result__ = unsafe { CreateStatusWindowA(style, lpsztext.param().abi(), hwndparent, wid) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn CreateStatusWindowW<P1>(style: i32, lpsztext: P1, hwndparent: super::super::Foundation::HWND, wid: u32) -> windows_core::Result<super::super::Foundation::HWND>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn CreateStatusWindowW(style : i32, lpsztext : windows_core::PCWSTR, hwndparent : super::super::Foundation:: HWND, wid : u32) -> super::super::Foundation:: HWND);
    let result__ = unsafe { CreateStatusWindowW(style, lpsztext.param().abi(), hwndparent, wid) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn CreateSyntheticPointerDevice(pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, maxcount: u32, mode: POINTER_FEEDBACK_MODE) -> windows_core::Result<HSYNTHETICPOINTERDEVICE> {
    windows_core::link!("user32.dll" "system" fn CreateSyntheticPointerDevice(pointertype : super::WindowsAndMessaging:: POINTER_INPUT_TYPE, maxcount : u32, mode : POINTER_FEEDBACK_MODE) -> HSYNTHETICPOINTERDEVICE);
    let result__ = unsafe { CreateSyntheticPointerDevice(pointertype, maxcount, mode) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn CreateToolbarEx(hwnd: super::super::Foundation::HWND, ws: u32, wid: u32, nbitmaps: i32, hbminst: super::super::Foundation::HINSTANCE, wbmid: usize, lpbuttons: *mut TBBUTTON, inumbuttons: i32, dxbutton: i32, dybutton: i32, dxbitmap: i32, dybitmap: i32, ustructsize: u32) -> windows_core::Result<super::super::Foundation::HWND> {
    windows_core::link!("comctl32.dll" "system" fn CreateToolbarEx(hwnd : super::super::Foundation:: HWND, ws : u32, wid : u32, nbitmaps : i32, hbminst : super::super::Foundation:: HINSTANCE, wbmid : usize, lpbuttons : *mut TBBUTTON, inumbuttons : i32, dxbutton : i32, dybutton : i32, dxbitmap : i32, dybitmap : i32, ustructsize : u32) -> super::super::Foundation:: HWND);
    let result__ = unsafe { CreateToolbarEx(hwnd, ws, wid, nbitmaps, hbminst, wbmid, lpbuttons as _, inumbuttons, dxbutton, dybutton, dxbitmap, dybitmap, ustructsize) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn CreateUpDownControl(dwstyle: u32, x: i32, y: i32, cx: i32, cy: i32, hparent: super::super::Foundation::HWND, nid: i32, hinst: super::super::Foundation::HINSTANCE, hbuddy: super::super::Foundation::HWND, nupper: i32, nlower: i32, npos: i32) -> super::super::Foundation::HWND {
    windows_core::link!("comctl32.dll" "system" fn CreateUpDownControl(dwstyle : u32, x : i32, y : i32, cx : i32, cy : i32, hparent : super::super::Foundation:: HWND, nid : i32, hinst : super::super::Foundation:: HINSTANCE, hbuddy : super::super::Foundation:: HWND, nupper : i32, nlower : i32, npos : i32) -> super::super::Foundation:: HWND);
    unsafe { CreateUpDownControl(dwstyle, x, y, cx, cy, hparent, nid, hinst, hbuddy, nupper, nlower, npos) }
}
#[inline]
pub unsafe fn DPA_Clone(hdpa: HDPA, hdpanew: Option<HDPA>) -> HDPA {
    windows_core::link!("comctl32.dll" "system" fn DPA_Clone(hdpa : HDPA, hdpanew : HDPA) -> HDPA);
    unsafe { DPA_Clone(hdpa, hdpanew.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_Create(citemgrow: i32) -> HDPA {
    windows_core::link!("comctl32.dll" "system" fn DPA_Create(citemgrow : i32) -> HDPA);
    unsafe { DPA_Create(citemgrow) }
}
#[inline]
pub unsafe fn DPA_CreateEx(cpgrow: i32, hheap: Option<super::super::Foundation::HANDLE>) -> HDPA {
    windows_core::link!("comctl32.dll" "system" fn DPA_CreateEx(cpgrow : i32, hheap : super::super::Foundation:: HANDLE) -> HDPA);
    unsafe { DPA_CreateEx(cpgrow, hheap.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_DeleteAllPtrs(hdpa: HDPA) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_DeleteAllPtrs(hdpa : HDPA) -> windows_core::BOOL);
    unsafe { DPA_DeleteAllPtrs(hdpa) }
}
#[inline]
pub unsafe fn DPA_DeletePtr(hdpa: HDPA, i: i32) -> *mut core::ffi::c_void {
    windows_core::link!("comctl32.dll" "system" fn DPA_DeletePtr(hdpa : HDPA, i : i32) -> *mut core::ffi::c_void);
    unsafe { DPA_DeletePtr(hdpa, i) }
}
#[inline]
pub unsafe fn DPA_Destroy(hdpa: Option<HDPA>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_Destroy(hdpa : HDPA) -> windows_core::BOOL);
    unsafe { DPA_Destroy(hdpa.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_DestroyCallback(hdpa: Option<HDPA>, pfncb: PFNDAENUMCALLBACK, pdata: Option<*const core::ffi::c_void>) {
    windows_core::link!("comctl32.dll" "system" fn DPA_DestroyCallback(hdpa : HDPA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
    unsafe { DPA_DestroyCallback(hdpa.unwrap_or(core::mem::zeroed()) as _, pfncb, pdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_EnumCallback(hdpa: Option<HDPA>, pfncb: PFNDAENUMCALLBACK, pdata: Option<*const core::ffi::c_void>) {
    windows_core::link!("comctl32.dll" "system" fn DPA_EnumCallback(hdpa : HDPA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
    unsafe { DPA_EnumCallback(hdpa.unwrap_or(core::mem::zeroed()) as _, pfncb, pdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_GetPtr(hdpa: HDPA, i: isize) -> *mut core::ffi::c_void {
    windows_core::link!("comctl32.dll" "system" fn DPA_GetPtr(hdpa : HDPA, i : isize) -> *mut core::ffi::c_void);
    unsafe { DPA_GetPtr(hdpa, i) }
}
#[inline]
pub unsafe fn DPA_GetPtrIndex(hdpa: HDPA, p: Option<*const core::ffi::c_void>) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DPA_GetPtrIndex(hdpa : HDPA, p : *const core::ffi::c_void) -> i32);
    unsafe { DPA_GetPtrIndex(hdpa, p.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_GetSize(hdpa: Option<HDPA>) -> u64 {
    windows_core::link!("comctl32.dll" "system" fn DPA_GetSize(hdpa : HDPA) -> u64);
    unsafe { DPA_GetSize(hdpa.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_Grow(pdpa: HDPA, cp: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_Grow(pdpa : HDPA, cp : i32) -> windows_core::BOOL);
    unsafe { DPA_Grow(pdpa, cp) }
}
#[inline]
pub unsafe fn DPA_InsertPtr(hdpa: HDPA, i: i32, p: Option<*const core::ffi::c_void>) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DPA_InsertPtr(hdpa : HDPA, i : i32, p : *const core::ffi::c_void) -> i32);
    unsafe { DPA_InsertPtr(hdpa, i, p.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DPA_LoadStream<P2>(phdpa: *mut HDPA, pfn: PFNDPASTREAM, pstream: P2, pvinstdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P2: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn DPA_LoadStream(phdpa : *mut HDPA, pfn : PFNDPASTREAM, pstream : * mut core::ffi::c_void, pvinstdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DPA_LoadStream(phdpa as _, pfn, pstream.param().abi(), pvinstdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DPA_Merge(hdpadest: HDPA, hdpasrc: HDPA, dwflags: u32, pfncompare: PFNDACOMPARE, pfnmerge: PFNDPAMERGE, lparam: super::super::Foundation::LPARAM) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_Merge(hdpadest : HDPA, hdpasrc : HDPA, dwflags : u32, pfncompare : PFNDACOMPARE, pfnmerge : PFNDPAMERGE, lparam : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { DPA_Merge(hdpadest, hdpasrc, dwflags, pfncompare, pfnmerge, lparam) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DPA_SaveStream<P2>(hdpa: HDPA, pfn: PFNDPASTREAM, pstream: P2, pvinstdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P2: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn DPA_SaveStream(hdpa : HDPA, pfn : PFNDPASTREAM, pstream : * mut core::ffi::c_void, pvinstdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DPA_SaveStream(hdpa, pfn, pstream.param().abi(), pvinstdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DPA_Search(hdpa: HDPA, pfind: Option<*const core::ffi::c_void>, istart: i32, pfncompare: PFNDACOMPARE, lparam: super::super::Foundation::LPARAM, options: u32) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DPA_Search(hdpa : HDPA, pfind : *const core::ffi::c_void, istart : i32, pfncompare : PFNDACOMPARE, lparam : super::super::Foundation:: LPARAM, options : u32) -> i32);
    unsafe { DPA_Search(hdpa, pfind.unwrap_or(core::mem::zeroed()) as _, istart, pfncompare, lparam, options) }
}
#[inline]
pub unsafe fn DPA_SetPtr(hdpa: HDPA, i: i32, p: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_SetPtr(hdpa : HDPA, i : i32, p : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DPA_SetPtr(hdpa, i, p.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DPA_Sort(hdpa: HDPA, pfncompare: PFNDACOMPARE, lparam: super::super::Foundation::LPARAM) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DPA_Sort(hdpa : HDPA, pfncompare : PFNDACOMPARE, lparam : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { DPA_Sort(hdpa, pfncompare, lparam) }
}
#[inline]
pub unsafe fn DSA_Clone(hdsa: HDSA) -> HDSA {
    windows_core::link!("comctl32.dll" "system" fn DSA_Clone(hdsa : HDSA) -> HDSA);
    unsafe { DSA_Clone(hdsa) }
}
#[inline]
pub unsafe fn DSA_Create(cbitem: i32, citemgrow: i32) -> HDSA {
    windows_core::link!("comctl32.dll" "system" fn DSA_Create(cbitem : i32, citemgrow : i32) -> HDSA);
    unsafe { DSA_Create(cbitem, citemgrow) }
}
#[inline]
pub unsafe fn DSA_DeleteAllItems(hdsa: HDSA) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_DeleteAllItems(hdsa : HDSA) -> windows_core::BOOL);
    unsafe { DSA_DeleteAllItems(hdsa) }
}
#[inline]
pub unsafe fn DSA_DeleteItem(hdsa: HDSA, i: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_DeleteItem(hdsa : HDSA, i : i32) -> windows_core::BOOL);
    unsafe { DSA_DeleteItem(hdsa, i) }
}
#[inline]
pub unsafe fn DSA_Destroy(hdsa: Option<HDSA>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_Destroy(hdsa : HDSA) -> windows_core::BOOL);
    unsafe { DSA_Destroy(hdsa.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DSA_DestroyCallback(hdsa: Option<HDSA>, pfncb: PFNDAENUMCALLBACK, pdata: Option<*const core::ffi::c_void>) {
    windows_core::link!("comctl32.dll" "system" fn DSA_DestroyCallback(hdsa : HDSA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
    unsafe { DSA_DestroyCallback(hdsa.unwrap_or(core::mem::zeroed()) as _, pfncb, pdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DSA_EnumCallback(hdsa: HDSA, pfncb: PFNDAENUMCALLBACK, pdata: Option<*const core::ffi::c_void>) {
    windows_core::link!("comctl32.dll" "system" fn DSA_EnumCallback(hdsa : HDSA, pfncb : PFNDAENUMCALLBACK, pdata : *const core::ffi::c_void));
    unsafe { DSA_EnumCallback(hdsa, pfncb, pdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DSA_GetItem(hdsa: HDSA, i: i32, pitem: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_GetItem(hdsa : HDSA, i : i32, pitem : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DSA_GetItem(hdsa, i, pitem as _) }
}
#[inline]
pub unsafe fn DSA_GetItemPtr(hdsa: HDSA, i: i32) -> *mut core::ffi::c_void {
    windows_core::link!("comctl32.dll" "system" fn DSA_GetItemPtr(hdsa : HDSA, i : i32) -> *mut core::ffi::c_void);
    unsafe { DSA_GetItemPtr(hdsa, i) }
}
#[inline]
pub unsafe fn DSA_GetSize(hdsa: Option<HDSA>) -> u64 {
    windows_core::link!("comctl32.dll" "system" fn DSA_GetSize(hdsa : HDSA) -> u64);
    unsafe { DSA_GetSize(hdsa.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DSA_InsertItem(hdsa: HDSA, i: i32, pitem: *const core::ffi::c_void) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DSA_InsertItem(hdsa : HDSA, i : i32, pitem : *const core::ffi::c_void) -> i32);
    unsafe { DSA_InsertItem(hdsa, i, pitem) }
}
#[inline]
pub unsafe fn DSA_SetItem(hdsa: HDSA, i: i32, pitem: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_SetItem(hdsa : HDSA, i : i32, pitem : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DSA_SetItem(hdsa, i, pitem) }
}
#[inline]
pub unsafe fn DSA_Sort(pdsa: HDSA, pfncompare: PFNDACOMPARE, lparam: super::super::Foundation::LPARAM) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DSA_Sort(pdsa : HDSA, pfncompare : PFNDACOMPARE, lparam : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { DSA_Sort(pdsa, pfncompare, lparam) }
}
#[inline]
pub unsafe fn DestroyPropertySheetPage(param0: HPROPSHEETPAGE) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn DestroyPropertySheetPage(param0 : HPROPSHEETPAGE) -> windows_core::BOOL);
    unsafe { DestroyPropertySheetPage(param0) }
}
#[inline]
pub unsafe fn DestroySyntheticPointerDevice(device: HSYNTHETICPOINTERDEVICE) {
    windows_core::link!("user32.dll" "system" fn DestroySyntheticPointerDevice(device : HSYNTHETICPOINTERDEVICE));
    unsafe { DestroySyntheticPointerDevice(device) }
}
#[inline]
pub unsafe fn DlgDirListA(hdlg: super::super::Foundation::HWND, lppathspec: windows_core::PSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    windows_core::link!("user32.dll" "system" fn DlgDirListA(hdlg : super::super::Foundation:: HWND, lppathspec : windows_core::PSTR, nidlistbox : i32, nidstaticpath : i32, ufiletype : DLG_DIR_LIST_FILE_TYPE) -> i32);
    unsafe { DlgDirListA(hdlg, core::mem::transmute(lppathspec), nidlistbox, nidstaticpath, ufiletype) }
}
#[inline]
pub unsafe fn DlgDirListComboBoxA(hdlg: super::super::Foundation::HWND, lppathspec: windows_core::PSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    windows_core::link!("user32.dll" "system" fn DlgDirListComboBoxA(hdlg : super::super::Foundation:: HWND, lppathspec : windows_core::PSTR, nidcombobox : i32, nidstaticpath : i32, ufiletype : DLG_DIR_LIST_FILE_TYPE) -> i32);
    unsafe { DlgDirListComboBoxA(hdlg, core::mem::transmute(lppathspec), nidcombobox, nidstaticpath, ufiletype) }
}
#[inline]
pub unsafe fn DlgDirListComboBoxW(hdlg: super::super::Foundation::HWND, lppathspec: windows_core::PWSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    windows_core::link!("user32.dll" "system" fn DlgDirListComboBoxW(hdlg : super::super::Foundation:: HWND, lppathspec : windows_core::PWSTR, nidcombobox : i32, nidstaticpath : i32, ufiletype : DLG_DIR_LIST_FILE_TYPE) -> i32);
    unsafe { DlgDirListComboBoxW(hdlg, core::mem::transmute(lppathspec), nidcombobox, nidstaticpath, ufiletype) }
}
#[inline]
pub unsafe fn DlgDirListW(hdlg: super::super::Foundation::HWND, lppathspec: windows_core::PWSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    windows_core::link!("user32.dll" "system" fn DlgDirListW(hdlg : super::super::Foundation:: HWND, lppathspec : windows_core::PWSTR, nidlistbox : i32, nidstaticpath : i32, ufiletype : DLG_DIR_LIST_FILE_TYPE) -> i32);
    unsafe { DlgDirListW(hdlg, core::mem::transmute(lppathspec), nidlistbox, nidstaticpath, ufiletype) }
}
#[inline]
pub unsafe fn DlgDirSelectComboBoxExA(hwnddlg: super::super::Foundation::HWND, lpstring: &mut [u8], idcombobox: i32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn DlgDirSelectComboBoxExA(hwnddlg : super::super::Foundation:: HWND, lpstring : windows_core::PSTR, cchout : i32, idcombobox : i32) -> windows_core::BOOL);
    unsafe { DlgDirSelectComboBoxExA(hwnddlg, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), idcombobox).ok() }
}
#[inline]
pub unsafe fn DlgDirSelectComboBoxExW(hwnddlg: super::super::Foundation::HWND, lpstring: &mut [u16], idcombobox: i32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn DlgDirSelectComboBoxExW(hwnddlg : super::super::Foundation:: HWND, lpstring : windows_core::PWSTR, cchout : i32, idcombobox : i32) -> windows_core::BOOL);
    unsafe { DlgDirSelectComboBoxExW(hwnddlg, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), idcombobox).ok() }
}
#[inline]
pub unsafe fn DlgDirSelectExA(hwnddlg: super::super::Foundation::HWND, lpstring: &mut [u8], idlistbox: i32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn DlgDirSelectExA(hwnddlg : super::super::Foundation:: HWND, lpstring : windows_core::PSTR, chcount : i32, idlistbox : i32) -> windows_core::BOOL);
    unsafe { DlgDirSelectExA(hwnddlg, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), idlistbox).ok() }
}
#[inline]
pub unsafe fn DlgDirSelectExW(hwnddlg: super::super::Foundation::HWND, lpstring: &mut [u16], idlistbox: i32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn DlgDirSelectExW(hwnddlg : super::super::Foundation:: HWND, lpstring : windows_core::PWSTR, chcount : i32, idlistbox : i32) -> windows_core::BOOL);
    unsafe { DlgDirSelectExW(hwnddlg, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), idlistbox).ok() }
}
#[inline]
pub unsafe fn DrawInsert(handparent: super::super::Foundation::HWND, hlb: super::super::Foundation::HWND, nitem: i32) {
    windows_core::link!("comctl32.dll" "system" fn DrawInsert(handparent : super::super::Foundation:: HWND, hlb : super::super::Foundation:: HWND, nitem : i32));
    unsafe { DrawInsert(handparent, hlb, nitem) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawShadowText(hdc: super::super::Graphics::Gdi::HDC, psztext: &[u16], prc: *const super::super::Foundation::RECT, dwflags: u32, crtext: super::super::Foundation::COLORREF, crshadow: super::super::Foundation::COLORREF, ixoffset: i32, iyoffset: i32) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn DrawShadowText(hdc : super::super::Graphics::Gdi:: HDC, psztext : windows_core::PCWSTR, cch : u32, prc : *const super::super::Foundation:: RECT, dwflags : u32, crtext : super::super::Foundation:: COLORREF, crshadow : super::super::Foundation:: COLORREF, ixoffset : i32, iyoffset : i32) -> i32);
    unsafe { DrawShadowText(hdc, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap(), prc, dwflags, crtext, crshadow, ixoffset, iyoffset) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawStatusTextA<P2>(hdc: super::super::Graphics::Gdi::HDC, lprc: *mut super::super::Foundation::RECT, psztext: P2, uflags: u32)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn DrawStatusTextA(hdc : super::super::Graphics::Gdi:: HDC, lprc : *mut super::super::Foundation:: RECT, psztext : windows_core::PCSTR, uflags : u32));
    unsafe { DrawStatusTextA(hdc, lprc as _, psztext.param().abi(), uflags) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawStatusTextW<P2>(hdc: super::super::Graphics::Gdi::HDC, lprc: *mut super::super::Foundation::RECT, psztext: P2, uflags: u32)
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn DrawStatusTextW(hdc : super::super::Graphics::Gdi:: HDC, lprc : *mut super::super::Foundation:: RECT, psztext : windows_core::PCWSTR, uflags : u32));
    unsafe { DrawStatusTextW(hdc, lprc as _, psztext.param().abi(), uflags) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeBackground(htheme: HTHEME, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, pcliprect: Option<*const super::super::Foundation::RECT>) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn DrawThemeBackground(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, prect : *const super::super::Foundation:: RECT, pcliprect : *const super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe { DrawThemeBackground(htheme, hdc, ipartid, istateid, prect, pcliprect.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeBackgroundEx(htheme: HTHEME, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, poptions: Option<*const DTBGOPTS>) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn DrawThemeBackgroundEx(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, prect : *const super::super::Foundation:: RECT, poptions : *const DTBGOPTS) -> windows_core::HRESULT);
    unsafe { DrawThemeBackgroundEx(htheme, hdc, ipartid, istateid, prect, poptions.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeEdge(htheme: HTHEME, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, pdestrect: *const super::super::Foundation::RECT, uedge: super::super::Graphics::Gdi::DRAWEDGE_FLAGS, uflags: super::super::Graphics::Gdi::DRAW_EDGE_FLAGS, pcontentrect: Option<*mut super::super::Foundation::RECT>) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn DrawThemeEdge(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, pdestrect : *const super::super::Foundation:: RECT, uedge : super::super::Graphics::Gdi:: DRAWEDGE_FLAGS, uflags : super::super::Graphics::Gdi:: DRAW_EDGE_FLAGS, pcontentrect : *mut super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe { DrawThemeEdge(htheme, hdc, ipartid, istateid, pdestrect, uedge, uflags, pcontentrect.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeIcon(htheme: HTHEME, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, himl: HIMAGELIST, iimageindex: i32) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn DrawThemeIcon(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, prect : *const super::super::Foundation:: RECT, himl : HIMAGELIST, iimageindex : i32) -> windows_core::HRESULT);
    unsafe { DrawThemeIcon(htheme, hdc, ipartid, istateid, prect, himl, iimageindex).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeParentBackground(hwnd: super::super::Foundation::HWND, hdc: super::super::Graphics::Gdi::HDC, prc: Option<*const super::super::Foundation::RECT>) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn DrawThemeParentBackground(hwnd : super::super::Foundation:: HWND, hdc : super::super::Graphics::Gdi:: HDC, prc : *const super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe { DrawThemeParentBackground(hwnd, hdc, prc.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeParentBackgroundEx(hwnd: super::super::Foundation::HWND, hdc: super::super::Graphics::Gdi::HDC, dwflags: DRAW_THEME_PARENT_BACKGROUND_FLAGS, prc: Option<*const super::super::Foundation::RECT>) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn DrawThemeParentBackgroundEx(hwnd : super::super::Foundation:: HWND, hdc : super::super::Graphics::Gdi:: HDC, dwflags : DRAW_THEME_PARENT_BACKGROUND_FLAGS, prc : *const super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe { DrawThemeParentBackgroundEx(hwnd, hdc, dwflags, prc.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeText(htheme: HTHEME, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: &[u16], dwtextflags: super::super::Graphics::Gdi::DRAW_TEXT_FORMAT, dwtextflags2: u32, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn DrawThemeText(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, psztext : windows_core::PCWSTR, cchtext : i32, dwtextflags : super::super::Graphics::Gdi:: DRAW_TEXT_FORMAT, dwtextflags2 : u32, prect : *const super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe { DrawThemeText(htheme, hdc, ipartid, istateid, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap(), dwtextflags, dwtextflags2, prect).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeTextEx(htheme: HTHEME, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: &[u16], dwtextflags: super::super::Graphics::Gdi::DRAW_TEXT_FORMAT, prect: *mut super::super::Foundation::RECT, poptions: Option<*const DTTOPTS>) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn DrawThemeTextEx(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, psztext : windows_core::PCWSTR, cchtext : i32, dwtextflags : super::super::Graphics::Gdi:: DRAW_TEXT_FORMAT, prect : *mut super::super::Foundation:: RECT, poptions : *const DTTOPTS) -> windows_core::HRESULT);
    unsafe { DrawThemeTextEx(htheme, hdc, ipartid, istateid, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap(), dwtextflags, prect as _, poptions.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn EnableScrollBar(hwnd: super::super::Foundation::HWND, wsbflags: u32, warrows: ENABLE_SCROLL_BAR_ARROWS) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn EnableScrollBar(hwnd : super::super::Foundation:: HWND, wsbflags : u32, warrows : ENABLE_SCROLL_BAR_ARROWS) -> windows_core::BOOL);
    unsafe { EnableScrollBar(hwnd, wsbflags, warrows).ok() }
}
#[inline]
pub unsafe fn EnableThemeDialogTexture(hwnd: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn EnableThemeDialogTexture(hwnd : super::super::Foundation:: HWND, dwflags : u32) -> windows_core::HRESULT);
    unsafe { EnableThemeDialogTexture(hwnd, dwflags).ok() }
}
#[inline]
pub unsafe fn EnableTheming(fenable: bool) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn EnableTheming(fenable : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { EnableTheming(fenable.into()).ok() }
}
#[inline]
pub unsafe fn EndBufferedAnimation(hbpanimation: isize, fupdatetarget: bool) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn EndBufferedAnimation(hbpanimation : isize, fupdatetarget : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { EndBufferedAnimation(hbpanimation, fupdatetarget.into()).ok() }
}
#[inline]
pub unsafe fn EndBufferedPaint(hbufferedpaint: isize, fupdatetarget: bool) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn EndBufferedPaint(hbufferedpaint : isize, fupdatetarget : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { EndBufferedPaint(hbufferedpaint, fupdatetarget.into()).ok() }
}
#[inline]
pub unsafe fn EndPanningFeedback(hwnd: super::super::Foundation::HWND, fanimateback: bool) -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn EndPanningFeedback(hwnd : super::super::Foundation:: HWND, fanimateback : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EndPanningFeedback(hwnd, fanimateback.into()) }
}
#[inline]
pub unsafe fn EvaluateProximityToPolygon(controlpolygon: &[super::super::Foundation::POINT], phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn EvaluateProximityToPolygon(numvertices : u32, controlpolygon : *const super::super::Foundation:: POINT, phittestinginput : *const TOUCH_HIT_TESTING_INPUT, pproximityeval : *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> windows_core::BOOL);
    unsafe { EvaluateProximityToPolygon(controlpolygon.len().try_into().unwrap(), core::mem::transmute(controlpolygon.as_ptr()), phittestinginput, pproximityeval as _).ok() }
}
#[inline]
pub unsafe fn EvaluateProximityToRect(controlboundingbox: *const super::super::Foundation::RECT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn EvaluateProximityToRect(controlboundingbox : *const super::super::Foundation:: RECT, phittestinginput : *const TOUCH_HIT_TESTING_INPUT, pproximityeval : *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> windows_core::BOOL);
    unsafe { EvaluateProximityToRect(controlboundingbox, phittestinginput, pproximityeval as _).ok() }
}
#[inline]
pub unsafe fn FlatSB_EnableScrollBar(param0: super::super::Foundation::HWND, param1: i32, param2: u32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_EnableScrollBar(param0 : super::super::Foundation:: HWND, param1 : i32, param2 : u32) -> windows_core::BOOL);
    unsafe { FlatSB_EnableScrollBar(param0, param1, param2) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_GetScrollInfo(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut super::WindowsAndMessaging::SCROLLINFO) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollInfo(param0 : super::super::Foundation:: HWND, code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS, param2 : *mut super::WindowsAndMessaging:: SCROLLINFO) -> windows_core::BOOL);
    unsafe { FlatSB_GetScrollInfo(param0, code, param2 as _) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_GetScrollPos(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollPos(param0 : super::super::Foundation:: HWND, code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS) -> i32);
    unsafe { FlatSB_GetScrollPos(param0, code) }
}
#[inline]
pub unsafe fn FlatSB_GetScrollProp(param0: super::super::Foundation::HWND, propindex: WSB_PROP, param2: *mut i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollProp(param0 : super::super::Foundation:: HWND, propindex : WSB_PROP, param2 : *mut i32) -> windows_core::BOOL);
    unsafe { FlatSB_GetScrollProp(param0, propindex, param2 as _) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_GetScrollRange(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut i32, param3: *mut i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_GetScrollRange(param0 : super::super::Foundation:: HWND, code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS, param2 : *mut i32, param3 : *mut i32) -> windows_core::BOOL);
    unsafe { FlatSB_GetScrollRange(param0, code, param2 as _, param3 as _) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_SetScrollInfo(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, psi: *mut super::WindowsAndMessaging::SCROLLINFO, fredraw: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollInfo(param0 : super::super::Foundation:: HWND, code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS, psi : *mut super::WindowsAndMessaging:: SCROLLINFO, fredraw : windows_core::BOOL) -> i32);
    unsafe { FlatSB_SetScrollInfo(param0, code, psi as _, fredraw.into()) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_SetScrollPos(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, pos: i32, fredraw: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollPos(param0 : super::super::Foundation:: HWND, code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS, pos : i32, fredraw : windows_core::BOOL) -> i32);
    unsafe { FlatSB_SetScrollPos(param0, code, pos, fredraw.into()) }
}
#[inline]
pub unsafe fn FlatSB_SetScrollProp(param0: super::super::Foundation::HWND, index: WSB_PROP, newvalue: isize, param3: bool) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollProp(param0 : super::super::Foundation:: HWND, index : u32, newvalue : isize, param3 : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { FlatSB_SetScrollProp(param0, index.0 as _, newvalue, param3.into()) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_SetScrollRange(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, min: i32, max: i32, fredraw: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_SetScrollRange(param0 : super::super::Foundation:: HWND, code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS, min : i32, max : i32, fredraw : windows_core::BOOL) -> i32);
    unsafe { FlatSB_SetScrollRange(param0, code, min, max, fredraw.into()) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_ShowScrollBar(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: bool) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn FlatSB_ShowScrollBar(param0 : super::super::Foundation:: HWND, code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS, param2 : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { FlatSB_ShowScrollBar(param0, code, param2.into()) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintBits(hbufferedpaint: isize, ppbbuffer: *mut *mut super::super::Graphics::Gdi::RGBQUAD, pcxrow: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetBufferedPaintBits(hbufferedpaint : isize, ppbbuffer : *mut *mut super::super::Graphics::Gdi:: RGBQUAD, pcxrow : *mut i32) -> windows_core::HRESULT);
    unsafe { GetBufferedPaintBits(hbufferedpaint, ppbbuffer as _, pcxrow as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC {
    windows_core::link!("uxtheme.dll" "system" fn GetBufferedPaintDC(hbufferedpaint : isize) -> super::super::Graphics::Gdi:: HDC);
    unsafe { GetBufferedPaintDC(hbufferedpaint) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintTargetDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC {
    windows_core::link!("uxtheme.dll" "system" fn GetBufferedPaintTargetDC(hbufferedpaint : isize) -> super::super::Graphics::Gdi:: HDC);
    unsafe { GetBufferedPaintTargetDC(hbufferedpaint) }
}
#[inline]
pub unsafe fn GetBufferedPaintTargetRect(hbufferedpaint: isize) -> windows_core::Result<super::super::Foundation::RECT> {
    windows_core::link!("uxtheme.dll" "system" fn GetBufferedPaintTargetRect(hbufferedpaint : isize, prc : *mut super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetBufferedPaintTargetRect(hbufferedpaint, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetComboBoxInfo(hwndcombo: super::super::Foundation::HWND, pcbi: *mut COMBOBOXINFO) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn GetComboBoxInfo(hwndcombo : super::super::Foundation:: HWND, pcbi : *mut COMBOBOXINFO) -> windows_core::BOOL);
    unsafe { GetComboBoxInfo(hwndcombo, pcbi as _).ok() }
}
#[inline]
pub unsafe fn GetCurrentThemeName(pszthemefilename: &mut [u16], pszcolorbuff: Option<&mut [u16]>, pszsizebuff: Option<&mut [u16]>) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetCurrentThemeName(pszthemefilename : windows_core::PWSTR, cchmaxnamechars : i32, pszcolorbuff : windows_core::PWSTR, cchmaxcolorchars : i32, pszsizebuff : windows_core::PWSTR, cchmaxsizechars : i32) -> windows_core::HRESULT);
    unsafe { GetCurrentThemeName(core::mem::transmute(pszthemefilename.as_ptr()), pszthemefilename.len().try_into().unwrap(), core::mem::transmute(pszcolorbuff.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszcolorbuff.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pszsizebuff.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszsizebuff.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
}
#[inline]
pub unsafe fn GetEffectiveClientRect(hwnd: super::super::Foundation::HWND, lprc: *mut super::super::Foundation::RECT, lpinfo: *const i32) {
    windows_core::link!("comctl32.dll" "system" fn GetEffectiveClientRect(hwnd : super::super::Foundation:: HWND, lprc : *mut super::super::Foundation:: RECT, lpinfo : *const i32));
    unsafe { GetEffectiveClientRect(hwnd, lprc as _, lpinfo) }
}
#[inline]
pub unsafe fn GetListBoxInfo(hwnd: super::super::Foundation::HWND) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetListBoxInfo(hwnd : super::super::Foundation:: HWND) -> u32);
    unsafe { GetListBoxInfo(hwnd) }
}
#[inline]
pub unsafe fn GetMUILanguage() -> u16 {
    windows_core::link!("comctl32.dll" "system" fn GetMUILanguage() -> u16);
    unsafe { GetMUILanguage() }
}
#[inline]
pub unsafe fn GetThemeAnimationProperty(htheme: HTHEME, istoryboardid: i32, itargetid: i32, eproperty: TA_PROPERTY, pvproperty: Option<*mut core::ffi::c_void>, cbsize: u32, pcbsizeout: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeAnimationProperty(htheme : HTHEME, istoryboardid : i32, itargetid : i32, eproperty : TA_PROPERTY, pvproperty : *mut core::ffi::c_void, cbsize : u32, pcbsizeout : *mut u32) -> windows_core::HRESULT);
    unsafe { GetThemeAnimationProperty(htheme, istoryboardid, itargetid, eproperty, pvproperty.unwrap_or(core::mem::zeroed()) as _, cbsize, pcbsizeout as _).ok() }
}
#[inline]
pub unsafe fn GetThemeAnimationTransform(htheme: HTHEME, istoryboardid: i32, itargetid: i32, dwtransformindex: u32, ptransform: Option<*mut TA_TRANSFORM>, cbsize: u32, pcbsizeout: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeAnimationTransform(htheme : HTHEME, istoryboardid : i32, itargetid : i32, dwtransformindex : u32, ptransform : *mut TA_TRANSFORM, cbsize : u32, pcbsizeout : *mut u32) -> windows_core::HRESULT);
    unsafe { GetThemeAnimationTransform(htheme, istoryboardid, itargetid, dwtransformindex, ptransform.unwrap_or(core::mem::zeroed()) as _, cbsize, pcbsizeout as _).ok() }
}
#[inline]
pub unsafe fn GetThemeAppProperties() -> SET_THEME_APP_PROPERTIES_FLAGS {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeAppProperties() -> SET_THEME_APP_PROPERTIES_FLAGS);
    unsafe { GetThemeAppProperties() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeBackgroundContentRect(htheme: HTHEME, hdc: Option<super::super::Graphics::Gdi::HDC>, ipartid: i32, istateid: i32, pboundingrect: *const super::super::Foundation::RECT) -> windows_core::Result<super::super::Foundation::RECT> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeBackgroundContentRect(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, pboundingrect : *const super::super::Foundation:: RECT, pcontentrect : *mut super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeBackgroundContentRect(htheme, hdc.unwrap_or(core::mem::zeroed()) as _, ipartid, istateid, pboundingrect, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeBackgroundExtent(htheme: HTHEME, hdc: Option<super::super::Graphics::Gdi::HDC>, ipartid: i32, istateid: i32, pcontentrect: *const super::super::Foundation::RECT) -> windows_core::Result<super::super::Foundation::RECT> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeBackgroundExtent(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, pcontentrect : *const super::super::Foundation:: RECT, pextentrect : *mut super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeBackgroundExtent(htheme, hdc.unwrap_or(core::mem::zeroed()) as _, ipartid, istateid, pcontentrect, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeBackgroundRegion(htheme: HTHEME, hdc: Option<super::super::Graphics::Gdi::HDC>, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT) -> windows_core::Result<super::super::Graphics::Gdi::HRGN> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeBackgroundRegion(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, prect : *const super::super::Foundation:: RECT, pregion : *mut super::super::Graphics::Gdi:: HRGN) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeBackgroundRegion(htheme, hdc.unwrap_or(core::mem::zeroed()) as _, ipartid, istateid, prect, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeBitmap(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, dwflags: GET_THEME_BITMAP_FLAGS) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeBitmap(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, dwflags : GET_THEME_BITMAP_FLAGS, phbitmap : *mut super::super::Graphics::Gdi:: HBITMAP) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeBitmap(htheme, ipartid, istateid, ipropid.0 as _, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetThemeBool(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeBool(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pfval : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeBool(htheme, ipartid, istateid, ipropid.0 as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetThemeColor(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> windows_core::Result<super::super::Foundation::COLORREF> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeColor(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pcolor : *mut super::super::Foundation:: COLORREF) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeColor(htheme, ipartid, istateid, ipropid.0 as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetThemeDocumentationProperty<P0, P1>(pszthemename: P0, pszpropertyname: P1, pszvaluebuff: &mut [u16]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("uxtheme.dll" "system" fn GetThemeDocumentationProperty(pszthemename : windows_core::PCWSTR, pszpropertyname : windows_core::PCWSTR, pszvaluebuff : windows_core::PWSTR, cchmaxvalchars : i32) -> windows_core::HRESULT);
    unsafe { GetThemeDocumentationProperty(pszthemename.param().abi(), pszpropertyname.param().abi(), core::mem::transmute(pszvaluebuff.as_ptr()), pszvaluebuff.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn GetThemeEnumValue(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> windows_core::Result<i32> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeEnumValue(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pival : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeEnumValue(htheme, ipartid, istateid, ipropid.0 as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetThemeFilename(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pszthemefilename: &mut [u16]) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeFilename(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pszthemefilename : windows_core::PWSTR, cchmaxbuffchars : i32) -> windows_core::HRESULT);
    unsafe { GetThemeFilename(htheme, ipartid, istateid, ipropid.0 as _, core::mem::transmute(pszthemefilename.as_ptr()), pszthemefilename.len().try_into().unwrap()).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeFont(htheme: HTHEME, hdc: Option<super::super::Graphics::Gdi::HDC>, ipartid: i32, istateid: i32, ipropid: i32, pfont: *mut super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeFont(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, ipropid : i32, pfont : *mut super::super::Graphics::Gdi:: LOGFONTW) -> windows_core::HRESULT);
    unsafe { GetThemeFont(htheme, hdc.unwrap_or(core::mem::zeroed()) as _, ipartid, istateid, ipropid, pfont as _).ok() }
}
#[inline]
pub unsafe fn GetThemeInt(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> windows_core::Result<i32> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeInt(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pival : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeInt(htheme, ipartid, istateid, ipropid.0 as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetThemeIntList(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pintlist: *mut INTLIST) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeIntList(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pintlist : *mut INTLIST) -> windows_core::HRESULT);
    unsafe { GetThemeIntList(htheme, ipartid, istateid, ipropid.0 as _, pintlist as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeMargins(htheme: HTHEME, hdc: Option<super::super::Graphics::Gdi::HDC>, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, prc: Option<*const super::super::Foundation::RECT>) -> windows_core::Result<MARGINS> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeMargins(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, ipropid : i32, prc : *const super::super::Foundation:: RECT, pmargins : *mut MARGINS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeMargins(htheme, hdc.unwrap_or(core::mem::zeroed()) as _, ipartid, istateid, ipropid.0 as _, prc.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeMetric(htheme: HTHEME, hdc: Option<super::super::Graphics::Gdi::HDC>, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> windows_core::Result<i32> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeMetric(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, ipropid : i32, pival : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeMetric(htheme, hdc.unwrap_or(core::mem::zeroed()) as _, ipartid, istateid, ipropid.0 as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemePartSize(htheme: HTHEME, hdc: Option<super::super::Graphics::Gdi::HDC>, ipartid: i32, istateid: i32, prc: Option<*const super::super::Foundation::RECT>, esize: THEMESIZE) -> windows_core::Result<super::super::Foundation::SIZE> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemePartSize(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, prc : *const super::super::Foundation:: RECT, esize : THEMESIZE, psz : *mut super::super::Foundation:: SIZE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemePartSize(htheme, hdc.unwrap_or(core::mem::zeroed()) as _, ipartid, istateid, prc.unwrap_or(core::mem::zeroed()) as _, esize, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetThemePosition(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> windows_core::Result<super::super::Foundation::POINT> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemePosition(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, ppoint : *mut super::super::Foundation:: POINT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemePosition(htheme, ipartid, istateid, ipropid.0 as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetThemePropertyOrigin(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: i32) -> windows_core::Result<PROPERTYORIGIN> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemePropertyOrigin(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, porigin : *mut PROPERTYORIGIN) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemePropertyOrigin(htheme, ipartid, istateid, ipropid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetThemeRect(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: i32) -> windows_core::Result<super::super::Foundation::RECT> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeRect(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, prect : *mut super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeRect(htheme, ipartid, istateid, ipropid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetThemeStream(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: i32, ppvstream: *mut *mut core::ffi::c_void, pcbstream: Option<*mut u32>, hinst: Option<super::super::Foundation::HINSTANCE>) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeStream(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, ppvstream : *mut *mut core::ffi::c_void, pcbstream : *mut u32, hinst : super::super::Foundation:: HINSTANCE) -> windows_core::HRESULT);
    unsafe { GetThemeStream(htheme, ipartid, istateid, ipropid, ppvstream as _, pcbstream.unwrap_or(core::mem::zeroed()) as _, hinst.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GetThemeString(htheme: HTHEME, ipartid: i32, istateid: i32, ipropid: i32, pszbuff: &mut [u16]) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeString(htheme : HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pszbuff : windows_core::PWSTR, cchmaxbuffchars : i32) -> windows_core::HRESULT);
    unsafe { GetThemeString(htheme, ipartid, istateid, ipropid, core::mem::transmute(pszbuff.as_ptr()), pszbuff.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn GetThemeSysBool(htheme: Option<HTHEME>, iboolid: THEME_PROPERTY_SYMBOL_ID) -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeSysBool(htheme : HTHEME, iboolid : i32) -> windows_core::BOOL);
    unsafe { GetThemeSysBool(htheme.unwrap_or(core::mem::zeroed()) as _, iboolid.0 as _) }
}
#[inline]
pub unsafe fn GetThemeSysColor(htheme: Option<HTHEME>, icolorid: i32) -> super::super::Foundation::COLORREF {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeSysColor(htheme : HTHEME, icolorid : i32) -> super::super::Foundation:: COLORREF);
    unsafe { GetThemeSysColor(htheme.unwrap_or(core::mem::zeroed()) as _, icolorid) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeSysColorBrush(htheme: Option<HTHEME>, icolorid: THEME_PROPERTY_SYMBOL_ID) -> super::super::Graphics::Gdi::HBRUSH {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeSysColorBrush(htheme : HTHEME, icolorid : i32) -> super::super::Graphics::Gdi:: HBRUSH);
    unsafe { GetThemeSysColorBrush(htheme.unwrap_or(core::mem::zeroed()) as _, icolorid.0 as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeSysFont(htheme: Option<HTHEME>, ifontid: THEME_PROPERTY_SYMBOL_ID, plf: *mut super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeSysFont(htheme : HTHEME, ifontid : i32, plf : *mut super::super::Graphics::Gdi:: LOGFONTW) -> windows_core::HRESULT);
    unsafe { GetThemeSysFont(htheme.unwrap_or(core::mem::zeroed()) as _, ifontid.0 as _, plf as _).ok() }
}
#[inline]
pub unsafe fn GetThemeSysInt(htheme: HTHEME, iintid: THEME_PROPERTY_SYMBOL_ID) -> windows_core::Result<i32> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeSysInt(htheme : HTHEME, iintid : i32, pivalue : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeSysInt(htheme, iintid.0 as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetThemeSysSize(htheme: Option<HTHEME>, isizeid: i32) -> i32 {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeSysSize(htheme : HTHEME, isizeid : i32) -> i32);
    unsafe { GetThemeSysSize(htheme.unwrap_or(core::mem::zeroed()) as _, isizeid) }
}
#[inline]
pub unsafe fn GetThemeSysString(htheme: HTHEME, istringid: THEME_PROPERTY_SYMBOL_ID, pszstringbuff: &mut [u16]) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeSysString(htheme : HTHEME, istringid : i32, pszstringbuff : windows_core::PWSTR, cchmaxstringchars : i32) -> windows_core::HRESULT);
    unsafe { GetThemeSysString(htheme, istringid.0 as _, core::mem::transmute(pszstringbuff.as_ptr()), pszstringbuff.len().try_into().unwrap()).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeTextExtent(htheme: HTHEME, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: &[u16], dwtextflags: super::super::Graphics::Gdi::DRAW_TEXT_FORMAT, pboundingrect: Option<*const super::super::Foundation::RECT>) -> windows_core::Result<super::super::Foundation::RECT> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeTextExtent(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, psztext : windows_core::PCWSTR, cchcharcount : i32, dwtextflags : super::super::Graphics::Gdi:: DRAW_TEXT_FORMAT, pboundingrect : *const super::super::Foundation:: RECT, pextentrect : *mut super::super::Foundation:: RECT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeTextExtent(htheme, hdc, ipartid, istateid, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap(), dwtextflags, pboundingrect.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeTextMetrics(htheme: HTHEME, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeTextMetrics(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, ptm : *mut super::super::Graphics::Gdi:: TEXTMETRICW) -> windows_core::HRESULT);
    unsafe { GetThemeTextMetrics(htheme, hdc, ipartid, istateid, ptm as _).ok() }
}
#[inline]
pub unsafe fn GetThemeTimingFunction(htheme: HTHEME, itimingfunctionid: i32, ptimingfunction: Option<*mut TA_TIMINGFUNCTION>, cbsize: u32, pcbsizeout: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeTimingFunction(htheme : HTHEME, itimingfunctionid : i32, ptimingfunction : *mut TA_TIMINGFUNCTION, cbsize : u32, pcbsizeout : *mut u32) -> windows_core::HRESULT);
    unsafe { GetThemeTimingFunction(htheme, itimingfunctionid, ptimingfunction.unwrap_or(core::mem::zeroed()) as _, cbsize, pcbsizeout as _).ok() }
}
#[inline]
pub unsafe fn GetThemeTransitionDuration(htheme: HTHEME, ipartid: i32, istateidfrom: i32, istateidto: i32, ipropid: i32) -> windows_core::Result<u32> {
    windows_core::link!("uxtheme.dll" "system" fn GetThemeTransitionDuration(htheme : HTHEME, ipartid : i32, istateidfrom : i32, istateidto : i32, ipropid : i32, pdwduration : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThemeTransitionDuration(htheme, ipartid, istateidfrom, istateidto, ipropid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetWindowFeedbackSetting(hwnd: super::super::Foundation::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, psize: *mut u32, config: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetWindowFeedbackSetting(hwnd : super::super::Foundation:: HWND, feedback : FEEDBACK_TYPE, dwflags : u32, psize : *mut u32, config : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetWindowFeedbackSetting(hwnd, feedback, dwflags, psize as _, config.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetWindowTheme(hwnd: super::super::Foundation::HWND) -> HTHEME {
    windows_core::link!("uxtheme.dll" "system" fn GetWindowTheme(hwnd : super::super::Foundation:: HWND) -> HTHEME);
    unsafe { GetWindowTheme(hwnd) }
}
#[inline]
pub unsafe fn HIMAGELIST_QueryInterface(himl: HIMAGELIST, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("comctl32.dll" "system" fn HIMAGELIST_QueryInterface(himl : HIMAGELIST, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HIMAGELIST_QueryInterface(himl, riid, ppv as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HitTestThemeBackground(htheme: HTHEME, hdc: Option<super::super::Graphics::Gdi::HDC>, ipartid: i32, istateid: i32, dwoptions: HIT_TEST_BACKGROUND_OPTIONS, prect: *const super::super::Foundation::RECT, hrgn: Option<super::super::Graphics::Gdi::HRGN>, pttest: super::super::Foundation::POINT) -> windows_core::Result<u16> {
    windows_core::link!("uxtheme.dll" "system" fn HitTestThemeBackground(htheme : HTHEME, hdc : super::super::Graphics::Gdi:: HDC, ipartid : i32, istateid : i32, dwoptions : HIT_TEST_BACKGROUND_OPTIONS, prect : *const super::super::Foundation:: RECT, hrgn : super::super::Graphics::Gdi:: HRGN, pttest : super::super::Foundation:: POINT, pwhittestcode : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HitTestThemeBackground(htheme, hdc.unwrap_or(core::mem::zeroed()) as _, ipartid, istateid, dwoptions, prect, hrgn.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pttest), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_Add(himl: HIMAGELIST, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: Option<super::super::Graphics::Gdi::HBITMAP>) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Add(himl : HIMAGELIST, hbmimage : super::super::Graphics::Gdi:: HBITMAP, hbmmask : super::super::Graphics::Gdi:: HBITMAP) -> i32);
    unsafe { ImageList_Add(himl, hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_AddMasked(himl: HIMAGELIST, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: super::super::Foundation::COLORREF) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_AddMasked(himl : HIMAGELIST, hbmimage : super::super::Graphics::Gdi:: HBITMAP, crmask : super::super::Foundation:: COLORREF) -> i32);
    unsafe { ImageList_AddMasked(himl, hbmimage, crmask) }
}
#[inline]
pub unsafe fn ImageList_BeginDrag(himltrack: HIMAGELIST, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_BeginDrag(himltrack : HIMAGELIST, itrack : i32, dxhotspot : i32, dyhotspot : i32) -> windows_core::BOOL);
    unsafe { ImageList_BeginDrag(himltrack, itrack, dxhotspot, dyhotspot) }
}
#[inline]
pub unsafe fn ImageList_CoCreateInstance<P1, T>(rclsid: *const windows_core::GUID, punkouter: P1) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { ImageList_CoCreateInstance(rclsid, punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn ImageList_Copy(himldst: HIMAGELIST, idst: i32, himlsrc: HIMAGELIST, isrc: i32, uflags: IMAGE_LIST_COPY_FLAGS) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Copy(himldst : HIMAGELIST, idst : i32, himlsrc : HIMAGELIST, isrc : i32, uflags : IMAGE_LIST_COPY_FLAGS) -> windows_core::BOOL);
    unsafe { ImageList_Copy(himldst, idst, himlsrc, isrc, uflags) }
}
#[inline]
pub unsafe fn ImageList_Create(cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> HIMAGELIST {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Create(cx : i32, cy : i32, flags : IMAGELIST_CREATION_FLAGS, cinitial : i32, cgrow : i32) -> HIMAGELIST);
    unsafe { ImageList_Create(cx, cy, flags, cinitial, cgrow) }
}
#[inline]
pub unsafe fn ImageList_Destroy(himl: Option<HIMAGELIST>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Destroy(himl : HIMAGELIST) -> windows_core::BOOL);
    unsafe { ImageList_Destroy(himl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ImageList_DragEnter(hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DragEnter(hwndlock : super::super::Foundation:: HWND, x : i32, y : i32) -> windows_core::BOOL);
    unsafe { ImageList_DragEnter(hwndlock, x, y) }
}
#[inline]
pub unsafe fn ImageList_DragLeave(hwndlock: super::super::Foundation::HWND) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DragLeave(hwndlock : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { ImageList_DragLeave(hwndlock) }
}
#[inline]
pub unsafe fn ImageList_DragMove(x: i32, y: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DragMove(x : i32, y : i32) -> windows_core::BOOL);
    unsafe { ImageList_DragMove(x, y) }
}
#[inline]
pub unsafe fn ImageList_DragShowNolock(fshow: bool) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DragShowNolock(fshow : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ImageList_DragShowNolock(fshow.into()) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_Draw(himl: HIMAGELIST, i: i32, hdcdst: super::super::Graphics::Gdi::HDC, x: i32, y: i32, fstyle: IMAGE_LIST_DRAW_STYLE) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Draw(himl : HIMAGELIST, i : i32, hdcdst : super::super::Graphics::Gdi:: HDC, x : i32, y : i32, fstyle : IMAGE_LIST_DRAW_STYLE) -> windows_core::BOOL);
    unsafe { ImageList_Draw(himl, i, hdcdst, x, y, fstyle) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_DrawEx(himl: HIMAGELIST, i: i32, hdcdst: super::super::Graphics::Gdi::HDC, x: i32, y: i32, dx: i32, dy: i32, rgbbk: super::super::Foundation::COLORREF, rgbfg: super::super::Foundation::COLORREF, fstyle: IMAGE_LIST_DRAW_STYLE) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DrawEx(himl : HIMAGELIST, i : i32, hdcdst : super::super::Graphics::Gdi:: HDC, x : i32, y : i32, dx : i32, dy : i32, rgbbk : super::super::Foundation:: COLORREF, rgbfg : super::super::Foundation:: COLORREF, fstyle : IMAGE_LIST_DRAW_STYLE) -> windows_core::BOOL);
    unsafe { ImageList_DrawEx(himl, i, hdcdst, x, y, dx, dy, rgbbk, rgbfg, fstyle) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_DrawIndirect(pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_DrawIndirect(pimldp : *const IMAGELISTDRAWPARAMS) -> windows_core::BOOL);
    unsafe { ImageList_DrawIndirect(pimldp) }
}
#[inline]
pub unsafe fn ImageList_Duplicate(himl: HIMAGELIST) -> HIMAGELIST {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Duplicate(himl : HIMAGELIST) -> HIMAGELIST);
    unsafe { ImageList_Duplicate(himl) }
}
#[inline]
pub unsafe fn ImageList_EndDrag() {
    windows_core::link!("comctl32.dll" "system" fn ImageList_EndDrag());
    unsafe { ImageList_EndDrag() }
}
#[inline]
pub unsafe fn ImageList_GetBkColor(himl: HIMAGELIST) -> super::super::Foundation::COLORREF {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetBkColor(himl : HIMAGELIST) -> super::super::Foundation:: COLORREF);
    unsafe { ImageList_GetBkColor(himl) }
}
#[inline]
pub unsafe fn ImageList_GetDragImage(ppt: Option<*mut super::super::Foundation::POINT>, ppthotspot: Option<*mut super::super::Foundation::POINT>) -> HIMAGELIST {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetDragImage(ppt : *mut super::super::Foundation:: POINT, ppthotspot : *mut super::super::Foundation:: POINT) -> HIMAGELIST);
    unsafe { ImageList_GetDragImage(ppt.unwrap_or(core::mem::zeroed()) as _, ppthotspot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_GetIcon(himl: HIMAGELIST, i: i32, flags: IMAGE_LIST_DRAW_STYLE) -> super::WindowsAndMessaging::HICON {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetIcon(himl : HIMAGELIST, i : i32, flags : IMAGE_LIST_DRAW_STYLE) -> super::WindowsAndMessaging:: HICON);
    unsafe { ImageList_GetIcon(himl, i, flags) }
}
#[inline]
pub unsafe fn ImageList_GetIconSize(himl: HIMAGELIST, cx: Option<*mut i32>, cy: Option<*mut i32>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetIconSize(himl : HIMAGELIST, cx : *mut i32, cy : *mut i32) -> windows_core::BOOL);
    unsafe { ImageList_GetIconSize(himl, cx.unwrap_or(core::mem::zeroed()) as _, cy.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ImageList_GetImageCount(himl: HIMAGELIST) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetImageCount(himl : HIMAGELIST) -> i32);
    unsafe { ImageList_GetImageCount(himl) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_GetImageInfo(himl: HIMAGELIST, i: i32, pimageinfo: *mut IMAGEINFO) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_GetImageInfo(himl : HIMAGELIST, i : i32, pimageinfo : *mut IMAGEINFO) -> windows_core::BOOL);
    unsafe { ImageList_GetImageInfo(himl, i, pimageinfo as _) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_LoadImageA<P1>(hi: super::super::Foundation::HINSTANCE, lpbmp: P1, cx: i32, cgrow: i32, crmask: super::super::Foundation::COLORREF, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_LoadImageA(hi : super::super::Foundation:: HINSTANCE, lpbmp : windows_core::PCSTR, cx : i32, cgrow : i32, crmask : super::super::Foundation:: COLORREF, utype : u32, uflags : super::WindowsAndMessaging:: IMAGE_FLAGS) -> HIMAGELIST);
    unsafe { ImageList_LoadImageA(hi, lpbmp.param().abi(), cx, cgrow, crmask, utype, uflags) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_LoadImageW<P1>(hi: super::super::Foundation::HINSTANCE, lpbmp: P1, cx: i32, cgrow: i32, crmask: super::super::Foundation::COLORREF, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_LoadImageW(hi : super::super::Foundation:: HINSTANCE, lpbmp : windows_core::PCWSTR, cx : i32, cgrow : i32, crmask : super::super::Foundation:: COLORREF, utype : u32, uflags : super::WindowsAndMessaging:: IMAGE_FLAGS) -> HIMAGELIST);
    unsafe { ImageList_LoadImageW(hi, lpbmp.param().abi(), cx, cgrow, crmask, utype, uflags) }
}
#[inline]
pub unsafe fn ImageList_Merge(himl1: HIMAGELIST, i1: i32, himl2: HIMAGELIST, i2: i32, dx: i32, dy: i32) -> HIMAGELIST {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Merge(himl1 : HIMAGELIST, i1 : i32, himl2 : HIMAGELIST, i2 : i32, dx : i32, dy : i32) -> HIMAGELIST);
    unsafe { ImageList_Merge(himl1, i1, himl2, i2, dx, dy) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_Read<P0>(pstm: P0) -> HIMAGELIST
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_Read(pstm : * mut core::ffi::c_void) -> HIMAGELIST);
    unsafe { ImageList_Read(pstm.param().abi()) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_ReadEx<P1>(dwflags: u32, pstm: P1, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_ReadEx(dwflags : u32, pstm : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ImageList_ReadEx(dwflags, pstm.param().abi(), riid, ppv as _).ok() }
}
#[inline]
pub unsafe fn ImageList_Remove(himl: HIMAGELIST, i: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Remove(himl : HIMAGELIST, i : i32) -> windows_core::BOOL);
    unsafe { ImageList_Remove(himl, i) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_Replace(himl: HIMAGELIST, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: Option<super::super::Graphics::Gdi::HBITMAP>) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_Replace(himl : HIMAGELIST, i : i32, hbmimage : super::super::Graphics::Gdi:: HBITMAP, hbmmask : super::super::Graphics::Gdi:: HBITMAP) -> windows_core::BOOL);
    unsafe { ImageList_Replace(himl, i, hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_ReplaceIcon(himl: HIMAGELIST, i: i32, hicon: super::WindowsAndMessaging::HICON) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn ImageList_ReplaceIcon(himl : HIMAGELIST, i : i32, hicon : super::WindowsAndMessaging:: HICON) -> i32);
    unsafe { ImageList_ReplaceIcon(himl, i, hicon) }
}
#[inline]
pub unsafe fn ImageList_SetBkColor(himl: HIMAGELIST, clrbk: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetBkColor(himl : HIMAGELIST, clrbk : super::super::Foundation:: COLORREF) -> super::super::Foundation:: COLORREF);
    unsafe { ImageList_SetBkColor(himl, clrbk) }
}
#[inline]
pub unsafe fn ImageList_SetDragCursorImage(himldrag: HIMAGELIST, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetDragCursorImage(himldrag : HIMAGELIST, idrag : i32, dxhotspot : i32, dyhotspot : i32) -> windows_core::BOOL);
    unsafe { ImageList_SetDragCursorImage(himldrag, idrag, dxhotspot, dyhotspot) }
}
#[inline]
pub unsafe fn ImageList_SetIconSize(himl: HIMAGELIST, cx: i32, cy: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetIconSize(himl : HIMAGELIST, cx : i32, cy : i32) -> windows_core::BOOL);
    unsafe { ImageList_SetIconSize(himl, cx, cy) }
}
#[inline]
pub unsafe fn ImageList_SetImageCount(himl: HIMAGELIST, unewcount: u32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetImageCount(himl : HIMAGELIST, unewcount : u32) -> windows_core::BOOL);
    unsafe { ImageList_SetImageCount(himl, unewcount) }
}
#[inline]
pub unsafe fn ImageList_SetOverlayImage(himl: HIMAGELIST, iimage: i32, ioverlay: i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ImageList_SetOverlayImage(himl : HIMAGELIST, iimage : i32, ioverlay : i32) -> windows_core::BOOL);
    unsafe { ImageList_SetOverlayImage(himl, iimage, ioverlay) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_Write<P1>(himl: HIMAGELIST, pstm: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_Write(himl : HIMAGELIST, pstm : * mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ImageList_Write(himl, pstm.param().abi()) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_WriteEx<P2>(himl: HIMAGELIST, dwflags: IMAGE_LIST_WRITE_STREAM_FLAGS, pstm: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_WriteEx(himl : HIMAGELIST, dwflags : IMAGE_LIST_WRITE_STREAM_FLAGS, pstm : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ImageList_WriteEx(himl, dwflags, pstm.param().abi()).ok() }
}
#[inline]
pub unsafe fn InitCommonControls() {
    windows_core::link!("comctl32.dll" "system" fn InitCommonControls());
    unsafe { InitCommonControls() }
}
#[inline]
pub unsafe fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn InitCommonControlsEx(picce : *const INITCOMMONCONTROLSEX) -> windows_core::BOOL);
    unsafe { InitCommonControlsEx(picce) }
}
#[inline]
pub unsafe fn InitMUILanguage(uilang: u16) {
    windows_core::link!("comctl32.dll" "system" fn InitMUILanguage(uilang : u16));
    unsafe { InitMUILanguage(uilang) }
}
#[inline]
pub unsafe fn InitializeFlatSB(param0: super::super::Foundation::HWND) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn InitializeFlatSB(param0 : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { InitializeFlatSB(param0) }
}
#[inline]
pub unsafe fn IsAppThemed() -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn IsAppThemed() -> windows_core::BOOL);
    unsafe { IsAppThemed() }
}
#[inline]
pub unsafe fn IsCharLowerW(ch: u16) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn IsCharLowerW(ch : u16) -> windows_core::BOOL);
    unsafe { IsCharLowerW(ch).ok() }
}
#[inline]
pub unsafe fn IsCompositionActive() -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn IsCompositionActive() -> windows_core::BOOL);
    unsafe { IsCompositionActive() }
}
#[inline]
pub unsafe fn IsDlgButtonChecked(hdlg: super::super::Foundation::HWND, nidbutton: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn IsDlgButtonChecked(hdlg : super::super::Foundation:: HWND, nidbutton : i32) -> u32);
    unsafe { IsDlgButtonChecked(hdlg, nidbutton) }
}
#[inline]
pub unsafe fn IsThemeActive() -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn IsThemeActive() -> windows_core::BOOL);
    unsafe { IsThemeActive() }
}
#[inline]
pub unsafe fn IsThemeBackgroundPartiallyTransparent(htheme: HTHEME, ipartid: i32, istateid: i32) -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn IsThemeBackgroundPartiallyTransparent(htheme : HTHEME, ipartid : i32, istateid : i32) -> windows_core::BOOL);
    unsafe { IsThemeBackgroundPartiallyTransparent(htheme, ipartid, istateid) }
}
#[inline]
pub unsafe fn IsThemeDialogTextureEnabled(hwnd: super::super::Foundation::HWND) -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn IsThemeDialogTextureEnabled(hwnd : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { IsThemeDialogTextureEnabled(hwnd) }
}
#[inline]
pub unsafe fn IsThemePartDefined(htheme: HTHEME, ipartid: i32, istateid: i32) -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn IsThemePartDefined(htheme : HTHEME, ipartid : i32, istateid : i32) -> windows_core::BOOL);
    unsafe { IsThemePartDefined(htheme, ipartid, istateid) }
}
#[inline]
pub unsafe fn LBItemFromPt(hlb: super::super::Foundation::HWND, pt: super::super::Foundation::POINT, bautoscroll: bool) -> i32 {
    windows_core::link!("comctl32.dll" "system" fn LBItemFromPt(hlb : super::super::Foundation:: HWND, pt : super::super::Foundation:: POINT, bautoscroll : windows_core::BOOL) -> i32);
    unsafe { LBItemFromPt(hlb, core::mem::transmute(pt), bautoscroll.into()) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn LoadIconMetric<P1>(hinst: Option<super::super::Foundation::HINSTANCE>, pszname: P1, lims: _LI_METRIC) -> windows_core::Result<super::WindowsAndMessaging::HICON>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn LoadIconMetric(hinst : super::super::Foundation:: HINSTANCE, pszname : windows_core::PCWSTR, lims : _LI_METRIC, phico : *mut super::WindowsAndMessaging:: HICON) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        LoadIconMetric(hinst.unwrap_or(core::mem::zeroed()) as _, pszname.param().abi(), lims, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn LoadIconWithScaleDown<P1>(hinst: Option<super::super::Foundation::HINSTANCE>, pszname: P1, cx: i32, cy: i32) -> windows_core::Result<super::WindowsAndMessaging::HICON>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn LoadIconWithScaleDown(hinst : super::super::Foundation:: HINSTANCE, pszname : windows_core::PCWSTR, cx : i32, cy : i32, phico : *mut super::WindowsAndMessaging:: HICON) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        LoadIconWithScaleDown(hinst.unwrap_or(core::mem::zeroed()) as _, pszname.param().abi(), cx, cy, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MakeDragList(hlb: super::super::Foundation::HWND) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn MakeDragList(hlb : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { MakeDragList(hlb) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn MenuHelp(umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, hmainmenu: super::WindowsAndMessaging::HMENU, hinst: super::super::Foundation::HINSTANCE, hwndstatus: super::super::Foundation::HWND, lpwids: *const u32) {
    windows_core::link!("comctl32.dll" "system" fn MenuHelp(umsg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, hmainmenu : super::WindowsAndMessaging:: HMENU, hinst : super::super::Foundation:: HINSTANCE, hwndstatus : super::super::Foundation:: HWND, lpwids : *const u32));
    unsafe { MenuHelp(umsg, wparam, lparam, hmainmenu, hinst, hwndstatus, lpwids) }
}
#[inline]
pub unsafe fn OpenThemeData<P1>(hwnd: Option<super::super::Foundation::HWND>, pszclasslist: P1) -> HTHEME
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("uxtheme.dll" "system" fn OpenThemeData(hwnd : super::super::Foundation:: HWND, pszclasslist : windows_core::PCWSTR) -> HTHEME);
    unsafe { OpenThemeData(hwnd.unwrap_or(core::mem::zeroed()) as _, pszclasslist.param().abi()) }
}
#[inline]
pub unsafe fn OpenThemeDataEx<P1>(hwnd: Option<super::super::Foundation::HWND>, pszclasslist: P1, dwflags: OPEN_THEME_DATA_FLAGS) -> HTHEME
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("uxtheme.dll" "system" fn OpenThemeDataEx(hwnd : super::super::Foundation:: HWND, pszclasslist : windows_core::PCWSTR, dwflags : OPEN_THEME_DATA_FLAGS) -> HTHEME);
    unsafe { OpenThemeDataEx(hwnd.unwrap_or(core::mem::zeroed()) as _, pszclasslist.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn PackTouchHitTestingProximityEvaluation(phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::LRESULT {
    windows_core::link!("user32.dll" "system" fn PackTouchHitTestingProximityEvaluation(phittestinginput : *const TOUCH_HIT_TESTING_INPUT, pproximityeval : *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation:: LRESULT);
    unsafe { PackTouchHitTestingProximityEvaluation(phittestinginput, pproximityeval) }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn PropertySheetA(param0: *mut PROPSHEETHEADERA_V2) -> isize {
    windows_core::link!("comctl32.dll" "system" fn PropertySheetA(param0 : *mut PROPSHEETHEADERA_V2) -> isize);
    unsafe { PropertySheetA(param0 as _) }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn PropertySheetW(param0: *mut PROPSHEETHEADERW_V2) -> isize {
    windows_core::link!("comctl32.dll" "system" fn PropertySheetW(param0 : *mut PROPSHEETHEADERW_V2) -> isize);
    unsafe { PropertySheetW(param0 as _) }
}
#[inline]
pub unsafe fn RegisterPointerDeviceNotifications(window: super::super::Foundation::HWND, notifyrange: bool) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn RegisterPointerDeviceNotifications(window : super::super::Foundation:: HWND, notifyrange : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RegisterPointerDeviceNotifications(window, notifyrange.into()).ok() }
}
#[inline]
pub unsafe fn RegisterTouchHitTestingWindow(hwnd: super::super::Foundation::HWND, value: u32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn RegisterTouchHitTestingWindow(hwnd : super::super::Foundation:: HWND, value : u32) -> windows_core::BOOL);
    unsafe { RegisterTouchHitTestingWindow(hwnd, value).ok() }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetScrollInfo(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, lpsi: *const super::WindowsAndMessaging::SCROLLINFO, redraw: bool) -> i32 {
    windows_core::link!("user32.dll" "system" fn SetScrollInfo(hwnd : super::super::Foundation:: HWND, nbar : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS, lpsi : *const super::WindowsAndMessaging:: SCROLLINFO, redraw : windows_core::BOOL) -> i32);
    unsafe { SetScrollInfo(hwnd, nbar, lpsi, redraw.into()) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetScrollPos(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, npos: i32, bredraw: bool) -> i32 {
    windows_core::link!("user32.dll" "system" fn SetScrollPos(hwnd : super::super::Foundation:: HWND, nbar : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS, npos : i32, bredraw : windows_core::BOOL) -> i32);
    unsafe { SetScrollPos(hwnd, nbar, npos, bredraw.into()) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetScrollRange(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, nminpos: i32, nmaxpos: i32, bredraw: bool) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SetScrollRange(hwnd : super::super::Foundation:: HWND, nbar : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS, nminpos : i32, nmaxpos : i32, bredraw : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetScrollRange(hwnd, nbar, nminpos, nmaxpos, bredraw.into()).ok() }
}
#[inline]
pub unsafe fn SetThemeAppProperties(dwflags: SET_THEME_APP_PROPERTIES_FLAGS) {
    windows_core::link!("uxtheme.dll" "system" fn SetThemeAppProperties(dwflags : SET_THEME_APP_PROPERTIES_FLAGS));
    unsafe { SetThemeAppProperties(dwflags) }
}
#[inline]
pub unsafe fn SetWindowFeedbackSetting(hwnd: super::super::Foundation::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, size: u32, configuration: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetWindowFeedbackSetting(hwnd : super::super::Foundation:: HWND, feedback : FEEDBACK_TYPE, dwflags : u32, size : u32, configuration : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetWindowFeedbackSetting(hwnd, feedback, dwflags, size, configuration.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetWindowTheme<P1, P2>(hwnd: super::super::Foundation::HWND, pszsubappname: P1, pszsubidlist: P2) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("uxtheme.dll" "system" fn SetWindowTheme(hwnd : super::super::Foundation:: HWND, pszsubappname : windows_core::PCWSTR, pszsubidlist : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SetWindowTheme(hwnd, pszsubappname.param().abi(), pszsubidlist.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetWindowThemeAttribute(hwnd: super::super::Foundation::HWND, eattribute: WINDOWTHEMEATTRIBUTETYPE, pvattribute: *const core::ffi::c_void, cbattribute: u32) -> windows_core::Result<()> {
    windows_core::link!("uxtheme.dll" "system" fn SetWindowThemeAttribute(hwnd : super::super::Foundation:: HWND, eattribute : WINDOWTHEMEATTRIBUTETYPE, pvattribute : *const core::ffi::c_void, cbattribute : u32) -> windows_core::HRESULT);
    unsafe { SetWindowThemeAttribute(hwnd, eattribute, pvattribute, cbattribute).ok() }
}
#[inline]
pub unsafe fn ShowHideMenuCtl(hwnd: super::super::Foundation::HWND, uflags: usize, lpinfo: *const i32) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn ShowHideMenuCtl(hwnd : super::super::Foundation:: HWND, uflags : usize, lpinfo : *const i32) -> windows_core::BOOL);
    unsafe { ShowHideMenuCtl(hwnd, uflags, lpinfo) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ShowScrollBar(hwnd: super::super::Foundation::HWND, wbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, bshow: bool) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn ShowScrollBar(hwnd : super::super::Foundation:: HWND, wbar : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS, bshow : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ShowScrollBar(hwnd, wbar, bshow.into()).ok() }
}
#[inline]
pub unsafe fn Str_SetPtrW<P1>(ppsz: *mut windows_core::PWSTR, psz: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn Str_SetPtrW(ppsz : *mut windows_core::PWSTR, psz : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { Str_SetPtrW(ppsz as _, psz.param().abi()) }
}
#[inline]
pub unsafe fn TaskDialog<P2, P3, P4, P6>(hwndowner: Option<super::super::Foundation::HWND>, hinstance: Option<super::super::Foundation::HINSTANCE>, pszwindowtitle: P2, pszmaininstruction: P3, pszcontent: P4, dwcommonbuttons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon: P6, pnbutton: Option<*mut i32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comctl32.dll" "system" fn TaskDialog(hwndowner : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszwindowtitle : windows_core::PCWSTR, pszmaininstruction : windows_core::PCWSTR, pszcontent : windows_core::PCWSTR, dwcommonbuttons : TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon : windows_core::PCWSTR, pnbutton : *mut i32) -> windows_core::HRESULT);
    unsafe { TaskDialog(hwndowner.unwrap_or(core::mem::zeroed()) as _, hinstance.unwrap_or(core::mem::zeroed()) as _, pszwindowtitle.param().abi(), pszmaininstruction.param().abi(), pszcontent.param().abi(), dwcommonbuttons, pszicon.param().abi(), pnbutton.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn TaskDialogIndirect(ptaskconfig: *const TASKDIALOGCONFIG, pnbutton: Option<*mut i32>, pnradiobutton: Option<*mut i32>, pfverificationflagchecked: Option<*mut windows_core::BOOL>) -> windows_core::Result<()> {
    windows_core::link!("comctl32.dll" "system" fn TaskDialogIndirect(ptaskconfig : *const TASKDIALOGCONFIG, pnbutton : *mut i32, pnradiobutton : *mut i32, pfverificationflagchecked : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { TaskDialogIndirect(ptaskconfig, pnbutton.unwrap_or(core::mem::zeroed()) as _, pnradiobutton.unwrap_or(core::mem::zeroed()) as _, pfverificationflagchecked.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn UninitializeFlatSB(param0: super::super::Foundation::HWND) -> windows_core::Result<()> {
    windows_core::link!("comctl32.dll" "system" fn UninitializeFlatSB(param0 : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    unsafe { UninitializeFlatSB(param0).ok() }
}
#[inline]
pub unsafe fn UpdatePanningFeedback(hwnd: super::super::Foundation::HWND, ltotaloverpanoffsetx: i32, ltotaloverpanoffsety: i32, fininertia: bool) -> windows_core::BOOL {
    windows_core::link!("uxtheme.dll" "system" fn UpdatePanningFeedback(hwnd : super::super::Foundation:: HWND, ltotaloverpanoffsetx : i32, ltotaloverpanoffsety : i32, fininertia : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { UpdatePanningFeedback(hwnd, ltotaloverpanoffsetx, ltotaloverpanoffsety, fininertia.into()) }
}
pub const ABS_DOWNDISABLED: ARROWBTNSTATES = ARROWBTNSTATES(8i32);
pub const ABS_DOWNHOT: ARROWBTNSTATES = ARROWBTNSTATES(6i32);
pub const ABS_DOWNHOVER: ARROWBTNSTATES = ARROWBTNSTATES(18i32);
pub const ABS_DOWNNORMAL: ARROWBTNSTATES = ARROWBTNSTATES(5i32);
pub const ABS_DOWNPRESSED: ARROWBTNSTATES = ARROWBTNSTATES(7i32);
pub const ABS_LEFTDISABLED: ARROWBTNSTATES = ARROWBTNSTATES(12i32);
pub const ABS_LEFTHOT: ARROWBTNSTATES = ARROWBTNSTATES(10i32);
pub const ABS_LEFTHOVER: ARROWBTNSTATES = ARROWBTNSTATES(19i32);
pub const ABS_LEFTNORMAL: ARROWBTNSTATES = ARROWBTNSTATES(9i32);
pub const ABS_LEFTPRESSED: ARROWBTNSTATES = ARROWBTNSTATES(11i32);
pub const ABS_RIGHTDISABLED: ARROWBTNSTATES = ARROWBTNSTATES(16i32);
pub const ABS_RIGHTHOT: ARROWBTNSTATES = ARROWBTNSTATES(14i32);
pub const ABS_RIGHTHOVER: ARROWBTNSTATES = ARROWBTNSTATES(20i32);
pub const ABS_RIGHTNORMAL: ARROWBTNSTATES = ARROWBTNSTATES(13i32);
pub const ABS_RIGHTPRESSED: ARROWBTNSTATES = ARROWBTNSTATES(15i32);
pub const ABS_UPDISABLED: ARROWBTNSTATES = ARROWBTNSTATES(4i32);
pub const ABS_UPHOT: ARROWBTNSTATES = ARROWBTNSTATES(2i32);
pub const ABS_UPHOVER: ARROWBTNSTATES = ARROWBTNSTATES(17i32);
pub const ABS_UPNORMAL: ARROWBTNSTATES = ARROWBTNSTATES(1i32);
pub const ABS_UPPRESSED: ARROWBTNSTATES = ARROWBTNSTATES(3i32);
pub const ACM_ISPLAYING: u32 = 1128u32;
pub const ACM_OPEN: u32 = 1127u32;
pub const ACM_OPENA: u32 = 1124u32;
pub const ACM_OPENW: u32 = 1127u32;
pub const ACM_PLAY: u32 = 1125u32;
pub const ACM_STOP: u32 = 1126u32;
pub const ACN_START: u32 = 1u32;
pub const ACN_STOP: u32 = 2u32;
pub const ACS_AUTOPLAY: u32 = 4u32;
pub const ACS_CENTER: u32 = 1u32;
pub const ACS_TIMER: u32 = 8u32;
pub const ACS_TRANSPARENT: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AEROWIZARDPARTS(pub i32);
pub const ALLOW_CONTROLS: SET_THEME_APP_PROPERTIES_FLAGS = SET_THEME_APP_PROPERTIES_FLAGS(2u32);
pub const ALLOW_NONCLIENT: SET_THEME_APP_PROPERTIES_FLAGS = SET_THEME_APP_PROPERTIES_FLAGS(1u32);
pub const ALLOW_WEBCONTENT: SET_THEME_APP_PROPERTIES_FLAGS = SET_THEME_APP_PROPERTIES_FLAGS(4u32);
pub const ANIMATE_CLASS: windows_core::PCWSTR = windows_core::w!("SysAnimate32");
pub const ANIMATE_CLASSA: windows_core::PCSTR = windows_core::s!("SysAnimate32");
pub const ANIMATE_CLASSW: windows_core::PCWSTR = windows_core::w!("SysAnimate32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ARROWBTNSTATES(pub i32);
pub const AW_BUTTON: AEROWIZARDPARTS = AEROWIZARDPARTS(5i32);
pub const AW_COMMANDAREA: AEROWIZARDPARTS = AEROWIZARDPARTS(4i32);
pub const AW_CONTENTAREA: AEROWIZARDPARTS = AEROWIZARDPARTS(3i32);
pub const AW_HEADERAREA: AEROWIZARDPARTS = AEROWIZARDPARTS(2i32);
pub const AW_S_CONTENTAREA_NOMARGIN: CONTENTAREASTATES = CONTENTAREASTATES(1i32);
pub const AW_S_HEADERAREA_NOMARGIN: HEADERAREASTATES = HEADERAREASTATES(1i32);
pub const AW_S_TITLEBAR_ACTIVE: TITLEBARSTATES = TITLEBARSTATES(1i32);
pub const AW_S_TITLEBAR_INACTIVE: TITLEBARSTATES = TITLEBARSTATES(2i32);
pub const AW_TITLEBAR: AEROWIZARDPARTS = AEROWIZARDPARTS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BACKGROUNDSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BACKGROUNDWITHBORDERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BALLOONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BALLOONSTEMSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BARBACKGROUNDSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BARITEMSTATES(pub i32);
pub const BCM_FIRST: u32 = 5632u32;
pub const BCM_GETIDEALSIZE: u32 = 5633u32;
pub const BCM_GETIMAGELIST: u32 = 5635u32;
pub const BCM_GETNOTE: u32 = 5642u32;
pub const BCM_GETNOTELENGTH: u32 = 5643u32;
pub const BCM_GETSPLITINFO: u32 = 5640u32;
pub const BCM_GETTEXTMARGIN: u32 = 5637u32;
pub const BCM_SETDROPDOWNSTATE: u32 = 5638u32;
pub const BCM_SETIMAGELIST: u32 = 5634u32;
pub const BCM_SETNOTE: u32 = 5641u32;
pub const BCM_SETSHIELD: u32 = 5644u32;
pub const BCM_SETSPLITINFO: u32 = 5639u32;
pub const BCM_SETTEXTMARGIN: u32 = 5636u32;
pub const BCN_DROPDOWN: u32 = 4294966048u32;
pub const BCN_FIRST: u32 = 4294966046u32;
pub const BCN_HOTITEMCHANGE: u32 = 4294966047u32;
pub const BCN_LAST: u32 = 4294965946u32;
pub const BCSIF_GLYPH: u32 = 1u32;
pub const BCSIF_IMAGE: u32 = 2u32;
pub const BCSIF_SIZE: u32 = 8u32;
pub const BCSIF_STYLE: u32 = 4u32;
pub const BCSS_ALIGNLEFT: u32 = 4u32;
pub const BCSS_IMAGE: u32 = 8u32;
pub const BCSS_NOSPLIT: u32 = 1u32;
pub const BCSS_STRETCH: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BGTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BODYSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BORDERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BORDERTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BORDER_HSCROLLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BORDER_HVSCROLLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BORDER_NOSCROLLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BORDER_VSCROLLSTATES(pub i32);
pub const BPAS_CUBIC: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(2i32);
pub const BPAS_LINEAR: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(1i32);
pub const BPAS_NONE: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(0i32);
pub const BPAS_SINE: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(3i32);
pub const BPBF_COMPATIBLEBITMAP: BP_BUFFERFORMAT = BP_BUFFERFORMAT(0i32);
pub const BPBF_DIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(1i32);
pub const BPBF_TOPDOWNDIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(2i32);
pub const BPBF_TOPDOWNMONODIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(3i32);
pub const BPPF_ERASE: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(1u32);
pub const BPPF_NOCLIP: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(2u32);
pub const BPPF_NONCLIENT: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BP_ANIMATIONPARAMS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub style: BP_ANIMATIONSTYLE,
    pub dwDuration: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BP_ANIMATIONSTYLE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BP_BUFFERFORMAT(pub i32);
pub const BP_CHECKBOX: BUTTONPARTS = BUTTONPARTS(3i32);
pub const BP_CHECKBOX_HCDISABLED: BUTTONPARTS = BUTTONPARTS(9i32);
pub const BP_COMMANDLINK: BUTTONPARTS = BUTTONPARTS(6i32);
pub const BP_COMMANDLINKGLYPH: BUTTONPARTS = BUTTONPARTS(7i32);
pub const BP_GROUPBOX: BUTTONPARTS = BUTTONPARTS(4i32);
pub const BP_GROUPBOX_HCDISABLED: BUTTONPARTS = BUTTONPARTS(10i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BP_PAINTPARAMS {
    pub cbSize: u32,
    pub dwFlags: BP_PAINTPARAMS_FLAGS,
    pub prcExclude: *const super::super::Foundation::RECT,
    pub pBlendFunction: *const super::super::Graphics::Gdi::BLENDFUNCTION,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for BP_PAINTPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BP_PAINTPARAMS_FLAGS(pub u32);
impl BP_PAINTPARAMS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for BP_PAINTPARAMS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for BP_PAINTPARAMS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const BP_PUSHBUTTON: BUTTONPARTS = BUTTONPARTS(1i32);
pub const BP_PUSHBUTTONDROPDOWN: BUTTONPARTS = BUTTONPARTS(11i32);
pub const BP_RADIOBUTTON: BUTTONPARTS = BUTTONPARTS(2i32);
pub const BP_RADIOBUTTON_HCDISABLED: BUTTONPARTS = BUTTONPARTS(8i32);
pub const BP_USERBUTTON: BUTTONPARTS = BUTTONPARTS(5i32);
pub const BST_CHECKED: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(1u32);
pub const BST_DROPDOWNPUSHED: u32 = 1024u32;
pub const BST_HOT: u32 = 512u32;
pub const BST_INDETERMINATE: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(2u32);
pub const BST_UNCHECKED: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(0u32);
pub const BS_COMMANDLINK: i32 = 14i32;
pub const BS_DEFCOMMANDLINK: i32 = 15i32;
pub const BS_DEFSPLITBUTTON: i32 = 13i32;
pub const BS_SPLITBUTTON: i32 = 12i32;
pub const BTNS_AUTOSIZE: u32 = 16u32;
pub const BTNS_BUTTON: u32 = 0u32;
pub const BTNS_CHECK: u32 = 2u32;
pub const BTNS_DROPDOWN: u32 = 8u32;
pub const BTNS_GROUP: u32 = 4u32;
pub const BTNS_NOPREFIX: u32 = 32u32;
pub const BTNS_SEP: u32 = 1u32;
pub const BTNS_SHOWTEXT: u32 = 64u32;
pub const BTNS_WHOLEDROPDOWN: u32 = 128u32;
pub const BT_BORDERFILL: BGTYPE = BGTYPE(1i32);
pub const BT_ELLIPSE: BORDERTYPE = BORDERTYPE(2i32);
pub const BT_IMAGEFILE: BGTYPE = BGTYPE(0i32);
pub const BT_NONE: BGTYPE = BGTYPE(2i32);
pub const BT_RECT: BORDERTYPE = BORDERTYPE(0i32);
pub const BT_ROUNDRECT: BORDERTYPE = BORDERTYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BUTTONPARTS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BUTTON_IMAGELIST {
    pub himl: HIMAGELIST,
    pub margin: super::super::Foundation::RECT,
    pub uAlign: BUTTON_IMAGELIST_ALIGN,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BUTTON_IMAGELIST_ALIGN(pub u32);
pub const BUTTON_IMAGELIST_ALIGN_BOTTOM: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(3u32);
pub const BUTTON_IMAGELIST_ALIGN_CENTER: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(4u32);
pub const BUTTON_IMAGELIST_ALIGN_LEFT: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(0u32);
pub const BUTTON_IMAGELIST_ALIGN_RIGHT: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(1u32);
pub const BUTTON_IMAGELIST_ALIGN_TOP: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BUTTON_SPLITINFO {
    pub mask: u32,
    pub himlGlyph: HIMAGELIST,
    pub uSplitStyle: u32,
    pub size: super::super::Foundation::SIZE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAPTIONSTATES(pub i32);
pub const CA_CENTER: CONTENTALIGNMENT = CONTENTALIGNMENT(1i32);
pub const CA_LEFT: CONTENTALIGNMENT = CONTENTALIGNMENT(0i32);
pub const CA_RIGHT: CONTENTALIGNMENT = CONTENTALIGNMENT(2i32);
pub const CBB_DISABLED: BORDERSTATES = BORDERSTATES(4i32);
pub const CBB_FOCUSED: BORDERSTATES = BORDERSTATES(3i32);
pub const CBB_HOT: BORDERSTATES = BORDERSTATES(2i32);
pub const CBB_NORMAL: BORDERSTATES = BORDERSTATES(1i32);
pub const CBCB_DISABLED: CUEBANNERSTATES = CUEBANNERSTATES(4i32);
pub const CBCB_HOT: CUEBANNERSTATES = CUEBANNERSTATES(2i32);
pub const CBCB_NORMAL: CUEBANNERSTATES = CUEBANNERSTATES(1i32);
pub const CBCB_PRESSED: CUEBANNERSTATES = CUEBANNERSTATES(3i32);
pub const CBDI_HIGHLIGHTED: DROPDOWNITEMSTATES = DROPDOWNITEMSTATES(2i32);
pub const CBDI_NORMAL: DROPDOWNITEMSTATES = DROPDOWNITEMSTATES(1i32);
pub const CBEIF_DI_SETITEM: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(268435456u32);
pub const CBEIF_IMAGE: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(2u32);
pub const CBEIF_INDENT: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(16u32);
pub const CBEIF_LPARAM: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(32u32);
pub const CBEIF_OVERLAY: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(8u32);
pub const CBEIF_SELECTEDIMAGE: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(4u32);
pub const CBEIF_TEXT: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(1u32);
pub const CBEMAXSTRLEN: u32 = 260u32;
pub const CBEM_GETCOMBOCONTROL: u32 = 1030u32;
pub const CBEM_GETEDITCONTROL: u32 = 1031u32;
pub const CBEM_GETEXSTYLE: u32 = 1033u32;
pub const CBEM_GETEXTENDEDSTYLE: u32 = 1033u32;
pub const CBEM_GETIMAGELIST: u32 = 1027u32;
pub const CBEM_GETITEM: u32 = 1037u32;
pub const CBEM_GETITEMA: u32 = 1028u32;
pub const CBEM_GETITEMW: u32 = 1037u32;
pub const CBEM_GETUNICODEFORMAT: u32 = 8198u32;
pub const CBEM_HASEDITCHANGED: u32 = 1034u32;
pub const CBEM_INSERTITEM: u32 = 1035u32;
pub const CBEM_INSERTITEMA: u32 = 1025u32;
pub const CBEM_INSERTITEMW: u32 = 1035u32;
pub const CBEM_SETEXSTYLE: u32 = 1032u32;
pub const CBEM_SETEXTENDEDSTYLE: u32 = 1038u32;
pub const CBEM_SETIMAGELIST: u32 = 1026u32;
pub const CBEM_SETITEM: u32 = 1036u32;
pub const CBEM_SETITEMA: u32 = 1029u32;
pub const CBEM_SETITEMW: u32 = 1036u32;
pub const CBEM_SETUNICODEFORMAT: u32 = 8197u32;
pub const CBEM_SETWINDOWTHEME: u32 = 8203u32;
pub const CBENF_DROPDOWN: u32 = 4u32;
pub const CBENF_ESCAPE: u32 = 3u32;
pub const CBENF_KILLFOCUS: u32 = 1u32;
pub const CBENF_RETURN: u32 = 2u32;
pub const CBEN_BEGINEDIT: u32 = 4294966492u32;
pub const CBEN_DELETEITEM: u32 = 4294966494u32;
pub const CBEN_DRAGBEGIN: u32 = 4294966487u32;
pub const CBEN_DRAGBEGINA: u32 = 4294966488u32;
pub const CBEN_DRAGBEGINW: u32 = 4294966487u32;
pub const CBEN_ENDEDIT: u32 = 4294966490u32;
pub const CBEN_ENDEDITA: u32 = 4294966491u32;
pub const CBEN_ENDEDITW: u32 = 4294966490u32;
pub const CBEN_FIRST: u32 = 4294966496u32;
pub const CBEN_GETDISPINFOA: u32 = 4294966496u32;
pub const CBEN_GETDISPINFOW: u32 = 4294966489u32;
pub const CBEN_INSERTITEM: u32 = 4294966495u32;
pub const CBEN_LAST: u32 = 4294966466u32;
pub const CBES_EX_CASESENSITIVE: u32 = 16u32;
pub const CBES_EX_NOEDITIMAGE: u32 = 1u32;
pub const CBES_EX_NOEDITIMAGEINDENT: u32 = 2u32;
pub const CBES_EX_NOSIZELIMIT: u32 = 8u32;
pub const CBES_EX_PATHWORDBREAKPROC: u32 = 4u32;
pub const CBES_EX_TEXTENDELLIPSIS: u32 = 32u32;
pub const CBM_FIRST: u32 = 5888u32;
pub const CBRO_DISABLED: READONLYSTATES = READONLYSTATES(4i32);
pub const CBRO_HOT: READONLYSTATES = READONLYSTATES(2i32);
pub const CBRO_NORMAL: READONLYSTATES = READONLYSTATES(1i32);
pub const CBRO_PRESSED: READONLYSTATES = READONLYSTATES(3i32);
pub const CBS_CHECKEDDISABLED: CHECKBOXSTATES = CHECKBOXSTATES(8i32);
pub const CBS_CHECKEDHOT: CHECKBOXSTATES = CHECKBOXSTATES(6i32);
pub const CBS_CHECKEDNORMAL: CHECKBOXSTATES = CHECKBOXSTATES(5i32);
pub const CBS_CHECKEDPRESSED: CHECKBOXSTATES = CHECKBOXSTATES(7i32);
pub const CBS_DISABLED: CLOSEBUTTONSTATES = CLOSEBUTTONSTATES(4i32);
pub const CBS_EXCLUDEDDISABLED: CHECKBOXSTATES = CHECKBOXSTATES(20i32);
pub const CBS_EXCLUDEDHOT: CHECKBOXSTATES = CHECKBOXSTATES(18i32);
pub const CBS_EXCLUDEDNORMAL: CHECKBOXSTATES = CHECKBOXSTATES(17i32);
pub const CBS_EXCLUDEDPRESSED: CHECKBOXSTATES = CHECKBOXSTATES(19i32);
pub const CBS_HOT: CLOSEBUTTONSTATES = CLOSEBUTTONSTATES(2i32);
pub const CBS_IMPLICITDISABLED: CHECKBOXSTATES = CHECKBOXSTATES(16i32);
pub const CBS_IMPLICITHOT: CHECKBOXSTATES = CHECKBOXSTATES(14i32);
pub const CBS_IMPLICITNORMAL: CHECKBOXSTATES = CHECKBOXSTATES(13i32);
pub const CBS_IMPLICITPRESSED: CHECKBOXSTATES = CHECKBOXSTATES(15i32);
pub const CBS_MIXEDDISABLED: CHECKBOXSTATES = CHECKBOXSTATES(12i32);
pub const CBS_MIXEDHOT: CHECKBOXSTATES = CHECKBOXSTATES(10i32);
pub const CBS_MIXEDNORMAL: CHECKBOXSTATES = CHECKBOXSTATES(9i32);
pub const CBS_MIXEDPRESSED: CHECKBOXSTATES = CHECKBOXSTATES(11i32);
pub const CBS_NORMAL: CLOSEBUTTONSTATES = CLOSEBUTTONSTATES(1i32);
pub const CBS_PUSHED: CLOSEBUTTONSTATES = CLOSEBUTTONSTATES(3i32);
pub const CBS_UNCHECKEDDISABLED: CHECKBOXSTATES = CHECKBOXSTATES(4i32);
pub const CBS_UNCHECKEDHOT: CHECKBOXSTATES = CHECKBOXSTATES(2i32);
pub const CBS_UNCHECKEDNORMAL: CHECKBOXSTATES = CHECKBOXSTATES(1i32);
pub const CBS_UNCHECKEDPRESSED: CHECKBOXSTATES = CHECKBOXSTATES(3i32);
pub const CBTBS_DISABLED: TRANSPARENTBACKGROUNDSTATES = TRANSPARENTBACKGROUNDSTATES(3i32);
pub const CBTBS_FOCUSED: TRANSPARENTBACKGROUNDSTATES = TRANSPARENTBACKGROUNDSTATES(4i32);
pub const CBTBS_HOT: TRANSPARENTBACKGROUNDSTATES = TRANSPARENTBACKGROUNDSTATES(2i32);
pub const CBTBS_NORMAL: TRANSPARENTBACKGROUNDSTATES = TRANSPARENTBACKGROUNDSTATES(1i32);
pub const CBXSL_DISABLED: DROPDOWNBUTTONLEFTSTATES = DROPDOWNBUTTONLEFTSTATES(4i32);
pub const CBXSL_HOT: DROPDOWNBUTTONLEFTSTATES = DROPDOWNBUTTONLEFTSTATES(2i32);
pub const CBXSL_NORMAL: DROPDOWNBUTTONLEFTSTATES = DROPDOWNBUTTONLEFTSTATES(1i32);
pub const CBXSL_PRESSED: DROPDOWNBUTTONLEFTSTATES = DROPDOWNBUTTONLEFTSTATES(3i32);
pub const CBXSR_DISABLED: DROPDOWNBUTTONRIGHTSTATES = DROPDOWNBUTTONRIGHTSTATES(4i32);
pub const CBXSR_HOT: DROPDOWNBUTTONRIGHTSTATES = DROPDOWNBUTTONRIGHTSTATES(2i32);
pub const CBXSR_NORMAL: DROPDOWNBUTTONRIGHTSTATES = DROPDOWNBUTTONRIGHTSTATES(1i32);
pub const CBXSR_PRESSED: DROPDOWNBUTTONRIGHTSTATES = DROPDOWNBUTTONRIGHTSTATES(3i32);
pub const CBXS_DISABLED: COMBOBOXSTYLESTATES = COMBOBOXSTYLESTATES(4i32);
pub const CBXS_HOT: COMBOBOXSTYLESTATES = COMBOBOXSTYLESTATES(2i32);
pub const CBXS_NORMAL: COMBOBOXSTYLESTATES = COMBOBOXSTYLESTATES(1i32);
pub const CBXS_PRESSED: COMBOBOXSTYLESTATES = COMBOBOXSTYLESTATES(3i32);
pub const CB_GETCUEBANNER: u32 = 5892u32;
pub const CB_GETMINVISIBLE: u32 = 5890u32;
pub const CB_SETCUEBANNER: u32 = 5891u32;
pub const CB_SETMINVISIBLE: u32 = 5889u32;
pub const CCF_NOTEXT: u32 = 1u32;
pub const CCHCCCLASS: u32 = 32u32;
pub const CCHCCDESC: u32 = 32u32;
pub const CCHCCTEXT: u32 = 256u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug)]
pub struct CCINFOA {
    pub szClass: [i8; 32],
    pub flOptions: u32,
    pub szDesc: [i8; 32],
    pub cxDefault: u32,
    pub cyDefault: u32,
    pub flStyleDefault: u32,
    pub flExtStyleDefault: u32,
    pub flCtrlTypeMask: u32,
    pub szTextDefault: [i8; 256],
    pub cStyleFlags: i32,
    pub aStyleFlags: *mut CCSTYLEFLAGA,
    pub lpfnStyle: LPFNCCSTYLEA,
    pub lpfnSizeToText: LPFNCCSIZETOTEXTA,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CCINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug)]
pub struct CCINFOW {
    pub szClass: [u16; 32],
    pub flOptions: u32,
    pub szDesc: [u16; 32],
    pub cxDefault: u32,
    pub cyDefault: u32,
    pub flStyleDefault: u32,
    pub flExtStyleDefault: u32,
    pub flCtrlTypeMask: u32,
    pub cStyleFlags: i32,
    pub aStyleFlags: *mut CCSTYLEFLAGW,
    pub szTextDefault: [u16; 256],
    pub lpfnStyle: LPFNCCSTYLEW,
    pub lpfnSizeToText: LPFNCCSIZETOTEXTW,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CCINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CCM_DPISCALE: u32 = 8204u32;
pub const CCM_FIRST: u32 = 8192u32;
pub const CCM_GETCOLORSCHEME: u32 = 8195u32;
pub const CCM_GETDROPTARGET: u32 = 8196u32;
pub const CCM_GETUNICODEFORMAT: u32 = 8198u32;
pub const CCM_GETVERSION: u32 = 8200u32;
pub const CCM_LAST: u32 = 8704u32;
pub const CCM_SETBKCOLOR: u32 = 8193u32;
pub const CCM_SETCOLORSCHEME: u32 = 8194u32;
pub const CCM_SETNOTIFYWINDOW: u32 = 8201u32;
pub const CCM_SETUNICODEFORMAT: u32 = 8197u32;
pub const CCM_SETVERSION: u32 = 8199u32;
pub const CCM_SETWINDOWTHEME: u32 = 8203u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CCSTYLEA {
    pub flStyle: u32,
    pub flExtStyle: u32,
    pub szText: [i8; 256],
    pub lgid: u16,
    pub wReserved1: u16,
}
impl Default for CCSTYLEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CCSTYLEFLAGA {
    pub flStyle: u32,
    pub flStyleMask: u32,
    pub pszStyle: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CCSTYLEFLAGW {
    pub flStyle: u32,
    pub flStyleMask: u32,
    pub pszStyle: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CCSTYLEW {
    pub flStyle: u32,
    pub flExtStyle: u32,
    pub szText: [u16; 256],
    pub lgid: u16,
    pub wReserved1: u16,
}
impl Default for CCSTYLEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CCS_ADJUSTABLE: i32 = 32i32;
pub const CCS_BOTTOM: i32 = 3i32;
pub const CCS_NODIVIDER: i32 = 64i32;
pub const CCS_NOMOVEY: i32 = 2i32;
pub const CCS_NOPARENTALIGN: i32 = 8i32;
pub const CCS_NORESIZE: i32 = 4i32;
pub const CCS_TOP: i32 = 1i32;
pub const CCS_VERT: i32 = 128i32;
pub const CDDS_ITEM: u32 = 65536u32;
pub const CDDS_ITEMPOSTERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65540u32);
pub const CDDS_ITEMPOSTPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65538u32);
pub const CDDS_ITEMPREERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65539u32);
pub const CDDS_ITEMPREPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65537u32);
pub const CDDS_POSTERASE: u32 = 4u32;
pub const CDDS_POSTPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(2u32);
pub const CDDS_PREERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(3u32);
pub const CDDS_PREPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(1u32);
pub const CDDS_SUBITEM: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(131072u32);
pub const CDIS_CHECKED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(8u32);
pub const CDIS_DEFAULT: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(32u32);
pub const CDIS_DISABLED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(4u32);
pub const CDIS_DROPHILITED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(4096u32);
pub const CDIS_FOCUS: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(16u32);
pub const CDIS_GRAYED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(2u32);
pub const CDIS_HOT: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(64u32);
pub const CDIS_INDETERMINATE: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(256u32);
pub const CDIS_MARKED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(128u32);
pub const CDIS_NEARHOT: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(1024u32);
pub const CDIS_OTHERSIDEHOT: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(2048u32);
pub const CDIS_SELECTED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(1u32);
pub const CDIS_SHOWKEYBOARDCUES: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(512u32);
pub const CDN_FIRST: u32 = 4294966695u32;
pub const CDN_LAST: u32 = 4294966597u32;
pub const CDRF_DODEFAULT: u32 = 0u32;
pub const CDRF_DOERASE: u32 = 8u32;
pub const CDRF_NEWFONT: u32 = 2u32;
pub const CDRF_NOTIFYITEMDRAW: u32 = 32u32;
pub const CDRF_NOTIFYPOSTERASE: u32 = 64u32;
pub const CDRF_NOTIFYPOSTPAINT: u32 = 16u32;
pub const CDRF_NOTIFYSUBITEMDRAW: u32 = 32u32;
pub const CDRF_SKIPDEFAULT: u32 = 4u32;
pub const CDRF_SKIPPOSTPAINT: u32 = 256u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHECKBOXSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHEVRONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHEVRONVERTSTATES(pub i32);
pub const CHEVSV_HOT: CHEVRONVERTSTATES = CHEVRONVERTSTATES(2i32);
pub const CHEVSV_NORMAL: CHEVRONVERTSTATES = CHEVRONVERTSTATES(1i32);
pub const CHEVSV_PRESSED: CHEVRONVERTSTATES = CHEVRONVERTSTATES(3i32);
pub const CHEVS_HOT: CHEVRONSTATES = CHEVRONSTATES(2i32);
pub const CHEVS_NORMAL: CHEVRONSTATES = CHEVRONSTATES(1i32);
pub const CHEVS_PRESSED: CHEVRONSTATES = CHEVRONSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLOCKPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLOCKSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLOSEBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLOSESTATES(pub i32);
pub const CLP_TIME: CLOCKPARTS = CLOCKPARTS(1i32);
pub const CLR_DEFAULT: i32 = -16777216i32;
pub const CLR_HILIGHT: i32 = -16777216i32;
pub const CLR_NONE: i32 = -1i32;
pub const CLS_HOT: CLOCKSTATES = CLOCKSTATES(2i32);
pub const CLS_NORMAL: CLOCKSTATES = CLOCKSTATES(1i32);
pub const CLS_PRESSED: CLOCKSTATES = CLOCKSTATES(3i32);
pub const CMB_MASKED: u32 = 2u32;
pub const CMDLGS_DEFAULTED: COMMANDLINKGLYPHSTATES = COMMANDLINKGLYPHSTATES(5i32);
pub const CMDLGS_DISABLED: COMMANDLINKGLYPHSTATES = COMMANDLINKGLYPHSTATES(4i32);
pub const CMDLGS_HOT: COMMANDLINKGLYPHSTATES = COMMANDLINKGLYPHSTATES(2i32);
pub const CMDLGS_NORMAL: COMMANDLINKGLYPHSTATES = COMMANDLINKGLYPHSTATES(1i32);
pub const CMDLGS_PRESSED: COMMANDLINKGLYPHSTATES = COMMANDLINKGLYPHSTATES(3i32);
pub const CMDLS_DEFAULTED: COMMANDLINKSTATES = COMMANDLINKSTATES(5i32);
pub const CMDLS_DEFAULTED_ANIMATING: COMMANDLINKSTATES = COMMANDLINKSTATES(6i32);
pub const CMDLS_DISABLED: COMMANDLINKSTATES = COMMANDLINKSTATES(4i32);
pub const CMDLS_HOT: COMMANDLINKSTATES = COMMANDLINKSTATES(2i32);
pub const CMDLS_NORMAL: COMMANDLINKSTATES = COMMANDLINKSTATES(1i32);
pub const CMDLS_PRESSED: COMMANDLINKSTATES = COMMANDLINKSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLLAPSEBUTTONSTATES(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COLORMAP {
    pub from: super::super::Foundation::COLORREF,
    pub to: super::super::Foundation::COLORREF,
}
pub const COLORMGMTDLGORD: u32 = 1551u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COLORSCHEME {
    pub dwSize: u32,
    pub clrBtnHighlight: super::super::Foundation::COLORREF,
    pub clrBtnShadow: super::super::Foundation::COLORREF,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMBOBOXEXITEMA {
    pub mask: COMBOBOX_EX_ITEM_FLAGS,
    pub iItem: isize,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMBOBOXEXITEMW {
    pub mask: COMBOBOX_EX_ITEM_FLAGS,
    pub iItem: isize,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMBOBOXINFO {
    pub cbSize: u32,
    pub rcItem: super::super::Foundation::RECT,
    pub rcButton: super::super::Foundation::RECT,
    pub stateButton: COMBOBOXINFO_BUTTON_STATE,
    pub hwndCombo: super::super::Foundation::HWND,
    pub hwndItem: super::super::Foundation::HWND,
    pub hwndList: super::super::Foundation::HWND,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMBOBOXINFO_BUTTON_STATE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMBOBOXPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMBOBOXSTYLESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMBOBOX_EX_ITEM_FLAGS(pub u32);
impl COMBOBOX_EX_ITEM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for COMBOBOX_EX_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for COMBOBOX_EX_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const COMCTL32_VERSION: u32 = 6u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMMANDLINKGLYPHSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMMANDLINKSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMMUNICATIONSPARTS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPAREITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub hwndItem: super::super::Foundation::HWND,
    pub itemID1: u32,
    pub itemData1: usize,
    pub itemID2: u32,
    pub itemData2: usize,
    pub dwLocaleId: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTENTALIGNMENT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTENTAREASTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTENTLINKSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTENTPANESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTROLLABELSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTROLPANELPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COPYSTATES(pub i32);
pub const CPANEL_BANNERAREA: CONTROLPANELPARTS = CONTROLPANELPARTS(18i32);
pub const CPANEL_BODYTEXT: CONTROLPANELPARTS = CONTROLPANELPARTS(6i32);
pub const CPANEL_BODYTITLE: CONTROLPANELPARTS = CONTROLPANELPARTS(19i32);
pub const CPANEL_BUTTON: CONTROLPANELPARTS = CONTROLPANELPARTS(14i32);
pub const CPANEL_CONTENTLINK: CONTROLPANELPARTS = CONTROLPANELPARTS(10i32);
pub const CPANEL_CONTENTPANE: CONTROLPANELPARTS = CONTROLPANELPARTS(2i32);
pub const CPANEL_CONTENTPANELABEL: CONTROLPANELPARTS = CONTROLPANELPARTS(4i32);
pub const CPANEL_CONTENTPANELINE: CONTROLPANELPARTS = CONTROLPANELPARTS(17i32);
pub const CPANEL_GROUPTEXT: CONTROLPANELPARTS = CONTROLPANELPARTS(9i32);
pub const CPANEL_HELPLINK: CONTROLPANELPARTS = CONTROLPANELPARTS(7i32);
pub const CPANEL_LARGECOMMANDAREA: CONTROLPANELPARTS = CONTROLPANELPARTS(12i32);
pub const CPANEL_MESSAGETEXT: CONTROLPANELPARTS = CONTROLPANELPARTS(15i32);
pub const CPANEL_NAVIGATIONPANE: CONTROLPANELPARTS = CONTROLPANELPARTS(1i32);
pub const CPANEL_NAVIGATIONPANELABEL: CONTROLPANELPARTS = CONTROLPANELPARTS(3i32);
pub const CPANEL_NAVIGATIONPANELINE: CONTROLPANELPARTS = CONTROLPANELPARTS(16i32);
pub const CPANEL_SECTIONTITLELINK: CONTROLPANELPARTS = CONTROLPANELPARTS(11i32);
pub const CPANEL_SMALLCOMMANDAREA: CONTROLPANELPARTS = CONTROLPANELPARTS(13i32);
pub const CPANEL_TASKLINK: CONTROLPANELPARTS = CONTROLPANELPARTS(8i32);
pub const CPANEL_TITLE: CONTROLPANELPARTS = CONTROLPANELPARTS(5i32);
pub const CPCL_DISABLED: CONTENTLINKSTATES = CONTENTLINKSTATES(4i32);
pub const CPCL_HOT: CONTENTLINKSTATES = CONTENTLINKSTATES(2i32);
pub const CPCL_NORMAL: CONTENTLINKSTATES = CONTENTLINKSTATES(1i32);
pub const CPCL_PRESSED: CONTENTLINKSTATES = CONTENTLINKSTATES(3i32);
pub const CPHL_DISABLED: HELPLINKSTATES = HELPLINKSTATES(4i32);
pub const CPHL_HOT: HELPLINKSTATES = HELPLINKSTATES(2i32);
pub const CPHL_NORMAL: HELPLINKSTATES = HELPLINKSTATES(1i32);
pub const CPHL_PRESSED: HELPLINKSTATES = HELPLINKSTATES(3i32);
pub const CPSTL_HOT: SECTIONTITLELINKSTATES = SECTIONTITLELINKSTATES(2i32);
pub const CPSTL_NORMAL: SECTIONTITLELINKSTATES = SECTIONTITLELINKSTATES(1i32);
pub const CPTL_DISABLED: TASKLINKSTATES = TASKLINKSTATES(4i32);
pub const CPTL_HOT: TASKLINKSTATES = TASKLINKSTATES(2i32);
pub const CPTL_NORMAL: TASKLINKSTATES = TASKLINKSTATES(1i32);
pub const CPTL_PAGE: TASKLINKSTATES = TASKLINKSTATES(5i32);
pub const CPTL_PRESSED: TASKLINKSTATES = TASKLINKSTATES(3i32);
pub const CP_BACKGROUND: COMBOBOXPARTS = COMBOBOXPARTS(2i32);
pub const CP_BORDER: COMBOBOXPARTS = COMBOBOXPARTS(4i32);
pub const CP_CUEBANNER: COMBOBOXPARTS = COMBOBOXPARTS(8i32);
pub const CP_DROPDOWNBUTTON: COMBOBOXPARTS = COMBOBOXPARTS(1i32);
pub const CP_DROPDOWNBUTTONLEFT: COMBOBOXPARTS = COMBOBOXPARTS(7i32);
pub const CP_DROPDOWNBUTTONRIGHT: COMBOBOXPARTS = COMBOBOXPARTS(6i32);
pub const CP_DROPDOWNITEM: COMBOBOXPARTS = COMBOBOXPARTS(9i32);
pub const CP_READONLY: COMBOBOXPARTS = COMBOBOXPARTS(5i32);
pub const CP_TRANSPARENTBACKGROUND: COMBOBOXPARTS = COMBOBOXPARTS(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATELINKSTATES(pub i32);
pub const CSST_TAB: COMMUNICATIONSPARTS = COMMUNICATIONSPARTS(1i32);
pub const CSTB_HOT: TABSTATES = TABSTATES(2i32);
pub const CSTB_NORMAL: TABSTATES = TABSTATES(1i32);
pub const CSTB_SELECTED: TABSTATES = TABSTATES(3i32);
pub const CS_ACTIVE: CAPTIONSTATES = CAPTIONSTATES(1i32);
pub const CS_DISABLED: CAPTIONSTATES = CAPTIONSTATES(3i32);
pub const CS_INACTIVE: CAPTIONSTATES = CAPTIONSTATES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CUEBANNERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DATEBORDERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DATEPICKERPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DATETEXTSTATES(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DATETIMEPICKERINFO {
    pub cbSize: u32,
    pub rcCheck: super::super::Foundation::RECT,
    pub stateCheck: u32,
    pub rcButton: super::super::Foundation::RECT,
    pub stateButton: u32,
    pub hwndEdit: super::super::Foundation::HWND,
    pub hwndUD: super::super::Foundation::HWND,
    pub hwndDropDown: super::super::Foundation::HWND,
}
pub const DATETIMEPICK_CLASS: windows_core::PCWSTR = windows_core::w!("SysDateTimePick32");
pub const DATETIMEPICK_CLASSA: windows_core::PCSTR = windows_core::s!("SysDateTimePick32");
pub const DATETIMEPICK_CLASSW: windows_core::PCWSTR = windows_core::w!("SysDateTimePick32");
pub const DA_ERR: i32 = -1i32;
pub const DA_LAST: u32 = 2147483647u32;
pub const DDCOPY_HIGHLIGHT: COPYSTATES = COPYSTATES(1i32);
pub const DDCOPY_NOHIGHLIGHT: COPYSTATES = COPYSTATES(2i32);
pub const DDCREATELINK_HIGHLIGHT: CREATELINKSTATES = CREATELINKSTATES(1i32);
pub const DDCREATELINK_NOHIGHLIGHT: CREATELINKSTATES = CREATELINKSTATES(2i32);
pub const DDL_ARCHIVE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(32u32);
pub const DDL_DIRECTORY: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(16u32);
pub const DDL_DRIVES: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(16384u32);
pub const DDL_EXCLUSIVE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(32768u32);
pub const DDL_HIDDEN: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(2u32);
pub const DDL_POSTMSGS: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(8192u32);
pub const DDL_READONLY: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(1u32);
pub const DDL_READWRITE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(0u32);
pub const DDL_SYSTEM: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(4u32);
pub const DDMOVE_HIGHLIGHT: MOVESTATES = MOVESTATES(1i32);
pub const DDMOVE_NOHIGHLIGHT: MOVESTATES = MOVESTATES(2i32);
pub const DDNONE_HIGHLIGHT: NONESTATES = NONESTATES(1i32);
pub const DDNONE_NOHIGHLIGHT: NONESTATES = NONESTATES(2i32);
pub const DDUPDATEMETADATA_HIGHLIGHT: UPDATEMETADATASTATES = UPDATEMETADATASTATES(1i32);
pub const DDUPDATEMETADATA_NOHIGHLIGHT: UPDATEMETADATASTATES = UPDATEMETADATASTATES(2i32);
pub const DDWARNING_HIGHLIGHT: WARNINGSTATES = WARNINGSTATES(1i32);
pub const DDWARNING_NOHIGHLIGHT: WARNINGSTATES = WARNINGSTATES(2i32);
pub const DD_COPY: DRAGDROPPARTS = DRAGDROPPARTS(1i32);
pub const DD_CREATELINK: DRAGDROPPARTS = DRAGDROPPARTS(4i32);
pub const DD_IMAGEBG: DRAGDROPPARTS = DRAGDROPPARTS(7i32);
pub const DD_MOVE: DRAGDROPPARTS = DRAGDROPPARTS(2i32);
pub const DD_NONE: DRAGDROPPARTS = DRAGDROPPARTS(6i32);
pub const DD_TEXTBG: DRAGDROPPARTS = DRAGDROPPARTS(8i32);
pub const DD_UPDATEMETADATA: DRAGDROPPARTS = DRAGDROPPARTS(3i32);
pub const DD_WARNING: DRAGDROPPARTS = DRAGDROPPARTS(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DELETEITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub itemID: u32,
    pub hwndItem: super::super::Foundation::HWND,
    pub itemData: usize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DLG_BUTTON_CHECK_STATE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DLG_DIR_LIST_FILE_TYPE(pub u32);
impl DLG_DIR_LIST_FILE_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DLG_DIR_LIST_FILE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DLG_DIR_LIST_FILE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DL_BEGINDRAG: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1157u32);
pub const DL_CANCELDRAG: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1160u32);
pub const DL_COPYCURSOR: u32 = 2u32;
pub const DL_CURSORSET: u32 = 0u32;
pub const DL_DRAGGING: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1158u32);
pub const DL_DROPPED: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1159u32);
pub const DL_MOVECURSOR: u32 = 3u32;
pub const DL_STOPCURSOR: u32 = 1u32;
pub const DNHZS_DISABLED: DOWNHORZSTATES = DOWNHORZSTATES(4i32);
pub const DNHZS_HOT: DOWNHORZSTATES = DOWNHORZSTATES(2i32);
pub const DNHZS_NORMAL: DOWNHORZSTATES = DOWNHORZSTATES(1i32);
pub const DNHZS_PRESSED: DOWNHORZSTATES = DOWNHORZSTATES(3i32);
pub const DNS_DISABLED: DOWNSTATES = DOWNSTATES(4i32);
pub const DNS_HOT: DOWNSTATES = DOWNSTATES(2i32);
pub const DNS_NORMAL: DOWNSTATES = DOWNSTATES(1i32);
pub const DNS_PRESSED: DOWNSTATES = DOWNSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOWNHORZSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOWNSTATES(pub i32);
pub const DPAMM_DELETE: DPAMM_MESSAGE = DPAMM_MESSAGE(2u32);
pub const DPAMM_INSERT: DPAMM_MESSAGE = DPAMM_MESSAGE(3u32);
pub const DPAMM_MERGE: DPAMM_MESSAGE = DPAMM_MESSAGE(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DPAMM_MESSAGE(pub u32);
pub const DPAM_INTERSECT: u32 = 8u32;
pub const DPAM_NORMAL: u32 = 2u32;
pub const DPAM_SORTED: u32 = 1u32;
pub const DPAM_UNION: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DPASTREAMINFO {
    pub iPos: i32,
    pub pvItem: *mut core::ffi::c_void,
}
impl Default for DPASTREAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DPAS_INSERTAFTER: u32 = 4u32;
pub const DPAS_INSERTBEFORE: u32 = 2u32;
pub const DPAS_SORTED: u32 = 1u32;
pub const DPA_APPEND: u32 = 2147483647u32;
pub const DPA_ERR: i32 = -1i32;
pub const DPDB_DISABLED: DATEBORDERSTATES = DATEBORDERSTATES(4i32);
pub const DPDB_FOCUSED: DATEBORDERSTATES = DATEBORDERSTATES(3i32);
pub const DPDB_HOT: DATEBORDERSTATES = DATEBORDERSTATES(2i32);
pub const DPDB_NORMAL: DATEBORDERSTATES = DATEBORDERSTATES(1i32);
pub const DPDT_DISABLED: DATETEXTSTATES = DATETEXTSTATES(2i32);
pub const DPDT_NORMAL: DATETEXTSTATES = DATETEXTSTATES(1i32);
pub const DPDT_SELECTED: DATETEXTSTATES = DATETEXTSTATES(3i32);
pub const DPSCBR_DISABLED: SHOWCALENDARBUTTONRIGHTSTATES = SHOWCALENDARBUTTONRIGHTSTATES(4i32);
pub const DPSCBR_HOT: SHOWCALENDARBUTTONRIGHTSTATES = SHOWCALENDARBUTTONRIGHTSTATES(2i32);
pub const DPSCBR_NORMAL: SHOWCALENDARBUTTONRIGHTSTATES = SHOWCALENDARBUTTONRIGHTSTATES(1i32);
pub const DPSCBR_PRESSED: SHOWCALENDARBUTTONRIGHTSTATES = SHOWCALENDARBUTTONRIGHTSTATES(3i32);
pub const DP_DATEBORDER: DATEPICKERPARTS = DATEPICKERPARTS(2i32);
pub const DP_DATETEXT: DATEPICKERPARTS = DATEPICKERPARTS(1i32);
pub const DP_SHOWCALENDARBUTTONRIGHT: DATEPICKERPARTS = DATEPICKERPARTS(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAGDROPPARTS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRAGLISTINFO {
    pub uNotification: DRAGLISTINFO_NOTIFICATION_FLAGS,
    pub hWnd: super::super::Foundation::HWND,
    pub ptCursor: super::super::Foundation::POINT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAGLISTINFO_NOTIFICATION_FLAGS(pub u32);
pub const DRAGLISTMSGSTRING: windows_core::PCWSTR = windows_core::w!("commctrl_DragListMsg");
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRAWITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemAction: ODA_FLAGS,
    pub itemState: ODS_FLAGS,
    pub hwndItem: super::super::Foundation::HWND,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub rcItem: super::super::Foundation::RECT,
    pub itemData: usize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAWITEMSTRUCT_CTL_TYPE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAW_THEME_PARENT_BACKGROUND_FLAGS(pub u32);
impl DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DROPDOWNBUTTONLEFTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DROPDOWNBUTTONRIGHTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DROPDOWNITEMSTATES(pub i32);
pub const DSA_APPEND: u32 = 2147483647u32;
pub const DSA_ERR: i32 = -1i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DTBGOPTS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub rcClip: super::super::Foundation::RECT,
}
pub const DTBG_CLIPRECT: u32 = 1u32;
pub const DTBG_COMPUTINGREGION: u32 = 16u32;
pub const DTBG_DRAWSOLID: u32 = 2u32;
pub const DTBG_MIRRORDC: u32 = 32u32;
pub const DTBG_NOMIRROR: u32 = 64u32;
pub const DTBG_OMITBORDER: u32 = 4u32;
pub const DTBG_OMITCONTENT: u32 = 8u32;
pub const DTM_CLOSEMONTHCAL: u32 = 4109u32;
pub const DTM_FIRST: u32 = 4096u32;
pub const DTM_GETDATETIMEPICKERINFO: u32 = 4110u32;
pub const DTM_GETIDEALSIZE: u32 = 4111u32;
pub const DTM_GETMCCOLOR: u32 = 4103u32;
pub const DTM_GETMCFONT: u32 = 4106u32;
pub const DTM_GETMCSTYLE: u32 = 4108u32;
pub const DTM_GETMONTHCAL: u32 = 4104u32;
pub const DTM_GETRANGE: u32 = 4099u32;
pub const DTM_GETSYSTEMTIME: u32 = 4097u32;
pub const DTM_SETFORMAT: u32 = 4146u32;
pub const DTM_SETFORMATA: u32 = 4101u32;
pub const DTM_SETFORMATW: u32 = 4146u32;
pub const DTM_SETMCCOLOR: u32 = 4102u32;
pub const DTM_SETMCFONT: u32 = 4105u32;
pub const DTM_SETMCSTYLE: u32 = 4107u32;
pub const DTM_SETRANGE: u32 = 4100u32;
pub const DTM_SETSYSTEMTIME: u32 = 4098u32;
pub const DTN_CLOSEUP: u32 = 4294966543u32;
pub const DTN_DATETIMECHANGE: u32 = 4294966537u32;
pub const DTN_DROPDOWN: u32 = 4294966542u32;
pub const DTN_FIRST: u32 = 4294966556u32;
pub const DTN_FIRST2: u32 = 4294966543u32;
pub const DTN_FORMAT: u32 = 4294966553u32;
pub const DTN_FORMATA: u32 = 4294966540u32;
pub const DTN_FORMATQUERY: u32 = 4294966554u32;
pub const DTN_FORMATQUERYA: u32 = 4294966541u32;
pub const DTN_FORMATQUERYW: u32 = 4294966554u32;
pub const DTN_FORMATW: u32 = 4294966553u32;
pub const DTN_LAST: u32 = 4294966551u32;
pub const DTN_LAST2: u32 = 4294966497u32;
pub const DTN_USERSTRING: u32 = 4294966551u32;
pub const DTN_USERSTRINGA: u32 = 4294966538u32;
pub const DTN_USERSTRINGW: u32 = 4294966551u32;
pub const DTN_WMKEYDOWN: u32 = 4294966552u32;
pub const DTN_WMKEYDOWNA: u32 = 4294966539u32;
pub const DTN_WMKEYDOWNW: u32 = 4294966552u32;
pub const DTPB_USECTLCOLORSTATIC: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(2u32);
pub const DTPB_USEERASEBKGND: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(4u32);
pub const DTPB_WINDOWDC: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(1u32);
pub const DTS_APPCANPARSE: u32 = 16u32;
pub const DTS_LONGDATEFORMAT: u32 = 4u32;
pub const DTS_RIGHTALIGN: u32 = 32u32;
pub const DTS_SHORTDATECENTURYFORMAT: u32 = 12u32;
pub const DTS_SHORTDATEFORMAT: u32 = 0u32;
pub const DTS_SHOWNONE: u32 = 2u32;
pub const DTS_TIMEFORMAT: u32 = 9u32;
pub const DTS_UPDOWN: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default)]
pub struct DTTOPTS {
    pub dwSize: u32,
    pub dwFlags: DTTOPTS_FLAGS,
    pub crText: super::super::Foundation::COLORREF,
    pub crBorder: super::super::Foundation::COLORREF,
    pub crShadow: super::super::Foundation::COLORREF,
    pub iTextShadowType: i32,
    pub ptShadowOffset: super::super::Foundation::POINT,
    pub iBorderSize: i32,
    pub iFontPropId: i32,
    pub iColorPropId: i32,
    pub iStateId: i32,
    pub fApplyOverlay: windows_core::BOOL,
    pub iGlowSize: i32,
    pub pfnDrawTextCallback: DTT_CALLBACK_PROC,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DTTOPTS_FLAGS(pub u32);
impl DTTOPTS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DTTOPTS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DTTOPTS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DTTOPTS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DTTOPTS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DTTOPTS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DTT_APPLYOVERLAY: DTTOPTS_FLAGS = DTTOPTS_FLAGS(1024u32);
pub const DTT_BORDERCOLOR: DTTOPTS_FLAGS = DTTOPTS_FLAGS(2u32);
pub const DTT_BORDERSIZE: DTTOPTS_FLAGS = DTTOPTS_FLAGS(32u32);
pub const DTT_CALCRECT: DTTOPTS_FLAGS = DTTOPTS_FLAGS(512u32);
pub const DTT_CALLBACK: DTTOPTS_FLAGS = DTTOPTS_FLAGS(4096u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type DTT_CALLBACK_PROC = Option<unsafe extern "system" fn(hdc: super::super::Graphics::Gdi::HDC, psztext: windows_core::PWSTR, cchtext: i32, prc: *mut super::super::Foundation::RECT, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> i32>;
pub const DTT_COLORPROP: DTTOPTS_FLAGS = DTTOPTS_FLAGS(128u32);
pub const DTT_COMPOSITED: DTTOPTS_FLAGS = DTTOPTS_FLAGS(8192u32);
pub const DTT_FLAGS2VALIDBITS: u32 = 1u32;
pub const DTT_FONTPROP: DTTOPTS_FLAGS = DTTOPTS_FLAGS(64u32);
pub const DTT_GLOWSIZE: DTTOPTS_FLAGS = DTTOPTS_FLAGS(2048u32);
pub const DTT_GRAYED: u32 = 1u32;
pub const DTT_SHADOWCOLOR: DTTOPTS_FLAGS = DTTOPTS_FLAGS(4u32);
pub const DTT_SHADOWOFFSET: DTTOPTS_FLAGS = DTTOPTS_FLAGS(16u32);
pub const DTT_SHADOWTYPE: DTTOPTS_FLAGS = DTTOPTS_FLAGS(8u32);
pub const DTT_STATEID: DTTOPTS_FLAGS = DTTOPTS_FLAGS(256u32);
pub const DTT_TEXTCOLOR: DTTOPTS_FLAGS = DTTOPTS_FLAGS(1u32);
pub const DTT_VALIDBITS: DTTOPTS_FLAGS = DTTOPTS_FLAGS(12287u32);
pub const EBHC_HOT: HEADERCLOSESTATES = HEADERCLOSESTATES(2i32);
pub const EBHC_NORMAL: HEADERCLOSESTATES = HEADERCLOSESTATES(1i32);
pub const EBHC_PRESSED: HEADERCLOSESTATES = HEADERCLOSESTATES(3i32);
pub const EBHP_HOT: HEADERPINSTATES = HEADERPINSTATES(2i32);
pub const EBHP_NORMAL: HEADERPINSTATES = HEADERPINSTATES(1i32);
pub const EBHP_PRESSED: HEADERPINSTATES = HEADERPINSTATES(3i32);
pub const EBHP_SELECTEDHOT: HEADERPINSTATES = HEADERPINSTATES(5i32);
pub const EBHP_SELECTEDNORMAL: HEADERPINSTATES = HEADERPINSTATES(4i32);
pub const EBHP_SELECTEDPRESSED: HEADERPINSTATES = HEADERPINSTATES(6i32);
pub const EBM_HOT: IEBARMENUSTATES = IEBARMENUSTATES(2i32);
pub const EBM_NORMAL: IEBARMENUSTATES = IEBARMENUSTATES(1i32);
pub const EBM_PRESSED: IEBARMENUSTATES = IEBARMENUSTATES(3i32);
pub const EBNGC_HOT: NORMALGROUPCOLLAPSESTATES = NORMALGROUPCOLLAPSESTATES(2i32);
pub const EBNGC_NORMAL: NORMALGROUPCOLLAPSESTATES = NORMALGROUPCOLLAPSESTATES(1i32);
pub const EBNGC_PRESSED: NORMALGROUPCOLLAPSESTATES = NORMALGROUPCOLLAPSESTATES(3i32);
pub const EBNGE_HOT: NORMALGROUPEXPANDSTATES = NORMALGROUPEXPANDSTATES(2i32);
pub const EBNGE_NORMAL: NORMALGROUPEXPANDSTATES = NORMALGROUPEXPANDSTATES(1i32);
pub const EBNGE_PRESSED: NORMALGROUPEXPANDSTATES = NORMALGROUPEXPANDSTATES(3i32);
pub const EBP_HEADERBACKGROUND: EXPLORERBARPARTS = EXPLORERBARPARTS(1i32);
pub const EBP_HEADERCLOSE: EXPLORERBARPARTS = EXPLORERBARPARTS(2i32);
pub const EBP_HEADERPIN: EXPLORERBARPARTS = EXPLORERBARPARTS(3i32);
pub const EBP_IEBARMENU: EXPLORERBARPARTS = EXPLORERBARPARTS(4i32);
pub const EBP_NORMALGROUPBACKGROUND: EXPLORERBARPARTS = EXPLORERBARPARTS(5i32);
pub const EBP_NORMALGROUPCOLLAPSE: EXPLORERBARPARTS = EXPLORERBARPARTS(6i32);
pub const EBP_NORMALGROUPEXPAND: EXPLORERBARPARTS = EXPLORERBARPARTS(7i32);
pub const EBP_NORMALGROUPHEAD: EXPLORERBARPARTS = EXPLORERBARPARTS(8i32);
pub const EBP_SPECIALGROUPBACKGROUND: EXPLORERBARPARTS = EXPLORERBARPARTS(9i32);
pub const EBP_SPECIALGROUPCOLLAPSE: EXPLORERBARPARTS = EXPLORERBARPARTS(10i32);
pub const EBP_SPECIALGROUPEXPAND: EXPLORERBARPARTS = EXPLORERBARPARTS(11i32);
pub const EBP_SPECIALGROUPHEAD: EXPLORERBARPARTS = EXPLORERBARPARTS(12i32);
pub const EBSGC_HOT: SPECIALGROUPCOLLAPSESTATES = SPECIALGROUPCOLLAPSESTATES(2i32);
pub const EBSGC_NORMAL: SPECIALGROUPCOLLAPSESTATES = SPECIALGROUPCOLLAPSESTATES(1i32);
pub const EBSGC_PRESSED: SPECIALGROUPCOLLAPSESTATES = SPECIALGROUPCOLLAPSESTATES(3i32);
pub const EBSGE_HOT: SPECIALGROUPEXPANDSTATES = SPECIALGROUPEXPANDSTATES(2i32);
pub const EBSGE_NORMAL: SPECIALGROUPEXPANDSTATES = SPECIALGROUPEXPANDSTATES(1i32);
pub const EBSGE_PRESSED: SPECIALGROUPEXPANDSTATES = SPECIALGROUPEXPANDSTATES(3i32);
pub const EBS_ASSIST: BACKGROUNDSTATES = BACKGROUNDSTATES(6i32);
pub const EBS_DISABLED: BACKGROUNDSTATES = BACKGROUNDSTATES(3i32);
pub const EBS_FOCUSED: BACKGROUNDSTATES = BACKGROUNDSTATES(4i32);
pub const EBS_HOT: BACKGROUNDSTATES = BACKGROUNDSTATES(2i32);
pub const EBS_NORMAL: BACKGROUNDSTATES = BACKGROUNDSTATES(1i32);
pub const EBS_READONLY: BACKGROUNDSTATES = BACKGROUNDSTATES(5i32);
pub const EBWBS_DISABLED: BACKGROUNDWITHBORDERSTATES = BACKGROUNDWITHBORDERSTATES(3i32);
pub const EBWBS_FOCUSED: BACKGROUNDWITHBORDERSTATES = BACKGROUNDWITHBORDERSTATES(4i32);
pub const EBWBS_HOT: BACKGROUNDWITHBORDERSTATES = BACKGROUNDWITHBORDERSTATES(2i32);
pub const EBWBS_NORMAL: BACKGROUNDWITHBORDERSTATES = BACKGROUNDWITHBORDERSTATES(1i32);
pub const ECM_FIRST: u32 = 5376u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_ENDOFLINE(pub i32);
pub const EC_ENDOFLINE_CR: EC_ENDOFLINE = EC_ENDOFLINE(2i32);
pub const EC_ENDOFLINE_CRLF: EC_ENDOFLINE = EC_ENDOFLINE(1i32);
pub const EC_ENDOFLINE_DETECTFROMCONTENT: EC_ENDOFLINE = EC_ENDOFLINE(0i32);
pub const EC_ENDOFLINE_LF: EC_ENDOFLINE = EC_ENDOFLINE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_SEARCHWEB_ENTRYPOINT(pub i32);
pub const EC_SEARCHWEB_ENTRYPOINT_CONTEXTMENU: EC_SEARCHWEB_ENTRYPOINT = EC_SEARCHWEB_ENTRYPOINT(1i32);
pub const EC_SEARCHWEB_ENTRYPOINT_EXTERNAL: EC_SEARCHWEB_ENTRYPOINT = EC_SEARCHWEB_ENTRYPOINT(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EDITBALLOONTIP {
    pub cbStruct: u32,
    pub pszTitle: windows_core::PCWSTR,
    pub pszText: windows_core::PCWSTR,
    pub ttiIcon: EDITBALLOONTIP_ICON,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDITBALLOONTIP_ICON(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDITBORDER_HSCROLLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDITBORDER_HVSCROLLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDITBORDER_NOSCROLLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDITBORDER_VSCROLLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDITPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDITTEXTSTATES(pub i32);
pub type EDITWORDBREAKPROCA = Option<unsafe extern "system" fn(lpch: windows_core::PCSTR, ichcurrent: i32, cch: i32, code: WORD_BREAK_ACTION) -> i32>;
pub type EDITWORDBREAKPROCW = Option<unsafe extern "system" fn(lpch: windows_core::PCWSTR, ichcurrent: i32, cch: i32, code: WORD_BREAK_ACTION) -> i32>;
pub const EMF_CENTERED: NMLVEMPTYMARKUP_FLAGS = NMLVEMPTYMARKUP_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EMPTYMARKUPPARTS(pub i32);
pub const EMP_MARKUPTEXT: EMPTYMARKUPPARTS = EMPTYMARKUPPARTS(1i32);
pub const EMT_LINKTEXT: MARKUPTEXTSTATES = MARKUPTEXTSTATES(2i32);
pub const EMT_NORMALTEXT: MARKUPTEXTSTATES = MARKUPTEXTSTATES(1i32);
pub const EM_CANUNDO: u32 = 198u32;
pub const EM_CHARFROMPOS: u32 = 215u32;
pub const EM_EMPTYUNDOBUFFER: u32 = 205u32;
pub const EM_ENABLEFEATURE: u32 = 218u32;
pub const EM_ENABLESEARCHWEB: u32 = 5390u32;
pub const EM_FILELINEFROMCHAR: u32 = 5395u32;
pub const EM_FILELINEINDEX: u32 = 5396u32;
pub const EM_FILELINELENGTH: u32 = 5397u32;
pub const EM_FMTLINES: u32 = 200u32;
pub const EM_GETCARETINDEX: u32 = 5394u32;
pub const EM_GETCUEBANNER: u32 = 5378u32;
pub const EM_GETENDOFLINE: u32 = 5389u32;
pub const EM_GETEXTENDEDSTYLE: u32 = 5387u32;
pub const EM_GETFILELINE: u32 = 5398u32;
pub const EM_GETFILELINECOUNT: u32 = 5399u32;
pub const EM_GETFIRSTVISIBLELINE: u32 = 206u32;
pub const EM_GETHANDLE: u32 = 189u32;
pub const EM_GETHILITE: u32 = 5382u32;
pub const EM_GETIMESTATUS: u32 = 217u32;
pub const EM_GETLIMITTEXT: u32 = 213u32;
pub const EM_GETLINE: u32 = 196u32;
pub const EM_GETLINECOUNT: u32 = 186u32;
pub const EM_GETMARGINS: u32 = 212u32;
pub const EM_GETMODIFY: u32 = 184u32;
pub const EM_GETPASSWORDCHAR: u32 = 210u32;
pub const EM_GETRECT: u32 = 178u32;
pub const EM_GETSEL: u32 = 176u32;
pub const EM_GETTHUMB: u32 = 190u32;
pub const EM_GETWORDBREAKPROC: u32 = 209u32;
pub const EM_HIDEBALLOONTIP: u32 = 5380u32;
pub const EM_LIMITTEXT: u32 = 197u32;
pub const EM_LINEFROMCHAR: u32 = 201u32;
pub const EM_LINEINDEX: u32 = 187u32;
pub const EM_LINELENGTH: u32 = 193u32;
pub const EM_LINESCROLL: u32 = 182u32;
pub const EM_NOSETFOCUS: u32 = 5383u32;
pub const EM_POSFROMCHAR: u32 = 214u32;
pub const EM_REPLACESEL: u32 = 194u32;
pub const EM_SCROLL: u32 = 181u32;
pub const EM_SCROLLCARET: u32 = 183u32;
pub const EM_SEARCHWEB: u32 = 5391u32;
pub const EM_SETCARETINDEX: u32 = 5393u32;
pub const EM_SETCUEBANNER: u32 = 5377u32;
pub const EM_SETENDOFLINE: u32 = 5388u32;
pub const EM_SETEXTENDEDSTYLE: u32 = 5386u32;
pub const EM_SETHANDLE: u32 = 188u32;
pub const EM_SETHILITE: u32 = 5381u32;
pub const EM_SETIMESTATUS: u32 = 216u32;
pub const EM_SETLIMITTEXT: u32 = 197u32;
pub const EM_SETMARGINS: u32 = 211u32;
pub const EM_SETMODIFY: u32 = 185u32;
pub const EM_SETPASSWORDCHAR: u32 = 204u32;
pub const EM_SETREADONLY: u32 = 207u32;
pub const EM_SETRECT: u32 = 179u32;
pub const EM_SETRECTNP: u32 = 180u32;
pub const EM_SETSEL: u32 = 177u32;
pub const EM_SETTABSTOPS: u32 = 203u32;
pub const EM_SETWORDBREAKPROC: u32 = 208u32;
pub const EM_SHOWBALLOONTIP: u32 = 5379u32;
pub const EM_TAKEFOCUS: u32 = 5384u32;
pub const EM_UNDO: u32 = 199u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENABLE_SCROLL_BAR_ARROWS(pub u32);
pub const EN_FIRST: u32 = 4294965776u32;
pub const EN_LAST: u32 = 4294965756u32;
pub const EN_SEARCHWEB: u32 = 4294965776u32;
pub const EPSHV_DISABLED: EDITBORDER_HVSCROLLSTATES = EDITBORDER_HVSCROLLSTATES(4i32);
pub const EPSHV_FOCUSED: EDITBORDER_HVSCROLLSTATES = EDITBORDER_HVSCROLLSTATES(3i32);
pub const EPSHV_HOT: EDITBORDER_HVSCROLLSTATES = EDITBORDER_HVSCROLLSTATES(2i32);
pub const EPSHV_NORMAL: EDITBORDER_HVSCROLLSTATES = EDITBORDER_HVSCROLLSTATES(1i32);
pub const EPSH_DISABLED: EDITBORDER_HSCROLLSTATES = EDITBORDER_HSCROLLSTATES(4i32);
pub const EPSH_FOCUSED: EDITBORDER_HSCROLLSTATES = EDITBORDER_HSCROLLSTATES(3i32);
pub const EPSH_HOT: EDITBORDER_HSCROLLSTATES = EDITBORDER_HSCROLLSTATES(2i32);
pub const EPSH_NORMAL: EDITBORDER_HSCROLLSTATES = EDITBORDER_HSCROLLSTATES(1i32);
pub const EPSN_DISABLED: EDITBORDER_NOSCROLLSTATES = EDITBORDER_NOSCROLLSTATES(4i32);
pub const EPSN_FOCUSED: EDITBORDER_NOSCROLLSTATES = EDITBORDER_NOSCROLLSTATES(3i32);
pub const EPSN_HOT: EDITBORDER_NOSCROLLSTATES = EDITBORDER_NOSCROLLSTATES(2i32);
pub const EPSN_NORMAL: EDITBORDER_NOSCROLLSTATES = EDITBORDER_NOSCROLLSTATES(1i32);
pub const EPSV_DISABLED: EDITBORDER_VSCROLLSTATES = EDITBORDER_VSCROLLSTATES(4i32);
pub const EPSV_FOCUSED: EDITBORDER_VSCROLLSTATES = EDITBORDER_VSCROLLSTATES(3i32);
pub const EPSV_HOT: EDITBORDER_VSCROLLSTATES = EDITBORDER_VSCROLLSTATES(2i32);
pub const EPSV_NORMAL: EDITBORDER_VSCROLLSTATES = EDITBORDER_VSCROLLSTATES(1i32);
pub const EP_BACKGROUND: EDITPARTS = EDITPARTS(3i32);
pub const EP_BACKGROUNDWITHBORDER: EDITPARTS = EDITPARTS(5i32);
pub const EP_CARET: EDITPARTS = EDITPARTS(2i32);
pub const EP_EDITBORDER_HSCROLL: EDITPARTS = EDITPARTS(7i32);
pub const EP_EDITBORDER_HVSCROLL: EDITPARTS = EDITPARTS(9i32);
pub const EP_EDITBORDER_NOSCROLL: EDITPARTS = EDITPARTS(6i32);
pub const EP_EDITBORDER_VSCROLL: EDITPARTS = EDITPARTS(8i32);
pub const EP_EDITTEXT: EDITPARTS = EDITPARTS(1i32);
pub const EP_PASSWORD: EDITPARTS = EDITPARTS(4i32);
pub const ESB_DISABLE_BOTH: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(3u32);
pub const ESB_DISABLE_DOWN: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_LEFT: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_DISABLE_LTUP: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_DISABLE_RIGHT: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_RTDN: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_UP: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_ENABLE_BOTH: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(0u32);
pub const ES_EX_ALLOWEOL_CR: i32 = 1i32;
pub const ES_EX_ALLOWEOL_LF: i32 = 2i32;
pub const ES_EX_CONVERT_EOL_ON_PASTE: i32 = 4i32;
pub const ES_EX_ZOOMABLE: i32 = 16i32;
pub const ETDT_DISABLE: u32 = 1u32;
pub const ETDT_ENABLE: u32 = 2u32;
pub const ETDT_USEAEROWIZARDTABTEXTURE: u32 = 8u32;
pub const ETDT_USETABTEXTURE: u32 = 4u32;
pub const ETS_ASSIST: EDITTEXTSTATES = EDITTEXTSTATES(7i32);
pub const ETS_CUEBANNER: EDITTEXTSTATES = EDITTEXTSTATES(8i32);
pub const ETS_DISABLED: EDITTEXTSTATES = EDITTEXTSTATES(4i32);
pub const ETS_FOCUSED: EDITTEXTSTATES = EDITTEXTSTATES(5i32);
pub const ETS_HOT: EDITTEXTSTATES = EDITTEXTSTATES(2i32);
pub const ETS_NORMAL: EDITTEXTSTATES = EDITTEXTSTATES(1i32);
pub const ETS_READONLY: EDITTEXTSTATES = EDITTEXTSTATES(6i32);
pub const ETS_SELECTED: EDITTEXTSTATES = EDITTEXTSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPANDBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPANDOBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPLORERBARPARTS(pub i32);
pub const FBS_EMPHASIZED: BODYSTATES = BODYSTATES(2i32);
pub const FBS_NORMAL: BODYSTATES = BODYSTATES(1i32);
pub const FEEDBACK_GESTURE_PRESSANDTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(11i32);
pub const FEEDBACK_MAX: FEEDBACK_TYPE = FEEDBACK_TYPE(-1i32);
pub const FEEDBACK_PEN_BARRELVISUALIZATION: FEEDBACK_TYPE = FEEDBACK_TYPE(2i32);
pub const FEEDBACK_PEN_DOUBLETAP: FEEDBACK_TYPE = FEEDBACK_TYPE(4i32);
pub const FEEDBACK_PEN_PRESSANDHOLD: FEEDBACK_TYPE = FEEDBACK_TYPE(5i32);
pub const FEEDBACK_PEN_RIGHTTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(6i32);
pub const FEEDBACK_PEN_TAP: FEEDBACK_TYPE = FEEDBACK_TYPE(3i32);
pub const FEEDBACK_TOUCH_CONTACTVISUALIZATION: FEEDBACK_TYPE = FEEDBACK_TYPE(1i32);
pub const FEEDBACK_TOUCH_DOUBLETAP: FEEDBACK_TYPE = FEEDBACK_TYPE(8i32);
pub const FEEDBACK_TOUCH_PRESSANDHOLD: FEEDBACK_TYPE = FEEDBACK_TYPE(9i32);
pub const FEEDBACK_TOUCH_RIGHTTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(10i32);
pub const FEEDBACK_TOUCH_TAP: FEEDBACK_TYPE = FEEDBACK_TYPE(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEEDBACK_TYPE(pub i32);
pub const FILEOPENORD: u32 = 1536u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILLTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILLVERTSTATES(pub i32);
pub const FINDDLGORD: u32 = 1540u32;
pub const FLH_HOVER: LINKHEADERSTATES = LINKHEADERSTATES(2i32);
pub const FLH_NORMAL: LINKHEADERSTATES = LINKHEADERSTATES(1i32);
pub const FLS_DISABLED: LABELSTATES = LABELSTATES(4i32);
pub const FLS_EMPHASIZED: LABELSTATES = LABELSTATES(3i32);
pub const FLS_NORMAL: LABELSTATES = LABELSTATES(1i32);
pub const FLS_SELECTED: LABELSTATES = LABELSTATES(2i32);
pub const FLYOUTLINK_HOVER: LINKSTATES = LINKSTATES(2i32);
pub const FLYOUTLINK_NORMAL: LINKSTATES = LINKSTATES(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FLYOUTPARTS(pub i32);
pub const FLYOUT_BODY: FLYOUTPARTS = FLYOUTPARTS(2i32);
pub const FLYOUT_DIVIDER: FLYOUTPARTS = FLYOUTPARTS(5i32);
pub const FLYOUT_HEADER: FLYOUTPARTS = FLYOUTPARTS(1i32);
pub const FLYOUT_LABEL: FLYOUTPARTS = FLYOUTPARTS(3i32);
pub const FLYOUT_LINK: FLYOUTPARTS = FLYOUTPARTS(4i32);
pub const FLYOUT_LINKAREA: FLYOUTPARTS = FLYOUTPARTS(7i32);
pub const FLYOUT_LINKHEADER: FLYOUTPARTS = FLYOUTPARTS(8i32);
pub const FLYOUT_WINDOW: FLYOUTPARTS = FLYOUTPARTS(6i32);
pub const FONTDLGORD: u32 = 1542u32;
pub const FORMATDLGORD30: u32 = 1544u32;
pub const FORMATDLGORD31: u32 = 1543u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FRAMEBOTTOMSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FRAMELEFTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FRAMERIGHTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FRAMESTATES(pub i32);
pub const FRB_ACTIVE: FRAMEBOTTOMSTATES = FRAMEBOTTOMSTATES(1i32);
pub const FRB_INACTIVE: FRAMEBOTTOMSTATES = FRAMEBOTTOMSTATES(2i32);
pub const FRL_ACTIVE: FRAMELEFTSTATES = FRAMELEFTSTATES(1i32);
pub const FRL_INACTIVE: FRAMELEFTSTATES = FRAMELEFTSTATES(2i32);
pub const FRR_ACTIVE: FRAMERIGHTSTATES = FRAMERIGHTSTATES(1i32);
pub const FRR_INACTIVE: FRAMERIGHTSTATES = FRAMERIGHTSTATES(2i32);
pub const FSB_ENCARTA_MODE: u32 = 1u32;
pub const FSB_FLAT_MODE: u32 = 2u32;
pub const FSB_REGULAR_MODE: u32 = 0u32;
pub const FS_ACTIVE: FRAMESTATES = FRAMESTATES(1i32);
pub const FS_INACTIVE: FRAMESTATES = FRAMESTATES(2i32);
pub const FT_HORZGRADIENT: FILLTYPE = FILLTYPE(2i32);
pub const FT_RADIALGRADIENT: FILLTYPE = FILLTYPE(3i32);
pub const FT_SOLID: FILLTYPE = FILLTYPE(0i32);
pub const FT_TILEIMAGE: FILLTYPE = FILLTYPE(4i32);
pub const FT_VERTGRADIENT: FILLTYPE = FILLTYPE(1i32);
pub const GBF_COPY: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(2u32);
pub const GBF_DIRECT: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(1u32);
pub const GBF_VALIDBITS: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(3u32);
pub const GBS_DISABLED: GROUPBOXSTATES = GROUPBOXSTATES(2i32);
pub const GBS_NORMAL: GROUPBOXSTATES = GROUPBOXSTATES(1i32);
pub const GDTR_MAX: u32 = 2u32;
pub const GDTR_MIN: u32 = 1u32;
pub const GDT_ERROR: i32 = -1i32;
pub const GDT_NONE: NMDATETIMECHANGE_FLAGS = NMDATETIMECHANGE_FLAGS(1u32);
pub const GDT_VALID: NMDATETIMECHANGE_FLAGS = NMDATETIMECHANGE_FLAGS(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_THEME_BITMAP_FLAGS(pub u32);
pub const GFST_DPI: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(2i32);
pub const GFST_NONE: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(0i32);
pub const GFST_SIZE: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(1i32);
pub const GLPS_CLOSED: GLYPHSTATES = GLYPHSTATES(1i32);
pub const GLPS_OPENED: GLYPHSTATES = GLYPHSTATES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLYPHFONTSIZINGTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLYPHSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLYPHTYPE(pub i32);
pub const GMR_DAYSTATE: u32 = 1u32;
pub const GMR_VISIBLE: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GRIDCELLBACKGROUNDSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GRIDCELLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GRIDCELLUPPERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GRIPPERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUPBOXSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUPHEADERLINESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUPHEADERSTATES(pub i32);
pub const GT_FONTGLYPH: GLYPHTYPE = GLYPHTYPE(2i32);
pub const GT_IMAGEGLYPH: GLYPHTYPE = GLYPHTYPE(1i32);
pub const GT_NONE: GLYPHTYPE = GLYPHTYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HALIGN(pub i32);
pub const HA_CENTER: HALIGN = HALIGN(1i32);
pub const HA_LEFT: HALIGN = HALIGN(0i32);
pub const HA_RIGHT: HALIGN = HALIGN(2i32);
pub const HBG_DETAILS: HEADERSTYLESTATES = HEADERSTYLESTATES(1i32);
pub const HBG_ICON: HEADERSTYLESTATES = HEADERSTYLESTATES(2i32);
pub const HBS_DISABLED: HELPBUTTONSTATES = HELPBUTTONSTATES(4i32);
pub const HBS_HOT: HELPBUTTONSTATES = HELPBUTTONSTATES(2i32);
pub const HBS_NORMAL: HELPBUTTONSTATES = HELPBUTTONSTATES(1i32);
pub const HBS_PUSHED: HELPBUTTONSTATES = HELPBUTTONSTATES(3i32);
pub const HDDFS_HOT: HEADERDROPDOWNFILTERSTATES = HEADERDROPDOWNFILTERSTATES(3i32);
pub const HDDFS_NORMAL: HEADERDROPDOWNFILTERSTATES = HEADERDROPDOWNFILTERSTATES(1i32);
pub const HDDFS_SOFTHOT: HEADERDROPDOWNFILTERSTATES = HEADERDROPDOWNFILTERSTATES(2i32);
pub const HDDS_HOT: HEADERDROPDOWNSTATES = HEADERDROPDOWNSTATES(3i32);
pub const HDDS_NORMAL: HEADERDROPDOWNSTATES = HEADERDROPDOWNSTATES(1i32);
pub const HDDS_SOFTHOT: HEADERDROPDOWNSTATES = HEADERDROPDOWNSTATES(2i32);
pub const HDFT_HASNOVALUE: HEADER_CONTROL_FORMAT_TYPE = HEADER_CONTROL_FORMAT_TYPE(32768u32);
pub const HDFT_ISDATE: HEADER_CONTROL_FORMAT_TYPE = HEADER_CONTROL_FORMAT_TYPE(2u32);
pub const HDFT_ISNUMBER: HEADER_CONTROL_FORMAT_TYPE = HEADER_CONTROL_FORMAT_TYPE(1u32);
pub const HDFT_ISSTRING: HEADER_CONTROL_FORMAT_TYPE = HEADER_CONTROL_FORMAT_TYPE(0u32);
pub const HDF_BITMAP: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(8192i32);
pub const HDF_BITMAP_ON_RIGHT: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(4096i32);
pub const HDF_CENTER: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(2i32);
pub const HDF_CHECKBOX: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(64i32);
pub const HDF_CHECKED: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(128i32);
pub const HDF_FIXEDWIDTH: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(256i32);
pub const HDF_IMAGE: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(2048i32);
pub const HDF_JUSTIFYMASK: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(3i32);
pub const HDF_LEFT: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(0i32);
pub const HDF_OWNERDRAW: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(32768i32);
pub const HDF_RIGHT: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(1i32);
pub const HDF_RTLREADING: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(4i32);
pub const HDF_SORTDOWN: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(512i32);
pub const HDF_SORTUP: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(1024i32);
pub const HDF_SPLITBUTTON: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(16777216i32);
pub const HDF_STRING: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(16384i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HDHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: HEADER_HITTEST_INFO_FLAGS,
    pub iItem: i32,
}
pub const HDIS_FOCUSED: HEADER_CONTROL_FORMAT_STATE = HEADER_CONTROL_FORMAT_STATE(1u32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HDITEMA {
    pub mask: HDI_MASK,
    pub cxy: i32,
    pub pszText: windows_core::PSTR,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: HEADER_CONTROL_FORMAT_FLAGS,
    pub lParam: super::super::Foundation::LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub r#type: HEADER_CONTROL_FORMAT_TYPE,
    pub pvFilter: *mut core::ffi::c_void,
    pub state: HEADER_CONTROL_FORMAT_STATE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for HDITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HDITEMW {
    pub mask: HDI_MASK,
    pub cxy: i32,
    pub pszText: windows_core::PWSTR,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: HEADER_CONTROL_FORMAT_FLAGS,
    pub lParam: super::super::Foundation::LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub r#type: HEADER_CONTROL_FORMAT_TYPE,
    pub pvFilter: *mut core::ffi::c_void,
    pub state: HEADER_CONTROL_FORMAT_STATE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for HDITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HDI_BITMAP: HDI_MASK = HDI_MASK(16u32);
pub const HDI_DI_SETITEM: HDI_MASK = HDI_MASK(64u32);
pub const HDI_FILTER: HDI_MASK = HDI_MASK(256u32);
pub const HDI_FORMAT: HDI_MASK = HDI_MASK(4u32);
pub const HDI_HEIGHT: HDI_MASK = HDI_MASK(1u32);
pub const HDI_IMAGE: HDI_MASK = HDI_MASK(32u32);
pub const HDI_LPARAM: HDI_MASK = HDI_MASK(8u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HDI_MASK(pub u32);
impl HDI_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HDI_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HDI_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HDI_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HDI_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HDI_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const HDI_ORDER: HDI_MASK = HDI_MASK(128u32);
pub const HDI_STATE: HDI_MASK = HDI_MASK(512u32);
pub const HDI_TEXT: HDI_MASK = HDI_MASK(2u32);
pub const HDI_WIDTH: HDI_MASK = HDI_MASK(1u32);
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HDLAYOUT {
    pub prc: *mut super::super::Foundation::RECT,
    pub pwpos: *mut super::WindowsAndMessaging::WINDOWPOS,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for HDLAYOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HDM_CLEARFILTER: u32 = 4632u32;
pub const HDM_CREATEDRAGIMAGE: u32 = 4624u32;
pub const HDM_DELETEITEM: u32 = 4610u32;
pub const HDM_EDITFILTER: u32 = 4631u32;
pub const HDM_FIRST: u32 = 4608u32;
pub const HDM_GETBITMAPMARGIN: u32 = 4629u32;
pub const HDM_GETFOCUSEDITEM: u32 = 4635u32;
pub const HDM_GETIMAGELIST: u32 = 4617u32;
pub const HDM_GETITEM: u32 = 4619u32;
pub const HDM_GETITEMA: u32 = 4611u32;
pub const HDM_GETITEMCOUNT: u32 = 4608u32;
pub const HDM_GETITEMDROPDOWNRECT: u32 = 4633u32;
pub const HDM_GETITEMRECT: u32 = 4615u32;
pub const HDM_GETITEMW: u32 = 4619u32;
pub const HDM_GETORDERARRAY: u32 = 4625u32;
pub const HDM_GETOVERFLOWRECT: u32 = 4634u32;
pub const HDM_GETUNICODEFORMAT: u32 = 8198u32;
pub const HDM_HITTEST: u32 = 4614u32;
pub const HDM_INSERTITEM: u32 = 4618u32;
pub const HDM_INSERTITEMA: u32 = 4609u32;
pub const HDM_INSERTITEMW: u32 = 4618u32;
pub const HDM_LAYOUT: u32 = 4613u32;
pub const HDM_ORDERTOINDEX: u32 = 4623u32;
pub const HDM_SETBITMAPMARGIN: u32 = 4628u32;
pub const HDM_SETFILTERCHANGETIMEOUT: u32 = 4630u32;
pub const HDM_SETFOCUSEDITEM: u32 = 4636u32;
pub const HDM_SETHOTDIVIDER: u32 = 4627u32;
pub const HDM_SETIMAGELIST: u32 = 4616u32;
pub const HDM_SETITEM: u32 = 4620u32;
pub const HDM_SETITEMA: u32 = 4612u32;
pub const HDM_SETITEMW: u32 = 4620u32;
pub const HDM_SETORDERARRAY: u32 = 4626u32;
pub const HDM_SETUNICODEFORMAT: u32 = 8197u32;
pub const HDN_BEGINDRAG: u32 = 4294966986u32;
pub const HDN_BEGINFILTEREDIT: u32 = 4294966982u32;
pub const HDN_BEGINTRACK: u32 = 4294966970u32;
pub const HDN_BEGINTRACKA: u32 = 4294966990u32;
pub const HDN_BEGINTRACKW: u32 = 4294966970u32;
pub const HDN_DIVIDERDBLCLICK: u32 = 4294966971u32;
pub const HDN_DIVIDERDBLCLICKA: u32 = 4294966991u32;
pub const HDN_DIVIDERDBLCLICKW: u32 = 4294966971u32;
pub const HDN_DROPDOWN: u32 = 4294966978u32;
pub const HDN_ENDDRAG: u32 = 4294966985u32;
pub const HDN_ENDFILTEREDIT: u32 = 4294966981u32;
pub const HDN_ENDTRACK: u32 = 4294966969u32;
pub const HDN_ENDTRACKA: u32 = 4294966989u32;
pub const HDN_ENDTRACKW: u32 = 4294966969u32;
pub const HDN_FILTERBTNCLICK: u32 = 4294966983u32;
pub const HDN_FILTERCHANGE: u32 = 4294966984u32;
pub const HDN_FIRST: u32 = 4294966996u32;
pub const HDN_GETDISPINFO: u32 = 4294966967u32;
pub const HDN_GETDISPINFOA: u32 = 4294966987u32;
pub const HDN_GETDISPINFOW: u32 = 4294966967u32;
pub const HDN_ITEMCHANGED: u32 = 4294966975u32;
pub const HDN_ITEMCHANGEDA: u32 = 4294966995u32;
pub const HDN_ITEMCHANGEDW: u32 = 4294966975u32;
pub const HDN_ITEMCHANGING: u32 = 4294966976u32;
pub const HDN_ITEMCHANGINGA: u32 = 4294966996u32;
pub const HDN_ITEMCHANGINGW: u32 = 4294966976u32;
pub const HDN_ITEMCLICK: u32 = 4294966974u32;
pub const HDN_ITEMCLICKA: u32 = 4294966994u32;
pub const HDN_ITEMCLICKW: u32 = 4294966974u32;
pub const HDN_ITEMDBLCLICK: u32 = 4294966973u32;
pub const HDN_ITEMDBLCLICKA: u32 = 4294966993u32;
pub const HDN_ITEMDBLCLICKW: u32 = 4294966973u32;
pub const HDN_ITEMKEYDOWN: u32 = 4294966979u32;
pub const HDN_ITEMSTATEICONCLICK: u32 = 4294966980u32;
pub const HDN_LAST: u32 = 4294966897u32;
pub const HDN_OVERFLOWCLICK: u32 = 4294966977u32;
pub const HDN_TRACK: u32 = 4294966968u32;
pub const HDN_TRACKA: u32 = 4294966988u32;
pub const HDN_TRACKW: u32 = 4294966968u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HDPA(pub isize);
impl HDPA {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HDPA {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("comctl32.dll" "system" fn DPA_Destroy(hdpa : isize) -> i32);
            unsafe {
                DPA_Destroy(self.0);
            }
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HDSA(pub isize);
impl HDSA {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HDSA {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("comctl32.dll" "system" fn DSA_Destroy(hdsa : isize) -> i32);
            unsafe {
                DSA_Destroy(self.0);
            }
        }
    }
}
pub const HDSIL_NORMAL: u32 = 0u32;
pub const HDSIL_STATE: u32 = 1u32;
pub const HDS_BUTTONS: u32 = 2u32;
pub const HDS_CHECKBOXES: u32 = 1024u32;
pub const HDS_DRAGDROP: u32 = 64u32;
pub const HDS_FILTERBAR: u32 = 256u32;
pub const HDS_FLAT: u32 = 512u32;
pub const HDS_FULLDRAG: u32 = 128u32;
pub const HDS_HIDDEN: u32 = 8u32;
pub const HDS_HORZ: u32 = 0u32;
pub const HDS_HOTTRACK: u32 = 4u32;
pub const HDS_NOSIZING: u32 = 2048u32;
pub const HDS_OVERFLOW: u32 = 4096u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HD_TEXTFILTERA {
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HD_TEXTFILTERW {
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERAREASTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERCLOSESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERDROPDOWNFILTERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERDROPDOWNSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERITEMLEFTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERITEMRIGHTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERITEMSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADEROVERFLOWSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERPINSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERSORTARROWSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADERSTYLESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADER_CONTROL_FORMAT_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADER_CONTROL_FORMAT_STATE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADER_CONTROL_FORMAT_TYPE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADER_CONTROL_NOTIFICATION_BUTTON(pub i32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_LEFT: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(0i32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_MIDDLE: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(2i32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_RIGHT: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEADER_HITTEST_INFO_FLAGS(pub u32);
impl HEADER_HITTEST_INFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HEADER_HITTEST_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HEADER_HITTEST_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HEADER_HITTEST_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HEADER_HITTEST_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HEADER_HITTEST_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HELPBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HELPLINKSTATES(pub i32);
pub const HGLPS_CLOSED: HOTGLYPHSTATES = HOTGLYPHSTATES(1i32);
pub const HGLPS_OPENED: HOTGLYPHSTATES = HOTGLYPHSTATES(2i32);
pub const HHT_ABOVE: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(256u32);
pub const HHT_BELOW: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(512u32);
pub const HHT_NOWHERE: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(1u32);
pub const HHT_ONDIVIDER: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(4u32);
pub const HHT_ONDIVOPEN: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(8u32);
pub const HHT_ONDROPDOWN: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(8192u32);
pub const HHT_ONFILTER: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(16u32);
pub const HHT_ONFILTERBUTTON: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(32u32);
pub const HHT_ONHEADER: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(2u32);
pub const HHT_ONITEMSTATEICON: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(4096u32);
pub const HHT_ONOVERFLOW: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(16384u32);
pub const HHT_TOLEFT: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(2048u32);
pub const HHT_TORIGHT: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(1024u32);
pub const HICF_ACCELERATOR: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(4u32);
pub const HICF_ARROWKEYS: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(2u32);
pub const HICF_DUPACCEL: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(8u32);
pub const HICF_ENTERING: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(16u32);
pub const HICF_LEAVING: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(32u32);
pub const HICF_LMOUSE: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(128u32);
pub const HICF_MOUSE: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(1u32);
pub const HICF_OTHER: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(0u32);
pub const HICF_RESELECT: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(64u32);
pub const HICF_TOGGLEDROPDOWN: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(256u32);
pub const HILS_HOT: HEADERITEMLEFTSTATES = HEADERITEMLEFTSTATES(2i32);
pub const HILS_NORMAL: HEADERITEMLEFTSTATES = HEADERITEMLEFTSTATES(1i32);
pub const HILS_PRESSED: HEADERITEMLEFTSTATES = HEADERITEMLEFTSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HIMAGELIST(pub isize);
impl HIMAGELIST {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HIMAGELIST {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("comctl32.dll" "system" fn ImageList_Destroy(himl : isize) -> i32);
            unsafe {
                ImageList_Destroy(self.0);
            }
        }
    }
}
pub const HIRS_HOT: HEADERITEMRIGHTSTATES = HEADERITEMRIGHTSTATES(2i32);
pub const HIRS_NORMAL: HEADERITEMRIGHTSTATES = HEADERITEMRIGHTSTATES(1i32);
pub const HIRS_PRESSED: HEADERITEMRIGHTSTATES = HEADERITEMRIGHTSTATES(3i32);
pub const HIST_ADDTOFAVORITES: u32 = 3u32;
pub const HIST_BACK: u32 = 0u32;
pub const HIST_FAVORITES: u32 = 2u32;
pub const HIST_FORWARD: u32 = 1u32;
pub const HIST_VIEWTREE: u32 = 4u32;
pub const HIS_HOT: HEADERITEMSTATES = HEADERITEMSTATES(2i32);
pub const HIS_ICONHOT: HEADERITEMSTATES = HEADERITEMSTATES(8i32);
pub const HIS_ICONNORMAL: HEADERITEMSTATES = HEADERITEMSTATES(7i32);
pub const HIS_ICONPRESSED: HEADERITEMSTATES = HEADERITEMSTATES(9i32);
pub const HIS_ICONSORTEDHOT: HEADERITEMSTATES = HEADERITEMSTATES(11i32);
pub const HIS_ICONSORTEDNORMAL: HEADERITEMSTATES = HEADERITEMSTATES(10i32);
pub const HIS_ICONSORTEDPRESSED: HEADERITEMSTATES = HEADERITEMSTATES(12i32);
pub const HIS_NORMAL: HEADERITEMSTATES = HEADERITEMSTATES(1i32);
pub const HIS_PRESSED: HEADERITEMSTATES = HEADERITEMSTATES(3i32);
pub const HIS_SORTEDHOT: HEADERITEMSTATES = HEADERITEMSTATES(5i32);
pub const HIS_SORTEDNORMAL: HEADERITEMSTATES = HEADERITEMSTATES(4i32);
pub const HIS_SORTEDPRESSED: HEADERITEMSTATES = HEADERITEMSTATES(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HIT_TEST_BACKGROUND_OPTIONS(pub u32);
pub const HKCOMB_A: u32 = 8u32;
pub const HKCOMB_C: u32 = 4u32;
pub const HKCOMB_CA: u32 = 64u32;
pub const HKCOMB_NONE: u32 = 1u32;
pub const HKCOMB_S: u32 = 2u32;
pub const HKCOMB_SA: u32 = 32u32;
pub const HKCOMB_SC: u32 = 16u32;
pub const HKCOMB_SCA: u32 = 128u32;
pub const HKM_GETHOTKEY: u32 = 1026u32;
pub const HKM_SETHOTKEY: u32 = 1025u32;
pub const HKM_SETRULES: u32 = 1027u32;
pub const HLS_LINKTEXT: HYPERLINKSTATES = HYPERLINKSTATES(2i32);
pub const HLS_NORMALTEXT: HYPERLINKSTATES = HYPERLINKSTATES(1i32);
pub const HOFS_HOT: HEADEROVERFLOWSTATES = HEADEROVERFLOWSTATES(2i32);
pub const HOFS_NORMAL: HEADEROVERFLOWSTATES = HEADEROVERFLOWSTATES(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HORZSCROLLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HORZTHUMBSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HOTGLYPHSTATES(pub i32);
pub const HOTKEYF_ALT: u32 = 4u32;
pub const HOTKEYF_CONTROL: u32 = 2u32;
pub const HOTKEYF_EXT: u32 = 8u32;
pub const HOTKEYF_SHIFT: u32 = 1u32;
pub const HOTKEY_CLASS: windows_core::PCWSTR = windows_core::w!("msctls_hotkey32");
pub const HOTKEY_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_hotkey32");
pub const HOTKEY_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_hotkey32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HOVERBACKGROUNDSTATES(pub i32);
pub const HOVER_DEFAULT: u32 = 4294967295u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPROPSHEETPAGE(pub *mut core::ffi::c_void);
impl HPROPSHEETPAGE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HPROPSHEETPAGE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("comctl32.dll" "system" fn DestroyPropertySheetPage(param0 : *mut core::ffi::c_void) -> i32);
            unsafe {
                DestroyPropertySheetPage(self.0);
            }
        }
    }
}
impl Default for HPROPSHEETPAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HP_HEADERDROPDOWN: HEADERPARTS = HEADERPARTS(5i32);
pub const HP_HEADERDROPDOWNFILTER: HEADERPARTS = HEADERPARTS(6i32);
pub const HP_HEADERITEM: HEADERPARTS = HEADERPARTS(1i32);
pub const HP_HEADERITEMLEFT: HEADERPARTS = HEADERPARTS(2i32);
pub const HP_HEADERITEMRIGHT: HEADERPARTS = HEADERPARTS(3i32);
pub const HP_HEADEROVERFLOW: HEADERPARTS = HEADERPARTS(7i32);
pub const HP_HEADERSORTARROW: HEADERPARTS = HEADERPARTS(4i32);
pub const HSAS_SORTEDDOWN: HEADERSORTARROWSTATES = HEADERSORTARROWSTATES(2i32);
pub const HSAS_SORTEDUP: HEADERSORTARROWSTATES = HEADERSORTARROWSTATES(1i32);
pub const HSS_DISABLED: HORZSCROLLSTATES = HORZSCROLLSTATES(4i32);
pub const HSS_HOT: HORZSCROLLSTATES = HORZSCROLLSTATES(2i32);
pub const HSS_NORMAL: HORZSCROLLSTATES = HORZSCROLLSTATES(1i32);
pub const HSS_PUSHED: HORZSCROLLSTATES = HORZSCROLLSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSYNTHETICPOINTERDEVICE(pub *mut core::ffi::c_void);
impl HSYNTHETICPOINTERDEVICE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HSYNTHETICPOINTERDEVICE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("user32.dll" "system" fn DestroySyntheticPointerDevice(device : *mut core::ffi::c_void));
            unsafe {
                DestroySyntheticPointerDevice(self.0);
            }
        }
    }
}
impl Default for HSYNTHETICPOINTERDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HTHEME(pub isize);
impl HTHEME {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for HTHEME {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("uxtheme.dll" "system" fn CloseThemeData(htheme : isize) -> i32);
            unsafe {
                CloseThemeData(self.0);
            }
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HTREEITEM(pub isize);
pub const HTS_DISABLED: HORZTHUMBSTATES = HORZTHUMBSTATES(4i32);
pub const HTS_HOT: HORZTHUMBSTATES = HORZTHUMBSTATES(2i32);
pub const HTS_NORMAL: HORZTHUMBSTATES = HORZTHUMBSTATES(1i32);
pub const HTS_PUSHED: HORZTHUMBSTATES = HORZTHUMBSTATES(3i32);
pub const HTTB_BACKGROUNDSEG: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(0u32);
pub const HTTB_CAPTION: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(4u32);
pub const HTTB_FIXEDBORDER: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(2u32);
pub const HTTB_RESIZINGBORDER: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(240u32);
pub const HTTB_RESIZINGBORDER_BOTTOM: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(128u32);
pub const HTTB_RESIZINGBORDER_LEFT: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(16u32);
pub const HTTB_RESIZINGBORDER_RIGHT: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(64u32);
pub const HTTB_RESIZINGBORDER_TOP: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(32u32);
pub const HTTB_SIZINGTEMPLATE: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(256u32);
pub const HTTB_SYSTEMSIZINGMARGINS: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(512u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HYPERLINKSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HYPERLINKTEXTSTATES(pub i32);
pub const ICC_ANIMATE_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(128u32);
pub const ICC_BAR_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(4u32);
pub const ICC_COOL_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(1024u32);
pub const ICC_DATE_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(256u32);
pub const ICC_HOTKEY_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(64u32);
pub const ICC_INTERNET_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(2048u32);
pub const ICC_LINK_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(32768u32);
pub const ICC_LISTVIEW_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(1u32);
pub const ICC_NATIVEFNTCTL_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(8192u32);
pub const ICC_PAGESCROLLER_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(4096u32);
pub const ICC_PROGRESS_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(32u32);
pub const ICC_STANDARD_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(16384u32);
pub const ICC_TAB_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(8u32);
pub const ICC_TREEVIEW_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(2u32);
pub const ICC_UPDOWN_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(16u32);
pub const ICC_USEREX_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(512u32);
pub const ICC_WIN95_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(255u32);
pub const ICE_ALPHA: ICONEFFECT = ICONEFFECT(4i32);
pub const ICE_GLOW: ICONEFFECT = ICONEFFECT(1i32);
pub const ICE_NONE: ICONEFFECT = ICONEFFECT(0i32);
pub const ICE_PULSE: ICONEFFECT = ICONEFFECT(3i32);
pub const ICE_SHADOW: ICONEFFECT = ICONEFFECT(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ICONEFFECT(pub i32);
pub const IDB_HIST_DISABLED: u32 = 14u32;
pub const IDB_HIST_HOT: u32 = 13u32;
pub const IDB_HIST_LARGE_COLOR: u32 = 9u32;
pub const IDB_HIST_NORMAL: u32 = 12u32;
pub const IDB_HIST_PRESSED: u32 = 15u32;
pub const IDB_HIST_SMALL_COLOR: u32 = 8u32;
pub const IDB_STD_LARGE_COLOR: u32 = 1u32;
pub const IDB_STD_SMALL_COLOR: u32 = 0u32;
pub const IDB_VIEW_LARGE_COLOR: u32 = 5u32;
pub const IDB_VIEW_SMALL_COLOR: u32 = 4u32;
pub const IDC_MANAGE_LINK: u32 = 1592u32;
pub const ID_PSRESTARTWINDOWS: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IEBARMENUSTATES(pub i32);
windows_core::imp::define_interface!(IImageList, IImageList_Vtbl, 0x46eb5926_582e_4017_9fdf_e8998daa0950);
windows_core::imp::interface_hierarchy!(IImageList, windows_core::IUnknown);
impl IImageList {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Add(&self, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: Option<super::super::Graphics::Gdi::HBITMAP>) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn ReplaceIcon(&self, i: i32, hicon: super::WindowsAndMessaging::HICON) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReplaceIcon)(windows_core::Interface::as_raw(self), i, hicon, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOverlayImage)(windows_core::Interface::as_raw(self), iimage, ioverlay).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Replace(&self, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: Option<super::super::Graphics::Gdi::HBITMAP>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Replace)(windows_core::Interface::as_raw(self), i, hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddMasked(&self, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: super::super::Foundation::COLORREF) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddMasked)(windows_core::Interface::as_raw(self), hbmimage, crmask, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Draw(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), pimldp).ok() }
    }
    pub unsafe fn Remove(&self, i: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), i).ok() }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self, i: i32, flags: u32) -> windows_core::Result<super::WindowsAndMessaging::HICON> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIcon)(windows_core::Interface::as_raw(self), i, flags, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImageInfo(&self, i: i32, pimageinfo: *mut IMAGEINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetImageInfo)(windows_core::Interface::as_raw(self), i, pimageinfo as _).ok() }
    }
    pub unsafe fn Copy<P1>(&self, idst: i32, punksrc: P1, isrc: i32, uflags: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), idst, punksrc.param().abi(), isrc, uflags).ok() }
    }
    pub unsafe fn Merge<P1>(&self, i1: i32, punk2: P1, i2: i32, dx: i32, dy: i32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Merge)(windows_core::Interface::as_raw(self), i1, punk2.param().abi(), i2, dx, dy, riid, ppv as _).ok() }
    }
    pub unsafe fn Clone(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), riid, ppv as _).ok() }
    }
    pub unsafe fn GetImageRect(&self, i: i32) -> windows_core::Result<super::super::Foundation::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageRect)(windows_core::Interface::as_raw(self), i, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIconSize)(windows_core::Interface::as_raw(self), cx as _, cy as _).ok() }
    }
    pub unsafe fn SetIconSize(&self, cx: i32, cy: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIconSize)(windows_core::Interface::as_raw(self), cx, cy).ok() }
    }
    pub unsafe fn GetImageCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetImageCount(&self, unewcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetImageCount)(windows_core::Interface::as_raw(self), unewcount).ok() }
    }
    pub unsafe fn SetBkColor(&self, clrbk: super::super::Foundation::COLORREF) -> windows_core::Result<super::super::Foundation::COLORREF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetBkColor)(windows_core::Interface::as_raw(self), clrbk, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBkColor(&self) -> windows_core::Result<super::super::Foundation::COLORREF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBkColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginDrag)(windows_core::Interface::as_raw(self), itrack, dxhotspot, dyhotspot).ok() }
    }
    pub unsafe fn EndDrag(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndDrag)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DragEnter(&self, hwndlock: Option<super::super::Foundation::HWND>, x: i32, y: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DragEnter)(windows_core::Interface::as_raw(self), hwndlock.unwrap_or(core::mem::zeroed()) as _, x, y).ok() }
    }
    pub unsafe fn DragLeave(&self, hwndlock: Option<super::super::Foundation::HWND>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DragLeave)(windows_core::Interface::as_raw(self), hwndlock.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn DragMove(&self, x: i32, y: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DragMove)(windows_core::Interface::as_raw(self), x, y).ok() }
    }
    pub unsafe fn SetDragCursorImage<P0>(&self, punk: P0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDragCursorImage)(windows_core::Interface::as_raw(self), punk.param().abi(), idrag, dxhotspot, dyhotspot).ok() }
    }
    pub unsafe fn DragShowNolock(&self, fshow: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DragShowNolock)(windows_core::Interface::as_raw(self), fshow.into()).ok() }
    }
    pub unsafe fn GetDragImage(&self, ppt: Option<*mut super::super::Foundation::POINT>, ppthotspot: Option<*mut super::super::Foundation::POINT>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDragImage)(windows_core::Interface::as_raw(self), ppt.unwrap_or(core::mem::zeroed()) as _, ppthotspot.unwrap_or(core::mem::zeroed()) as _, riid, ppv as _).ok() }
    }
    pub unsafe fn GetItemFlags(&self, i: i32) -> windows_core::Result<IMAGE_LIST_ITEM_FLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemFlags)(windows_core::Interface::as_raw(self), i, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOverlayImage(&self, ioverlay: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOverlayImage)(windows_core::Interface::as_raw(self), ioverlay, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HBITMAP, super::super::Graphics::Gdi::HBITMAP, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Add: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub ReplaceIcon: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::WindowsAndMessaging::HICON, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    ReplaceIcon: usize,
    pub SetOverlayImage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Replace: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Graphics::Gdi::HBITMAP, super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Replace: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddMasked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HBITMAP, super::super::Foundation::COLORREF, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddMasked: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *const IMAGELISTDRAWPARAMS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Draw: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut super::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetImageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut IMAGEINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetImageInfo: usize,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub Merge: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, i32, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImageRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetIconSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetIconSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetImageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetImageCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetBkColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::COLORREF, *mut super::super::Foundation::COLORREF) -> windows_core::HRESULT,
    pub GetBkColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::COLORREF) -> windows_core::HRESULT,
    pub BeginDrag: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub EndDrag: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DragEnter: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, i32, i32) -> windows_core::HRESULT,
    pub DragLeave: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub DragMove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetDragCursorImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub DragShowNolock: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetDragImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::POINT, *mut super::super::Foundation::POINT, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut IMAGE_LIST_ITEM_FLAGS) -> windows_core::HRESULT,
    pub GetOverlayImage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImageList_Impl: windows_core::IUnknownImpl {
    fn Add(&self, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<i32>;
    fn ReplaceIcon(&self, i: i32, hicon: super::WindowsAndMessaging::HICON) -> windows_core::Result<i32>;
    fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> windows_core::Result<()>;
    fn Replace(&self, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>;
    fn AddMasked(&self, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: super::super::Foundation::COLORREF) -> windows_core::Result<i32>;
    fn Draw(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::Result<()>;
    fn Remove(&self, i: i32) -> windows_core::Result<()>;
    fn GetIcon(&self, i: i32, flags: u32) -> windows_core::Result<super::WindowsAndMessaging::HICON>;
    fn GetImageInfo(&self, i: i32, pimageinfo: *mut IMAGEINFO) -> windows_core::Result<()>;
    fn Copy(&self, idst: i32, punksrc: windows_core::Ref<windows_core::IUnknown>, isrc: i32, uflags: u32) -> windows_core::Result<()>;
    fn Merge(&self, i1: i32, punk2: windows_core::Ref<windows_core::IUnknown>, i2: i32, dx: i32, dy: i32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Clone(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetImageRect(&self, i: i32) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> windows_core::Result<()>;
    fn SetIconSize(&self, cx: i32, cy: i32) -> windows_core::Result<()>;
    fn GetImageCount(&self) -> windows_core::Result<i32>;
    fn SetImageCount(&self, unewcount: u32) -> windows_core::Result<()>;
    fn SetBkColor(&self, clrbk: super::super::Foundation::COLORREF) -> windows_core::Result<super::super::Foundation::COLORREF>;
    fn GetBkColor(&self) -> windows_core::Result<super::super::Foundation::COLORREF>;
    fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::Result<()>;
    fn EndDrag(&self) -> windows_core::Result<()>;
    fn DragEnter(&self, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> windows_core::Result<()>;
    fn DragLeave(&self, hwndlock: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn DragMove(&self, x: i32, y: i32) -> windows_core::Result<()>;
    fn SetDragCursorImage(&self, punk: windows_core::Ref<windows_core::IUnknown>, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::Result<()>;
    fn DragShowNolock(&self, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetDragImage(&self, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetItemFlags(&self, i: i32) -> windows_core::Result<IMAGE_LIST_ITEM_FLAGS>;
    fn GetOverlayImage(&self, ioverlay: i32) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IImageList_Vtbl {
    pub const fn new<Identity: IImageList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, pi: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::Add(this, core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&hbmmask)) {
                    Ok(ok__) => {
                        pi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReplaceIcon<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, hicon: super::WindowsAndMessaging::HICON, pi: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::ReplaceIcon(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&hicon)) {
                    Ok(ok__) => {
                        pi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverlayImage<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, ioverlay: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::SetOverlayImage(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&ioverlay)).into()
            }
        }
        unsafe extern "system" fn Replace<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Replace(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&hbmmask)).into()
            }
        }
        unsafe extern "system" fn AddMasked<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: super::super::Foundation::COLORREF, pi: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::AddMasked(this, core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&crmask)) {
                    Ok(ok__) => {
                        pi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Draw<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Draw(this, core::mem::transmute_copy(&pimldp)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Remove(this, core::mem::transmute_copy(&i)).into()
            }
        }
        unsafe extern "system" fn GetIcon<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, flags: u32, picon: *mut super::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetIcon(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        picon.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImageInfo<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, pimageinfo: *mut IMAGEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::GetImageInfo(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&pimageinfo)).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idst: i32, punksrc: *mut core::ffi::c_void, isrc: i32, uflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Copy(this, core::mem::transmute_copy(&idst), core::mem::transmute_copy(&punksrc), core::mem::transmute_copy(&isrc), core::mem::transmute_copy(&uflags)).into()
            }
        }
        unsafe extern "system" fn Merge<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i1: i32, punk2: *mut core::ffi::c_void, i2: i32, dx: i32, dy: i32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Merge(this, core::mem::transmute_copy(&i1), core::mem::transmute_copy(&punk2), core::mem::transmute_copy(&i2), core::mem::transmute_copy(&dx), core::mem::transmute_copy(&dy), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Clone(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetImageRect<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetImageRect(this, core::mem::transmute_copy(&i)) {
                    Ok(ok__) => {
                        prc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIconSize<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: *mut i32, cy: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::GetIconSize(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
            }
        }
        unsafe extern "system" fn SetIconSize<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: i32, cy: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::SetIconSize(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
            }
        }
        unsafe extern "system" fn GetImageCount<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pi: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetImageCount(this) {
                    Ok(ok__) => {
                        pi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetImageCount<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unewcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::SetImageCount(this, core::mem::transmute_copy(&unewcount)).into()
            }
        }
        unsafe extern "system" fn SetBkColor<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clrbk: super::super::Foundation::COLORREF, pclr: *mut super::super::Foundation::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::SetBkColor(this, core::mem::transmute_copy(&clrbk)) {
                    Ok(ok__) => {
                        pclr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBkColor<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclr: *mut super::super::Foundation::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetBkColor(this) {
                    Ok(ok__) => {
                        pclr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginDrag<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::BeginDrag(this, core::mem::transmute_copy(&itrack), core::mem::transmute_copy(&dxhotspot), core::mem::transmute_copy(&dyhotspot)).into()
            }
        }
        unsafe extern "system" fn EndDrag<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::EndDrag(this).into()
            }
        }
        unsafe extern "system" fn DragEnter<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::DragEnter(this, core::mem::transmute_copy(&hwndlock), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn DragLeave<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndlock: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::DragLeave(this, core::mem::transmute_copy(&hwndlock)).into()
            }
        }
        unsafe extern "system" fn DragMove<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::DragMove(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn SetDragCursorImage<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::SetDragCursorImage(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&idrag), core::mem::transmute_copy(&dxhotspot), core::mem::transmute_copy(&dyhotspot)).into()
            }
        }
        unsafe extern "system" fn DragShowNolock<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::DragShowNolock(this, core::mem::transmute_copy(&fshow)).into()
            }
        }
        unsafe extern "system" fn GetDragImage<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::GetDragImage(this, core::mem::transmute_copy(&ppt), core::mem::transmute_copy(&ppthotspot), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetItemFlags<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, dwflags: *mut IMAGE_LIST_ITEM_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetItemFlags(this, core::mem::transmute_copy(&i)) {
                    Ok(ok__) => {
                        dwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOverlayImage<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ioverlay: i32, piindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetOverlayImage(this, core::mem::transmute_copy(&ioverlay)) {
                    Ok(ok__) => {
                        piindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            ReplaceIcon: ReplaceIcon::<Identity, OFFSET>,
            SetOverlayImage: SetOverlayImage::<Identity, OFFSET>,
            Replace: Replace::<Identity, OFFSET>,
            AddMasked: AddMasked::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            GetIcon: GetIcon::<Identity, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Merge: Merge::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetImageRect: GetImageRect::<Identity, OFFSET>,
            GetIconSize: GetIconSize::<Identity, OFFSET>,
            SetIconSize: SetIconSize::<Identity, OFFSET>,
            GetImageCount: GetImageCount::<Identity, OFFSET>,
            SetImageCount: SetImageCount::<Identity, OFFSET>,
            SetBkColor: SetBkColor::<Identity, OFFSET>,
            GetBkColor: GetBkColor::<Identity, OFFSET>,
            BeginDrag: BeginDrag::<Identity, OFFSET>,
            EndDrag: EndDrag::<Identity, OFFSET>,
            DragEnter: DragEnter::<Identity, OFFSET>,
            DragLeave: DragLeave::<Identity, OFFSET>,
            DragMove: DragMove::<Identity, OFFSET>,
            SetDragCursorImage: SetDragCursorImage::<Identity, OFFSET>,
            DragShowNolock: DragShowNolock::<Identity, OFFSET>,
            GetDragImage: GetDragImage::<Identity, OFFSET>,
            GetItemFlags: GetItemFlags::<Identity, OFFSET>,
            GetOverlayImage: GetOverlayImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IImageList {}
windows_core::imp::define_interface!(IImageList2, IImageList2_Vtbl, 0x192b9d83_50fc_457b_90a0_2b82a8b5dae1);
impl core::ops::Deref for IImageList2 {
    type Target = IImageList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImageList2, windows_core::IUnknown, IImageList);
impl IImageList2 {
    pub unsafe fn Resize(&self, cxnewiconsize: i32, cynewiconsize: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), cxnewiconsize, cynewiconsize).ok() }
    }
    pub unsafe fn GetOriginalSize(&self, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOriginalSize)(windows_core::Interface::as_raw(self), iimage, dwflags, pcx as _, pcy as _).ok() }
    }
    pub unsafe fn SetOriginalSize(&self, iimage: i32, cx: i32, cy: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOriginalSize)(windows_core::Interface::as_raw(self), iimage, cx, cy).ok() }
    }
    pub unsafe fn SetCallback<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCallback)(windows_core::Interface::as_raw(self), punk.param().abi()).ok() }
    }
    pub unsafe fn GetCallback(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCallback)(windows_core::Interface::as_raw(self), riid, ppv as _).ok() }
    }
    pub unsafe fn ForceImagePresent(&self, iimage: i32, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ForceImagePresent)(windows_core::Interface::as_raw(self), iimage, dwflags).ok() }
    }
    pub unsafe fn DiscardImages(&self, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DiscardImages)(windows_core::Interface::as_raw(self), ifirstimage, ilastimage, dwflags).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn PreloadImages(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PreloadImages)(windows_core::Interface::as_raw(self), pimldp).ok() }
    }
    pub unsafe fn GetStatistics(&self, pils: *mut IMAGELISTSTATS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), pils as _).ok() }
    }
    pub unsafe fn Initialize(&self, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), cx, cy, flags, cinitial, cgrow).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Replace2<P3>(&self, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: Option<super::super::Graphics::Gdi::HBITMAP>, punk: P3, dwflags: u32) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Replace2)(windows_core::Interface::as_raw(self), i, hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _, punk.param().abi(), dwflags).ok() }
    }
    pub unsafe fn ReplaceFromImageList<P1, P3>(&self, i: i32, pil: P1, isrc: i32, punk: P3, dwflags: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IImageList>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReplaceFromImageList)(windows_core::Interface::as_raw(self), i, pil.param().abi(), isrc, punk.param().abi(), dwflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList2_Vtbl {
    pub base__: IImageList_Vtbl,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetOriginalSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetOriginalSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub SetCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForceImagePresent: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub DiscardImages: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub PreloadImages: unsafe extern "system" fn(*mut core::ffi::c_void, *const IMAGELISTDRAWPARAMS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    PreloadImages: usize,
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAGELISTSTATS) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, IMAGELIST_CREATION_FLAGS, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Replace2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Graphics::Gdi::HBITMAP, super::super::Graphics::Gdi::HBITMAP, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Replace2: usize,
    pub ReplaceFromImageList: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImageList2_Impl: IImageList_Impl {
    fn Resize(&self, cxnewiconsize: i32, cynewiconsize: i32) -> windows_core::Result<()>;
    fn GetOriginalSize(&self, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> windows_core::Result<()>;
    fn SetOriginalSize(&self, iimage: i32, cx: i32, cy: i32) -> windows_core::Result<()>;
    fn SetCallback(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetCallback(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ForceImagePresent(&self, iimage: i32, dwflags: u32) -> windows_core::Result<()>;
    fn DiscardImages(&self, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> windows_core::Result<()>;
    fn PreloadImages(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::Result<()>;
    fn GetStatistics(&self, pils: *mut IMAGELISTSTATS) -> windows_core::Result<()>;
    fn Initialize(&self, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> windows_core::Result<()>;
    fn Replace2(&self, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: windows_core::Ref<windows_core::IUnknown>, dwflags: u32) -> windows_core::Result<()>;
    fn ReplaceFromImageList(&self, i: i32, pil: windows_core::Ref<IImageList>, isrc: i32, punk: windows_core::Ref<windows_core::IUnknown>, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IImageList2_Vtbl {
    pub const fn new<Identity: IImageList2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resize<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cxnewiconsize: i32, cynewiconsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::Resize(this, core::mem::transmute_copy(&cxnewiconsize), core::mem::transmute_copy(&cynewiconsize)).into()
            }
        }
        unsafe extern "system" fn GetOriginalSize<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::GetOriginalSize(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pcx), core::mem::transmute_copy(&pcy)).into()
            }
        }
        unsafe extern "system" fn SetOriginalSize<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, cx: i32, cy: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::SetOriginalSize(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
            }
        }
        unsafe extern "system" fn SetCallback<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::SetCallback(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn GetCallback<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::GetCallback(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn ForceImagePresent<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::ForceImagePresent(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn DiscardImages<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::DiscardImages(this, core::mem::transmute_copy(&ifirstimage), core::mem::transmute_copy(&ilastimage), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn PreloadImages<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::PreloadImages(this, core::mem::transmute_copy(&pimldp)).into()
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pils: *mut IMAGELISTSTATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::GetStatistics(this, core::mem::transmute_copy(&pils)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::Initialize(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&cinitial), core::mem::transmute_copy(&cgrow)).into()
            }
        }
        unsafe extern "system" fn Replace2<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::Replace2(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&hbmmask), core::mem::transmute_copy(&punk), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn ReplaceFromImageList<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, pil: *mut core::ffi::c_void, isrc: i32, punk: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::ReplaceFromImageList(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&pil), core::mem::transmute_copy(&isrc), core::mem::transmute_copy(&punk), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: IImageList_Vtbl::new::<Identity, OFFSET>(),
            Resize: Resize::<Identity, OFFSET>,
            GetOriginalSize: GetOriginalSize::<Identity, OFFSET>,
            SetOriginalSize: SetOriginalSize::<Identity, OFFSET>,
            SetCallback: SetCallback::<Identity, OFFSET>,
            GetCallback: GetCallback::<Identity, OFFSET>,
            ForceImagePresent: ForceImagePresent::<Identity, OFFSET>,
            DiscardImages: DiscardImages::<Identity, OFFSET>,
            PreloadImages: PreloadImages::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            Replace2: Replace2::<Identity, OFFSET>,
            ReplaceFromImageList: ReplaceFromImageList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageList2 as windows_core::Interface>::IID || iid == &<IImageList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IImageList2 {}
pub const ILCF_MOVE: IMAGE_LIST_COPY_FLAGS = IMAGE_LIST_COPY_FLAGS(0u32);
pub const ILCF_SWAP: IMAGE_LIST_COPY_FLAGS = IMAGE_LIST_COPY_FLAGS(1u32);
pub const ILC_COLOR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(0u32);
pub const ILC_COLOR16: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(16u32);
pub const ILC_COLOR24: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(24u32);
pub const ILC_COLOR32: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(32u32);
pub const ILC_COLOR4: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(4u32);
pub const ILC_COLOR8: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(8u32);
pub const ILC_COLORDDB: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(254u32);
pub const ILC_HIGHQUALITYSCALE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(131072u32);
pub const ILC_MASK: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(1u32);
pub const ILC_MIRROR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(8192u32);
pub const ILC_ORIGINALSIZE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(65536u32);
pub const ILC_PALETTE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(2048u32);
pub const ILC_PERITEMMIRROR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(32768u32);
pub const ILDI_PURGE: u32 = 1u32;
pub const ILDI_QUERYACCESS: u32 = 8u32;
pub const ILDI_RESETACCESS: u32 = 4u32;
pub const ILDI_STANDBY: u32 = 2u32;
pub const ILDRF_IMAGELOWQUALITY: u32 = 1u32;
pub const ILDRF_OVERLAYLOWQUALITY: u32 = 16u32;
pub const ILD_ASYNC: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(32768u32);
pub const ILD_BLEND: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
pub const ILD_BLEND25: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(2u32);
pub const ILD_BLEND50: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
pub const ILD_DPISCALE: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(16384u32);
pub const ILD_FOCUS: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(2u32);
pub const ILD_IMAGE: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(32u32);
pub const ILD_MASK: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(16u32);
pub const ILD_NORMAL: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(0u32);
pub const ILD_OVERLAYMASK: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(3840u32);
pub const ILD_PRESERVEALPHA: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4096u32);
pub const ILD_ROP: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(64u32);
pub const ILD_SCALE: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(8192u32);
pub const ILD_SELECTED: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
pub const ILD_TRANSPARENT: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(1u32);
pub const ILFIP_ALWAYS: u32 = 0u32;
pub const ILFIP_FROMSTANDBY: u32 = 1u32;
pub const ILGOS_ALWAYS: u32 = 0u32;
pub const ILGOS_FROMSTANDBY: u32 = 1u32;
pub const ILGT_ASYNC: u32 = 1u32;
pub const ILGT_NORMAL: u32 = 0u32;
pub const ILIF_ALPHA: IMAGE_LIST_ITEM_FLAGS = IMAGE_LIST_ITEM_FLAGS(1u32);
pub const ILIF_LOWQUALITY: IMAGE_LIST_ITEM_FLAGS = IMAGE_LIST_ITEM_FLAGS(2u32);
pub const ILP_DOWNLEVEL: IMAGE_LIST_WRITE_STREAM_FLAGS = IMAGE_LIST_WRITE_STREAM_FLAGS(1u32);
pub const ILP_NORMAL: IMAGE_LIST_WRITE_STREAM_FLAGS = IMAGE_LIST_WRITE_STREAM_FLAGS(0u32);
pub const ILR_DEFAULT: u32 = 0u32;
pub const ILR_HORIZONTAL_CENTER: u32 = 1u32;
pub const ILR_HORIZONTAL_LEFT: u32 = 0u32;
pub const ILR_HORIZONTAL_RIGHT: u32 = 2u32;
pub const ILR_SCALE_ASPECTRATIO: u32 = 256u32;
pub const ILR_SCALE_CLIP: u32 = 0u32;
pub const ILR_VERTICAL_BOTTOM: u32 = 32u32;
pub const ILR_VERTICAL_CENTER: u32 = 16u32;
pub const ILR_VERTICAL_TOP: u32 = 0u32;
pub const ILS_ALPHA: u32 = 8u32;
pub const ILS_GLOW: u32 = 1u32;
pub const ILS_NORMAL: u32 = 0u32;
pub const ILS_SATURATE: u32 = 4u32;
pub const ILS_SHADOW: u32 = 2u32;
pub const IL_HORIZONTAL: IMAGELAYOUT = IMAGELAYOUT(1i32);
pub const IL_VERTICAL: IMAGELAYOUT = IMAGELAYOUT(0i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGEINFO {
    pub hbmImage: super::super::Graphics::Gdi::HBITMAP,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub Unused1: i32,
    pub Unused2: i32,
    pub rcImage: super::super::Foundation::RECT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGELAYOUT(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGELISTDRAWPARAMS {
    pub cbSize: u32,
    pub himl: HIMAGELIST,
    pub i: i32,
    pub hdcDst: super::super::Graphics::Gdi::HDC,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub xBitmap: i32,
    pub yBitmap: i32,
    pub rgbBk: super::super::Foundation::COLORREF,
    pub rgbFg: super::super::Foundation::COLORREF,
    pub fStyle: u32,
    pub dwRop: u32,
    pub fState: u32,
    pub Frame: u32,
    pub crEffect: super::super::Foundation::COLORREF,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGELISTSTATS {
    pub cbSize: u32,
    pub cAlloc: i32,
    pub cUsed: i32,
    pub cStandby: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGELIST_CREATION_FLAGS(pub u32);
impl IMAGELIST_CREATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IMAGELIST_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IMAGELIST_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGESELECTTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGE_LIST_COPY_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGE_LIST_DRAW_STYLE(pub u32);
impl IMAGE_LIST_DRAW_STYLE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IMAGE_LIST_DRAW_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IMAGE_LIST_DRAW_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGE_LIST_ITEM_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGE_LIST_WRITE_STREAM_FLAGS(pub u32);
impl IMAGE_LIST_WRITE_STREAM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IMAGE_LIST_WRITE_STREAM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IMAGE_LIST_WRITE_STREAM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IMAGE_LIST_WRITE_STREAM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IMAGE_LIST_WRITE_STREAM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IMAGE_LIST_WRITE_STREAM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INFOTIPSIZE: u32 = 1024u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INITCOMMONCONTROLSEX {
    pub dwSize: u32,
    pub dwICC: INITCOMMONCONTROLSEX_ICC,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INITCOMMONCONTROLSEX_ICC(pub u32);
impl INITCOMMONCONTROLSEX_ICC {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for INITCOMMONCONTROLSEX_ICC {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for INITCOMMONCONTROLSEX_ICC {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTLIST {
    pub iValueCount: i32,
    pub iValues: [i32; 402],
}
impl Default for INTLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INVALID_LINK_INDEX: i32 = -1i32;
pub const IPM_CLEARADDRESS: u32 = 1124u32;
pub const IPM_GETADDRESS: u32 = 1126u32;
pub const IPM_ISBLANK: u32 = 1129u32;
pub const IPM_SETADDRESS: u32 = 1125u32;
pub const IPM_SETFOCUS: u32 = 1128u32;
pub const IPM_SETRANGE: u32 = 1127u32;
pub const IPN_FIELDCHANGED: u32 = 4294966436u32;
pub const IPN_FIRST: u32 = 4294966436u32;
pub const IPN_LAST: u32 = 4294966417u32;
pub const IST_DPI: IMAGESELECTTYPE = IMAGESELECTTYPE(2i32);
pub const IST_NONE: IMAGESELECTTYPE = IMAGESELECTTYPE(0i32);
pub const IST_SIZE: IMAGESELECTTYPE = IMAGESELECTTYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ITEMSTATES(pub i32);
pub const I_CHILDRENAUTO: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(-2i32);
pub const I_CHILDRENCALLBACK: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(-1i32);
pub const I_GROUPIDCALLBACK: LVITEMA_GROUP_ID = LVITEMA_GROUP_ID(-1i32);
pub const I_GROUPIDNONE: LVITEMA_GROUP_ID = LVITEMA_GROUP_ID(-2i32);
pub const I_IMAGECALLBACK: i32 = -1i32;
pub const I_IMAGENONE: i32 = -2i32;
pub const I_INDENTCALLBACK: i32 = -1i32;
pub const I_ONE_OR_MORE: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(1i32);
pub const I_ZERO: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(0i32);
pub const ImageList: windows_core::GUID = windows_core::GUID::from_u128(0x7c476ba2_02b1_48f4_8048_b24619ddc058);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LABELSTATES(pub i32);
pub const LBCP_BORDER_HSCROLL: LISTBOXPARTS = LISTBOXPARTS(1i32);
pub const LBCP_BORDER_HVSCROLL: LISTBOXPARTS = LISTBOXPARTS(2i32);
pub const LBCP_BORDER_NOSCROLL: LISTBOXPARTS = LISTBOXPARTS(3i32);
pub const LBCP_BORDER_VSCROLL: LISTBOXPARTS = LISTBOXPARTS(4i32);
pub const LBCP_ITEM: LISTBOXPARTS = LISTBOXPARTS(5i32);
pub const LBPSHV_DISABLED: BORDER_HVSCROLLSTATES = BORDER_HVSCROLLSTATES(4i32);
pub const LBPSHV_FOCUSED: BORDER_HVSCROLLSTATES = BORDER_HVSCROLLSTATES(2i32);
pub const LBPSHV_HOT: BORDER_HVSCROLLSTATES = BORDER_HVSCROLLSTATES(3i32);
pub const LBPSHV_NORMAL: BORDER_HVSCROLLSTATES = BORDER_HVSCROLLSTATES(1i32);
pub const LBPSH_DISABLED: BORDER_HSCROLLSTATES = BORDER_HSCROLLSTATES(4i32);
pub const LBPSH_FOCUSED: BORDER_HSCROLLSTATES = BORDER_HSCROLLSTATES(2i32);
pub const LBPSH_HOT: BORDER_HSCROLLSTATES = BORDER_HSCROLLSTATES(3i32);
pub const LBPSH_NORMAL: BORDER_HSCROLLSTATES = BORDER_HSCROLLSTATES(1i32);
pub const LBPSI_HOT: ITEMSTATES = ITEMSTATES(1i32);
pub const LBPSI_HOTSELECTED: ITEMSTATES = ITEMSTATES(2i32);
pub const LBPSI_SELECTED: ITEMSTATES = ITEMSTATES(3i32);
pub const LBPSI_SELECTEDNOTFOCUS: ITEMSTATES = ITEMSTATES(4i32);
pub const LBPSN_DISABLED: BORDER_NOSCROLLSTATES = BORDER_NOSCROLLSTATES(4i32);
pub const LBPSN_FOCUSED: BORDER_NOSCROLLSTATES = BORDER_NOSCROLLSTATES(2i32);
pub const LBPSN_HOT: BORDER_NOSCROLLSTATES = BORDER_NOSCROLLSTATES(3i32);
pub const LBPSN_NORMAL: BORDER_NOSCROLLSTATES = BORDER_NOSCROLLSTATES(1i32);
pub const LBPSV_DISABLED: BORDER_VSCROLLSTATES = BORDER_VSCROLLSTATES(4i32);
pub const LBPSV_FOCUSED: BORDER_VSCROLLSTATES = BORDER_VSCROLLSTATES(2i32);
pub const LBPSV_HOT: BORDER_VSCROLLSTATES = BORDER_VSCROLLSTATES(3i32);
pub const LBPSV_NORMAL: BORDER_VSCROLLSTATES = BORDER_VSCROLLSTATES(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub item: LITEM,
}
pub const LIF_ITEMID: LIST_ITEM_FLAGS = LIST_ITEM_FLAGS(4u32);
pub const LIF_ITEMINDEX: LIST_ITEM_FLAGS = LIST_ITEM_FLAGS(1u32);
pub const LIF_STATE: LIST_ITEM_FLAGS = LIST_ITEM_FLAGS(2u32);
pub const LIF_URL: LIST_ITEM_FLAGS = LIST_ITEM_FLAGS(8u32);
pub const LIM_LARGE: _LI_METRIC = _LI_METRIC(1i32);
pub const LIM_SMALL: _LI_METRIC = _LI_METRIC(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LINKHEADERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LINKPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LINKSTATES(pub i32);
pub const LISS_DISABLED: LISTITEMSTATES = LISTITEMSTATES(4i32);
pub const LISS_HOT: LISTITEMSTATES = LISTITEMSTATES(2i32);
pub const LISS_HOTSELECTED: LISTITEMSTATES = LISTITEMSTATES(6i32);
pub const LISS_NORMAL: LISTITEMSTATES = LISTITEMSTATES(1i32);
pub const LISS_SELECTED: LISTITEMSTATES = LISTITEMSTATES(3i32);
pub const LISS_SELECTEDNOTFOCUS: LISTITEMSTATES = LISTITEMSTATES(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LISTBOXPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LISTITEMSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LISTVIEWPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LIST_ITEM_FLAGS(pub u32);
impl LIST_ITEM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LIST_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LIST_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LIST_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LIST_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LIST_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LIST_ITEM_STATE_FLAGS(pub u32);
impl LIST_ITEM_STATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LIST_ITEM_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LIST_ITEM_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LIST_ITEM_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LIST_ITEM_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LIST_ITEM_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LIST_VIEW_BACKGROUND_IMAGE_FLAGS(pub u32);
impl LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LIST_VIEW_GROUP_ALIGN_FLAGS(pub u32);
impl LIST_VIEW_GROUP_ALIGN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LIST_VIEW_GROUP_ALIGN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LIST_VIEW_GROUP_ALIGN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LIST_VIEW_GROUP_ALIGN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LIST_VIEW_GROUP_ALIGN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LIST_VIEW_GROUP_ALIGN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LIST_VIEW_GROUP_STATE_FLAGS(pub u32);
impl LIST_VIEW_GROUP_STATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LIST_VIEW_GROUP_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LIST_VIEW_GROUP_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LIST_VIEW_GROUP_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LIST_VIEW_GROUP_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LIST_VIEW_GROUP_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(pub i32);
impl LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LIST_VIEW_ITEM_FLAGS(pub u32);
impl LIST_VIEW_ITEM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LIST_VIEW_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LIST_VIEW_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LIST_VIEW_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LIST_VIEW_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LIST_VIEW_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LIST_VIEW_ITEM_STATE_FLAGS(pub u32);
pub const LIS_DEFAULTCOLORS: LIST_ITEM_STATE_FLAGS = LIST_ITEM_STATE_FLAGS(16u32);
pub const LIS_ENABLED: LIST_ITEM_STATE_FLAGS = LIST_ITEM_STATE_FLAGS(2u32);
pub const LIS_FOCUSED: LIST_ITEM_STATE_FLAGS = LIST_ITEM_STATE_FLAGS(1u32);
pub const LIS_HOTTRACK: LIST_ITEM_STATE_FLAGS = LIST_ITEM_STATE_FLAGS(8u32);
pub const LIS_VISITED: LIST_ITEM_STATE_FLAGS = LIST_ITEM_STATE_FLAGS(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LITEM {
    pub mask: LIST_ITEM_FLAGS,
    pub iLink: i32,
    pub state: LIST_ITEM_STATE_FLAGS,
    pub stateMask: LIST_ITEM_STATE_FLAGS,
    pub szID: [u16; 48],
    pub szUrl: [u16; 2084],
}
impl Default for LITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LM_GETIDEALHEIGHT: u32 = 1793u32;
pub const LM_GETIDEALSIZE: u32 = 1793u32;
pub const LM_GETITEM: u32 = 1795u32;
pub const LM_HITTEST: u32 = 1792u32;
pub const LM_SETITEM: u32 = 1794u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOGOFFBUTTONSSTATES(pub i32);
pub type LPFNADDPROPSHEETPAGES = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: LPFNSVADDPROPSHEETPAGE, param2: super::super::Foundation::LPARAM) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type LPFNCCINFOA = Option<unsafe extern "system" fn(acci: *mut CCINFOA) -> u32>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type LPFNCCINFOW = Option<unsafe extern "system" fn(acci: *mut CCINFOW) -> u32>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type LPFNCCSIZETOTEXTA = Option<unsafe extern "system" fn(flstyle: u32, flextstyle: u32, hfont: super::super::Graphics::Gdi::HFONT, psztext: windows_core::PCSTR) -> i32>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type LPFNCCSIZETOTEXTW = Option<unsafe extern "system" fn(flstyle: u32, flextstyle: u32, hfont: super::super::Graphics::Gdi::HFONT, psztext: windows_core::PCWSTR) -> i32>;
pub type LPFNCCSTYLEA = Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pccs: *mut CCSTYLEA) -> windows_core::BOOL>;
pub type LPFNCCSTYLEW = Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pccs: *mut CCSTYLEW) -> windows_core::BOOL>;
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPFNPSPCALLBACKA = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: PSPCB_MESSAGE, ppsp: *mut PROPSHEETPAGEA) -> u32>;
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPFNPSPCALLBACKW = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: PSPCB_MESSAGE, ppsp: *mut PROPSHEETPAGEW) -> u32>;
pub type LPFNSVADDPROPSHEETPAGE = Option<unsafe extern "system" fn(param0: HPROPSHEETPAGE, param1: super::super::Foundation::LPARAM) -> windows_core::BOOL>;
pub const LP_HYPERLINK: LINKPARTS = LINKPARTS(1i32);
pub const LVA_ALIGNLEFT: u32 = 1u32;
pub const LVA_ALIGNTOP: u32 = 2u32;
pub const LVA_DEFAULT: u32 = 0u32;
pub const LVA_SNAPTOGRID: u32 = 5u32;
pub const LVBKIF_FLAG_ALPHABLEND: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(536870912u32);
pub const LVBKIF_FLAG_TILEOFFSET: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(256u32);
pub const LVBKIF_SOURCE_HBITMAP: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(1u32);
pub const LVBKIF_SOURCE_MASK: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(3u32);
pub const LVBKIF_SOURCE_NONE: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(0u32);
pub const LVBKIF_SOURCE_URL: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(2u32);
pub const LVBKIF_STYLE_MASK: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(16u32);
pub const LVBKIF_STYLE_NORMAL: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(0u32);
pub const LVBKIF_STYLE_TILE: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(16u32);
pub const LVBKIF_TYPE_WATERMARK: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(268435456u32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVBKIMAGEA {
    pub ulFlags: LIST_VIEW_BACKGROUND_IMAGE_FLAGS,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub pszImage: windows_core::PSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVBKIMAGEW {
    pub ulFlags: LIST_VIEW_BACKGROUND_IMAGE_FLAGS,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub pszImage: windows_core::PWSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
pub const LVCB_HOVER: COLLAPSEBUTTONSTATES = COLLAPSEBUTTONSTATES(2i32);
pub const LVCB_NORMAL: COLLAPSEBUTTONSTATES = COLLAPSEBUTTONSTATES(1i32);
pub const LVCB_PUSHED: COLLAPSEBUTTONSTATES = COLLAPSEBUTTONSTATES(3i32);
pub const LVCDI_GROUP: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(1u32);
pub const LVCDI_ITEM: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(0u32);
pub const LVCDI_ITEMSLIST: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(2u32);
pub const LVCDRF_NOGROUPFRAME: u32 = 131072u32;
pub const LVCDRF_NOSELECT: u32 = 65536u32;
pub const LVCFMT_BITMAP_ON_RIGHT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(4096i32);
pub const LVCFMT_CENTER: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(2i32);
pub const LVCFMT_COL_HAS_IMAGES: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(32768i32);
pub const LVCFMT_FILL: LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS = LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(2097152i32);
pub const LVCFMT_FIXED_RATIO: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(524288i32);
pub const LVCFMT_FIXED_WIDTH: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(256i32);
pub const LVCFMT_IMAGE: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(2048i32);
pub const LVCFMT_JUSTIFYMASK: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(3i32);
pub const LVCFMT_LEFT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(0i32);
pub const LVCFMT_LINE_BREAK: LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS = LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(1048576i32);
pub const LVCFMT_NO_DPI_SCALE: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(262144i32);
pub const LVCFMT_NO_TITLE: LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS = LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(8388608i32);
pub const LVCFMT_RIGHT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(1i32);
pub const LVCFMT_SPLITBUTTON: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(16777216i32);
pub const LVCFMT_TILE_PLACEMENTMASK: LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS = LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(3145728i32);
pub const LVCFMT_WRAP: LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS = LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(4194304i32);
pub const LVCF_DEFAULTWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(128u32);
pub const LVCF_FMT: LVCOLUMNW_MASK = LVCOLUMNW_MASK(1u32);
pub const LVCF_IDEALWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(256u32);
pub const LVCF_IMAGE: LVCOLUMNW_MASK = LVCOLUMNW_MASK(16u32);
pub const LVCF_MINWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(64u32);
pub const LVCF_ORDER: LVCOLUMNW_MASK = LVCOLUMNW_MASK(32u32);
pub const LVCF_SUBITEM: LVCOLUMNW_MASK = LVCOLUMNW_MASK(8u32);
pub const LVCF_TEXT: LVCOLUMNW_MASK = LVCOLUMNW_MASK(4u32);
pub const LVCF_WIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVCOLUMNA {
    pub mask: LVCOLUMNW_MASK,
    pub fmt: LVCOLUMNW_FORMAT,
    pub cx: i32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVCOLUMNW {
    pub mask: LVCOLUMNW_MASK,
    pub fmt: LVCOLUMNW_FORMAT,
    pub cx: i32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVCOLUMNW_FORMAT(pub i32);
impl LVCOLUMNW_FORMAT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LVCOLUMNW_FORMAT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LVCOLUMNW_FORMAT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVCOLUMNW_MASK(pub u32);
impl LVCOLUMNW_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LVCOLUMNW_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LVCOLUMNW_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LVCOLUMNW_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LVCOLUMNW_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LVCOLUMNW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LVEB_HOVER: EXPANDBUTTONSTATES = EXPANDBUTTONSTATES(2i32);
pub const LVEB_NORMAL: EXPANDBUTTONSTATES = EXPANDBUTTONSTATES(1i32);
pub const LVEB_PUSHED: EXPANDBUTTONSTATES = EXPANDBUTTONSTATES(3i32);
pub const LVFF_ITEMCOUNT: u32 = 1u32;
pub const LVFIF_STATE: LVFOOTERITEM_MASK = LVFOOTERITEM_MASK(2u32);
pub const LVFIF_TEXT: LVFOOTERITEM_MASK = LVFOOTERITEM_MASK(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVFINDINFOA {
    pub flags: LVFINDINFOW_FLAGS,
    pub psz: windows_core::PCSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub pt: super::super::Foundation::POINT,
    pub vkDirection: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVFINDINFOW {
    pub flags: LVFINDINFOW_FLAGS,
    pub psz: windows_core::PCWSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub pt: super::super::Foundation::POINT,
    pub vkDirection: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVFINDINFOW_FLAGS(pub u32);
impl LVFINDINFOW_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LVFINDINFOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LVFINDINFOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LVFIS_FOCUSED: u32 = 1u32;
pub const LVFI_NEARESTXY: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(64u32);
pub const LVFI_PARAM: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(1u32);
pub const LVFI_PARTIAL: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(8u32);
pub const LVFI_STRING: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(2u32);
pub const LVFI_SUBSTRING: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(4u32);
pub const LVFI_WRAP: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(32u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVFOOTERINFO {
    pub mask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub cItems: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVFOOTERITEM {
    pub mask: LVFOOTERITEM_MASK,
    pub iItem: i32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub state: u32,
    pub stateMask: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVFOOTERITEM_MASK(pub u32);
pub const LVGA_FOOTER_CENTER: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(16u32);
pub const LVGA_FOOTER_LEFT: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(8u32);
pub const LVGA_FOOTER_RIGHT: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(32u32);
pub const LVGA_HEADER_CENTER: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(2u32);
pub const LVGA_HEADER_LEFT: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(1u32);
pub const LVGA_HEADER_RIGHT: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(4u32);
pub const LVGF_ALIGN: LVGROUP_MASK = LVGROUP_MASK(8u32);
pub const LVGF_DESCRIPTIONBOTTOM: LVGROUP_MASK = LVGROUP_MASK(2048u32);
pub const LVGF_DESCRIPTIONTOP: LVGROUP_MASK = LVGROUP_MASK(1024u32);
pub const LVGF_EXTENDEDIMAGE: LVGROUP_MASK = LVGROUP_MASK(8192u32);
pub const LVGF_FOOTER: LVGROUP_MASK = LVGROUP_MASK(2u32);
pub const LVGF_GROUPID: LVGROUP_MASK = LVGROUP_MASK(16u32);
pub const LVGF_HEADER: LVGROUP_MASK = LVGROUP_MASK(1u32);
pub const LVGF_ITEMS: LVGROUP_MASK = LVGROUP_MASK(16384u32);
pub const LVGF_NONE: LVGROUP_MASK = LVGROUP_MASK(0u32);
pub const LVGF_STATE: LVGROUP_MASK = LVGROUP_MASK(4u32);
pub const LVGF_SUBSET: LVGROUP_MASK = LVGROUP_MASK(32768u32);
pub const LVGF_SUBSETITEMS: LVGROUP_MASK = LVGROUP_MASK(65536u32);
pub const LVGF_SUBTITLE: LVGROUP_MASK = LVGROUP_MASK(256u32);
pub const LVGF_TASK: LVGROUP_MASK = LVGROUP_MASK(512u32);
pub const LVGF_TITLEIMAGE: LVGROUP_MASK = LVGROUP_MASK(4096u32);
pub const LVGGR_GROUP: u32 = 0u32;
pub const LVGGR_HEADER: u32 = 1u32;
pub const LVGGR_LABEL: u32 = 2u32;
pub const LVGGR_SUBSETLINK: u32 = 3u32;
pub const LVGHL_CLOSE: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(9i32);
pub const LVGHL_CLOSEHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(10i32);
pub const LVGHL_CLOSEMIXEDSELECTION: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(15i32);
pub const LVGHL_CLOSEMIXEDSELECTIONHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(16i32);
pub const LVGHL_CLOSESELECTED: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(11i32);
pub const LVGHL_CLOSESELECTEDHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(12i32);
pub const LVGHL_CLOSESELECTEDNOTFOCUSED: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(13i32);
pub const LVGHL_CLOSESELECTEDNOTFOCUSEDHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(14i32);
pub const LVGHL_OPEN: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(1i32);
pub const LVGHL_OPENHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(2i32);
pub const LVGHL_OPENMIXEDSELECTION: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(7i32);
pub const LVGHL_OPENMIXEDSELECTIONHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(8i32);
pub const LVGHL_OPENSELECTED: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(3i32);
pub const LVGHL_OPENSELECTEDHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(4i32);
pub const LVGHL_OPENSELECTEDNOTFOCUSED: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(5i32);
pub const LVGHL_OPENSELECTEDNOTFOCUSEDHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(6i32);
pub const LVGH_CLOSE: GROUPHEADERSTATES = GROUPHEADERSTATES(9i32);
pub const LVGH_CLOSEHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(10i32);
pub const LVGH_CLOSEMIXEDSELECTION: GROUPHEADERSTATES = GROUPHEADERSTATES(15i32);
pub const LVGH_CLOSEMIXEDSELECTIONHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(16i32);
pub const LVGH_CLOSESELECTED: GROUPHEADERSTATES = GROUPHEADERSTATES(11i32);
pub const LVGH_CLOSESELECTEDHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(12i32);
pub const LVGH_CLOSESELECTEDNOTFOCUSED: GROUPHEADERSTATES = GROUPHEADERSTATES(13i32);
pub const LVGH_CLOSESELECTEDNOTFOCUSEDHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(14i32);
pub const LVGH_OPEN: GROUPHEADERSTATES = GROUPHEADERSTATES(1i32);
pub const LVGH_OPENHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(2i32);
pub const LVGH_OPENMIXEDSELECTION: GROUPHEADERSTATES = GROUPHEADERSTATES(7i32);
pub const LVGH_OPENMIXEDSELECTIONHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(8i32);
pub const LVGH_OPENSELECTED: GROUPHEADERSTATES = GROUPHEADERSTATES(3i32);
pub const LVGH_OPENSELECTEDHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(4i32);
pub const LVGH_OPENSELECTEDNOTFOCUSED: GROUPHEADERSTATES = GROUPHEADERSTATES(5i32);
pub const LVGH_OPENSELECTEDNOTFOCUSEDHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(6i32);
pub const LVGIT_UNFOLDED: NMLVGETINFOTIP_FLAGS = NMLVGETINFOTIP_FLAGS(1u32);
pub const LVGIT_ZERO: NMLVGETINFOTIP_FLAGS = NMLVGETINFOTIP_FLAGS(0u32);
pub const LVGMF_BORDERCOLOR: u32 = 2u32;
pub const LVGMF_BORDERSIZE: u32 = 1u32;
pub const LVGMF_NONE: u32 = 0u32;
pub const LVGMF_TEXTCOLOR: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVGROUP {
    pub cbSize: u32,
    pub mask: LVGROUP_MASK,
    pub pszHeader: windows_core::PWSTR,
    pub cchHeader: i32,
    pub pszFooter: windows_core::PWSTR,
    pub cchFooter: i32,
    pub iGroupId: i32,
    pub stateMask: LIST_VIEW_GROUP_STATE_FLAGS,
    pub state: LIST_VIEW_GROUP_STATE_FLAGS,
    pub uAlign: LIST_VIEW_GROUP_ALIGN_FLAGS,
    pub pszSubtitle: windows_core::PWSTR,
    pub cchSubtitle: u32,
    pub pszTask: windows_core::PWSTR,
    pub cchTask: u32,
    pub pszDescriptionTop: windows_core::PWSTR,
    pub cchDescriptionTop: u32,
    pub pszDescriptionBottom: windows_core::PWSTR,
    pub cchDescriptionBottom: u32,
    pub iTitleImage: i32,
    pub iExtendedImage: i32,
    pub iFirstItem: i32,
    pub cItems: u32,
    pub pszSubsetTitle: windows_core::PWSTR,
    pub cchSubsetTitle: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVGROUPMETRICS {
    pub cbSize: u32,
    pub mask: u32,
    pub Left: u32,
    pub Top: u32,
    pub Right: u32,
    pub Bottom: u32,
    pub crLeft: super::super::Foundation::COLORREF,
    pub crTop: super::super::Foundation::COLORREF,
    pub crRight: super::super::Foundation::COLORREF,
    pub crBottom: super::super::Foundation::COLORREF,
    pub crHeader: super::super::Foundation::COLORREF,
    pub crFooter: super::super::Foundation::COLORREF,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVGROUP_MASK(pub u32);
impl LVGROUP_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LVGROUP_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LVGROUP_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LVGROUP_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LVGROUP_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LVGROUP_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LVGS_COLLAPSED: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(1u32);
pub const LVGS_COLLAPSIBLE: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(8u32);
pub const LVGS_FOCUSED: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(16u32);
pub const LVGS_HIDDEN: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(2u32);
pub const LVGS_NOHEADER: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(4u32);
pub const LVGS_NORMAL: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(0u32);
pub const LVGS_SELECTED: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(32u32);
pub const LVGS_SUBSETED: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(64u32);
pub const LVGS_SUBSETLINKFOCUSED: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(128u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: LVHITTESTINFO_FLAGS,
    pub iItem: i32,
    pub iSubItem: i32,
    pub iGroup: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVHITTESTINFO_FLAGS(pub u32);
impl LVHITTESTINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LVHITTESTINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LVHITTESTINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LVHT_ABOVE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(8u32);
pub const LVHT_BELOW: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(16u32);
pub const LVHT_EX_FOOTER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(134217728u32);
pub const LVHT_EX_GROUP: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(4076863488u32);
pub const LVHT_EX_GROUP_BACKGROUND: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(2147483648u32);
pub const LVHT_EX_GROUP_COLLAPSE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(1073741824u32);
pub const LVHT_EX_GROUP_FOOTER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(536870912u32);
pub const LVHT_EX_GROUP_HEADER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(268435456u32);
pub const LVHT_EX_GROUP_STATEICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(16777216u32);
pub const LVHT_EX_GROUP_SUBSETLINK: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(33554432u32);
pub const LVHT_EX_ONCONTENTS: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(67108864u32);
pub const LVHT_NOWHERE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(1u32);
pub const LVHT_ONITEMICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(2u32);
pub const LVHT_ONITEMLABEL: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(4u32);
pub const LVHT_ONITEMSTATEICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(8u32);
pub const LVHT_TOLEFT: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(64u32);
pub const LVHT_TORIGHT: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(32u32);
pub const LVIF_COLFMT: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(65536u32);
pub const LVIF_COLUMNS: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(512u32);
pub const LVIF_DI_SETITEM: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(4096u32);
pub const LVIF_GROUPID: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(256u32);
pub const LVIF_IMAGE: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(2u32);
pub const LVIF_INDENT: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(16u32);
pub const LVIF_NORECOMPUTE: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(2048u32);
pub const LVIF_PARAM: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(4u32);
pub const LVIF_STATE: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(8u32);
pub const LVIF_TEXT: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(1u32);
pub const LVIM_AFTER: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct LVINSERTGROUPSORTED {
    pub pfnGroupCompare: PFNLVGROUPCOMPARE,
    pub pvData: *mut core::ffi::c_void,
    pub lvGroup: LVGROUP,
}
impl Default for LVINSERTGROUPSORTED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVINSERTMARK {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iItem: i32,
    pub dwReserved: u32,
}
pub const LVIR_BOUNDS: u32 = 0u32;
pub const LVIR_ICON: u32 = 1u32;
pub const LVIR_LABEL: u32 = 2u32;
pub const LVIR_SELECTBOUNDS: u32 = 3u32;
pub const LVIS_ACTIVATING: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(32u32);
pub const LVIS_CUT: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(4u32);
pub const LVIS_DROPHILITED: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(8u32);
pub const LVIS_FOCUSED: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(1u32);
pub const LVIS_GLOW: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(16u32);
pub const LVIS_OVERLAYMASK: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(3840u32);
pub const LVIS_SELECTED: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(2u32);
pub const LVIS_STATEIMAGEMASK: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(61440u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LVITEMA {
    pub mask: LIST_VIEW_ITEM_FLAGS,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: LIST_VIEW_ITEM_STATE_FLAGS,
    pub stateMask: LIST_VIEW_ITEM_STATE_FLAGS,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
    pub iGroupId: i32,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS,
    pub iGroup: i32,
}
impl Default for LVITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVITEMA_GROUP_ID(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVITEMINDEX {
    pub iItem: i32,
    pub iGroup: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LVITEMW {
    pub mask: LIST_VIEW_ITEM_FLAGS,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: LIST_VIEW_ITEM_STATE_FLAGS,
    pub stateMask: LIST_VIEW_ITEM_STATE_FLAGS,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
    pub iGroupId: i32,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS,
    pub iGroup: i32,
}
impl Default for LVITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LVKF_ALT: u32 = 1u32;
pub const LVKF_CONTROL: u32 = 2u32;
pub const LVKF_SHIFT: u32 = 4u32;
pub const LVM_APPROXIMATEVIEWRECT: u32 = 4160u32;
pub const LVM_ARRANGE: u32 = 4118u32;
pub const LVM_CANCELEDITLABEL: u32 = 4275u32;
pub const LVM_CREATEDRAGIMAGE: u32 = 4129u32;
pub const LVM_DELETEALLITEMS: u32 = 4105u32;
pub const LVM_DELETECOLUMN: u32 = 4124u32;
pub const LVM_DELETEITEM: u32 = 4104u32;
pub const LVM_EDITLABEL: u32 = 4214u32;
pub const LVM_EDITLABELA: u32 = 4119u32;
pub const LVM_EDITLABELW: u32 = 4214u32;
pub const LVM_ENABLEGROUPVIEW: u32 = 4253u32;
pub const LVM_ENSUREVISIBLE: u32 = 4115u32;
pub const LVM_FINDITEM: u32 = 4179u32;
pub const LVM_FINDITEMA: u32 = 4109u32;
pub const LVM_FINDITEMW: u32 = 4179u32;
pub const LVM_FIRST: u32 = 4096u32;
pub const LVM_GETBKCOLOR: u32 = 4096u32;
pub const LVM_GETBKIMAGE: u32 = 4235u32;
pub const LVM_GETBKIMAGEA: u32 = 4165u32;
pub const LVM_GETBKIMAGEW: u32 = 4235u32;
pub const LVM_GETCALLBACKMASK: u32 = 4106u32;
pub const LVM_GETCOLUMN: u32 = 4191u32;
pub const LVM_GETCOLUMNA: u32 = 4121u32;
pub const LVM_GETCOLUMNORDERARRAY: u32 = 4155u32;
pub const LVM_GETCOLUMNW: u32 = 4191u32;
pub const LVM_GETCOLUMNWIDTH: u32 = 4125u32;
pub const LVM_GETCOUNTPERPAGE: u32 = 4136u32;
pub const LVM_GETEDITCONTROL: u32 = 4120u32;
pub const LVM_GETEMPTYTEXT: u32 = 4300u32;
pub const LVM_GETEXTENDEDLISTVIEWSTYLE: u32 = 4151u32;
pub const LVM_GETFOCUSEDGROUP: u32 = 4189u32;
pub const LVM_GETFOOTERINFO: u32 = 4302u32;
pub const LVM_GETFOOTERITEM: u32 = 4304u32;
pub const LVM_GETFOOTERITEMRECT: u32 = 4303u32;
pub const LVM_GETFOOTERRECT: u32 = 4301u32;
pub const LVM_GETGROUPCOUNT: u32 = 4248u32;
pub const LVM_GETGROUPINFO: u32 = 4245u32;
pub const LVM_GETGROUPINFOBYINDEX: u32 = 4249u32;
pub const LVM_GETGROUPMETRICS: u32 = 4252u32;
pub const LVM_GETGROUPRECT: u32 = 4194u32;
pub const LVM_GETGROUPSTATE: u32 = 4188u32;
pub const LVM_GETHEADER: u32 = 4127u32;
pub const LVM_GETHOTCURSOR: u32 = 4159u32;
pub const LVM_GETHOTITEM: u32 = 4157u32;
pub const LVM_GETHOVERTIME: u32 = 4168u32;
pub const LVM_GETIMAGELIST: u32 = 4098u32;
pub const LVM_GETINSERTMARK: u32 = 4263u32;
pub const LVM_GETINSERTMARKCOLOR: u32 = 4267u32;
pub const LVM_GETINSERTMARKRECT: u32 = 4265u32;
pub const LVM_GETISEARCHSTRING: u32 = 4213u32;
pub const LVM_GETISEARCHSTRINGA: u32 = 4148u32;
pub const LVM_GETISEARCHSTRINGW: u32 = 4213u32;
pub const LVM_GETITEM: u32 = 4171u32;
pub const LVM_GETITEMA: u32 = 4101u32;
pub const LVM_GETITEMCOUNT: u32 = 4100u32;
pub const LVM_GETITEMINDEXRECT: u32 = 4305u32;
pub const LVM_GETITEMPOSITION: u32 = 4112u32;
pub const LVM_GETITEMRECT: u32 = 4110u32;
pub const LVM_GETITEMSPACING: u32 = 4147u32;
pub const LVM_GETITEMSTATE: u32 = 4140u32;
pub const LVM_GETITEMTEXT: u32 = 4211u32;
pub const LVM_GETITEMTEXTA: u32 = 4141u32;
pub const LVM_GETITEMTEXTW: u32 = 4211u32;
pub const LVM_GETITEMW: u32 = 4171u32;
pub const LVM_GETNEXTITEM: u32 = 4108u32;
pub const LVM_GETNEXTITEMINDEX: u32 = 4307u32;
pub const LVM_GETNUMBEROFWORKAREAS: u32 = 4169u32;
pub const LVM_GETORIGIN: u32 = 4137u32;
pub const LVM_GETOUTLINECOLOR: u32 = 4272u32;
pub const LVM_GETSELECTEDCOLUMN: u32 = 4270u32;
pub const LVM_GETSELECTEDCOUNT: u32 = 4146u32;
pub const LVM_GETSELECTIONMARK: u32 = 4162u32;
pub const LVM_GETSTRINGWIDTH: u32 = 4183u32;
pub const LVM_GETSTRINGWIDTHA: u32 = 4113u32;
pub const LVM_GETSTRINGWIDTHW: u32 = 4183u32;
pub const LVM_GETSUBITEMRECT: u32 = 4152u32;
pub const LVM_GETTEXTBKCOLOR: u32 = 4133u32;
pub const LVM_GETTEXTCOLOR: u32 = 4131u32;
pub const LVM_GETTILEINFO: u32 = 4261u32;
pub const LVM_GETTILEVIEWINFO: u32 = 4259u32;
pub const LVM_GETTOOLTIPS: u32 = 4174u32;
pub const LVM_GETTOPINDEX: u32 = 4135u32;
pub const LVM_GETUNICODEFORMAT: u32 = 8198u32;
pub const LVM_GETVIEW: u32 = 4239u32;
pub const LVM_GETVIEWRECT: u32 = 4130u32;
pub const LVM_GETWORKAREAS: u32 = 4166u32;
pub const LVM_HASGROUP: u32 = 4257u32;
pub const LVM_HITTEST: u32 = 4114u32;
pub const LVM_INSERTCOLUMN: u32 = 4193u32;
pub const LVM_INSERTCOLUMNA: u32 = 4123u32;
pub const LVM_INSERTCOLUMNW: u32 = 4193u32;
pub const LVM_INSERTGROUP: u32 = 4241u32;
pub const LVM_INSERTGROUPSORTED: u32 = 4255u32;
pub const LVM_INSERTITEM: u32 = 4173u32;
pub const LVM_INSERTITEMA: u32 = 4103u32;
pub const LVM_INSERTITEMW: u32 = 4173u32;
pub const LVM_INSERTMARKHITTEST: u32 = 4264u32;
pub const LVM_ISGROUPVIEWENABLED: u32 = 4271u32;
pub const LVM_ISITEMVISIBLE: u32 = 4278u32;
pub const LVM_MAPIDTOINDEX: u32 = 4277u32;
pub const LVM_MAPINDEXTOID: u32 = 4276u32;
pub const LVM_MOVEGROUP: u32 = 4247u32;
pub const LVM_MOVEITEMTOGROUP: u32 = 4250u32;
pub const LVM_REDRAWITEMS: u32 = 4117u32;
pub const LVM_REMOVEALLGROUPS: u32 = 4256u32;
pub const LVM_REMOVEGROUP: u32 = 4246u32;
pub const LVM_SCROLL: u32 = 4116u32;
pub const LVM_SETBKCOLOR: u32 = 4097u32;
pub const LVM_SETBKIMAGE: u32 = 4234u32;
pub const LVM_SETBKIMAGEA: u32 = 4164u32;
pub const LVM_SETBKIMAGEW: u32 = 4234u32;
pub const LVM_SETCALLBACKMASK: u32 = 4107u32;
pub const LVM_SETCOLUMN: u32 = 4192u32;
pub const LVM_SETCOLUMNA: u32 = 4122u32;
pub const LVM_SETCOLUMNORDERARRAY: u32 = 4154u32;
pub const LVM_SETCOLUMNW: u32 = 4192u32;
pub const LVM_SETCOLUMNWIDTH: u32 = 4126u32;
pub const LVM_SETEXTENDEDLISTVIEWSTYLE: u32 = 4150u32;
pub const LVM_SETGROUPINFO: u32 = 4243u32;
pub const LVM_SETGROUPMETRICS: u32 = 4251u32;
pub const LVM_SETHOTCURSOR: u32 = 4158u32;
pub const LVM_SETHOTITEM: u32 = 4156u32;
pub const LVM_SETHOVERTIME: u32 = 4167u32;
pub const LVM_SETICONSPACING: u32 = 4149u32;
pub const LVM_SETIMAGELIST: u32 = 4099u32;
pub const LVM_SETINFOTIP: u32 = 4269u32;
pub const LVM_SETINSERTMARK: u32 = 4262u32;
pub const LVM_SETINSERTMARKCOLOR: u32 = 4266u32;
pub const LVM_SETITEM: u32 = 4172u32;
pub const LVM_SETITEMA: u32 = 4102u32;
pub const LVM_SETITEMCOUNT: u32 = 4143u32;
pub const LVM_SETITEMINDEXSTATE: u32 = 4306u32;
pub const LVM_SETITEMPOSITION: u32 = 4111u32;
pub const LVM_SETITEMPOSITION32: u32 = 4145u32;
pub const LVM_SETITEMSTATE: u32 = 4139u32;
pub const LVM_SETITEMTEXT: u32 = 4212u32;
pub const LVM_SETITEMTEXTA: u32 = 4142u32;
pub const LVM_SETITEMTEXTW: u32 = 4212u32;
pub const LVM_SETITEMW: u32 = 4172u32;
pub const LVM_SETOUTLINECOLOR: u32 = 4273u32;
pub const LVM_SETSELECTEDCOLUMN: u32 = 4236u32;
pub const LVM_SETSELECTIONMARK: u32 = 4163u32;
pub const LVM_SETTEXTBKCOLOR: u32 = 4134u32;
pub const LVM_SETTEXTCOLOR: u32 = 4132u32;
pub const LVM_SETTILEINFO: u32 = 4260u32;
pub const LVM_SETTILEVIEWINFO: u32 = 4258u32;
pub const LVM_SETTOOLTIPS: u32 = 4170u32;
pub const LVM_SETUNICODEFORMAT: u32 = 8197u32;
pub const LVM_SETVIEW: u32 = 4238u32;
pub const LVM_SETWORKAREAS: u32 = 4161u32;
pub const LVM_SORTGROUPS: u32 = 4254u32;
pub const LVM_SORTITEMS: u32 = 4144u32;
pub const LVM_SORTITEMSEX: u32 = 4177u32;
pub const LVM_SUBITEMHITTEST: u32 = 4153u32;
pub const LVM_UPDATE: u32 = 4138u32;
pub const LVNI_ABOVE: u32 = 256u32;
pub const LVNI_ALL: u32 = 0u32;
pub const LVNI_BELOW: u32 = 512u32;
pub const LVNI_CUT: u32 = 4u32;
pub const LVNI_DROPHILITED: u32 = 8u32;
pub const LVNI_FOCUSED: u32 = 1u32;
pub const LVNI_PREVIOUS: u32 = 32u32;
pub const LVNI_SAMEGROUPONLY: u32 = 128u32;
pub const LVNI_SELECTED: u32 = 2u32;
pub const LVNI_TOLEFT: u32 = 1024u32;
pub const LVNI_TORIGHT: u32 = 2048u32;
pub const LVNI_VISIBLEONLY: u32 = 64u32;
pub const LVNI_VISIBLEORDER: u32 = 16u32;
pub const LVNSCH_DEFAULT: i32 = -1i32;
pub const LVNSCH_ERROR: i32 = -2i32;
pub const LVNSCH_IGNORE: i32 = -3i32;
pub const LVN_BEGINDRAG: u32 = 4294967187u32;
pub const LVN_BEGINLABELEDIT: u32 = 4294967121u32;
pub const LVN_BEGINLABELEDITA: u32 = 4294967191u32;
pub const LVN_BEGINLABELEDITW: u32 = 4294967121u32;
pub const LVN_BEGINRDRAG: u32 = 4294967185u32;
pub const LVN_BEGINSCROLL: u32 = 4294967116u32;
pub const LVN_COLUMNCLICK: u32 = 4294967188u32;
pub const LVN_COLUMNDROPDOWN: u32 = 4294967132u32;
pub const LVN_COLUMNOVERFLOWCLICK: u32 = 4294967130u32;
pub const LVN_DELETEALLITEMS: u32 = 4294967192u32;
pub const LVN_DELETEITEM: u32 = 4294967193u32;
pub const LVN_ENDLABELEDIT: u32 = 4294967120u32;
pub const LVN_ENDLABELEDITA: u32 = 4294967190u32;
pub const LVN_ENDLABELEDITW: u32 = 4294967120u32;
pub const LVN_ENDSCROLL: u32 = 4294967115u32;
pub const LVN_FIRST: u32 = 4294967196u32;
pub const LVN_GETDISPINFO: u32 = 4294967119u32;
pub const LVN_GETDISPINFOA: u32 = 4294967146u32;
pub const LVN_GETDISPINFOW: u32 = 4294967119u32;
pub const LVN_GETEMPTYMARKUP: u32 = 4294967109u32;
pub const LVN_GETINFOTIP: u32 = 4294967138u32;
pub const LVN_GETINFOTIPA: u32 = 4294967139u32;
pub const LVN_GETINFOTIPW: u32 = 4294967138u32;
pub const LVN_HOTTRACK: u32 = 4294967175u32;
pub const LVN_INCREMENTALSEARCH: u32 = 4294967133u32;
pub const LVN_INCREMENTALSEARCHA: u32 = 4294967134u32;
pub const LVN_INCREMENTALSEARCHW: u32 = 4294967133u32;
pub const LVN_INSERTITEM: u32 = 4294967194u32;
pub const LVN_ITEMACTIVATE: u32 = 4294967182u32;
pub const LVN_ITEMCHANGED: u32 = 4294967195u32;
pub const LVN_ITEMCHANGING: u32 = 4294967196u32;
pub const LVN_KEYDOWN: u32 = 4294967141u32;
pub const LVN_LAST: u32 = 4294967097u32;
pub const LVN_LINKCLICK: u32 = 4294967112u32;
pub const LVN_MARQUEEBEGIN: u32 = 4294967140u32;
pub const LVN_ODCACHEHINT: u32 = 4294967183u32;
pub const LVN_ODFINDITEM: u32 = 4294967117u32;
pub const LVN_ODFINDITEMA: u32 = 4294967144u32;
pub const LVN_ODFINDITEMW: u32 = 4294967117u32;
pub const LVN_ODSTATECHANGED: u32 = 4294967181u32;
pub const LVN_SETDISPINFO: u32 = 4294967118u32;
pub const LVN_SETDISPINFOA: u32 = 4294967145u32;
pub const LVN_SETDISPINFOW: u32 = 4294967118u32;
pub const LVP_COLLAPSEBUTTON: LISTVIEWPARTS = LISTVIEWPARTS(9i32);
pub const LVP_COLUMNDETAIL: LISTVIEWPARTS = LISTVIEWPARTS(10i32);
pub const LVP_EMPTYTEXT: LISTVIEWPARTS = LISTVIEWPARTS(5i32);
pub const LVP_EXPANDBUTTON: LISTVIEWPARTS = LISTVIEWPARTS(8i32);
pub const LVP_GROUPHEADER: LISTVIEWPARTS = LISTVIEWPARTS(6i32);
pub const LVP_GROUPHEADERLINE: LISTVIEWPARTS = LISTVIEWPARTS(7i32);
pub const LVP_LISTDETAIL: LISTVIEWPARTS = LISTVIEWPARTS(3i32);
pub const LVP_LISTGROUP: LISTVIEWPARTS = LISTVIEWPARTS(2i32);
pub const LVP_LISTITEM: LISTVIEWPARTS = LISTVIEWPARTS(1i32);
pub const LVP_LISTSORTEDDETAIL: LISTVIEWPARTS = LISTVIEWPARTS(4i32);
pub const LVSCW_AUTOSIZE: i32 = -1i32;
pub const LVSCW_AUTOSIZE_USEHEADER: i32 = -2i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVSETINFOTIP {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pszText: windows_core::PWSTR,
    pub iItem: i32,
    pub iSubItem: i32,
}
pub const LVSICF_NOINVALIDATEALL: u32 = 1u32;
pub const LVSICF_NOSCROLL: u32 = 2u32;
pub const LVSIL_GROUPHEADER: u32 = 3u32;
pub const LVSIL_NORMAL: u32 = 0u32;
pub const LVSIL_SMALL: u32 = 1u32;
pub const LVSIL_STATE: u32 = 2u32;
pub const LVS_ALIGNLEFT: u32 = 2048u32;
pub const LVS_ALIGNMASK: u32 = 3072u32;
pub const LVS_ALIGNTOP: u32 = 0u32;
pub const LVS_AUTOARRANGE: u32 = 256u32;
pub const LVS_EDITLABELS: u32 = 512u32;
pub const LVS_EX_AUTOAUTOARRANGE: u32 = 16777216u32;
pub const LVS_EX_AUTOCHECKSELECT: u32 = 134217728u32;
pub const LVS_EX_AUTOSIZECOLUMNS: u32 = 268435456u32;
pub const LVS_EX_BORDERSELECT: u32 = 32768u32;
pub const LVS_EX_CHECKBOXES: u32 = 4u32;
pub const LVS_EX_COLUMNOVERFLOW: u32 = 2147483648u32;
pub const LVS_EX_COLUMNSNAPPOINTS: u32 = 1073741824u32;
pub const LVS_EX_DOUBLEBUFFER: u32 = 65536u32;
pub const LVS_EX_FLATSB: u32 = 256u32;
pub const LVS_EX_FULLROWSELECT: u32 = 32u32;
pub const LVS_EX_GRIDLINES: u32 = 1u32;
pub const LVS_EX_HEADERDRAGDROP: u32 = 16u32;
pub const LVS_EX_HEADERINALLVIEWS: u32 = 33554432u32;
pub const LVS_EX_HIDELABELS: u32 = 131072u32;
pub const LVS_EX_INFOTIP: u32 = 1024u32;
pub const LVS_EX_JUSTIFYCOLUMNS: u32 = 2097152u32;
pub const LVS_EX_LABELTIP: u32 = 16384u32;
pub const LVS_EX_MULTIWORKAREAS: u32 = 8192u32;
pub const LVS_EX_ONECLICKACTIVATE: u32 = 64u32;
pub const LVS_EX_REGIONAL: u32 = 512u32;
pub const LVS_EX_SIMPLESELECT: u32 = 1048576u32;
pub const LVS_EX_SINGLEROW: u32 = 262144u32;
pub const LVS_EX_SNAPTOGRID: u32 = 524288u32;
pub const LVS_EX_SUBITEMIMAGES: u32 = 2u32;
pub const LVS_EX_TRACKSELECT: u32 = 8u32;
pub const LVS_EX_TRANSPARENTBKGND: u32 = 4194304u32;
pub const LVS_EX_TRANSPARENTSHADOWTEXT: u32 = 8388608u32;
pub const LVS_EX_TWOCLICKACTIVATE: u32 = 128u32;
pub const LVS_EX_UNDERLINECOLD: u32 = 4096u32;
pub const LVS_EX_UNDERLINEHOT: u32 = 2048u32;
pub const LVS_ICON: u32 = 0u32;
pub const LVS_LIST: u32 = 3u32;
pub const LVS_NOCOLUMNHEADER: u32 = 16384u32;
pub const LVS_NOLABELWRAP: u32 = 128u32;
pub const LVS_NOSCROLL: u32 = 8192u32;
pub const LVS_NOSORTHEADER: u32 = 32768u32;
pub const LVS_OWNERDATA: u32 = 4096u32;
pub const LVS_OWNERDRAWFIXED: u32 = 1024u32;
pub const LVS_REPORT: u32 = 1u32;
pub const LVS_SHAREIMAGELISTS: u32 = 64u32;
pub const LVS_SHOWSELALWAYS: u32 = 8u32;
pub const LVS_SINGLESEL: u32 = 4u32;
pub const LVS_SMALLICON: u32 = 2u32;
pub const LVS_SORTASCENDING: u32 = 16u32;
pub const LVS_SORTDESCENDING: u32 = 32u32;
pub const LVS_TYPEMASK: u32 = 3u32;
pub const LVS_TYPESTYLEMASK: u32 = 64512u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LVTILEINFO {
    pub cbSize: u32,
    pub iItem: i32,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut i32,
}
impl Default for LVTILEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LVTILEVIEWINFO {
    pub cbSize: u32,
    pub dwMask: LVTILEVIEWINFO_MASK,
    pub dwFlags: LVTILEVIEWINFO_FLAGS,
    pub sizeTile: super::super::Foundation::SIZE,
    pub cLines: i32,
    pub rcLabelMargin: super::super::Foundation::RECT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVTILEVIEWINFO_FLAGS(pub u32);
impl LVTILEVIEWINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LVTILEVIEWINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LVTILEVIEWINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LVTILEVIEWINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LVTILEVIEWINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LVTILEVIEWINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LVTILEVIEWINFO_MASK(pub u32);
impl LVTILEVIEWINFO_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LVTILEVIEWINFO_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LVTILEVIEWINFO_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LVTILEVIEWINFO_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LVTILEVIEWINFO_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LVTILEVIEWINFO_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LVTVIF_AUTOSIZE: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(0u32);
pub const LVTVIF_EXTENDED: u32 = 4u32;
pub const LVTVIF_FIXEDHEIGHT: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(2u32);
pub const LVTVIF_FIXEDSIZE: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(3u32);
pub const LVTVIF_FIXEDWIDTH: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(1u32);
pub const LVTVIM_COLUMNS: LVTILEVIEWINFO_MASK = LVTILEVIEWINFO_MASK(2u32);
pub const LVTVIM_LABELMARGIN: LVTILEVIEWINFO_MASK = LVTILEVIEWINFO_MASK(4u32);
pub const LVTVIM_TILESIZE: LVTILEVIEWINFO_MASK = LVTILEVIEWINFO_MASK(1u32);
pub const LV_MAX_WORKAREAS: u32 = 16u32;
pub const LV_VIEW_DETAILS: u32 = 1u32;
pub const LV_VIEW_ICON: u32 = 0u32;
pub const LV_VIEW_LIST: u32 = 3u32;
pub const LV_VIEW_MAX: u32 = 4u32;
pub const LV_VIEW_SMALLICON: u32 = 2u32;
pub const LV_VIEW_TILE: u32 = 4u32;
pub const LWS_IGNORERETURN: u32 = 2u32;
pub const LWS_NOPREFIX: u32 = 4u32;
pub const LWS_RIGHT: u32 = 32u32;
pub const LWS_TRANSPARENT: u32 = 1u32;
pub const LWS_USECUSTOMTEXT: u32 = 16u32;
pub const LWS_USEVISUALSTYLE: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MARGINS {
    pub cxLeftWidth: i32,
    pub cxRightWidth: i32,
    pub cyTopHeight: i32,
    pub cyBottomHeight: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MARKUPTEXTSTATES(pub i32);
pub const MAXBS_DISABLED: MAXBUTTONSTATES = MAXBUTTONSTATES(4i32);
pub const MAXBS_HOT: MAXBUTTONSTATES = MAXBUTTONSTATES(2i32);
pub const MAXBS_NORMAL: MAXBUTTONSTATES = MAXBUTTONSTATES(1i32);
pub const MAXBS_PUSHED: MAXBUTTONSTATES = MAXBUTTONSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MAXBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MAXCAPTIONSTATES(pub i32);
pub const MAXPROPPAGES: u32 = 100u32;
pub const MAX_INTLIST_COUNT: u32 = 402u32;
pub const MAX_LINKID_TEXT: u32 = 48u32;
pub const MAX_THEMECOLOR: u32 = 64u32;
pub const MAX_THEMESIZE: u32 = 64u32;
pub const MBI_DISABLED: BARITEMSTATES = BARITEMSTATES(4i32);
pub const MBI_DISABLEDHOT: BARITEMSTATES = BARITEMSTATES(5i32);
pub const MBI_DISABLEDPUSHED: BARITEMSTATES = BARITEMSTATES(6i32);
pub const MBI_HOT: BARITEMSTATES = BARITEMSTATES(2i32);
pub const MBI_NORMAL: BARITEMSTATES = BARITEMSTATES(1i32);
pub const MBI_PUSHED: BARITEMSTATES = BARITEMSTATES(3i32);
pub const MB_ACTIVE: BARBACKGROUNDSTATES = BARBACKGROUNDSTATES(1i32);
pub const MB_INACTIVE: BARBACKGROUNDSTATES = BARBACKGROUNDSTATES(2i32);
pub const MCB_BITMAP: POPUPCHECKBACKGROUNDSTATES = POPUPCHECKBACKGROUNDSTATES(3i32);
pub const MCB_DISABLED: POPUPCHECKBACKGROUNDSTATES = POPUPCHECKBACKGROUNDSTATES(1i32);
pub const MCB_NORMAL: POPUPCHECKBACKGROUNDSTATES = POPUPCHECKBACKGROUNDSTATES(2i32);
pub const MCGCB_HOT: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(2i32);
pub const MCGCB_SELECTED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(1i32);
pub const MCGCB_SELECTEDHOT: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(3i32);
pub const MCGCB_SELECTEDNOTFOCUSED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(4i32);
pub const MCGCB_TODAY: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(5i32);
pub const MCGCB_TODAYSELECTED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(6i32);
pub const MCGCU_HASSTATE: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(2i32);
pub const MCGCU_HASSTATEHOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(3i32);
pub const MCGCU_HOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(1i32);
pub const MCGCU_SELECTED: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(4i32);
pub const MCGCU_SELECTEDHOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(5i32);
pub const MCGC_HASSTATE: GRIDCELLSTATES = GRIDCELLSTATES(2i32);
pub const MCGC_HASSTATEHOT: GRIDCELLSTATES = GRIDCELLSTATES(3i32);
pub const MCGC_HOT: GRIDCELLSTATES = GRIDCELLSTATES(1i32);
pub const MCGC_SELECTED: GRIDCELLSTATES = GRIDCELLSTATES(6i32);
pub const MCGC_SELECTEDHOT: GRIDCELLSTATES = GRIDCELLSTATES(7i32);
pub const MCGC_TODAY: GRIDCELLSTATES = GRIDCELLSTATES(4i32);
pub const MCGC_TODAYSELECTED: GRIDCELLSTATES = GRIDCELLSTATES(5i32);
pub const MCGIF_DATE: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(1u32);
pub const MCGIF_NAME: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(4u32);
pub const MCGIF_RECT: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(2u32);
pub const MCGIP_CALENDAR: MCGRIDINFO_PART = MCGRIDINFO_PART(4u32);
pub const MCGIP_CALENDARBODY: MCGRIDINFO_PART = MCGRIDINFO_PART(6u32);
pub const MCGIP_CALENDARCELL: MCGRIDINFO_PART = MCGRIDINFO_PART(8u32);
pub const MCGIP_CALENDARCONTROL: MCGRIDINFO_PART = MCGRIDINFO_PART(0u32);
pub const MCGIP_CALENDARHEADER: MCGRIDINFO_PART = MCGRIDINFO_PART(5u32);
pub const MCGIP_CALENDARROW: MCGRIDINFO_PART = MCGRIDINFO_PART(7u32);
pub const MCGIP_FOOTER: MCGRIDINFO_PART = MCGRIDINFO_PART(3u32);
pub const MCGIP_NEXT: MCGRIDINFO_PART = MCGRIDINFO_PART(1u32);
pub const MCGIP_PREV: MCGRIDINFO_PART = MCGRIDINFO_PART(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MCGRIDINFO {
    pub cbSize: u32,
    pub dwPart: MCGRIDINFO_PART,
    pub dwFlags: MCGRIDINFO_FLAGS,
    pub iCalendar: i32,
    pub iRow: i32,
    pub iCol: i32,
    pub bSelected: windows_core::BOOL,
    pub stStart: super::super::Foundation::SYSTEMTIME,
    pub stEnd: super::super::Foundation::SYSTEMTIME,
    pub rc: super::super::Foundation::RECT,
    pub pszName: windows_core::PWSTR,
    pub cchName: usize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MCGRIDINFO_FLAGS(pub u32);
impl MCGRIDINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MCGRIDINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MCGRIDINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MCGRIDINFO_PART(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MCHITTESTINFO {
    pub cbSize: u32,
    pub pt: super::super::Foundation::POINT,
    pub uHit: MCHITTESTINFO_HIT_FLAGS,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub rc: super::super::Foundation::RECT,
    pub iOffset: i32,
    pub iRow: i32,
    pub iCol: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MCHITTESTINFO_HIT_FLAGS(pub u32);
impl MCHITTESTINFO_HIT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MCHITTESTINFO_HIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MCHITTESTINFO_HIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MCHITTESTINFO_HIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MCHITTESTINFO_HIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MCHITTESTINFO_HIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MCHT_CALENDAR: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131072u32);
pub const MCHT_CALENDARBK: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131072u32);
pub const MCHT_CALENDARCONTROL: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(1048576u32);
pub const MCHT_CALENDARDATE: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131073u32);
pub const MCHT_CALENDARDATEMAX: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131077u32);
pub const MCHT_CALENDARDATEMIN: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131076u32);
pub const MCHT_CALENDARDATENEXT: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(16908289u32);
pub const MCHT_CALENDARDATEPREV: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(33685505u32);
pub const MCHT_CALENDARDAY: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131074u32);
pub const MCHT_CALENDARWEEKNUM: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131075u32);
pub const MCHT_NEXT: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(16777216u32);
pub const MCHT_NOWHERE: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(0u32);
pub const MCHT_PREV: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(33554432u32);
pub const MCHT_TITLE: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(65536u32);
pub const MCHT_TITLEBK: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(65536u32);
pub const MCHT_TITLEBTNNEXT: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(16842755u32);
pub const MCHT_TITLEBTNPREV: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(33619971u32);
pub const MCHT_TITLEMONTH: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(65537u32);
pub const MCHT_TITLEYEAR: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(65538u32);
pub const MCHT_TODAYLINK: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(196608u32);
pub const MCMV_CENTURY: MONTH_CALDENDAR_MESSAGES_VIEW = MONTH_CALDENDAR_MESSAGES_VIEW(3u32);
pub const MCMV_DECADE: MONTH_CALDENDAR_MESSAGES_VIEW = MONTH_CALDENDAR_MESSAGES_VIEW(2u32);
pub const MCMV_MAX: MONTH_CALDENDAR_MESSAGES_VIEW = MONTH_CALDENDAR_MESSAGES_VIEW(3u32);
pub const MCMV_MONTH: MONTH_CALDENDAR_MESSAGES_VIEW = MONTH_CALDENDAR_MESSAGES_VIEW(0u32);
pub const MCMV_YEAR: MONTH_CALDENDAR_MESSAGES_VIEW = MONTH_CALDENDAR_MESSAGES_VIEW(1u32);
pub const MCM_FIRST: u32 = 4096u32;
pub const MCM_GETCALENDARBORDER: u32 = 4127u32;
pub const MCM_GETCALENDARCOUNT: u32 = 4119u32;
pub const MCM_GETCALENDARGRIDINFO: u32 = 4120u32;
pub const MCM_GETCALID: u32 = 4123u32;
pub const MCM_GETCOLOR: u32 = 4107u32;
pub const MCM_GETCURRENTVIEW: u32 = 4118u32;
pub const MCM_GETCURSEL: u32 = 4097u32;
pub const MCM_GETFIRSTDAYOFWEEK: u32 = 4112u32;
pub const MCM_GETMAXSELCOUNT: u32 = 4099u32;
pub const MCM_GETMAXTODAYWIDTH: u32 = 4117u32;
pub const MCM_GETMINREQRECT: u32 = 4105u32;
pub const MCM_GETMONTHDELTA: u32 = 4115u32;
pub const MCM_GETMONTHRANGE: u32 = 4103u32;
pub const MCM_GETRANGE: u32 = 4113u32;
pub const MCM_GETSELRANGE: u32 = 4101u32;
pub const MCM_GETTODAY: u32 = 4109u32;
pub const MCM_GETUNICODEFORMAT: u32 = 8198u32;
pub const MCM_HITTEST: u32 = 4110u32;
pub const MCM_SETCALENDARBORDER: u32 = 4126u32;
pub const MCM_SETCALID: u32 = 4124u32;
pub const MCM_SETCOLOR: u32 = 4106u32;
pub const MCM_SETCURRENTVIEW: u32 = 4128u32;
pub const MCM_SETCURSEL: u32 = 4098u32;
pub const MCM_SETDAYSTATE: u32 = 4104u32;
pub const MCM_SETFIRSTDAYOFWEEK: u32 = 4111u32;
pub const MCM_SETMAXSELCOUNT: u32 = 4100u32;
pub const MCM_SETMONTHDELTA: u32 = 4116u32;
pub const MCM_SETRANGE: u32 = 4114u32;
pub const MCM_SETSELRANGE: u32 = 4102u32;
pub const MCM_SETTODAY: u32 = 4108u32;
pub const MCM_SETUNICODEFORMAT: u32 = 8197u32;
pub const MCM_SIZERECTTOMIN: u32 = 4125u32;
pub const MCNN_DISABLED: NAVNEXTSTATES = NAVNEXTSTATES(4i32);
pub const MCNN_HOT: NAVNEXTSTATES = NAVNEXTSTATES(2i32);
pub const MCNN_NORMAL: NAVNEXTSTATES = NAVNEXTSTATES(1i32);
pub const MCNN_PRESSED: NAVNEXTSTATES = NAVNEXTSTATES(3i32);
pub const MCNP_DISABLED: NAVPREVSTATES = NAVPREVSTATES(4i32);
pub const MCNP_HOT: NAVPREVSTATES = NAVPREVSTATES(2i32);
pub const MCNP_NORMAL: NAVPREVSTATES = NAVPREVSTATES(1i32);
pub const MCNP_PRESSED: NAVPREVSTATES = NAVPREVSTATES(3i32);
pub const MCN_FIRST: u32 = 4294966550u32;
pub const MCN_GETDAYSTATE: u32 = 4294966549u32;
pub const MCN_LAST: u32 = 4294966544u32;
pub const MCN_SELCHANGE: u32 = 4294966547u32;
pub const MCN_SELECT: u32 = 4294966550u32;
pub const MCN_VIEWCHANGE: u32 = 4294966546u32;
pub const MCSC_BACKGROUND: u32 = 0u32;
pub const MCSC_MONTHBK: u32 = 4u32;
pub const MCSC_TEXT: u32 = 1u32;
pub const MCSC_TITLEBK: u32 = 2u32;
pub const MCSC_TITLETEXT: u32 = 3u32;
pub const MCSC_TRAILINGTEXT: u32 = 5u32;
pub const MCS_DAYSTATE: u32 = 1u32;
pub const MCS_MULTISELECT: u32 = 2u32;
pub const MCS_NOSELCHANGEONNAV: u32 = 256u32;
pub const MCS_NOTODAY: u32 = 16u32;
pub const MCS_NOTODAYCIRCLE: u32 = 8u32;
pub const MCS_NOTRAILINGDATES: u32 = 64u32;
pub const MCS_SHORTDAYSOFWEEK: u32 = 128u32;
pub const MCS_WEEKNUMBERS: u32 = 4u32;
pub const MCTGCU_HASSTATE: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(2i32);
pub const MCTGCU_HASSTATEHOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(3i32);
pub const MCTGCU_HOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(1i32);
pub const MCTGCU_SELECTED: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(4i32);
pub const MCTGCU_SELECTEDHOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(5i32);
pub const MCTGC_HASSTATE: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(2i32);
pub const MCTGC_HASSTATEHOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(3i32);
pub const MCTGC_HOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(1i32);
pub const MCTGC_SELECTED: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(6i32);
pub const MCTGC_SELECTEDHOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(7i32);
pub const MCTGC_TODAY: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(4i32);
pub const MCTGC_TODAYSELECTED: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(5i32);
pub const MC_BACKGROUND: MONTHCALPARTS = MONTHCALPARTS(1i32);
pub const MC_BORDERS: MONTHCALPARTS = MONTHCALPARTS(2i32);
pub const MC_BULLETDISABLED: POPUPCHECKSTATES = POPUPCHECKSTATES(4i32);
pub const MC_BULLETNORMAL: POPUPCHECKSTATES = POPUPCHECKSTATES(3i32);
pub const MC_CHECKMARKDISABLED: POPUPCHECKSTATES = POPUPCHECKSTATES(2i32);
pub const MC_CHECKMARKNORMAL: POPUPCHECKSTATES = POPUPCHECKSTATES(1i32);
pub const MC_COLHEADERSPLITTER: MONTHCALPARTS = MONTHCALPARTS(4i32);
pub const MC_GRIDBACKGROUND: MONTHCALPARTS = MONTHCALPARTS(3i32);
pub const MC_GRIDCELL: MONTHCALPARTS = MONTHCALPARTS(6i32);
pub const MC_GRIDCELLBACKGROUND: MONTHCALPARTS = MONTHCALPARTS(5i32);
pub const MC_GRIDCELLUPPER: MONTHCALPARTS = MONTHCALPARTS(7i32);
pub const MC_NAVNEXT: MONTHCALPARTS = MONTHCALPARTS(10i32);
pub const MC_NAVPREV: MONTHCALPARTS = MONTHCALPARTS(11i32);
pub const MC_TRAILINGGRIDCELL: MONTHCALPARTS = MONTHCALPARTS(8i32);
pub const MC_TRAILINGGRIDCELLUPPER: MONTHCALPARTS = MONTHCALPARTS(9i32);
pub const MDCL_DISABLED: MDICLOSEBUTTONSTATES = MDICLOSEBUTTONSTATES(4i32);
pub const MDCL_HOT: MDICLOSEBUTTONSTATES = MDICLOSEBUTTONSTATES(2i32);
pub const MDCL_NORMAL: MDICLOSEBUTTONSTATES = MDICLOSEBUTTONSTATES(1i32);
pub const MDCL_PUSHED: MDICLOSEBUTTONSTATES = MDICLOSEBUTTONSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MDICLOSEBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MDIMINBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MDIRESTOREBUTTONSTATES(pub i32);
pub const MDMI_DISABLED: MDIMINBUTTONSTATES = MDIMINBUTTONSTATES(4i32);
pub const MDMI_HOT: MDIMINBUTTONSTATES = MDIMINBUTTONSTATES(2i32);
pub const MDMI_NORMAL: MDIMINBUTTONSTATES = MDIMINBUTTONSTATES(1i32);
pub const MDMI_PUSHED: MDIMINBUTTONSTATES = MDIMINBUTTONSTATES(3i32);
pub const MDP_NEWAPPBUTTON: MENUBANDPARTS = MENUBANDPARTS(1i32);
pub const MDP_SEPERATOR: MENUBANDPARTS = MENUBANDPARTS(2i32);
pub const MDRE_DISABLED: MDIRESTOREBUTTONSTATES = MDIRESTOREBUTTONSTATES(4i32);
pub const MDRE_HOT: MDIRESTOREBUTTONSTATES = MDIRESTOREBUTTONSTATES(2i32);
pub const MDRE_NORMAL: MDIRESTOREBUTTONSTATES = MDIRESTOREBUTTONSTATES(1i32);
pub const MDRE_PUSHED: MDIRESTOREBUTTONSTATES = MDIRESTOREBUTTONSTATES(3i32);
pub const MDS_CHECKED: MENUBANDSTATES = MENUBANDSTATES(5i32);
pub const MDS_DISABLED: MENUBANDSTATES = MENUBANDSTATES(4i32);
pub const MDS_HOT: MENUBANDSTATES = MENUBANDSTATES(2i32);
pub const MDS_HOTCHECKED: MENUBANDSTATES = MENUBANDSTATES(6i32);
pub const MDS_NORMAL: MENUBANDSTATES = MENUBANDSTATES(1i32);
pub const MDS_PRESSED: MENUBANDSTATES = MENUBANDSTATES(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEASUREITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemWidth: u32,
    pub itemHeight: u32,
    pub itemData: usize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MENUBANDPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MENUBANDSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MENUPARTS(pub i32);
pub const MENU_BARBACKGROUND: MENUPARTS = MENUPARTS(7i32);
pub const MENU_BARITEM: MENUPARTS = MENUPARTS(8i32);
pub const MENU_CHEVRON_TMSCHEMA: MENUPARTS = MENUPARTS(5i32);
pub const MENU_MENUBARDROPDOWN_TMSCHEMA: MENUPARTS = MENUPARTS(4i32);
pub const MENU_MENUBARITEM_TMSCHEMA: MENUPARTS = MENUPARTS(3i32);
pub const MENU_MENUDROPDOWN_TMSCHEMA: MENUPARTS = MENUPARTS(2i32);
pub const MENU_MENUITEM_TMSCHEMA: MENUPARTS = MENUPARTS(1i32);
pub const MENU_POPUPBACKGROUND: MENUPARTS = MENUPARTS(9i32);
pub const MENU_POPUPBORDERS: MENUPARTS = MENUPARTS(10i32);
pub const MENU_POPUPCHECK: MENUPARTS = MENUPARTS(11i32);
pub const MENU_POPUPCHECKBACKGROUND: MENUPARTS = MENUPARTS(12i32);
pub const MENU_POPUPGUTTER: MENUPARTS = MENUPARTS(13i32);
pub const MENU_POPUPITEM: MENUPARTS = MENUPARTS(14i32);
pub const MENU_POPUPITEMKBFOCUS: MENUPARTS = MENUPARTS(26i32);
pub const MENU_POPUPITEM_FOCUSABLE: MENUPARTS = MENUPARTS(27i32);
pub const MENU_POPUPSEPARATOR: MENUPARTS = MENUPARTS(15i32);
pub const MENU_POPUPSUBMENU: MENUPARTS = MENUPARTS(16i32);
pub const MENU_POPUPSUBMENU_HCHOT: MENUPARTS = MENUPARTS(21i32);
pub const MENU_SEPARATOR_TMSCHEMA: MENUPARTS = MENUPARTS(6i32);
pub const MENU_SYSTEMCLOSE: MENUPARTS = MENUPARTS(17i32);
pub const MENU_SYSTEMCLOSE_HCHOT: MENUPARTS = MENUPARTS(22i32);
pub const MENU_SYSTEMMAXIMIZE: MENUPARTS = MENUPARTS(18i32);
pub const MENU_SYSTEMMAXIMIZE_HCHOT: MENUPARTS = MENUPARTS(23i32);
pub const MENU_SYSTEMMINIMIZE: MENUPARTS = MENUPARTS(19i32);
pub const MENU_SYSTEMMINIMIZE_HCHOT: MENUPARTS = MENUPARTS(24i32);
pub const MENU_SYSTEMRESTORE: MENUPARTS = MENUPARTS(20i32);
pub const MENU_SYSTEMRESTORE_HCHOT: MENUPARTS = MENUPARTS(25i32);
pub const MINBS_DISABLED: MINBUTTONSTATES = MINBUTTONSTATES(4i32);
pub const MINBS_HOT: MINBUTTONSTATES = MINBUTTONSTATES(2i32);
pub const MINBS_NORMAL: MINBUTTONSTATES = MINBUTTONSTATES(1i32);
pub const MINBS_PUSHED: MINBUTTONSTATES = MINBUTTONSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MINBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MINCAPTIONSTATES(pub i32);
pub const MNCS_ACTIVE: MINCAPTIONSTATES = MINCAPTIONSTATES(1i32);
pub const MNCS_DISABLED: MINCAPTIONSTATES = MINCAPTIONSTATES(3i32);
pub const MNCS_INACTIVE: MINCAPTIONSTATES = MINCAPTIONSTATES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MONTHCALPARTS(pub i32);
pub const MONTHCAL_CLASS: windows_core::PCWSTR = windows_core::w!("SysMonthCal32");
pub const MONTHCAL_CLASSA: windows_core::PCSTR = windows_core::s!("SysMonthCal32");
pub const MONTHCAL_CLASSW: windows_core::PCWSTR = windows_core::w!("SysMonthCal32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MONTH_CALDENDAR_MESSAGES_VIEW(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOREPROGRAMSARROWBACKSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOREPROGRAMSARROWSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOREPROGRAMSTABSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOVESTATES(pub i32);
pub const MPIF_DISABLED: POPUPITEMFOCUSABLESTATES = POPUPITEMFOCUSABLESTATES(3i32);
pub const MPIF_DISABLEDHOT: POPUPITEMFOCUSABLESTATES = POPUPITEMFOCUSABLESTATES(4i32);
pub const MPIF_HOT: POPUPITEMFOCUSABLESTATES = POPUPITEMFOCUSABLESTATES(2i32);
pub const MPIF_NORMAL: POPUPITEMFOCUSABLESTATES = POPUPITEMFOCUSABLESTATES(1i32);
pub const MPIKBFOCUS_NORMAL: POPUPITEMKBFOCUSSTATES = POPUPITEMKBFOCUSSTATES(1i32);
pub const MPI_DISABLED: POPUPITEMSTATES = POPUPITEMSTATES(3i32);
pub const MPI_DISABLEDHOT: POPUPITEMSTATES = POPUPITEMSTATES(4i32);
pub const MPI_HOT: POPUPITEMSTATES = POPUPITEMSTATES(2i32);
pub const MPI_NORMAL: POPUPITEMSTATES = POPUPITEMSTATES(1i32);
pub const MSGF_COMMCTRL_BEGINDRAG: u32 = 16896u32;
pub const MSGF_COMMCTRL_DRAGSELECT: u32 = 16898u32;
pub const MSGF_COMMCTRL_SIZEHEADER: u32 = 16897u32;
pub const MSGF_COMMCTRL_TOOLBARCUST: u32 = 16899u32;
pub const MSMHC_HOT: POPUPSUBMENUHCHOTSTATES = POPUPSUBMENUHCHOTSTATES(1i32);
pub const MSM_DISABLED: POPUPSUBMENUSTATES = POPUPSUBMENUSTATES(2i32);
pub const MSM_NORMAL: POPUPSUBMENUSTATES = POPUPSUBMENUSTATES(1i32);
pub const MSYSCHC_HOT: SYSTEMCLOSEHCHOTSTATES = SYSTEMCLOSEHCHOTSTATES(1i32);
pub const MSYSC_DISABLED: SYSTEMCLOSESTATES = SYSTEMCLOSESTATES(2i32);
pub const MSYSC_NORMAL: SYSTEMCLOSESTATES = SYSTEMCLOSESTATES(1i32);
pub const MSYSMNHC_HOT: SYSTEMMINIMIZEHCHOTSTATES = SYSTEMMINIMIZEHCHOTSTATES(1i32);
pub const MSYSMN_DISABLED: SYSTEMMINIMIZESTATES = SYSTEMMINIMIZESTATES(2i32);
pub const MSYSMN_NORMAL: SYSTEMMINIMIZESTATES = SYSTEMMINIMIZESTATES(1i32);
pub const MSYSMXHC_HOT: SYSTEMMAXIMIZEHCHOTSTATES = SYSTEMMAXIMIZEHCHOTSTATES(1i32);
pub const MSYSMX_DISABLED: SYSTEMMAXIMIZESTATES = SYSTEMMAXIMIZESTATES(2i32);
pub const MSYSMX_NORMAL: SYSTEMMAXIMIZESTATES = SYSTEMMAXIMIZESTATES(1i32);
pub const MSYSRHC_HOT: SYSTEMRESTOREHCHOTSTATES = SYSTEMRESTOREHCHOTSTATES(1i32);
pub const MSYSR_DISABLED: SYSTEMRESTORESTATES = SYSTEMRESTORESTATES(2i32);
pub const MSYSR_NORMAL: SYSTEMRESTORESTATES = SYSTEMRESTORESTATES(1i32);
pub const MULTIFILEOPENORD: u32 = 1537u32;
pub const MXCS_ACTIVE: MAXCAPTIONSTATES = MAXCAPTIONSTATES(1i32);
pub const MXCS_DISABLED: MAXCAPTIONSTATES = MAXCAPTIONSTATES(3i32);
pub const MXCS_INACTIVE: MAXCAPTIONSTATES = MAXCAPTIONSTATES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NAVIGATIONPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NAVNEXTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NAVPREVSTATES(pub i32);
pub const NAV_BACKBUTTON: NAVIGATIONPARTS = NAVIGATIONPARTS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NAV_BACKBUTTONSTATES(pub i32);
pub const NAV_BB_DISABLED: NAV_BACKBUTTONSTATES = NAV_BACKBUTTONSTATES(4i32);
pub const NAV_BB_HOT: NAV_BACKBUTTONSTATES = NAV_BACKBUTTONSTATES(2i32);
pub const NAV_BB_NORMAL: NAV_BACKBUTTONSTATES = NAV_BACKBUTTONSTATES(1i32);
pub const NAV_BB_PRESSED: NAV_BACKBUTTONSTATES = NAV_BACKBUTTONSTATES(3i32);
pub const NAV_FB_DISABLED: NAV_FORWARDBUTTONSTATES = NAV_FORWARDBUTTONSTATES(4i32);
pub const NAV_FB_HOT: NAV_FORWARDBUTTONSTATES = NAV_FORWARDBUTTONSTATES(2i32);
pub const NAV_FB_NORMAL: NAV_FORWARDBUTTONSTATES = NAV_FORWARDBUTTONSTATES(1i32);
pub const NAV_FB_PRESSED: NAV_FORWARDBUTTONSTATES = NAV_FORWARDBUTTONSTATES(3i32);
pub const NAV_FORWARDBUTTON: NAVIGATIONPARTS = NAVIGATIONPARTS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NAV_FORWARDBUTTONSTATES(pub i32);
pub const NAV_MB_DISABLED: NAV_MENUBUTTONSTATES = NAV_MENUBUTTONSTATES(4i32);
pub const NAV_MB_HOT: NAV_MENUBUTTONSTATES = NAV_MENUBUTTONSTATES(2i32);
pub const NAV_MB_NORMAL: NAV_MENUBUTTONSTATES = NAV_MENUBUTTONSTATES(1i32);
pub const NAV_MB_PRESSED: NAV_MENUBUTTONSTATES = NAV_MENUBUTTONSTATES(3i32);
pub const NAV_MENUBUTTON: NAVIGATIONPARTS = NAVIGATIONPARTS(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NAV_MENUBUTTONSTATES(pub i32);
pub const NEWFILEOPENORD: u32 = 1547u32;
pub const NEWFILEOPENV2ORD: u32 = 1552u32;
pub const NEWFILEOPENV3ORD: u32 = 1553u32;
pub const NEWFORMATDLGWITHLINK: u32 = 1591u32;
pub const NFS_ALL: u32 = 16u32;
pub const NFS_BUTTON: u32 = 8u32;
pub const NFS_EDIT: u32 = 1u32;
pub const NFS_LISTCOMBO: u32 = 4u32;
pub const NFS_STATIC: u32 = 2u32;
pub const NFS_USEFONTASSOC: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMBCDROPDOWN {
    pub hdr: NMHDR,
    pub rcButton: super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMBCHOTITEM {
    pub hdr: NMHDR,
    pub dwFlags: NMTBHOTITEM_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMCBEDRAGBEGINA {
    pub hdr: NMHDR,
    pub iItemid: i32,
    pub szText: [i8; 260],
}
impl Default for NMCBEDRAGBEGINA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMCBEDRAGBEGINW {
    pub hdr: NMHDR,
    pub iItemid: i32,
    pub szText: [u16; 260],
}
impl Default for NMCBEDRAGBEGINW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMCBEENDEDITA {
    pub hdr: NMHDR,
    pub fChanged: windows_core::BOOL,
    pub iNewSelection: i32,
    pub szText: [i8; 260],
    pub iWhy: i32,
}
impl Default for NMCBEENDEDITA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMCBEENDEDITW {
    pub hdr: NMHDR,
    pub fChanged: windows_core::BOOL,
    pub iNewSelection: i32,
    pub szText: [u16; 260],
    pub iWhy: i32,
}
impl Default for NMCBEENDEDITW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCHAR {
    pub hdr: NMHDR,
    pub ch: u32,
    pub dwItemPrev: u32,
    pub dwItemNext: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCOMBOBOXEXA {
    pub hdr: NMHDR,
    pub ceItem: COMBOBOXEXITEMA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCOMBOBOXEXW {
    pub hdr: NMHDR,
    pub ceItem: COMBOBOXEXITEMW,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCUSTOMDRAW {
    pub hdr: NMHDR,
    pub dwDrawStage: NMCUSTOMDRAW_DRAW_STAGE,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub rc: super::super::Foundation::RECT,
    pub dwItemSpec: usize,
    pub uItemState: NMCUSTOMDRAW_DRAW_STATE_FLAGS,
    pub lItemlParam: super::super::Foundation::LPARAM,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMCUSTOMDRAW_DRAW_STAGE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMCUSTOMDRAW_DRAW_STATE_FLAGS(pub u32);
impl NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMCUSTOMSPLITRECTINFO {
    pub hdr: NMHDR,
    pub rcClient: super::super::Foundation::RECT,
    pub rcButton: super::super::Foundation::RECT,
    pub rcSplit: super::super::Foundation::RECT,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMCUSTOMTEXT {
    pub hdr: NMHDR,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub lpString: windows_core::PCWSTR,
    pub nCount: i32,
    pub lpRect: *mut super::super::Foundation::RECT,
    pub uFormat: u32,
    pub fLink: windows_core::BOOL,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for NMCUSTOMTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMECHANGE {
    pub nmhdr: NMHDR,
    pub dwFlags: NMDATETIMECHANGE_FLAGS,
    pub st: super::super::Foundation::SYSTEMTIME,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMDATETIMECHANGE_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMDATETIMEFORMATA {
    pub nmhdr: NMHDR,
    pub pszFormat: windows_core::PCSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub pszDisplay: windows_core::PCSTR,
    pub szDisplay: [i8; 64],
}
impl Default for NMDATETIMEFORMATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMEFORMATQUERYA {
    pub nmhdr: NMHDR,
    pub pszFormat: windows_core::PCSTR,
    pub szMax: super::super::Foundation::SIZE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMEFORMATQUERYW {
    pub nmhdr: NMHDR,
    pub pszFormat: windows_core::PCWSTR,
    pub szMax: super::super::Foundation::SIZE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMDATETIMEFORMATW {
    pub nmhdr: NMHDR,
    pub pszFormat: windows_core::PCWSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub pszDisplay: windows_core::PCWSTR,
    pub szDisplay: [u16; 64],
}
impl Default for NMDATETIMEFORMATW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMESTRINGA {
    pub nmhdr: NMHDR,
    pub pszUserString: windows_core::PCSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMESTRINGW {
    pub nmhdr: NMHDR,
    pub pszUserString: windows_core::PCWSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMEWMKEYDOWNA {
    pub nmhdr: NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: windows_core::PCSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMDATETIMEWMKEYDOWNW {
    pub nmhdr: NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: windows_core::PCWSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMDAYSTATE {
    pub nmhdr: NMHDR,
    pub stStart: super::super::Foundation::SYSTEMTIME,
    pub cDayState: i32,
    pub prgDayState: *mut u32,
}
impl Default for NMDAYSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMHDDISPINFOA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub mask: HDI_MASK,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMHDDISPINFOW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub mask: HDI_MASK,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMHDFILTERBTNCLICK {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMHDR {
    pub hwndFrom: super::super::Foundation::HWND,
    pub idFrom: usize,
    pub code: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMHEADERA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iButton: HEADER_CONTROL_NOTIFICATION_BUTTON,
    pub pitem: *mut HDITEMA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for NMHEADERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMHEADERW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iButton: HEADER_CONTROL_NOTIFICATION_BUTTON,
    pub pitem: *mut HDITEMW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for NMHEADERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMIPADDRESS {
    pub hdr: NMHDR,
    pub iField: i32,
    pub iValue: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMITEMACTIVATE {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: super::super::Foundation::POINT,
    pub lParam: super::super::Foundation::LPARAM,
    pub uKeyFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMKEY {
    pub hdr: NMHDR,
    pub nVKey: u32,
    pub uFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLINK {
    pub hdr: NMHDR,
    pub item: LITEM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLISTVIEW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: LIST_VIEW_ITEM_FLAGS,
    pub ptAction: super::super::Foundation::POINT,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVCACHEHINT {
    pub hdr: NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: super::super::Foundation::COLORREF,
    pub clrTextBk: super::super::Foundation::COLORREF,
    pub iSubItem: i32,
    pub dwItemType: NMLVCUSTOMDRAW_ITEM_TYPE,
    pub clrFace: super::super::Foundation::COLORREF,
    pub iIconEffect: i32,
    pub iIconPhase: i32,
    pub iPartId: i32,
    pub iStateId: i32,
    pub rcText: super::super::Foundation::RECT,
    pub uAlign: LIST_VIEW_GROUP_ALIGN_FLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVCUSTOMDRAW_ITEM_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVDISPINFOA {
    pub hdr: NMHDR,
    pub item: LVITEMA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVDISPINFOW {
    pub hdr: NMHDR,
    pub item: LVITEMW,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMLVEMPTYMARKUP {
    pub hdr: NMHDR,
    pub dwFlags: NMLVEMPTYMARKUP_FLAGS,
    pub szMarkup: [u16; 2084],
}
impl Default for NMLVEMPTYMARKUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVEMPTYMARKUP_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVFINDITEMA {
    pub hdr: NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVFINDITEMW {
    pub hdr: NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOW,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVGETINFOTIPA {
    pub hdr: NMHDR,
    pub dwFlags: NMLVGETINFOTIP_FLAGS,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVGETINFOTIPW {
    pub hdr: NMHDR,
    pub dwFlags: NMLVGETINFOTIP_FLAGS,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMLVGETINFOTIP_FLAGS(pub u32);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NMLVKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVLINK {
    pub hdr: NMHDR,
    pub link: LITEM,
    pub iItem: i32,
    pub iSubItem: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVODSTATECHANGE {
    pub hdr: NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
    pub uNewState: LIST_VIEW_ITEM_STATE_FLAGS,
    pub uOldState: LIST_VIEW_ITEM_STATE_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMLVSCROLL {
    pub hdr: NMHDR,
    pub dx: i32,
    pub dy: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMMOUSE {
    pub hdr: NMHDR,
    pub dwItemSpec: usize,
    pub dwItemData: usize,
    pub pt: super::super::Foundation::POINT,
    pub dwHitInfo: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMOBJECTNOTIFY {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub piid: *const windows_core::GUID,
    pub pObject: *mut core::ffi::c_void,
    pub hResult: windows_core::HRESULT,
    pub dwFlags: u32,
}
impl Default for NMOBJECTNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMPGCALCSIZE {
    pub hdr: NMHDR,
    pub dwFlag: NMPGCALCSIZE_FLAGS,
    pub iWidth: i32,
    pub iHeight: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMPGCALCSIZE_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMPGHOTITEM {
    pub hdr: NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NMPGSCROLL {
    pub hdr: NMHDR,
    pub fwKeys: NMPGSCROLL_KEYS,
    pub rcParent: super::super::Foundation::RECT,
    pub iDir: NMPGSCROLL_DIR,
    pub iXpos: i32,
    pub iYpos: i32,
    pub iScroll: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMPGSCROLL_DIR(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMPGSCROLL_KEYS(pub u16);
impl NMPGSCROLL_KEYS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NMPGSCROLL_KEYS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NMPGSCROLL_KEYS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NMPGSCROLL_KEYS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NMPGSCROLL_KEYS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NMPGSCROLL_KEYS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMRBAUTOSIZE {
    pub hdr: NMHDR,
    pub fChanged: windows_core::BOOL,
    pub rcTarget: super::super::Foundation::RECT,
    pub rcActual: super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMREBAR {
    pub hdr: NMHDR,
    pub dwMask: NMREBAR_MASK_FLAGS,
    pub uBand: u32,
    pub fStyle: u32,
    pub wID: u32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMREBARAUTOBREAK {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub uMsg: u32,
    pub fStyleCurrent: u32,
    pub fAutoBreak: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMREBARCHEVRON {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub rc: super::super::Foundation::RECT,
    pub lParamNM: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMREBARCHILDSIZE {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub rcChild: super::super::Foundation::RECT,
    pub rcBand: super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMREBARSPLITTER {
    pub hdr: NMHDR,
    pub rcSizing: super::super::Foundation::RECT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMREBAR_MASK_FLAGS(pub u32);
impl NMREBAR_MASK_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NMREBAR_MASK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NMREBAR_MASK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMSEARCHWEB {
    pub hdr: NMHDR,
    pub entrypoint: EC_SEARCHWEB_ENTRYPOINT,
    pub hasQueryText: windows_core::BOOL,
    pub invokeSucceeded: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMSELCHANGE {
    pub nmhdr: NMHDR,
    pub stSelStart: super::super::Foundation::SYSTEMTIME,
    pub stSelEnd: super::super::Foundation::SYSTEMTIME,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub hbrMonoDither: super::super::Graphics::Gdi::HBRUSH,
    pub hbrLines: super::super::Graphics::Gdi::HBRUSH,
    pub hpenLines: super::super::Graphics::Gdi::HPEN,
    pub clrText: super::super::Foundation::COLORREF,
    pub clrMark: super::super::Foundation::COLORREF,
    pub clrTextHighlight: super::super::Foundation::COLORREF,
    pub clrBtnFace: super::super::Foundation::COLORREF,
    pub clrBtnHighlight: super::super::Foundation::COLORREF,
    pub clrHighlightHotTrack: super::super::Foundation::COLORREF,
    pub rcText: super::super::Foundation::RECT,
    pub nStringBkMode: i32,
    pub nHLStringBkMode: i32,
    pub iListGap: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBDISPINFOA {
    pub hdr: NMHDR,
    pub dwMask: NMTBDISPINFOW_MASK,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: windows_core::PSTR,
    pub cchText: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBDISPINFOW {
    pub hdr: NMHDR,
    pub dwMask: NMTBDISPINFOW_MASK,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: windows_core::PWSTR,
    pub cchText: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTBDISPINFOW_MASK(pub u32);
impl NMTBDISPINFOW_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NMTBDISPINFOW_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NMTBDISPINFOW_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBGETINFOTIPA {
    pub hdr: NMHDR,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBGETINFOTIPW {
    pub hdr: NMHDR,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTBHOTITEM {
    pub hdr: NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: NMTBHOTITEM_FLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMTBHOTITEM_FLAGS(pub u32);
impl NMTBHOTITEM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NMTBHOTITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NMTBHOTITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMTBRESTORE {
    pub hdr: NMHDR,
    pub pData: *mut u32,
    pub pCurrent: *mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub cbBytesPerRecord: i32,
    pub tbButton: TBBUTTON,
}
impl Default for NMTBRESTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMTBSAVE {
    pub hdr: NMHDR,
    pub pData: *mut u32,
    pub pCurrent: *mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub tbButton: TBBUTTON,
}
impl Default for NMTBSAVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NMTCKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTOOLBARA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: windows_core::PSTR,
    pub rcButton: super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTOOLBARW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: windows_core::PWSTR,
    pub rcButton: super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTOOLTIPSCREATED {
    pub hdr: NMHDR,
    pub hwndToolTips: super::super::Foundation::HWND,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTRBTHUMBPOSCHANGING {
    pub hdr: NMHDR,
    pub dwPos: u32,
    pub nReason: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTREEVIEWA {
    pub hdr: NMHDR,
    pub action: NM_TREEVIEW_ACTION,
    pub itemOld: TVITEMA,
    pub itemNew: TVITEMA,
    pub ptDrag: super::super::Foundation::POINT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTREEVIEWW {
    pub hdr: NMHDR,
    pub action: NM_TREEVIEW_ACTION,
    pub itemOld: TVITEMW,
    pub itemNew: TVITEMW,
    pub ptDrag: super::super::Foundation::POINT,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTTCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub uDrawFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMTTDISPINFOA {
    pub hdr: NMHDR,
    pub lpszText: windows_core::PSTR,
    pub szText: [i8; 80],
    pub hinst: super::super::Foundation::HINSTANCE,
    pub uFlags: TOOLTIP_FLAGS,
    pub lParam: super::super::Foundation::LPARAM,
}
impl Default for NMTTDISPINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMTTDISPINFOW {
    pub hdr: NMHDR,
    pub lpszText: windows_core::PWSTR,
    pub szText: [u16; 80],
    pub hinst: super::super::Foundation::HINSTANCE,
    pub uFlags: TOOLTIP_FLAGS,
    pub lParam: super::super::Foundation::LPARAM,
}
impl Default for NMTTDISPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NMTVASYNCDRAW {
    pub hdr: NMHDR,
    pub pimldp: *mut IMAGELISTDRAWPARAMS,
    pub hr: windows_core::HRESULT,
    pub hItem: HTREEITEM,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwRetFlags: u32,
    pub iRetImageIndex: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for NMTVASYNCDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: super::super::Foundation::COLORREF,
    pub clrTextBk: super::super::Foundation::COLORREF,
    pub iLevel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVDISPINFOA {
    pub hdr: NMHDR,
    pub item: TVITEMA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVDISPINFOEXA {
    pub hdr: NMHDR,
    pub item: TVITEMEXA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVDISPINFOEXW {
    pub hdr: NMHDR,
    pub item: TVITEMEXW,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVDISPINFOW {
    pub hdr: NMHDR,
    pub item: TVITEMW,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVGETINFOTIPA {
    pub hdr: NMHDR,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVGETINFOTIPW {
    pub hdr: NMHDR,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVITEMCHANGE {
    pub hdr: NMHDR,
    pub uChanged: u32,
    pub hItem: HTREEITEM,
    pub uStateNew: u32,
    pub uStateOld: u32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NMTVKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMTVSTATEIMAGECHANGING {
    pub hdr: NMHDR,
    pub hti: HTREEITEM,
    pub iOldStateImageIndex: i32,
    pub iNewStateImageIndex: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMUPDOWN {
    pub hdr: NMHDR,
    pub iPos: i32,
    pub iDelta: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NMVIEWCHANGE {
    pub nmhdr: NMHDR,
    pub dwOldView: MONTH_CALDENDAR_MESSAGES_VIEW,
    pub dwNewView: MONTH_CALDENDAR_MESSAGES_VIEW,
}
pub const NM_CHAR: u32 = 4294967278u32;
pub const NM_CLICK: u32 = 4294967294u32;
pub const NM_CUSTOMDRAW: u32 = 4294967284u32;
pub const NM_CUSTOMTEXT: u32 = 4294967272u32;
pub const NM_DBLCLK: u32 = 4294967293u32;
pub const NM_FIRST: u32 = 0u32;
pub const NM_FONTCHANGED: u32 = 4294967273u32;
pub const NM_GETCUSTOMSPLITRECT: u32 = 4294966049u32;
pub const NM_HOVER: u32 = 4294967283u32;
pub const NM_KEYDOWN: u32 = 4294967281u32;
pub const NM_KILLFOCUS: u32 = 4294967288u32;
pub const NM_LAST: u32 = 4294967197u32;
pub const NM_LDOWN: u32 = 4294967276u32;
pub const NM_NCHITTEST: u32 = 4294967282u32;
pub const NM_OUTOFMEMORY: u32 = 4294967295u32;
pub const NM_RCLICK: u32 = 4294967291u32;
pub const NM_RDBLCLK: u32 = 4294967290u32;
pub const NM_RDOWN: u32 = 4294967275u32;
pub const NM_RELEASEDCAPTURE: u32 = 4294967280u32;
pub const NM_RETURN: u32 = 4294967292u32;
pub const NM_SETCURSOR: u32 = 4294967279u32;
pub const NM_SETFOCUS: u32 = 4294967289u32;
pub const NM_THEMECHANGED: u32 = 4294967274u32;
pub const NM_TOOLTIPSCREATED: u32 = 4294967277u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NM_TREEVIEW_ACTION(pub u32);
impl NM_TREEVIEW_ACTION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NM_TREEVIEW_ACTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NM_TREEVIEW_ACTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NM_TREEVIEW_ACTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NM_TREEVIEW_ACTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NM_TREEVIEW_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const NM_TVSTATEIMAGECHANGING: u32 = 4294967272u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NONESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NORMALGROUPCOLLAPSESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NORMALGROUPEXPANDSTATES(pub i32);
pub const ODA_DRAWENTIRE: ODA_FLAGS = ODA_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ODA_FLAGS(pub u32);
pub const ODA_FOCUS: ODA_FLAGS = ODA_FLAGS(4u32);
pub const ODA_SELECT: ODA_FLAGS = ODA_FLAGS(2u32);
pub const ODS_CHECKED: ODS_FLAGS = ODS_FLAGS(8u32);
pub const ODS_COMBOBOXEDIT: ODS_FLAGS = ODS_FLAGS(4096u32);
pub const ODS_DEFAULT: ODS_FLAGS = ODS_FLAGS(32u32);
pub const ODS_DISABLED: ODS_FLAGS = ODS_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ODS_FLAGS(pub u32);
pub const ODS_FOCUS: ODS_FLAGS = ODS_FLAGS(16u32);
pub const ODS_GRAYED: ODS_FLAGS = ODS_FLAGS(2u32);
pub const ODS_HOTLIGHT: ODS_FLAGS = ODS_FLAGS(64u32);
pub const ODS_INACTIVE: ODS_FLAGS = ODS_FLAGS(128u32);
pub const ODS_NOACCEL: ODS_FLAGS = ODS_FLAGS(256u32);
pub const ODS_NOFOCUSRECT: ODS_FLAGS = ODS_FLAGS(512u32);
pub const ODS_SELECTED: ODS_FLAGS = ODS_FLAGS(1u32);
pub const ODT_BUTTON: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(4u32);
pub const ODT_COMBOBOX: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(3u32);
pub const ODT_HEADER: u32 = 100u32;
pub const ODT_LISTBOX: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(2u32);
pub const ODT_LISTVIEW: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(102u32);
pub const ODT_MENU: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(1u32);
pub const ODT_STATIC: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(5u32);
pub const ODT_TAB: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(101u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFSETTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPENBOXSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPEN_THEME_DATA_FLAGS(pub u32);
impl OPEN_THEME_DATA_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OPEN_THEME_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OPEN_THEME_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const OTD_FORCE_RECT_SIZING: OPEN_THEME_DATA_FLAGS = OPEN_THEME_DATA_FLAGS(1u32);
pub const OTD_NONCLIENT: OPEN_THEME_DATA_FLAGS = OPEN_THEME_DATA_FLAGS(2u32);
pub const OT_ABOVELASTBUTTON: OFFSETTYPE = OFFSETTYPE(12i32);
pub const OT_BELOWLASTBUTTON: OFFSETTYPE = OFFSETTYPE(13i32);
pub const OT_BOTTOMLEFT: OFFSETTYPE = OFFSETTYPE(3i32);
pub const OT_BOTTOMMIDDLE: OFFSETTYPE = OFFSETTYPE(5i32);
pub const OT_BOTTOMRIGHT: OFFSETTYPE = OFFSETTYPE(4i32);
pub const OT_LEFTOFCAPTION: OFFSETTYPE = OFFSETTYPE(8i32);
pub const OT_LEFTOFLASTBUTTON: OFFSETTYPE = OFFSETTYPE(10i32);
pub const OT_MIDDLELEFT: OFFSETTYPE = OFFSETTYPE(6i32);
pub const OT_MIDDLERIGHT: OFFSETTYPE = OFFSETTYPE(7i32);
pub const OT_RIGHTOFCAPTION: OFFSETTYPE = OFFSETTYPE(9i32);
pub const OT_RIGHTOFLASTBUTTON: OFFSETTYPE = OFFSETTYPE(11i32);
pub const OT_TOPLEFT: OFFSETTYPE = OFFSETTYPE(0i32);
pub const OT_TOPMIDDLE: OFFSETTYPE = OFFSETTYPE(2i32);
pub const OT_TOPRIGHT: OFFSETTYPE = OFFSETTYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAGEPARTS(pub i32);
pub const PAGESETUPDLGORD: u32 = 1546u32;
pub const PAGESETUPDLGORDMOTIF: u32 = 1550u32;
pub const PBBS_NORMAL: TRANSPARENTBARSTATES = TRANSPARENTBARSTATES(1i32);
pub const PBBS_PARTIAL: TRANSPARENTBARSTATES = TRANSPARENTBARSTATES(2i32);
pub const PBBVS_NORMAL: TRANSPARENTBARVERTSTATES = TRANSPARENTBARVERTSTATES(1i32);
pub const PBBVS_PARTIAL: TRANSPARENTBARVERTSTATES = TRANSPARENTBARVERTSTATES(2i32);
pub const PBDDS_DISABLED: PUSHBUTTONDROPDOWNSTATES = PUSHBUTTONDROPDOWNSTATES(2i32);
pub const PBDDS_NORMAL: PUSHBUTTONDROPDOWNSTATES = PUSHBUTTONDROPDOWNSTATES(1i32);
pub const PBFS_ERROR: FILLSTATES = FILLSTATES(2i32);
pub const PBFS_NORMAL: FILLSTATES = FILLSTATES(1i32);
pub const PBFS_PARTIAL: FILLSTATES = FILLSTATES(4i32);
pub const PBFS_PAUSED: FILLSTATES = FILLSTATES(3i32);
pub const PBFVS_ERROR: FILLVERTSTATES = FILLVERTSTATES(2i32);
pub const PBFVS_NORMAL: FILLVERTSTATES = FILLVERTSTATES(1i32);
pub const PBFVS_PARTIAL: FILLVERTSTATES = FILLVERTSTATES(4i32);
pub const PBFVS_PAUSED: FILLVERTSTATES = FILLVERTSTATES(3i32);
pub const PBM_DELTAPOS: u32 = 1027u32;
pub const PBM_GETBARCOLOR: u32 = 1039u32;
pub const PBM_GETBKCOLOR: u32 = 1038u32;
pub const PBM_GETPOS: u32 = 1032u32;
pub const PBM_GETRANGE: u32 = 1031u32;
pub const PBM_GETSTATE: u32 = 1041u32;
pub const PBM_GETSTEP: u32 = 1037u32;
pub const PBM_SETBARCOLOR: u32 = 1033u32;
pub const PBM_SETBKCOLOR: u32 = 8193u32;
pub const PBM_SETMARQUEE: u32 = 1034u32;
pub const PBM_SETPOS: u32 = 1026u32;
pub const PBM_SETRANGE: u32 = 1025u32;
pub const PBM_SETRANGE32: u32 = 1030u32;
pub const PBM_SETSTATE: u32 = 1040u32;
pub const PBM_SETSTEP: u32 = 1028u32;
pub const PBM_STEPIT: u32 = 1029u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PBRANGE {
    pub iLow: i32,
    pub iHigh: i32,
}
pub const PBST_ERROR: u32 = 2u32;
pub const PBST_NORMAL: u32 = 1u32;
pub const PBST_PAUSED: u32 = 3u32;
pub const PBS_DEFAULTED: PUSHBUTTONSTATES = PUSHBUTTONSTATES(5i32);
pub const PBS_DEFAULTED_ANIMATING: PUSHBUTTONSTATES = PUSHBUTTONSTATES(6i32);
pub const PBS_DISABLED: PUSHBUTTONSTATES = PUSHBUTTONSTATES(4i32);
pub const PBS_HOT: PUSHBUTTONSTATES = PUSHBUTTONSTATES(2i32);
pub const PBS_MARQUEE: u32 = 8u32;
pub const PBS_NORMAL: PUSHBUTTONSTATES = PUSHBUTTONSTATES(1i32);
pub const PBS_PRESSED: PUSHBUTTONSTATES = PUSHBUTTONSTATES(3i32);
pub const PBS_SMOOTH: u32 = 1u32;
pub const PBS_SMOOTHREVERSE: u32 = 16u32;
pub const PBS_VERTICAL: u32 = 4u32;
pub type PFNDACOMPARE = Option<unsafe extern "system" fn(p1: *const core::ffi::c_void, p2: *const core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32>;
pub type PFNDACOMPARECONST = Option<unsafe extern "system" fn(p1: *const core::ffi::c_void, p2: *const core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32>;
pub type PFNDAENUMCALLBACK = Option<unsafe extern "system" fn(p: *const core::ffi::c_void, pdata: *const core::ffi::c_void) -> i32>;
pub type PFNDAENUMCALLBACKCONST = Option<unsafe extern "system" fn(p: *const core::ffi::c_void, pdata: *const core::ffi::c_void) -> i32>;
pub type PFNDPAMERGE = Option<unsafe extern "system" fn(umsg: DPAMM_MESSAGE, pvdest: *const core::ffi::c_void, pvsrc: *const core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> *mut core::ffi::c_void>;
pub type PFNDPAMERGECONST = Option<unsafe extern "system" fn(umsg: DPAMM_MESSAGE, pvdest: *const core::ffi::c_void, pvsrc: *const core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> *mut core::ffi::c_void>;
#[cfg(feature = "Win32_System_Com")]
pub type PFNDPASTREAM = Option<unsafe extern "system" fn(pinfo: *const DPASTREAMINFO, pstream: windows_core::Ref<super::super::System::Com::IStream>, pvinstdata: *const core::ffi::c_void) -> windows_core::HRESULT>;
pub type PFNLVCOMPARE = Option<unsafe extern "system" fn(param0: super::super::Foundation::LPARAM, param1: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> i32>;
pub type PFNLVGROUPCOMPARE = Option<unsafe extern "system" fn(param0: i32, param1: i32, param2: *mut core::ffi::c_void) -> i32>;
pub type PFNPROPSHEETCALLBACK = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::LPARAM) -> i32>;
pub type PFNTVCOMPARE = Option<unsafe extern "system" fn(lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM, lparamsort: super::super::Foundation::LPARAM) -> i32>;
pub type PFTASKDIALOGCALLBACK = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, msg: TASKDIALOG_NOTIFICATIONS, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lprefdata: isize) -> windows_core::HRESULT>;
pub const PGB_BOTTOMORRIGHT: u32 = 1u32;
pub const PGB_TOPORLEFT: u32 = 0u32;
pub const PGF_CALCHEIGHT: NMPGCALCSIZE_FLAGS = NMPGCALCSIZE_FLAGS(2u32);
pub const PGF_CALCWIDTH: NMPGCALCSIZE_FLAGS = NMPGCALCSIZE_FLAGS(1u32);
pub const PGF_DEPRESSED: u32 = 4u32;
pub const PGF_GRAYED: u32 = 2u32;
pub const PGF_HOT: u32 = 8u32;
pub const PGF_INVISIBLE: u32 = 0u32;
pub const PGF_NORMAL: u32 = 1u32;
pub const PGF_SCROLLDOWN: NMPGSCROLL_DIR = NMPGSCROLL_DIR(2i32);
pub const PGF_SCROLLLEFT: NMPGSCROLL_DIR = NMPGSCROLL_DIR(4i32);
pub const PGF_SCROLLRIGHT: NMPGSCROLL_DIR = NMPGSCROLL_DIR(8i32);
pub const PGF_SCROLLUP: NMPGSCROLL_DIR = NMPGSCROLL_DIR(1i32);
pub const PGK_CONTROL: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(2u16);
pub const PGK_MENU: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(4u16);
pub const PGK_NONE: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(0u16);
pub const PGK_SHIFT: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(1u16);
pub const PGM_FIRST: u32 = 5120u32;
pub const PGM_FORWARDMOUSE: u32 = 5123u32;
pub const PGM_GETBKCOLOR: u32 = 5125u32;
pub const PGM_GETBORDER: u32 = 5127u32;
pub const PGM_GETBUTTONSIZE: u32 = 5131u32;
pub const PGM_GETBUTTONSTATE: u32 = 5132u32;
pub const PGM_GETDROPTARGET: u32 = 8196u32;
pub const PGM_GETPOS: u32 = 5129u32;
pub const PGM_RECALCSIZE: u32 = 5122u32;
pub const PGM_SETBKCOLOR: u32 = 5124u32;
pub const PGM_SETBORDER: u32 = 5126u32;
pub const PGM_SETBUTTONSIZE: u32 = 5130u32;
pub const PGM_SETCHILD: u32 = 5121u32;
pub const PGM_SETPOS: u32 = 5128u32;
pub const PGM_SETSCROLLINFO: u32 = 5133u32;
pub const PGN_CALCSIZE: u32 = 4294966394u32;
pub const PGN_FIRST: u32 = 4294966396u32;
pub const PGN_HOTITEMCHANGE: u32 = 4294966393u32;
pub const PGN_LAST: u32 = 4294966346u32;
pub const PGN_SCROLL: u32 = 4294966395u32;
pub const PGRP_DOWN: PAGEPARTS = PAGEPARTS(2i32);
pub const PGRP_DOWNHORZ: PAGEPARTS = PAGEPARTS(4i32);
pub const PGRP_UP: PAGEPARTS = PAGEPARTS(1i32);
pub const PGRP_UPHORZ: PAGEPARTS = PAGEPARTS(3i32);
pub const PGS_AUTOSCROLL: u32 = 2u32;
pub const PGS_DRAGNDROP: u32 = 4u32;
pub const PGS_HORZ: u32 = 1u32;
pub const PGS_VERT: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTER_DEVICE_CURSOR_INFO {
    pub cursorId: u32,
    pub cursor: POINTER_DEVICE_CURSOR_TYPE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINTER_DEVICE_CURSOR_TYPE(pub i32);
pub const POINTER_DEVICE_CURSOR_TYPE_ERASER: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(2i32);
pub const POINTER_DEVICE_CURSOR_TYPE_MAX: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(-1i32);
pub const POINTER_DEVICE_CURSOR_TYPE_TIP: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(1i32);
pub const POINTER_DEVICE_CURSOR_TYPE_UNKNOWN: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(0i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POINTER_DEVICE_INFO {
    pub displayOrientation: u32,
    pub device: super::super::Foundation::HANDLE,
    pub pointerDeviceType: POINTER_DEVICE_TYPE,
    pub monitor: super::super::Graphics::Gdi::HMONITOR,
    pub startingCursorId: u32,
    pub maxActiveContacts: u16,
    pub productString: [u16; 520],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for POINTER_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTER_DEVICE_PROPERTY {
    pub logicalMin: i32,
    pub logicalMax: i32,
    pub physicalMin: i32,
    pub physicalMax: i32,
    pub unit: u32,
    pub unitExponent: u32,
    pub usagePageId: u16,
    pub usageId: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINTER_DEVICE_TYPE(pub i32);
pub const POINTER_DEVICE_TYPE_EXTERNAL_PEN: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(2i32);
pub const POINTER_DEVICE_TYPE_INTEGRATED_PEN: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(1i32);
pub const POINTER_DEVICE_TYPE_MAX: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(-1i32);
pub const POINTER_DEVICE_TYPE_TOUCH: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(3i32);
pub const POINTER_DEVICE_TYPE_TOUCH_PAD: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(4i32);
pub const POINTER_FEEDBACK_DEFAULT: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(1i32);
pub const POINTER_FEEDBACK_INDIRECT: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINTER_FEEDBACK_MODE(pub i32);
pub const POINTER_FEEDBACK_NONE: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(3i32);
#[repr(C)]
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct POINTER_TYPE_INFO {
    pub r#type: super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub Anonymous: POINTER_TYPE_INFO_0,
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for POINTER_TYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union POINTER_TYPE_INFO_0 {
    pub touchInfo: super::Input::Pointer::POINTER_TOUCH_INFO,
    pub penInfo: super::Input::Pointer::POINTER_PEN_INFO,
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for POINTER_TYPE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POPUPCHECKBACKGROUNDSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POPUPCHECKSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POPUPITEMFOCUSABLESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POPUPITEMKBFOCUSSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POPUPITEMSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POPUPSUBMENUHCHOTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POPUPSUBMENUSTATES(pub i32);
pub const PO_CLASS: PROPERTYORIGIN = PROPERTYORIGIN(2i32);
pub const PO_GLOBAL: PROPERTYORIGIN = PROPERTYORIGIN(3i32);
pub const PO_NOTFOUND: PROPERTYORIGIN = PROPERTYORIGIN(4i32);
pub const PO_PART: PROPERTYORIGIN = PROPERTYORIGIN(1i32);
pub const PO_STATE: PROPERTYORIGIN = PROPERTYORIGIN(0i32);
pub const PP_BAR: PROGRESSPARTS = PROGRESSPARTS(1i32);
pub const PP_BARVERT: PROGRESSPARTS = PROGRESSPARTS(2i32);
pub const PP_CHUNK: PROGRESSPARTS = PROGRESSPARTS(3i32);
pub const PP_CHUNKVERT: PROGRESSPARTS = PROGRESSPARTS(4i32);
pub const PP_FILL: PROGRESSPARTS = PROGRESSPARTS(5i32);
pub const PP_FILLVERT: PROGRESSPARTS = PROGRESSPARTS(6i32);
pub const PP_MOVEOVERLAY: PROGRESSPARTS = PROGRESSPARTS(8i32);
pub const PP_MOVEOVERLAYVERT: PROGRESSPARTS = PROGRESSPARTS(10i32);
pub const PP_PULSEOVERLAY: PROGRESSPARTS = PROGRESSPARTS(7i32);
pub const PP_PULSEOVERLAYVERT: PROGRESSPARTS = PROGRESSPARTS(9i32);
pub const PP_TRANSPARENTBAR: PROGRESSPARTS = PROGRESSPARTS(11i32);
pub const PP_TRANSPARENTBARVERT: PROGRESSPARTS = PROGRESSPARTS(12i32);
pub const PRINTDLGEXORD: u32 = 1549u32;
pub const PRINTDLGORD: u32 = 1538u32;
pub const PRNSETUPDLGORD: u32 = 1539u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROGRESSPARTS(pub i32);
pub const PROGRESS_CLASS: windows_core::PCWSTR = windows_core::w!("msctls_progress32");
pub const PROGRESS_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_progress32");
pub const PROGRESS_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_progress32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPERTYORIGIN(pub i32);
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETHEADERA_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERA_V1_0,
    pub pszCaption: windows_core::PCSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERA_V1_1,
    pub Anonymous3: PROPSHEETHEADERA_V1_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERA_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V1_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERA_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V1_1 {
    pub nStartPage: u32,
    pub pStartPage: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERA_V1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V1_2 {
    pub ppsp: *mut PROPSHEETPAGEA,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERA_V1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETHEADERA_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERA_V2_0,
    pub pszCaption: windows_core::PCSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERA_V2_1,
    pub Anonymous3: PROPSHEETHEADERA_V2_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
    pub Anonymous4: PROPSHEETHEADERA_V2_3,
    pub hplWatermark: super::super::Graphics::Gdi::HPALETTE,
    pub Anonymous5: PROPSHEETHEADERA_V2_4,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERA_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V2_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERA_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V2_1 {
    pub nStartPage: u32,
    pub pStartPage: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERA_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V2_2 {
    pub ppsp: *mut PROPSHEETPAGEA,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERA_V2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V2_3 {
    pub hbmWatermark: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmWatermark: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERA_V2_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V2_4 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERA_V2_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETHEADERW_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERW_V1_0,
    pub pszCaption: windows_core::PCWSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERW_V1_1,
    pub Anonymous3: PROPSHEETHEADERW_V1_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERW_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V1_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERW_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V1_1 {
    pub nStartPage: u32,
    pub pStartPage: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERW_V1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V1_2 {
    pub ppsp: *mut PROPSHEETPAGEW,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERW_V1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETHEADERW_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERW_V2_0,
    pub pszCaption: windows_core::PCWSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERW_V2_1,
    pub Anonymous3: PROPSHEETHEADERW_V2_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
    pub Anonymous4: PROPSHEETHEADERW_V2_3,
    pub hplWatermark: super::super::Graphics::Gdi::HPALETTE,
    pub Anonymous5: PROPSHEETHEADERW_V2_4,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERW_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V2_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERW_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V2_1 {
    pub nStartPage: u32,
    pub pStartPage: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERW_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V2_2 {
    pub ppsp: *mut PROPSHEETPAGEW,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERW_V2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V2_3 {
    pub hbmWatermark: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmWatermark: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERW_V2_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V2_4 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETHEADERW_V2_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_0,
    pub Anonymous2: PROPSHEETPAGEA_1,
    pub pszTitle: windows_core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_core::PCSTR,
    pub pszHeaderSubTitle: windows_core::PCSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub Anonymous3: PROPSHEETPAGEA_2,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_0 {
    pub pszTemplate: windows_core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_2 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEA_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V1_0,
    pub Anonymous2: PROPSHEETPAGEA_V1_1,
    pub pszTitle: windows_core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V1_0 {
    pub pszTemplate: windows_core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V1_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_V1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEA_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V2_0,
    pub Anonymous2: PROPSHEETPAGEA_V2_1,
    pub pszTitle: windows_core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_core::PCSTR,
    pub pszHeaderSubTitle: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V2_0 {
    pub pszTemplate: windows_core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V2_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEA_V3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V3_0,
    pub Anonymous2: PROPSHEETPAGEA_V3_1,
    pub pszTitle: windows_core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_core::PCSTR,
    pub pszHeaderSubTitle: windows_core::PCSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V3_0 {
    pub pszTemplate: windows_core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V3_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEA_V3_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_0,
    pub Anonymous2: PROPSHEETPAGEW_1,
    pub pszTitle: windows_core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_core::PCWSTR,
    pub pszHeaderSubTitle: windows_core::PCWSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub Anonymous3: PROPSHEETPAGEW_2,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_0 {
    pub pszTemplate: windows_core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_2 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEW_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V1_0,
    pub Anonymous2: PROPSHEETPAGEW_V1_1,
    pub pszTitle: windows_core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V1_0 {
    pub pszTemplate: windows_core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V1_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_V1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEW_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V2_0,
    pub Anonymous2: PROPSHEETPAGEW_V2_1,
    pub pszTitle: windows_core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_core::PCWSTR,
    pub pszHeaderSubTitle: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V2_0 {
    pub pszTemplate: windows_core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V2_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEW_V3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V3_0,
    pub Anonymous2: PROPSHEETPAGEW_V3_1,
    pub pszTitle: windows_core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_core::PCWSTR,
    pub pszHeaderSubTitle: windows_core::PCWSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V3_0 {
    pub pszTemplate: windows_core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V3_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PROPSHEETPAGEW_V3_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROP_LG_CXDLG: u32 = 252u32;
pub const PROP_LG_CYDLG: u32 = 218u32;
pub const PROP_MED_CXDLG: u32 = 227u32;
pub const PROP_MED_CYDLG: u32 = 215u32;
pub const PROP_SM_CXDLG: u32 = 212u32;
pub const PROP_SM_CYDLG: u32 = 188u32;
pub const PSBTN_APPLYNOW: u32 = 4u32;
pub const PSBTN_BACK: u32 = 0u32;
pub const PSBTN_CANCEL: u32 = 5u32;
pub const PSBTN_FINISH: u32 = 2u32;
pub const PSBTN_HELP: u32 = 6u32;
pub const PSBTN_MAX: u32 = 6u32;
pub const PSBTN_NEXT: u32 = 1u32;
pub const PSBTN_OK: u32 = 3u32;
pub const PSCB_BUTTONPRESSED: u32 = 3u32;
pub const PSCB_INITIALIZED: u32 = 1u32;
pub const PSCB_PRECREATE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PSHNOTIFY {
    pub hdr: NMHDR,
    pub lParam: super::super::Foundation::LPARAM,
}
pub const PSH_AEROWIZARD: u32 = 16384u32;
pub const PSH_DEFAULT: u32 = 0u32;
pub const PSH_HASHELP: u32 = 512u32;
pub const PSH_HEADER: u32 = 524288u32;
pub const PSH_HEADERBITMAP: u32 = 134217728u32;
pub const PSH_MODELESS: u32 = 1024u32;
pub const PSH_NOAPPLYNOW: u32 = 128u32;
pub const PSH_NOCONTEXTHELP: u32 = 33554432u32;
pub const PSH_NOMARGIN: u32 = 268435456u32;
pub const PSH_PROPSHEETPAGE: u32 = 8u32;
pub const PSH_PROPTITLE: u32 = 1u32;
pub const PSH_RESIZABLE: u32 = 67108864u32;
pub const PSH_RTLREADING: u32 = 2048u32;
pub const PSH_STRETCHWATERMARK: u32 = 262144u32;
pub const PSH_USECALLBACK: u32 = 256u32;
pub const PSH_USEHBMHEADER: u32 = 1048576u32;
pub const PSH_USEHBMWATERMARK: u32 = 65536u32;
pub const PSH_USEHICON: u32 = 2u32;
pub const PSH_USEHPLWATERMARK: u32 = 131072u32;
pub const PSH_USEICONID: u32 = 4u32;
pub const PSH_USEPAGELANG: u32 = 2097152u32;
pub const PSH_USEPSTARTPAGE: u32 = 64u32;
pub const PSH_WATERMARK: u32 = 32768u32;
pub const PSH_WIZARD: u32 = 32u32;
pub const PSH_WIZARD97: u32 = 8192u32;
pub const PSH_WIZARDCONTEXTHELP: u32 = 4096u32;
pub const PSH_WIZARDHASFINISH: u32 = 16u32;
pub const PSH_WIZARD_LITE: u32 = 4194304u32;
pub const PSM_ADDPAGE: u32 = 1127u32;
pub const PSM_APPLY: u32 = 1134u32;
pub const PSM_CANCELTOCLOSE: u32 = 1131u32;
pub const PSM_CHANGED: u32 = 1128u32;
pub const PSM_ENABLEWIZBUTTONS: u32 = 1163u32;
pub const PSM_GETCURRENTPAGEHWND: u32 = 1142u32;
pub const PSM_GETRESULT: u32 = 1159u32;
pub const PSM_GETTABCONTROL: u32 = 1140u32;
pub const PSM_HWNDTOINDEX: u32 = 1153u32;
pub const PSM_IDTOINDEX: u32 = 1157u32;
pub const PSM_INDEXTOHWND: u32 = 1154u32;
pub const PSM_INDEXTOID: u32 = 1158u32;
pub const PSM_INDEXTOPAGE: u32 = 1156u32;
pub const PSM_INSERTPAGE: u32 = 1143u32;
pub const PSM_ISDIALOGMESSAGE: u32 = 1141u32;
pub const PSM_PAGETOINDEX: u32 = 1155u32;
pub const PSM_PRESSBUTTON: u32 = 1137u32;
pub const PSM_QUERYSIBLINGS: u32 = 1132u32;
pub const PSM_REBOOTSYSTEM: u32 = 1130u32;
pub const PSM_RECALCPAGESIZES: u32 = 1160u32;
pub const PSM_REMOVEPAGE: u32 = 1126u32;
pub const PSM_RESTARTWINDOWS: u32 = 1129u32;
pub const PSM_SETBUTTONTEXT: u32 = 1164u32;
pub const PSM_SETBUTTONTEXTW: u32 = 1164u32;
pub const PSM_SETCURSEL: u32 = 1125u32;
pub const PSM_SETCURSELID: u32 = 1138u32;
pub const PSM_SETFINISHTEXT: u32 = 1145u32;
pub const PSM_SETFINISHTEXTA: u32 = 1139u32;
pub const PSM_SETFINISHTEXTW: u32 = 1145u32;
pub const PSM_SETHEADERSUBTITLE: u32 = 1152u32;
pub const PSM_SETHEADERSUBTITLEA: u32 = 1151u32;
pub const PSM_SETHEADERSUBTITLEW: u32 = 1152u32;
pub const PSM_SETHEADERTITLE: u32 = 1150u32;
pub const PSM_SETHEADERTITLEA: u32 = 1149u32;
pub const PSM_SETHEADERTITLEW: u32 = 1150u32;
pub const PSM_SETNEXTTEXT: u32 = 1161u32;
pub const PSM_SETNEXTTEXTW: u32 = 1161u32;
pub const PSM_SETTITLE: u32 = 1144u32;
pub const PSM_SETTITLEA: u32 = 1135u32;
pub const PSM_SETTITLEW: u32 = 1144u32;
pub const PSM_SETWIZBUTTONS: u32 = 1136u32;
pub const PSM_SHOWWIZBUTTONS: u32 = 1162u32;
pub const PSM_UNCHANGED: u32 = 1133u32;
pub const PSNRET_INVALID: u32 = 1u32;
pub const PSNRET_INVALID_NOCHANGEPAGE: u32 = 2u32;
pub const PSNRET_MESSAGEHANDLED: u32 = 3u32;
pub const PSNRET_NOERROR: u32 = 0u32;
pub const PSN_APPLY: u32 = 4294967094u32;
pub const PSN_FIRST: u32 = 4294967096u32;
pub const PSN_GETOBJECT: u32 = 4294967086u32;
pub const PSN_HELP: u32 = 4294967091u32;
pub const PSN_KILLACTIVE: u32 = 4294967095u32;
pub const PSN_LAST: u32 = 4294966997u32;
pub const PSN_QUERYCANCEL: u32 = 4294967087u32;
pub const PSN_QUERYINITIALFOCUS: u32 = 4294967083u32;
pub const PSN_RESET: u32 = 4294967093u32;
pub const PSN_SETACTIVE: u32 = 4294967096u32;
pub const PSN_TRANSLATEACCELERATOR: u32 = 4294967084u32;
pub const PSN_WIZBACK: u32 = 4294967090u32;
pub const PSN_WIZFINISH: u32 = 4294967088u32;
pub const PSN_WIZNEXT: u32 = 4294967089u32;
pub const PSPCB_ADDREF: PSPCB_MESSAGE = PSPCB_MESSAGE(0u32);
pub const PSPCB_CREATE: PSPCB_MESSAGE = PSPCB_MESSAGE(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSPCB_MESSAGE(pub u32);
pub const PSPCB_RELEASE: PSPCB_MESSAGE = PSPCB_MESSAGE(1u32);
pub const PSPCB_SI_INITDIALOG: PSPCB_MESSAGE = PSPCB_MESSAGE(1025u32);
pub const PSP_DEFAULT: u32 = 0u32;
pub const PSP_DLGINDIRECT: u32 = 1u32;
pub const PSP_HASHELP: u32 = 32u32;
pub const PSP_HIDEHEADER: u32 = 2048u32;
pub const PSP_PREMATURE: u32 = 1024u32;
pub const PSP_RTLREADING: u32 = 16u32;
pub const PSP_USECALLBACK: u32 = 128u32;
pub const PSP_USEFUSIONCONTEXT: u32 = 16384u32;
pub const PSP_USEHEADERSUBTITLE: u32 = 8192u32;
pub const PSP_USEHEADERTITLE: u32 = 4096u32;
pub const PSP_USEHICON: u32 = 2u32;
pub const PSP_USEICONID: u32 = 4u32;
pub const PSP_USEREFPARENT: u32 = 64u32;
pub const PSP_USETITLE: u32 = 8u32;
pub const PSWIZBF_ELEVATIONREQUIRED: u32 = 1u32;
pub const PSWIZB_BACK: u32 = 1u32;
pub const PSWIZB_CANCEL: u32 = 16u32;
pub const PSWIZB_DISABLEDFINISH: u32 = 8u32;
pub const PSWIZB_FINISH: u32 = 4u32;
pub const PSWIZB_NEXT: u32 = 2u32;
pub const PSWIZB_RESTORE: u32 = 1u32;
pub const PSWIZB_SHOW: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PUSHBUTTONDROPDOWNSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PUSHBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIOBUTTONSTATES(pub i32);
pub const RBAB_ADDBAND: u32 = 2u32;
pub const RBAB_AUTOSIZE: u32 = 1u32;
pub const RBBIM_BACKGROUND: u32 = 128u32;
pub const RBBIM_CHEVRONLOCATION: u32 = 4096u32;
pub const RBBIM_CHEVRONSTATE: u32 = 8192u32;
pub const RBBIM_CHILD: u32 = 16u32;
pub const RBBIM_CHILDSIZE: u32 = 32u32;
pub const RBBIM_COLORS: u32 = 2u32;
pub const RBBIM_HEADERSIZE: u32 = 2048u32;
pub const RBBIM_ID: u32 = 256u32;
pub const RBBIM_IDEALSIZE: u32 = 512u32;
pub const RBBIM_IMAGE: u32 = 8u32;
pub const RBBIM_LPARAM: u32 = 1024u32;
pub const RBBIM_SIZE: u32 = 64u32;
pub const RBBIM_STYLE: u32 = 1u32;
pub const RBBIM_TEXT: u32 = 4u32;
pub const RBBS_BREAK: u32 = 1u32;
pub const RBBS_CHILDEDGE: u32 = 4u32;
pub const RBBS_FIXEDBMP: u32 = 32u32;
pub const RBBS_FIXEDSIZE: u32 = 2u32;
pub const RBBS_GRIPPERALWAYS: u32 = 128u32;
pub const RBBS_HIDDEN: u32 = 8u32;
pub const RBBS_HIDETITLE: u32 = 1024u32;
pub const RBBS_NOGRIPPER: u32 = 256u32;
pub const RBBS_NOVERT: u32 = 16u32;
pub const RBBS_TOPALIGN: u32 = 2048u32;
pub const RBBS_USECHEVRON: u32 = 512u32;
pub const RBBS_VARIABLEHEIGHT: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RBHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: u32,
    pub iBand: i32,
}
pub const RBHT_CAPTION: u32 = 2u32;
pub const RBHT_CHEVRON: u32 = 8u32;
pub const RBHT_CLIENT: u32 = 3u32;
pub const RBHT_GRABBER: u32 = 4u32;
pub const RBHT_NOWHERE: u32 = 1u32;
pub const RBHT_SPLITTER: u32 = 16u32;
pub const RBIM_IMAGELIST: u32 = 1u32;
pub const RBNM_ID: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(1u32);
pub const RBNM_LPARAM: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(4u32);
pub const RBNM_STYLE: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(2u32);
pub const RBN_AUTOBREAK: u32 = 4294966443u32;
pub const RBN_AUTOSIZE: u32 = 4294966462u32;
pub const RBN_BEGINDRAG: u32 = 4294966461u32;
pub const RBN_CHEVRONPUSHED: u32 = 4294966455u32;
pub const RBN_CHILDSIZE: u32 = 4294966457u32;
pub const RBN_DELETEDBAND: u32 = 4294966458u32;
pub const RBN_DELETINGBAND: u32 = 4294966459u32;
pub const RBN_ENDDRAG: u32 = 4294966460u32;
pub const RBN_FIRST: u32 = 4294966465u32;
pub const RBN_GETOBJECT: u32 = 4294966464u32;
pub const RBN_HEIGHTCHANGE: u32 = 4294966465u32;
pub const RBN_LAST: u32 = 4294966437u32;
pub const RBN_LAYOUTCHANGED: u32 = 4294966463u32;
pub const RBN_MINMAX: u32 = 4294966444u32;
pub const RBN_SPLITTERDRAG: u32 = 4294966454u32;
pub const RBSTR_CHANGERECT: u32 = 1u32;
pub const RBS_AUTOSIZE: u32 = 8192u32;
pub const RBS_BANDBORDERS: u32 = 1024u32;
pub const RBS_CHECKEDDISABLED: RADIOBUTTONSTATES = RADIOBUTTONSTATES(8i32);
pub const RBS_CHECKEDHOT: RADIOBUTTONSTATES = RADIOBUTTONSTATES(6i32);
pub const RBS_CHECKEDNORMAL: RADIOBUTTONSTATES = RADIOBUTTONSTATES(5i32);
pub const RBS_CHECKEDPRESSED: RADIOBUTTONSTATES = RADIOBUTTONSTATES(7i32);
pub const RBS_DBLCLKTOGGLE: u32 = 32768u32;
pub const RBS_DISABLED: RESTOREBUTTONSTATES = RESTOREBUTTONSTATES(4i32);
pub const RBS_FIXEDORDER: u32 = 2048u32;
pub const RBS_HOT: RESTOREBUTTONSTATES = RESTOREBUTTONSTATES(2i32);
pub const RBS_NORMAL: RESTOREBUTTONSTATES = RESTOREBUTTONSTATES(1i32);
pub const RBS_PUSHED: RESTOREBUTTONSTATES = RESTOREBUTTONSTATES(3i32);
pub const RBS_REGISTERDROP: u32 = 4096u32;
pub const RBS_TOOLTIPS: u32 = 256u32;
pub const RBS_UNCHECKEDDISABLED: RADIOBUTTONSTATES = RADIOBUTTONSTATES(4i32);
pub const RBS_UNCHECKEDHOT: RADIOBUTTONSTATES = RADIOBUTTONSTATES(2i32);
pub const RBS_UNCHECKEDNORMAL: RADIOBUTTONSTATES = RADIOBUTTONSTATES(1i32);
pub const RBS_UNCHECKEDPRESSED: RADIOBUTTONSTATES = RADIOBUTTONSTATES(3i32);
pub const RBS_VARHEIGHT: u32 = 512u32;
pub const RBS_VERTICALGRIPPER: u32 = 16384u32;
pub const RB_BEGINDRAG: u32 = 1048u32;
pub const RB_DELETEBAND: u32 = 1026u32;
pub const RB_DRAGMOVE: u32 = 1050u32;
pub const RB_ENDDRAG: u32 = 1049u32;
pub const RB_GETBANDBORDERS: u32 = 1058u32;
pub const RB_GETBANDCOUNT: u32 = 1036u32;
pub const RB_GETBANDINFO: u32 = 1052u32;
pub const RB_GETBANDINFOA: u32 = 1053u32;
pub const RB_GETBANDINFOW: u32 = 1052u32;
pub const RB_GETBANDMARGINS: u32 = 1064u32;
pub const RB_GETBARHEIGHT: u32 = 1051u32;
pub const RB_GETBARINFO: u32 = 1027u32;
pub const RB_GETBKCOLOR: u32 = 1044u32;
pub const RB_GETCOLORSCHEME: u32 = 8195u32;
pub const RB_GETDROPTARGET: u32 = 8196u32;
pub const RB_GETEXTENDEDSTYLE: u32 = 1066u32;
pub const RB_GETPALETTE: u32 = 1062u32;
pub const RB_GETRECT: u32 = 1033u32;
pub const RB_GETROWCOUNT: u32 = 1037u32;
pub const RB_GETROWHEIGHT: u32 = 1038u32;
pub const RB_GETTEXTCOLOR: u32 = 1046u32;
pub const RB_GETTOOLTIPS: u32 = 1041u32;
pub const RB_GETUNICODEFORMAT: u32 = 8198u32;
pub const RB_HITTEST: u32 = 1032u32;
pub const RB_IDTOINDEX: u32 = 1040u32;
pub const RB_INSERTBAND: u32 = 1034u32;
pub const RB_INSERTBANDA: u32 = 1025u32;
pub const RB_INSERTBANDW: u32 = 1034u32;
pub const RB_MAXIMIZEBAND: u32 = 1055u32;
pub const RB_MINIMIZEBAND: u32 = 1054u32;
pub const RB_MOVEBAND: u32 = 1063u32;
pub const RB_PUSHCHEVRON: u32 = 1067u32;
pub const RB_SETBANDINFO: u32 = 1035u32;
pub const RB_SETBANDINFOA: u32 = 1030u32;
pub const RB_SETBANDINFOW: u32 = 1035u32;
pub const RB_SETBANDWIDTH: u32 = 1068u32;
pub const RB_SETBARINFO: u32 = 1028u32;
pub const RB_SETBKCOLOR: u32 = 1043u32;
pub const RB_SETCOLORSCHEME: u32 = 8194u32;
pub const RB_SETEXTENDEDSTYLE: u32 = 1065u32;
pub const RB_SETPALETTE: u32 = 1061u32;
pub const RB_SETPARENT: u32 = 1031u32;
pub const RB_SETTEXTCOLOR: u32 = 1045u32;
pub const RB_SETTOOLTIPS: u32 = 1042u32;
pub const RB_SETUNICODEFORMAT: u32 = 8197u32;
pub const RB_SETWINDOWTHEME: u32 = 8203u32;
pub const RB_SHOWBAND: u32 = 1059u32;
pub const RB_SIZETORECT: u32 = 1047u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READONLYSTATES(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REBARBANDINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: super::super::Foundation::COLORREF,
    pub clrBack: super::super::Foundation::COLORREF,
    pub lpText: windows_core::PSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: super::super::Foundation::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: super::super::Graphics::Gdi::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: super::super::Foundation::RECT,
    pub uChevronState: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REBARBANDINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: super::super::Foundation::COLORREF,
    pub clrBack: super::super::Foundation::COLORREF,
    pub lpText: windows_core::PWSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: super::super::Foundation::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: super::super::Graphics::Gdi::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: super::super::Foundation::RECT,
    pub uChevronState: u32,
}
pub const REBARCLASSNAME: windows_core::PCWSTR = windows_core::w!("ReBarWindow32");
pub const REBARCLASSNAMEA: windows_core::PCSTR = windows_core::s!("ReBarWindow32");
pub const REBARCLASSNAMEW: windows_core::PCWSTR = windows_core::w!("ReBarWindow32");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REBARINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub himl: HIMAGELIST,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REBARPARTS(pub i32);
pub const REPLACEDLGORD: u32 = 1541u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESTOREBUTTONSTATES(pub i32);
pub const RP_BACKGROUND: REBARPARTS = REBARPARTS(6i32);
pub const RP_BAND: REBARPARTS = REBARPARTS(3i32);
pub const RP_CHEVRON: REBARPARTS = REBARPARTS(4i32);
pub const RP_CHEVRONVERT: REBARPARTS = REBARPARTS(5i32);
pub const RP_GRIPPER: REBARPARTS = REBARPARTS(1i32);
pub const RP_GRIPPERVERT: REBARPARTS = REBARPARTS(2i32);
pub const RP_SPLITTER: REBARPARTS = REBARPARTS(7i32);
pub const RP_SPLITTERVERT: REBARPARTS = REBARPARTS(8i32);
pub const RUNDLGORD: u32 = 1545u32;
pub const SBARS_SIZEGRIP: u32 = 256u32;
pub const SBARS_TOOLTIPS: u32 = 2048u32;
pub const SBN_FIRST: u32 = 4294966416u32;
pub const SBN_LAST: u32 = 4294966397u32;
pub const SBN_SIMPLEMODECHANGE: u32 = 4294966416u32;
pub const SBP_ARROWBTN: SCROLLBARPARTS = SCROLLBARPARTS(1i32);
pub const SBP_GRIPPERHORZ: SCROLLBARPARTS = SCROLLBARPARTS(8i32);
pub const SBP_GRIPPERVERT: SCROLLBARPARTS = SCROLLBARPARTS(9i32);
pub const SBP_LOWERTRACKHORZ: SCROLLBARPARTS = SCROLLBARPARTS(4i32);
pub const SBP_LOWERTRACKVERT: SCROLLBARPARTS = SCROLLBARPARTS(6i32);
pub const SBP_SIZEBOX: SCROLLBARPARTS = SCROLLBARPARTS(10i32);
pub const SBP_SIZEBOXBKGND: SCROLLBARPARTS = SCROLLBARPARTS(11i32);
pub const SBP_THUMBBTNHORZ: SCROLLBARPARTS = SCROLLBARPARTS(2i32);
pub const SBP_THUMBBTNVERT: SCROLLBARPARTS = SCROLLBARPARTS(3i32);
pub const SBP_UPPERTRACKHORZ: SCROLLBARPARTS = SCROLLBARPARTS(5i32);
pub const SBP_UPPERTRACKVERT: SCROLLBARPARTS = SCROLLBARPARTS(7i32);
pub const SBS_DISABLED: SYSBUTTONSTATES = SYSBUTTONSTATES(4i32);
pub const SBS_HOT: SYSBUTTONSTATES = SYSBUTTONSTATES(2i32);
pub const SBS_NORMAL: SYSBUTTONSTATES = SYSBUTTONSTATES(1i32);
pub const SBS_PUSHED: SYSBUTTONSTATES = SYSBUTTONSTATES(3i32);
pub const SBT_NOBORDERS: u32 = 256u32;
pub const SBT_NOTABPARSING: u32 = 2048u32;
pub const SBT_OWNERDRAW: u32 = 4096u32;
pub const SBT_POPOUT: u32 = 512u32;
pub const SBT_RTLREADING: u32 = 1024u32;
pub const SBT_TOOLTIPS: u32 = 2048u32;
pub const SB_GETBORDERS: u32 = 1031u32;
pub const SB_GETICON: u32 = 1044u32;
pub const SB_GETPARTS: u32 = 1030u32;
pub const SB_GETRECT: u32 = 1034u32;
pub const SB_GETTEXT: u32 = 1037u32;
pub const SB_GETTEXTA: u32 = 1026u32;
pub const SB_GETTEXTLENGTH: u32 = 1036u32;
pub const SB_GETTEXTLENGTHA: u32 = 1027u32;
pub const SB_GETTEXTLENGTHW: u32 = 1036u32;
pub const SB_GETTEXTW: u32 = 1037u32;
pub const SB_GETTIPTEXTA: u32 = 1042u32;
pub const SB_GETTIPTEXTW: u32 = 1043u32;
pub const SB_GETUNICODEFORMAT: u32 = 8198u32;
pub const SB_ISSIMPLE: u32 = 1038u32;
pub const SB_SETBKCOLOR: u32 = 8193u32;
pub const SB_SETICON: u32 = 1039u32;
pub const SB_SETMINHEIGHT: u32 = 1032u32;
pub const SB_SETPARTS: u32 = 1028u32;
pub const SB_SETTEXT: u32 = 1035u32;
pub const SB_SETTEXTA: u32 = 1025u32;
pub const SB_SETTEXTW: u32 = 1035u32;
pub const SB_SETTIPTEXTA: u32 = 1040u32;
pub const SB_SETTIPTEXTW: u32 = 1041u32;
pub const SB_SETUNICODEFORMAT: u32 = 8197u32;
pub const SB_SIMPLE: u32 = 1033u32;
pub const SB_SIMPLEID: u32 = 255u32;
pub const SCBS_DISABLED: SMALLCLOSEBUTTONSTATES = SMALLCLOSEBUTTONSTATES(4i32);
pub const SCBS_HOT: SMALLCLOSEBUTTONSTATES = SMALLCLOSEBUTTONSTATES(2i32);
pub const SCBS_NORMAL: SMALLCLOSEBUTTONSTATES = SMALLCLOSEBUTTONSTATES(1i32);
pub const SCBS_PUSHED: SMALLCLOSEBUTTONSTATES = SMALLCLOSEBUTTONSTATES(3i32);
pub const SCRBS_DISABLED: SCROLLBARSTYLESTATES = SCROLLBARSTYLESTATES(4i32);
pub const SCRBS_HOT: SCROLLBARSTYLESTATES = SCROLLBARSTYLESTATES(2i32);
pub const SCRBS_HOVER: SCROLLBARSTYLESTATES = SCROLLBARSTYLESTATES(5i32);
pub const SCRBS_NORMAL: SCROLLBARSTYLESTATES = SCROLLBARSTYLESTATES(1i32);
pub const SCRBS_PRESSED: SCROLLBARSTYLESTATES = SCROLLBARSTYLESTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCROLLBARPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCROLLBARSTYLESTATES(pub i32);
pub const SCS_ACTIVE: SMALLCAPTIONSTATES = SMALLCAPTIONSTATES(1i32);
pub const SCS_DISABLED: SMALLCAPTIONSTATES = SMALLCAPTIONSTATES(3i32);
pub const SCS_INACTIVE: SMALLCAPTIONSTATES = SMALLCAPTIONSTATES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECTIONTITLELINKSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SET_THEME_APP_PROPERTIES_FLAGS(pub u32);
impl SET_THEME_APP_PROPERTIES_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SET_THEME_APP_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SET_THEME_APP_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SET_THEME_APP_PROPERTIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SET_THEME_APP_PROPERTIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SET_THEME_APP_PROPERTIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SFRB_ACTIVE: SMALLFRAMEBOTTOMSTATES = SMALLFRAMEBOTTOMSTATES(1i32);
pub const SFRB_INACTIVE: SMALLFRAMEBOTTOMSTATES = SMALLFRAMEBOTTOMSTATES(2i32);
pub const SFRL_ACTIVE: SMALLFRAMELEFTSTATES = SMALLFRAMELEFTSTATES(1i32);
pub const SFRL_INACTIVE: SMALLFRAMELEFTSTATES = SMALLFRAMELEFTSTATES(2i32);
pub const SFRR_ACTIVE: SMALLFRAMERIGHTSTATES = SMALLFRAMERIGHTSTATES(1i32);
pub const SFRR_INACTIVE: SMALLFRAMERIGHTSTATES = SMALLFRAMERIGHTSTATES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHOWCALENDARBUTTONRIGHTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIZEBOXSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIZINGTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMALLCAPTIONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMALLCLOSEBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMALLFRAMEBOTTOMSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMALLFRAMELEFTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMALLFRAMERIGHTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOFTWAREEXPLORERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPECIALGROUPCOLLAPSESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPECIALGROUPEXPANDSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPINPARTS(pub i32);
pub const SPLITSV_HOT: SPLITTERVERTSTATES = SPLITTERVERTSTATES(2i32);
pub const SPLITSV_NORMAL: SPLITTERVERTSTATES = SPLITTERVERTSTATES(1i32);
pub const SPLITSV_PRESSED: SPLITTERVERTSTATES = SPLITTERVERTSTATES(3i32);
pub const SPLITS_HOT: SPLITTERSTATES = SPLITTERSTATES(2i32);
pub const SPLITS_NORMAL: SPLITTERSTATES = SPLITTERSTATES(1i32);
pub const SPLITS_PRESSED: SPLITTERSTATES = SPLITTERSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPLITTERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPLITTERVERTSTATES(pub i32);
pub const SPLS_HOT: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(2i32);
pub const SPLS_NORMAL: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(1i32);
pub const SPLS_PRESSED: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(3i32);
pub const SPMPT_DISABLED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(4i32);
pub const SPMPT_FOCUSED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(5i32);
pub const SPMPT_HOT: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(2i32);
pub const SPMPT_NORMAL: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(1i32);
pub const SPMPT_SELECTED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(3i32);
pub const SPNP_DOWN: SPINPARTS = SPINPARTS(2i32);
pub const SPNP_DOWNHORZ: SPINPARTS = SPINPARTS(4i32);
pub const SPNP_UP: SPINPARTS = SPINPARTS(1i32);
pub const SPNP_UPHORZ: SPINPARTS = SPINPARTS(3i32);
pub const SPOB_DISABLED: OPENBOXSTATES = OPENBOXSTATES(4i32);
pub const SPOB_FOCUSED: OPENBOXSTATES = OPENBOXSTATES(5i32);
pub const SPOB_HOT: OPENBOXSTATES = OPENBOXSTATES(2i32);
pub const SPOB_NORMAL: OPENBOXSTATES = OPENBOXSTATES(1i32);
pub const SPOB_SELECTED: OPENBOXSTATES = OPENBOXSTATES(3i32);
pub const SPP_LOGOFF: STARTPANELPARTS = STARTPANELPARTS(8i32);
pub const SPP_LOGOFFBUTTONS: STARTPANELPARTS = STARTPANELPARTS(9i32);
pub const SPP_LOGOFFSPLITBUTTONDROPDOWN: STARTPANELPARTS = STARTPANELPARTS(19i32);
pub const SPP_MOREPROGRAMS: STARTPANELPARTS = STARTPANELPARTS(2i32);
pub const SPP_MOREPROGRAMSARROW: STARTPANELPARTS = STARTPANELPARTS(3i32);
pub const SPP_MOREPROGRAMSARROWBACK: STARTPANELPARTS = STARTPANELPARTS(17i32);
pub const SPP_MOREPROGRAMSTAB: STARTPANELPARTS = STARTPANELPARTS(12i32);
pub const SPP_NSCHOST: STARTPANELPARTS = STARTPANELPARTS(13i32);
pub const SPP_OPENBOX: STARTPANELPARTS = STARTPANELPARTS(15i32);
pub const SPP_PLACESLIST: STARTPANELPARTS = STARTPANELPARTS(6i32);
pub const SPP_PLACESLISTSEPARATOR: STARTPANELPARTS = STARTPANELPARTS(7i32);
pub const SPP_PREVIEW: STARTPANELPARTS = STARTPANELPARTS(11i32);
pub const SPP_PROGLIST: STARTPANELPARTS = STARTPANELPARTS(4i32);
pub const SPP_PROGLISTSEPARATOR: STARTPANELPARTS = STARTPANELPARTS(5i32);
pub const SPP_SEARCHVIEW: STARTPANELPARTS = STARTPANELPARTS(16i32);
pub const SPP_SOFTWAREEXPLORER: STARTPANELPARTS = STARTPANELPARTS(14i32);
pub const SPP_TOPMATCH: STARTPANELPARTS = STARTPANELPARTS(18i32);
pub const SPP_USERPANE: STARTPANELPARTS = STARTPANELPARTS(1i32);
pub const SPP_USERPICTURE: STARTPANELPARTS = STARTPANELPARTS(10i32);
pub const SPSB_HOT: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(2i32);
pub const SPSB_NORMAL: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(1i32);
pub const SPSB_PRESSED: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(3i32);
pub const SPSE_DISABLED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(4i32);
pub const SPSE_FOCUSED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(5i32);
pub const SPSE_HOT: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(2i32);
pub const SPSE_NORMAL: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(1i32);
pub const SPSE_SELECTED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(3i32);
pub const SPS_HOT: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(2i32);
pub const SPS_NORMAL: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(1i32);
pub const SPS_PRESSED: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(3i32);
pub const SP_GRIPPER: STATUSPARTS = STATUSPARTS(3i32);
pub const SP_GRIPPERPANE: STATUSPARTS = STATUSPARTS(2i32);
pub const SP_PANE: STATUSPARTS = STATUSPARTS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STANDARDSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STARTPANELPARTS(pub i32);
pub const STATE_SYSTEM_FOCUSABLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(1048576u32);
pub const STATE_SYSTEM_INVISIBLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(32768u32);
pub const STATE_SYSTEM_OFFSCREEN: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(65536u32);
pub const STATE_SYSTEM_PRESSED: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(8u32);
pub const STATE_SYSTEM_UNAVAILABLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STATICPARTS(pub i32);
pub const STATUSCLASSNAME: windows_core::PCWSTR = windows_core::w!("msctls_statusbar32");
pub const STATUSCLASSNAMEA: windows_core::PCSTR = windows_core::s!("msctls_statusbar32");
pub const STATUSCLASSNAMEW: windows_core::PCWSTR = windows_core::w!("msctls_statusbar32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STATUSPARTS(pub i32);
pub const STAT_TEXT: STATICPARTS = STATICPARTS(1i32);
pub const STD_COPY: u32 = 1u32;
pub const STD_CUT: u32 = 0u32;
pub const STD_DELETE: u32 = 5u32;
pub const STD_FILENEW: u32 = 6u32;
pub const STD_FILEOPEN: u32 = 7u32;
pub const STD_FILESAVE: u32 = 8u32;
pub const STD_FIND: u32 = 12u32;
pub const STD_HELP: u32 = 11u32;
pub const STD_PASTE: u32 = 2u32;
pub const STD_PRINT: u32 = 14u32;
pub const STD_PRINTPRE: u32 = 9u32;
pub const STD_PROPERTIES: u32 = 10u32;
pub const STD_REDOW: u32 = 4u32;
pub const STD_REPLACE: u32 = 13u32;
pub const STD_UNDO: u32 = 3u32;
pub const ST_STRETCH: SIZINGTYPE = SIZINGTYPE(1i32);
pub const ST_TILE: SIZINGTYPE = SIZINGTYPE(2i32);
pub const ST_TRUESIZE: SIZINGTYPE = SIZINGTYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSBUTTONSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMCLOSEHCHOTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMCLOSESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMMAXIMIZEHCHOTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMMAXIMIZESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMMINIMIZEHCHOTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMMINIMIZESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMRESTOREHCHOTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMRESTORESTATES(pub i32);
pub const SZB_HALFBOTTOMLEFTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(6i32);
pub const SZB_HALFBOTTOMRIGHTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(5i32);
pub const SZB_HALFTOPLEFTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(8i32);
pub const SZB_HALFTOPRIGHTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(7i32);
pub const SZB_LEFTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(2i32);
pub const SZB_RIGHTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(1i32);
pub const SZB_TOPLEFTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(4i32);
pub const SZB_TOPRIGHTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(3i32);
pub const SZ_THDOCPROP_AUTHOR: windows_core::PCWSTR = windows_core::w!("author");
pub const SZ_THDOCPROP_CANONICALNAME: windows_core::PCWSTR = windows_core::w!("ThemeName");
pub const SZ_THDOCPROP_DISPLAYNAME: windows_core::PCWSTR = windows_core::w!("DisplayName");
pub const SZ_THDOCPROP_TOOLTIP: windows_core::PCWSTR = windows_core::w!("ToolTip");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TABITEMBOTHEDGESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TABITEMLEFTEDGESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TABITEMRIGHTEDGESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TABITEMSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TABPARTS(pub i32);
pub const TABP_AEROWIZARDBODY: TABPARTS = TABPARTS(11i32);
pub const TABP_BODY: TABPARTS = TABPARTS(10i32);
pub const TABP_PANE: TABPARTS = TABPARTS(9i32);
pub const TABP_TABITEM: TABPARTS = TABPARTS(1i32);
pub const TABP_TABITEMBOTHEDGE: TABPARTS = TABPARTS(4i32);
pub const TABP_TABITEMLEFTEDGE: TABPARTS = TABPARTS(2i32);
pub const TABP_TABITEMRIGHTEDGE: TABPARTS = TABPARTS(3i32);
pub const TABP_TOPTABITEM: TABPARTS = TABPARTS(5i32);
pub const TABP_TOPTABITEMBOTHEDGE: TABPARTS = TABPARTS(8i32);
pub const TABP_TOPTABITEMLEFTEDGE: TABPARTS = TABPARTS(6i32);
pub const TABP_TOPTABITEMRIGHTEDGE: TABPARTS = TABPARTS(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TABSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TAB_CONTROL_ITEM_STATE(pub u32);
pub const TAPF_ALLOWCOLLECTION: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(4i32);
pub const TAPF_HASBACKGROUND: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(8i32);
pub const TAPF_HASPERSPECTIVE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(16i32);
pub const TAPF_HASSTAGGER: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(1i32);
pub const TAPF_ISRTLAWARE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(2i32);
pub const TAPF_NONE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(0i32);
pub const TAP_FLAGS: TA_PROPERTY = TA_PROPERTY(0i32);
pub const TAP_STAGGERDELAY: TA_PROPERTY = TA_PROPERTY(2i32);
pub const TAP_STAGGERDELAYCAP: TA_PROPERTY = TA_PROPERTY(3i32);
pub const TAP_STAGGERDELAYFACTOR: TA_PROPERTY = TA_PROPERTY(4i32);
pub const TAP_TRANSFORMCOUNT: TA_PROPERTY = TA_PROPERTY(1i32);
pub const TAP_ZORDER: TA_PROPERTY = TA_PROPERTY(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKBANDPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKBARPARTS(pub i32);
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct TASKDIALOGCONFIG {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub dwFlags: TASKDIALOG_FLAGS,
    pub dwCommonButtons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pub pszWindowTitle: windows_core::PCWSTR,
    pub Anonymous1: TASKDIALOGCONFIG_0,
    pub pszMainInstruction: windows_core::PCWSTR,
    pub pszContent: windows_core::PCWSTR,
    pub cButtons: u32,
    pub pButtons: *const TASKDIALOG_BUTTON,
    pub nDefaultButton: i32,
    pub cRadioButtons: u32,
    pub pRadioButtons: *const TASKDIALOG_BUTTON,
    pub nDefaultRadioButton: i32,
    pub pszVerificationText: windows_core::PCWSTR,
    pub pszExpandedInformation: windows_core::PCWSTR,
    pub pszExpandedControlText: windows_core::PCWSTR,
    pub pszCollapsedControlText: windows_core::PCWSTR,
    pub Anonymous2: TASKDIALOGCONFIG_1,
    pub pszFooter: windows_core::PCWSTR,
    pub pfCallback: PFTASKDIALOGCALLBACK,
    pub lpCallbackData: isize,
    pub cxWidth: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for TASKDIALOGCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union TASKDIALOGCONFIG_0 {
    pub hMainIcon: super::WindowsAndMessaging::HICON,
    pub pszMainIcon: windows_core::PCWSTR,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for TASKDIALOGCONFIG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union TASKDIALOGCONFIG_1 {
    pub hFooterIcon: super::WindowsAndMessaging::HICON,
    pub pszFooterIcon: windows_core::PCWSTR,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for TASKDIALOGCONFIG_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKDIALOGPARTS(pub i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct TASKDIALOG_BUTTON {
    pub nButtonID: i32,
    pub pszButtonText: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKDIALOG_COMMON_BUTTON_FLAGS(pub i32);
impl TASKDIALOG_COMMON_BUTTON_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TASKDIALOG_COMMON_BUTTON_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TASKDIALOG_COMMON_BUTTON_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TASKDIALOG_COMMON_BUTTON_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TASKDIALOG_COMMON_BUTTON_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TASKDIALOG_COMMON_BUTTON_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKDIALOG_ELEMENTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKDIALOG_FLAGS(pub i32);
impl TASKDIALOG_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TASKDIALOG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TASKDIALOG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TASKDIALOG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TASKDIALOG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TASKDIALOG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKDIALOG_ICON_ELEMENTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKDIALOG_MESSAGES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKDIALOG_NOTIFICATIONS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKLINKSTATES(pub i32);
pub const TATF_HASINITIALVALUES: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(2i32);
pub const TATF_HASORIGINVALUES: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(4i32);
pub const TATF_NONE: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(0i32);
pub const TATF_TARGETVALUES_USER: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(1i32);
pub const TATT_CLIP: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(3i32);
pub const TATT_OPACITY: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(2i32);
pub const TATT_SCALE_2D: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(1i32);
pub const TATT_TRANSLATE_2D: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TA_CUBIC_BEZIER {
    pub header: TA_TIMINGFUNCTION,
    pub rX0: f32,
    pub rY0: f32,
    pub rX1: f32,
    pub rY1: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TA_PROPERTY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TA_PROPERTY_FLAG(pub i32);
impl TA_PROPERTY_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TA_PROPERTY_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TA_PROPERTY_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TA_PROPERTY_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TA_PROPERTY_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TA_PROPERTY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TA_TIMINGFUNCTION {
    pub eTimingFunctionType: TA_TIMINGFUNCTION_TYPE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TA_TIMINGFUNCTION_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TA_TRANSFORM {
    pub eTransformType: TA_TRANSFORM_TYPE,
    pub dwTimingFunctionId: u32,
    pub dwStartTime: u32,
    pub dwDurationTime: u32,
    pub eFlags: TA_TRANSFORM_FLAG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TA_TRANSFORM_2D {
    pub header: TA_TRANSFORM,
    pub rX: f32,
    pub rY: f32,
    pub rInitialX: f32,
    pub rInitialY: f32,
    pub rOriginX: f32,
    pub rOriginY: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TA_TRANSFORM_CLIP {
    pub header: TA_TRANSFORM,
    pub rLeft: f32,
    pub rTop: f32,
    pub rRight: f32,
    pub rBottom: f32,
    pub rInitialLeft: f32,
    pub rInitialTop: f32,
    pub rInitialRight: f32,
    pub rInitialBottom: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TA_TRANSFORM_FLAG(pub i32);
impl TA_TRANSFORM_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TA_TRANSFORM_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TA_TRANSFORM_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TA_TRANSFORM_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TA_TRANSFORM_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TA_TRANSFORM_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TA_TRANSFORM_OPACITY {
    pub header: TA_TRANSFORM,
    pub rOpacity: f32,
    pub rInitialOpacity: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TA_TRANSFORM_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBADDBITMAP {
    pub hInst: super::super::Foundation::HINSTANCE,
    pub nID: usize,
}
pub const TBBF_LARGE: u32 = 1u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TBBUTTON {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 2],
    pub dwData: usize,
    pub iString: isize,
}
#[cfg(target_arch = "x86")]
impl Default for TBBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TBBUTTON {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 6],
    pub dwData: usize,
    pub iString: isize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for TBBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBBUTTONINFOA {
    pub cbSize: u32,
    pub dwMask: TBBUTTONINFOW_MASK,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: windows_core::PSTR,
    pub cchText: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBBUTTONINFOW {
    pub cbSize: u32,
    pub dwMask: TBBUTTONINFOW_MASK,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: windows_core::PWSTR,
    pub cchText: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBBUTTONINFOW_MASK(pub u32);
impl TBBUTTONINFOW_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TBBUTTONINFOW_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TBBUTTONINFOW_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TBCDRF_BLENDICON: u32 = 2097152u32;
pub const TBCDRF_HILITEHOTTRACK: u32 = 131072u32;
pub const TBCDRF_NOBACKGROUND: u32 = 4194304u32;
pub const TBCDRF_NOEDGES: u32 = 65536u32;
pub const TBCDRF_NOETCHEDEFFECT: u32 = 1048576u32;
pub const TBCDRF_NOMARK: u32 = 524288u32;
pub const TBCDRF_NOOFFSET: u32 = 262144u32;
pub const TBCDRF_USECDCOLORS: u32 = 8388608u32;
pub const TBCD_CHANNEL: u32 = 3u32;
pub const TBCD_THUMB: u32 = 2u32;
pub const TBCD_TICS: u32 = 1u32;
pub const TBDDRET_DEFAULT: u32 = 0u32;
pub const TBDDRET_NODEFAULT: u32 = 1u32;
pub const TBDDRET_TREATPRESSED: u32 = 2u32;
pub const TBIF_BYINDEX: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(2147483648u32);
pub const TBIF_COMMAND: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(32u32);
pub const TBIF_IMAGE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(1u32);
pub const TBIF_LPARAM: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(16u32);
pub const TBIF_SIZE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(64u32);
pub const TBIF_STATE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(4u32);
pub const TBIF_STYLE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(8u32);
pub const TBIF_TEXT: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(2u32);
pub const TBIMHT_AFTER: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(1u32);
pub const TBIMHT_BACKGROUND: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(2u32);
pub const TBIMHT_NONE: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(0u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBINSERTMARK {
    pub iButton: i32,
    pub dwFlags: TBINSERTMARK_FLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBINSERTMARK_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBMETRICS {
    pub cbSize: u32,
    pub dwMask: u32,
    pub cxPad: i32,
    pub cyPad: i32,
    pub cxBarPad: i32,
    pub cyBarPad: i32,
    pub cxButtonSpacing: i32,
    pub cyButtonSpacing: i32,
}
pub const TBMF_BARPAD: u32 = 2u32;
pub const TBMF_BUTTONSPACING: u32 = 4u32;
pub const TBMF_PAD: u32 = 1u32;
pub const TBM_CLEARSEL: u32 = 1043u32;
pub const TBM_CLEARTICS: u32 = 1033u32;
pub const TBM_GETBUDDY: u32 = 1057u32;
pub const TBM_GETCHANNELRECT: u32 = 1050u32;
pub const TBM_GETLINESIZE: u32 = 1048u32;
pub const TBM_GETNUMTICS: u32 = 1040u32;
pub const TBM_GETPAGESIZE: u32 = 1046u32;
pub const TBM_GETPTICS: u32 = 1038u32;
pub const TBM_GETRANGEMAX: u32 = 1026u32;
pub const TBM_GETRANGEMIN: u32 = 1025u32;
pub const TBM_GETSELEND: u32 = 1042u32;
pub const TBM_GETSELSTART: u32 = 1041u32;
pub const TBM_GETTHUMBLENGTH: u32 = 1052u32;
pub const TBM_GETTHUMBRECT: u32 = 1049u32;
pub const TBM_GETTIC: u32 = 1027u32;
pub const TBM_GETTICPOS: u32 = 1039u32;
pub const TBM_GETTOOLTIPS: u32 = 1054u32;
pub const TBM_GETUNICODEFORMAT: u32 = 8198u32;
pub const TBM_SETBUDDY: u32 = 1056u32;
pub const TBM_SETLINESIZE: u32 = 1047u32;
pub const TBM_SETPAGESIZE: u32 = 1045u32;
pub const TBM_SETPOS: u32 = 1029u32;
pub const TBM_SETPOSNOTIFY: u32 = 1058u32;
pub const TBM_SETRANGE: u32 = 1030u32;
pub const TBM_SETRANGEMAX: u32 = 1032u32;
pub const TBM_SETRANGEMIN: u32 = 1031u32;
pub const TBM_SETSEL: u32 = 1034u32;
pub const TBM_SETSELEND: u32 = 1036u32;
pub const TBM_SETSELSTART: u32 = 1035u32;
pub const TBM_SETTHUMBLENGTH: u32 = 1051u32;
pub const TBM_SETTIC: u32 = 1028u32;
pub const TBM_SETTICFREQ: u32 = 1044u32;
pub const TBM_SETTIPSIDE: u32 = 1055u32;
pub const TBM_SETTOOLTIPS: u32 = 1053u32;
pub const TBM_SETUNICODEFORMAT: u32 = 8197u32;
pub const TBNF_DI_SETITEM: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(268435456u32);
pub const TBNF_IMAGE: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(1u32);
pub const TBNF_TEXT: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(2u32);
pub const TBNRF_ENDCUSTOMIZE: u32 = 2u32;
pub const TBNRF_HIDEHELP: u32 = 1u32;
pub const TBN_BEGINADJUST: u32 = 4294966593u32;
pub const TBN_BEGINDRAG: u32 = 4294966595u32;
pub const TBN_CUSTHELP: u32 = 4294966587u32;
pub const TBN_DELETINGBUTTON: u32 = 4294966581u32;
pub const TBN_DRAGOUT: u32 = 4294966582u32;
pub const TBN_DRAGOVER: u32 = 4294966569u32;
pub const TBN_DROPDOWN: u32 = 4294966586u32;
pub const TBN_DUPACCELERATOR: u32 = 4294966571u32;
pub const TBN_ENDADJUST: u32 = 4294966592u32;
pub const TBN_ENDDRAG: u32 = 4294966594u32;
pub const TBN_FIRST: u32 = 4294966596u32;
pub const TBN_GETBUTTONINFO: u32 = 4294966576u32;
pub const TBN_GETBUTTONINFOA: u32 = 4294966596u32;
pub const TBN_GETBUTTONINFOW: u32 = 4294966576u32;
pub const TBN_GETDISPINFO: u32 = 4294966579u32;
pub const TBN_GETDISPINFOA: u32 = 4294966580u32;
pub const TBN_GETDISPINFOW: u32 = 4294966579u32;
pub const TBN_GETINFOTIP: u32 = 4294966577u32;
pub const TBN_GETINFOTIPA: u32 = 4294966578u32;
pub const TBN_GETINFOTIPW: u32 = 4294966577u32;
pub const TBN_GETOBJECT: u32 = 4294966584u32;
pub const TBN_HOTITEMCHANGE: u32 = 4294966583u32;
pub const TBN_INITCUSTOMIZE: u32 = 4294966573u32;
pub const TBN_LAST: u32 = 4294966576u32;
pub const TBN_MAPACCELERATOR: u32 = 4294966568u32;
pub const TBN_QUERYDELETE: u32 = 4294966589u32;
pub const TBN_QUERYINSERT: u32 = 4294966590u32;
pub const TBN_RESET: u32 = 4294966591u32;
pub const TBN_RESTORE: u32 = 4294966575u32;
pub const TBN_SAVE: u32 = 4294966574u32;
pub const TBN_TOOLBARCHANGE: u32 = 4294966588u32;
pub const TBN_WRAPACCELERATOR: u32 = 4294966570u32;
pub const TBN_WRAPHOTITEM: u32 = 4294966572u32;
pub const TBP_BACKGROUNDBOTTOM: TASKBARPARTS = TASKBARPARTS(1i32);
pub const TBP_BACKGROUNDLEFT: TASKBARPARTS = TASKBARPARTS(4i32);
pub const TBP_BACKGROUNDRIGHT: TASKBARPARTS = TASKBARPARTS(2i32);
pub const TBP_BACKGROUNDTOP: TASKBARPARTS = TASKBARPARTS(3i32);
pub const TBP_SIZINGBARBOTTOM: TASKBARPARTS = TASKBARPARTS(5i32);
pub const TBP_SIZINGBARLEFT: TASKBARPARTS = TASKBARPARTS(8i32);
pub const TBP_SIZINGBARRIGHT: TASKBARPARTS = TASKBARPARTS(6i32);
pub const TBP_SIZINGBARTOP: TASKBARPARTS = TASKBARPARTS(7i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBREPLACEBITMAP {
    pub hInstOld: super::super::Foundation::HINSTANCE,
    pub nIDOld: usize,
    pub hInstNew: super::super::Foundation::HINSTANCE,
    pub nIDNew: usize,
    pub nButtons: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBSAVEPARAMSA {
    pub hkr: super::super::System::Registry::HKEY,
    pub pszSubKey: windows_core::PCSTR,
    pub pszValueName: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBSAVEPARAMSW {
    pub hkr: super::super::System::Registry::HKEY,
    pub pszSubKey: windows_core::PCWSTR,
    pub pszValueName: windows_core::PCWSTR,
}
pub const TBSTATE_CHECKED: u32 = 1u32;
pub const TBSTATE_ELLIPSES: u32 = 64u32;
pub const TBSTATE_ENABLED: u32 = 4u32;
pub const TBSTATE_HIDDEN: u32 = 8u32;
pub const TBSTATE_INDETERMINATE: u32 = 16u32;
pub const TBSTATE_MARKED: u32 = 128u32;
pub const TBSTATE_PRESSED: u32 = 2u32;
pub const TBSTATE_WRAP: u32 = 32u32;
pub const TBSTYLE_ALTDRAG: u32 = 1024u32;
pub const TBSTYLE_AUTOSIZE: u32 = 16u32;
pub const TBSTYLE_BUTTON: u32 = 0u32;
pub const TBSTYLE_CHECK: u32 = 2u32;
pub const TBSTYLE_CUSTOMERASE: u32 = 8192u32;
pub const TBSTYLE_DROPDOWN: u32 = 8u32;
pub const TBSTYLE_EX_DOUBLEBUFFER: u32 = 128u32;
pub const TBSTYLE_EX_DRAWDDARROWS: u32 = 1u32;
pub const TBSTYLE_EX_HIDECLIPPEDBUTTONS: u32 = 16u32;
pub const TBSTYLE_EX_MIXEDBUTTONS: u32 = 8u32;
pub const TBSTYLE_EX_MULTICOLUMN: u32 = 2u32;
pub const TBSTYLE_EX_VERTICAL: u32 = 4u32;
pub const TBSTYLE_FLAT: u32 = 2048u32;
pub const TBSTYLE_GROUP: u32 = 4u32;
pub const TBSTYLE_LIST: u32 = 4096u32;
pub const TBSTYLE_NOPREFIX: u32 = 32u32;
pub const TBSTYLE_REGISTERDROP: u32 = 16384u32;
pub const TBSTYLE_SEP: u32 = 1u32;
pub const TBSTYLE_TOOLTIPS: u32 = 256u32;
pub const TBSTYLE_TRANSPARENT: u32 = 32768u32;
pub const TBSTYLE_WRAPABLE: u32 = 512u32;
pub const TBS_AUTOTICKS: u32 = 1u32;
pub const TBS_BOTH: u32 = 8u32;
pub const TBS_BOTTOM: u32 = 0u32;
pub const TBS_DOWNISLEFT: u32 = 1024u32;
pub const TBS_ENABLESELRANGE: u32 = 32u32;
pub const TBS_FIXEDLENGTH: u32 = 64u32;
pub const TBS_HORZ: u32 = 0u32;
pub const TBS_LEFT: u32 = 4u32;
pub const TBS_NOTHUMB: u32 = 128u32;
pub const TBS_NOTICKS: u32 = 16u32;
pub const TBS_NOTIFYBEFOREMOVE: u32 = 2048u32;
pub const TBS_REVERSED: u32 = 512u32;
pub const TBS_RIGHT: u32 = 0u32;
pub const TBS_TOOLTIPS: u32 = 256u32;
pub const TBS_TOP: u32 = 4u32;
pub const TBS_TRANSPARENTBKGND: u32 = 4096u32;
pub const TBS_VERT: u32 = 2u32;
pub const TBTS_BOTTOM: u32 = 2u32;
pub const TBTS_LEFT: u32 = 1u32;
pub const TBTS_RIGHT: u32 = 3u32;
pub const TBTS_TOP: u32 = 0u32;
pub const TB_ADDBITMAP: u32 = 1043u32;
pub const TB_ADDBUTTONS: u32 = 1092u32;
pub const TB_ADDBUTTONSA: u32 = 1044u32;
pub const TB_ADDBUTTONSW: u32 = 1092u32;
pub const TB_ADDSTRING: u32 = 1101u32;
pub const TB_ADDSTRINGA: u32 = 1052u32;
pub const TB_ADDSTRINGW: u32 = 1101u32;
pub const TB_AUTOSIZE: u32 = 1057u32;
pub const TB_BOTTOM: u32 = 7u32;
pub const TB_BUTTONCOUNT: u32 = 1048u32;
pub const TB_BUTTONSTRUCTSIZE: u32 = 1054u32;
pub const TB_CHANGEBITMAP: u32 = 1067u32;
pub const TB_CHECKBUTTON: u32 = 1026u32;
pub const TB_COMMANDTOINDEX: u32 = 1049u32;
pub const TB_CUSTOMIZE: u32 = 1051u32;
pub const TB_DELETEBUTTON: u32 = 1046u32;
pub const TB_ENABLEBUTTON: u32 = 1025u32;
pub const TB_ENDTRACK: u32 = 8u32;
pub const TB_GETANCHORHIGHLIGHT: u32 = 1098u32;
pub const TB_GETBITMAP: u32 = 1068u32;
pub const TB_GETBITMAPFLAGS: u32 = 1065u32;
pub const TB_GETBUTTON: u32 = 1047u32;
pub const TB_GETBUTTONINFO: u32 = 1087u32;
pub const TB_GETBUTTONINFOA: u32 = 1089u32;
pub const TB_GETBUTTONINFOW: u32 = 1087u32;
pub const TB_GETBUTTONSIZE: u32 = 1082u32;
pub const TB_GETBUTTONTEXT: u32 = 1099u32;
pub const TB_GETBUTTONTEXTA: u32 = 1069u32;
pub const TB_GETBUTTONTEXTW: u32 = 1099u32;
pub const TB_GETCOLORSCHEME: u32 = 8195u32;
pub const TB_GETDISABLEDIMAGELIST: u32 = 1079u32;
pub const TB_GETEXTENDEDSTYLE: u32 = 1109u32;
pub const TB_GETHOTIMAGELIST: u32 = 1077u32;
pub const TB_GETHOTITEM: u32 = 1095u32;
pub const TB_GETIDEALSIZE: u32 = 1123u32;
pub const TB_GETIMAGELIST: u32 = 1073u32;
pub const TB_GETIMAGELISTCOUNT: u32 = 1122u32;
pub const TB_GETINSERTMARK: u32 = 1103u32;
pub const TB_GETINSERTMARKCOLOR: u32 = 1113u32;
pub const TB_GETITEMDROPDOWNRECT: u32 = 1127u32;
pub const TB_GETITEMRECT: u32 = 1053u32;
pub const TB_GETMAXSIZE: u32 = 1107u32;
pub const TB_GETMETRICS: u32 = 1125u32;
pub const TB_GETOBJECT: u32 = 1086u32;
pub const TB_GETPADDING: u32 = 1110u32;
pub const TB_GETPRESSEDIMAGELIST: u32 = 1129u32;
pub const TB_GETRECT: u32 = 1075u32;
pub const TB_GETROWS: u32 = 1064u32;
pub const TB_GETSTATE: u32 = 1042u32;
pub const TB_GETSTRING: u32 = 1115u32;
pub const TB_GETSTRINGA: u32 = 1116u32;
pub const TB_GETSTRINGW: u32 = 1115u32;
pub const TB_GETSTYLE: u32 = 1081u32;
pub const TB_GETTEXTROWS: u32 = 1085u32;
pub const TB_GETTOOLTIPS: u32 = 1059u32;
pub const TB_GETUNICODEFORMAT: u32 = 8198u32;
pub const TB_HASACCELERATOR: u32 = 1119u32;
pub const TB_HIDEBUTTON: u32 = 1028u32;
pub const TB_HITTEST: u32 = 1093u32;
pub const TB_INDETERMINATE: u32 = 1029u32;
pub const TB_INSERTBUTTON: u32 = 1091u32;
pub const TB_INSERTBUTTONA: u32 = 1045u32;
pub const TB_INSERTBUTTONW: u32 = 1091u32;
pub const TB_INSERTMARKHITTEST: u32 = 1105u32;
pub const TB_ISBUTTONCHECKED: u32 = 1034u32;
pub const TB_ISBUTTONENABLED: u32 = 1033u32;
pub const TB_ISBUTTONHIDDEN: u32 = 1036u32;
pub const TB_ISBUTTONHIGHLIGHTED: u32 = 1038u32;
pub const TB_ISBUTTONINDETERMINATE: u32 = 1037u32;
pub const TB_ISBUTTONPRESSED: u32 = 1035u32;
pub const TB_LINEDOWN: u32 = 1u32;
pub const TB_LINEUP: u32 = 0u32;
pub const TB_LOADIMAGES: u32 = 1074u32;
pub const TB_MAPACCELERATOR: u32 = 1114u32;
pub const TB_MAPACCELERATORA: u32 = 1102u32;
pub const TB_MAPACCELERATORW: u32 = 1114u32;
pub const TB_MARKBUTTON: u32 = 1030u32;
pub const TB_MOVEBUTTON: u32 = 1106u32;
pub const TB_PAGEDOWN: u32 = 3u32;
pub const TB_PAGEUP: u32 = 2u32;
pub const TB_PRESSBUTTON: u32 = 1027u32;
pub const TB_REPLACEBITMAP: u32 = 1070u32;
pub const TB_SAVERESTORE: u32 = 1100u32;
pub const TB_SAVERESTOREA: u32 = 1050u32;
pub const TB_SAVERESTOREW: u32 = 1100u32;
pub const TB_SETANCHORHIGHLIGHT: u32 = 1097u32;
pub const TB_SETBITMAPSIZE: u32 = 1056u32;
pub const TB_SETBOUNDINGSIZE: u32 = 1117u32;
pub const TB_SETBUTTONINFO: u32 = 1088u32;
pub const TB_SETBUTTONINFOA: u32 = 1090u32;
pub const TB_SETBUTTONINFOW: u32 = 1088u32;
pub const TB_SETBUTTONSIZE: u32 = 1055u32;
pub const TB_SETBUTTONWIDTH: u32 = 1083u32;
pub const TB_SETCMDID: u32 = 1066u32;
pub const TB_SETCOLORSCHEME: u32 = 8194u32;
pub const TB_SETDISABLEDIMAGELIST: u32 = 1078u32;
pub const TB_SETDRAWTEXTFLAGS: u32 = 1094u32;
pub const TB_SETEXTENDEDSTYLE: u32 = 1108u32;
pub const TB_SETHOTIMAGELIST: u32 = 1076u32;
pub const TB_SETHOTITEM: u32 = 1096u32;
pub const TB_SETHOTITEM2: u32 = 1118u32;
pub const TB_SETIMAGELIST: u32 = 1072u32;
pub const TB_SETINDENT: u32 = 1071u32;
pub const TB_SETINSERTMARK: u32 = 1104u32;
pub const TB_SETINSERTMARKCOLOR: u32 = 1112u32;
pub const TB_SETLISTGAP: u32 = 1120u32;
pub const TB_SETMAXTEXTROWS: u32 = 1084u32;
pub const TB_SETMETRICS: u32 = 1126u32;
pub const TB_SETPADDING: u32 = 1111u32;
pub const TB_SETPARENT: u32 = 1061u32;
pub const TB_SETPRESSEDIMAGELIST: u32 = 1128u32;
pub const TB_SETROWS: u32 = 1063u32;
pub const TB_SETSTATE: u32 = 1041u32;
pub const TB_SETSTYLE: u32 = 1080u32;
pub const TB_SETTOOLTIPS: u32 = 1060u32;
pub const TB_SETUNICODEFORMAT: u32 = 8197u32;
pub const TB_SETWINDOWTHEME: u32 = 8203u32;
pub const TB_THUMBPOSITION: u32 = 4u32;
pub const TB_THUMBTRACK: u32 = 5u32;
pub const TB_TOP: u32 = 6u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: TCHITTESTINFO_FLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCHITTESTINFO_FLAGS(pub u32);
pub const TCHT_NOWHERE: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(1u32);
pub const TCHT_ONITEM: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(6u32);
pub const TCHT_ONITEMICON: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(2u32);
pub const TCHT_ONITEMLABEL: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(4u32);
pub const TCIF_IMAGE: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(2u32);
pub const TCIF_PARAM: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(8u32);
pub const TCIF_RTLREADING: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(4u32);
pub const TCIF_STATE: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(16u32);
pub const TCIF_TEXT: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(1u32);
pub const TCIS_BUTTONPRESSED: TAB_CONTROL_ITEM_STATE = TAB_CONTROL_ITEM_STATE(1u32);
pub const TCIS_HIGHLIGHTED: TAB_CONTROL_ITEM_STATE = TAB_CONTROL_ITEM_STATE(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCITEMA {
    pub mask: TCITEMHEADERA_MASK,
    pub dwState: TAB_CONTROL_ITEM_STATE,
    pub dwStateMask: TAB_CONTROL_ITEM_STATE,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCITEMHEADERA {
    pub mask: TCITEMHEADERA_MASK,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCITEMHEADERA_MASK(pub u32);
impl TCITEMHEADERA_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TCITEMHEADERA_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TCITEMHEADERA_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TCITEMHEADERA_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TCITEMHEADERA_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TCITEMHEADERA_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCITEMHEADERW {
    pub mask: TCITEMHEADERA_MASK,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCITEMW {
    pub mask: TCITEMHEADERA_MASK,
    pub dwState: TAB_CONTROL_ITEM_STATE,
    pub dwStateMask: TAB_CONTROL_ITEM_STATE,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
pub const TCM_ADJUSTRECT: u32 = 4904u32;
pub const TCM_DELETEALLITEMS: u32 = 4873u32;
pub const TCM_DELETEITEM: u32 = 4872u32;
pub const TCM_DESELECTALL: u32 = 4914u32;
pub const TCM_FIRST: u32 = 4864u32;
pub const TCM_GETCURFOCUS: u32 = 4911u32;
pub const TCM_GETCURSEL: u32 = 4875u32;
pub const TCM_GETEXTENDEDSTYLE: u32 = 4917u32;
pub const TCM_GETIMAGELIST: u32 = 4866u32;
pub const TCM_GETITEM: u32 = 4924u32;
pub const TCM_GETITEMA: u32 = 4869u32;
pub const TCM_GETITEMCOUNT: u32 = 4868u32;
pub const TCM_GETITEMRECT: u32 = 4874u32;
pub const TCM_GETITEMW: u32 = 4924u32;
pub const TCM_GETROWCOUNT: u32 = 4908u32;
pub const TCM_GETTOOLTIPS: u32 = 4909u32;
pub const TCM_GETUNICODEFORMAT: u32 = 8198u32;
pub const TCM_HIGHLIGHTITEM: u32 = 4915u32;
pub const TCM_HITTEST: u32 = 4877u32;
pub const TCM_INSERTITEM: u32 = 4926u32;
pub const TCM_INSERTITEMA: u32 = 4871u32;
pub const TCM_INSERTITEMW: u32 = 4926u32;
pub const TCM_REMOVEIMAGE: u32 = 4906u32;
pub const TCM_SETCURFOCUS: u32 = 4912u32;
pub const TCM_SETCURSEL: u32 = 4876u32;
pub const TCM_SETEXTENDEDSTYLE: u32 = 4916u32;
pub const TCM_SETIMAGELIST: u32 = 4867u32;
pub const TCM_SETITEM: u32 = 4925u32;
pub const TCM_SETITEMA: u32 = 4870u32;
pub const TCM_SETITEMEXTRA: u32 = 4878u32;
pub const TCM_SETITEMSIZE: u32 = 4905u32;
pub const TCM_SETITEMW: u32 = 4925u32;
pub const TCM_SETMINTABWIDTH: u32 = 4913u32;
pub const TCM_SETPADDING: u32 = 4907u32;
pub const TCM_SETTOOLTIPS: u32 = 4910u32;
pub const TCM_SETUNICODEFORMAT: u32 = 8197u32;
pub const TCN_FIRST: u32 = 4294966746u32;
pub const TCN_FOCUSCHANGE: u32 = 4294966742u32;
pub const TCN_GETOBJECT: u32 = 4294966743u32;
pub const TCN_KEYDOWN: u32 = 4294966746u32;
pub const TCN_LAST: u32 = 4294966716u32;
pub const TCN_SELCHANGE: u32 = 4294966745u32;
pub const TCN_SELCHANGING: u32 = 4294966744u32;
pub const TCS_BOTTOM: u32 = 2u32;
pub const TCS_BUTTONS: u32 = 256u32;
pub const TCS_EX_FLATSEPARATORS: u32 = 1u32;
pub const TCS_EX_REGISTERDROP: u32 = 2u32;
pub const TCS_FIXEDWIDTH: u32 = 1024u32;
pub const TCS_FLATBUTTONS: u32 = 8u32;
pub const TCS_FOCUSNEVER: u32 = 32768u32;
pub const TCS_FOCUSONBUTTONDOWN: u32 = 4096u32;
pub const TCS_FORCEICONLEFT: u32 = 16u32;
pub const TCS_FORCELABELLEFT: u32 = 32u32;
pub const TCS_HOTTRACK: u32 = 64u32;
pub const TCS_MULTILINE: u32 = 512u32;
pub const TCS_MULTISELECT: u32 = 4u32;
pub const TCS_OWNERDRAWFIXED: u32 = 8192u32;
pub const TCS_RAGGEDRIGHT: u32 = 2048u32;
pub const TCS_RIGHT: u32 = 2u32;
pub const TCS_RIGHTJUSTIFY: u32 = 0u32;
pub const TCS_SCROLLOPPOSITE: u32 = 1u32;
pub const TCS_SINGLELINE: u32 = 0u32;
pub const TCS_TABS: u32 = 0u32;
pub const TCS_TOOLTIPS: u32 = 16384u32;
pub const TCS_VERTICAL: u32 = 128u32;
pub const TDCBF_ABORT_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(65536i32);
pub const TDCBF_CANCEL_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(8i32);
pub const TDCBF_CLOSE_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(32i32);
pub const TDCBF_CONTINUE_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(524288i32);
pub const TDCBF_HELP_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(1048576i32);
pub const TDCBF_IGNORE_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(131072i32);
pub const TDCBF_NO_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(4i32);
pub const TDCBF_OK_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(1i32);
pub const TDCBF_RETRY_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(16i32);
pub const TDCBF_TRYAGAIN_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(262144i32);
pub const TDCBF_YES_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(2i32);
pub const TDE_CONTENT: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(0i32);
pub const TDE_EXPANDED_INFORMATION: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(1i32);
pub const TDE_FOOTER: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(2i32);
pub const TDE_MAIN_INSTRUCTION: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(3i32);
pub const TDF_ALLOW_DIALOG_CANCELLATION: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(8i32);
pub const TDF_CALLBACK_TIMER: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(2048i32);
pub const TDF_CAN_BE_MINIMIZED: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(32768i32);
pub const TDF_ENABLE_HYPERLINKS: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(1i32);
pub const TDF_EXPANDED_BY_DEFAULT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(128i32);
pub const TDF_EXPAND_FOOTER_AREA: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(64i32);
pub const TDF_NO_DEFAULT_RADIO_BUTTON: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16384i32);
pub const TDF_NO_SET_FOREGROUND: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(65536i32);
pub const TDF_POSITION_RELATIVE_TO_WINDOW: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(4096i32);
pub const TDF_RTL_LAYOUT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(8192i32);
pub const TDF_SHOW_MARQUEE_PROGRESS_BAR: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(1024i32);
pub const TDF_SHOW_PROGRESS_BAR: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(512i32);
pub const TDF_SIZE_TO_CONTENT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16777216i32);
pub const TDF_USE_COMMAND_LINKS: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16i32);
pub const TDF_USE_COMMAND_LINKS_NO_ICON: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(32i32);
pub const TDF_USE_HICON_FOOTER: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(4i32);
pub const TDF_USE_HICON_MAIN: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(2i32);
pub const TDF_VERIFICATION_FLAG_CHECKED: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(256i32);
pub const TDIE_ICON_FOOTER: TASKDIALOG_ICON_ELEMENTS = TASKDIALOG_ICON_ELEMENTS(1i32);
pub const TDIE_ICON_MAIN: TASKDIALOG_ICON_ELEMENTS = TASKDIALOG_ICON_ELEMENTS(0i32);
pub const TDLGCPS_STANDALONE: CONTENTPANESTATES = CONTENTPANESTATES(1i32);
pub const TDLGEBS_EXPANDEDDISABLED: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(8i32);
pub const TDLGEBS_EXPANDEDHOVER: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(5i32);
pub const TDLGEBS_EXPANDEDNORMAL: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(4i32);
pub const TDLGEBS_EXPANDEDPRESSED: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(6i32);
pub const TDLGEBS_HOVER: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(2i32);
pub const TDLGEBS_NORMAL: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(1i32);
pub const TDLGEBS_NORMALDISABLED: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(7i32);
pub const TDLGEBS_PRESSED: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(3i32);
pub const TDLG_BUTTONSECTION: TASKDIALOGPARTS = TASKDIALOGPARTS(10i32);
pub const TDLG_BUTTONWRAPPER: TASKDIALOGPARTS = TASKDIALOGPARTS(11i32);
pub const TDLG_COMMANDLINKPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(7i32);
pub const TDLG_CONTENTICON: TASKDIALOGPARTS = TASKDIALOGPARTS(5i32);
pub const TDLG_CONTENTPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(4i32);
pub const TDLG_CONTROLPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(9i32);
pub const TDLG_EXPANDEDCONTENT: TASKDIALOGPARTS = TASKDIALOGPARTS(6i32);
pub const TDLG_EXPANDEDFOOTERAREA: TASKDIALOGPARTS = TASKDIALOGPARTS(18i32);
pub const TDLG_EXPANDOBUTTON: TASKDIALOGPARTS = TASKDIALOGPARTS(13i32);
pub const TDLG_EXPANDOTEXT: TASKDIALOGPARTS = TASKDIALOGPARTS(12i32);
pub const TDLG_FOOTNOTEAREA: TASKDIALOGPARTS = TASKDIALOGPARTS(16i32);
pub const TDLG_FOOTNOTEPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(15i32);
pub const TDLG_FOOTNOTESEPARATOR: TASKDIALOGPARTS = TASKDIALOGPARTS(17i32);
pub const TDLG_IMAGEALIGNMENT: TASKDIALOGPARTS = TASKDIALOGPARTS(20i32);
pub const TDLG_MAINICON: TASKDIALOGPARTS = TASKDIALOGPARTS(3i32);
pub const TDLG_MAININSTRUCTIONPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(2i32);
pub const TDLG_PRIMARYPANEL: TASKDIALOGPARTS = TASKDIALOGPARTS(1i32);
pub const TDLG_PROGRESSBAR: TASKDIALOGPARTS = TASKDIALOGPARTS(19i32);
pub const TDLG_RADIOBUTTONPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(21i32);
pub const TDLG_SECONDARYPANEL: TASKDIALOGPARTS = TASKDIALOGPARTS(8i32);
pub const TDLG_VERIFICATIONTEXT: TASKDIALOGPARTS = TASKDIALOGPARTS(14i32);
pub const TDM_CLICK_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1126i32);
pub const TDM_CLICK_RADIO_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1134i32);
pub const TDM_CLICK_VERIFICATION: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1137i32);
pub const TDM_ENABLE_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1135i32);
pub const TDM_ENABLE_RADIO_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1136i32);
pub const TDM_NAVIGATE_PAGE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1125i32);
pub const TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1139i32);
pub const TDM_SET_ELEMENT_TEXT: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1132i32);
pub const TDM_SET_MARQUEE_PROGRESS_BAR: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1127i32);
pub const TDM_SET_PROGRESS_BAR_MARQUEE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1131i32);
pub const TDM_SET_PROGRESS_BAR_POS: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1130i32);
pub const TDM_SET_PROGRESS_BAR_RANGE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1129i32);
pub const TDM_SET_PROGRESS_BAR_STATE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1128i32);
pub const TDM_UPDATE_ELEMENT_TEXT: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1138i32);
pub const TDM_UPDATE_ICON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1140i32);
pub const TDN_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(2i32);
pub const TDN_CREATED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(0i32);
pub const TDN_DESTROYED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(5i32);
pub const TDN_DIALOG_CONSTRUCTED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(7i32);
pub const TDN_EXPANDO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(10i32);
pub const TDN_HELP: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(9i32);
pub const TDN_HYPERLINK_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(3i32);
pub const TDN_NAVIGATED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(1i32);
pub const TDN_RADIO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(6i32);
pub const TDN_TIMER: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(4i32);
pub const TDN_VERIFICATION_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(8i32);
pub const TDP_FLASHBUTTON: TASKBANDPARTS = TASKBANDPARTS(2i32);
pub const TDP_FLASHBUTTONGROUPMENU: TASKBANDPARTS = TASKBANDPARTS(3i32);
pub const TDP_GROUPCOUNT: TASKBANDPARTS = TASKBANDPARTS(1i32);
pub const TD_ERROR_ICON: windows_core::PCWSTR = windows_core::PCWSTR(65534u16 as _);
pub const TD_INFORMATION_ICON: windows_core::PCWSTR = windows_core::PCWSTR(65533u16 as _);
pub const TD_SHIELD_ICON: windows_core::PCWSTR = windows_core::PCWSTR(65532u16 as _);
pub const TD_WARNING_ICON: windows_core::PCWSTR = windows_core::PCWSTR(65535u16 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TEXTSELECTIONGRIPPERPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TEXTSHADOWTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TEXTSTYLEPARTS(pub i32);
pub const TEXT_BODYTEXT: TEXTSTYLEPARTS = TEXTSTYLEPARTS(4i32);
pub const TEXT_BODYTITLE: TEXTSTYLEPARTS = TEXTSTYLEPARTS(3i32);
pub const TEXT_CONTROLLABEL: TEXTSTYLEPARTS = TEXTSTYLEPARTS(9i32);
pub const TEXT_EXPANDED: TEXTSTYLEPARTS = TEXTSTYLEPARTS(7i32);
pub const TEXT_HYPERLINKTEXT: TEXTSTYLEPARTS = TEXTSTYLEPARTS(6i32);
pub const TEXT_INSTRUCTION: TEXTSTYLEPARTS = TEXTSTYLEPARTS(2i32);
pub const TEXT_LABEL: TEXTSTYLEPARTS = TEXTSTYLEPARTS(8i32);
pub const TEXT_MAININSTRUCTION: TEXTSTYLEPARTS = TEXTSTYLEPARTS(1i32);
pub const TEXT_SECONDARYTEXT: TEXTSTYLEPARTS = TEXTSTYLEPARTS(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THEMESIZE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THEME_PROPERTY_SYMBOL_ID(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THUMBBOTTOMSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THUMBLEFTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THUMBRIGHTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THUMBSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THUMBTOPSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THUMBVERTSTATES(pub i32);
pub const TIBES_DISABLED: TABITEMBOTHEDGESTATES = TABITEMBOTHEDGESTATES(4i32);
pub const TIBES_FOCUSED: TABITEMBOTHEDGESTATES = TABITEMBOTHEDGESTATES(5i32);
pub const TIBES_HOT: TABITEMBOTHEDGESTATES = TABITEMBOTHEDGESTATES(2i32);
pub const TIBES_NORMAL: TABITEMBOTHEDGESTATES = TABITEMBOTHEDGESTATES(1i32);
pub const TIBES_SELECTED: TABITEMBOTHEDGESTATES = TABITEMBOTHEDGESTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TICSSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TICSVERTSTATES(pub i32);
pub const TILES_DISABLED: TABITEMLEFTEDGESTATES = TABITEMLEFTEDGESTATES(4i32);
pub const TILES_FOCUSED: TABITEMLEFTEDGESTATES = TABITEMLEFTEDGESTATES(5i32);
pub const TILES_HOT: TABITEMLEFTEDGESTATES = TABITEMLEFTEDGESTATES(2i32);
pub const TILES_NORMAL: TABITEMLEFTEDGESTATES = TABITEMLEFTEDGESTATES(1i32);
pub const TILES_SELECTED: TABITEMLEFTEDGESTATES = TABITEMLEFTEDGESTATES(3i32);
pub const TIRES_DISABLED: TABITEMRIGHTEDGESTATES = TABITEMRIGHTEDGESTATES(4i32);
pub const TIRES_FOCUSED: TABITEMRIGHTEDGESTATES = TABITEMRIGHTEDGESTATES(5i32);
pub const TIRES_HOT: TABITEMRIGHTEDGESTATES = TABITEMRIGHTEDGESTATES(2i32);
pub const TIRES_NORMAL: TABITEMRIGHTEDGESTATES = TABITEMRIGHTEDGESTATES(1i32);
pub const TIRES_SELECTED: TABITEMRIGHTEDGESTATES = TABITEMRIGHTEDGESTATES(3i32);
pub const TIS_DISABLED: TABITEMSTATES = TABITEMSTATES(4i32);
pub const TIS_FOCUSED: TABITEMSTATES = TABITEMSTATES(5i32);
pub const TIS_HOT: TABITEMSTATES = TABITEMSTATES(2i32);
pub const TIS_NORMAL: TABITEMSTATES = TABITEMSTATES(1i32);
pub const TIS_SELECTED: TABITEMSTATES = TABITEMSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TITLEBARSTATES(pub i32);
pub const TKP_THUMB: TRACKBARPARTS = TRACKBARPARTS(3i32);
pub const TKP_THUMBBOTTOM: TRACKBARPARTS = TRACKBARPARTS(4i32);
pub const TKP_THUMBLEFT: TRACKBARPARTS = TRACKBARPARTS(7i32);
pub const TKP_THUMBRIGHT: TRACKBARPARTS = TRACKBARPARTS(8i32);
pub const TKP_THUMBTOP: TRACKBARPARTS = TRACKBARPARTS(5i32);
pub const TKP_THUMBVERT: TRACKBARPARTS = TRACKBARPARTS(6i32);
pub const TKP_TICS: TRACKBARPARTS = TRACKBARPARTS(9i32);
pub const TKP_TICSVERT: TRACKBARPARTS = TRACKBARPARTS(10i32);
pub const TKP_TRACK: TRACKBARPARTS = TRACKBARPARTS(1i32);
pub const TKP_TRACKVERT: TRACKBARPARTS = TRACKBARPARTS(2i32);
pub const TKS_NORMAL: TRACKBARSTYLESTATES = TRACKBARSTYLESTATES(1i32);
pub const TMTVS_RESERVEDHIGH: u32 = 19999u32;
pub const TMTVS_RESERVEDLOW: u32 = 100000u32;
pub const TMT_ACCENTCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3823u32);
pub const TMT_ACTIVEBORDER: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1611u32);
pub const TMT_ACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1603u32);
pub const TMT_ALIAS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1404u32);
pub const TMT_ALPHALEVEL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2402u32);
pub const TMT_ALPHATHRESHOLD: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2415u32);
pub const TMT_ALWAYSSHOWSIZINGBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2208u32);
pub const TMT_ANIMATIONBUTTONRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5005u32);
pub const TMT_ANIMATIONDELAY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2428u32);
pub const TMT_ANIMATIONDURATION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5006u32);
pub const TMT_APPWORKSPACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1613u32);
pub const TMT_ATLASIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8000u32);
pub const TMT_ATLASINPUTIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8001u32);
pub const TMT_ATLASRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8002u32);
pub const TMT_AUTHOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(604u32);
pub const TMT_AUTOSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2202u32);
pub const TMT_BACKGROUND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1602u32);
pub const TMT_BGFILL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2205u32);
pub const TMT_BGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4001u32);
pub const TMT_BITMAPREF: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(215u32);
pub const TMT_BLENDCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5003u32);
pub const TMT_BODYFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(809u32);
pub const TMT_BODYTEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3827u32);
pub const TMT_BOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(203u32);
pub const TMT_BORDERCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3801u32);
pub const TMT_BORDERCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3822u32);
pub const TMT_BORDERONLY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2203u32);
pub const TMT_BORDERSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2403u32);
pub const TMT_BORDERTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4002u32);
pub const TMT_BTNFACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1616u32);
pub const TMT_BTNHIGHLIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1621u32);
pub const TMT_BTNSHADOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1617u32);
pub const TMT_BTNTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1619u32);
pub const TMT_BUTTONALTERNATEFACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1626u32);
pub const TMT_CAPTIONBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1205u32);
pub const TMT_CAPTIONBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1204u32);
pub const TMT_CAPTIONFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(801u32);
pub const TMT_CAPTIONMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3603u32);
pub const TMT_CAPTIONTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1610u32);
pub const TMT_CHARSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(403u32);
pub const TMT_CLASSICVALUE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3202u32);
pub const TMT_COLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(204u32);
pub const TMT_COLORIZATIONCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2431u32);
pub const TMT_COLORIZATIONOPACITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2432u32);
pub const TMT_COLORSCHEMES: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(401u32);
pub const TMT_COMPANY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(603u32);
pub const TMT_COMPOSITED: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2204u32);
pub const TMT_COMPOSITEDOPAQUE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2219u32);
pub const TMT_CONTENTALIGNMENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4006u32);
pub const TMT_CONTENTMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3602u32);
pub const TMT_COPYRIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(605u32);
pub const TMT_CSSNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1401u32);
pub const TMT_CUSTOMSPLITRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5004u32);
pub const TMT_DEFAULTPANESIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5002u32);
pub const TMT_DESCRIPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(608u32);
pub const TMT_DIBDATA: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2u32);
pub const TMT_DISKSTREAM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(213u32);
pub const TMT_DISPLAYNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(601u32);
pub const TMT_DKSHADOW3D: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1622u32);
pub const TMT_DRAWBORDERS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2214u32);
pub const TMT_EDGEDKSHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3807u32);
pub const TMT_EDGEFILLCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3808u32);
pub const TMT_EDGEHIGHLIGHTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3805u32);
pub const TMT_EDGELIGHTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3804u32);
pub const TMT_EDGESHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3806u32);
pub const TMT_ENUM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(200u32);
pub const TMT_FILENAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(206u32);
pub const TMT_FILLCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3802u32);
pub const TMT_FILLCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3821u32);
pub const TMT_FILLTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4003u32);
pub const TMT_FIRSTBOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_FIRSTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1601u32);
pub const TMT_FIRSTFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(801u32);
pub const TMT_FIRSTINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_FIRSTSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1201u32);
pub const TMT_FIRSTSTRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1401u32);
pub const TMT_FIRST_RCSTRING_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(601u32);
pub const TMT_FLATMENUS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_FLOAT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(216u32);
pub const TMT_FLOATLIST: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(217u32);
pub const TMT_FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(210u32);
pub const TMT_FRAMESPERSECOND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2426u32);
pub const TMT_FROMCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2001u32);
pub const TMT_FROMCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2002u32);
pub const TMT_FROMCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2003u32);
pub const TMT_FROMCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2004u32);
pub const TMT_FROMCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2005u32);
pub const TMT_FROMHUE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1801u32);
pub const TMT_FROMHUE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1802u32);
pub const TMT_FROMHUE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1803u32);
pub const TMT_FROMHUE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1804u32);
pub const TMT_FROMHUE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1805u32);
pub const TMT_GLOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3816u32);
pub const TMT_GLOWINTENSITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2429u32);
pub const TMT_GLYPHDIBDATA: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8u32);
pub const TMT_GLYPHFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2601u32);
pub const TMT_GLYPHFONTSIZINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4014u32);
pub const TMT_GLYPHIMAGEFILE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3008u32);
pub const TMT_GLYPHINDEX: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2418u32);
pub const TMT_GLYPHONLY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2207u32);
pub const TMT_GLYPHTEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3819u32);
pub const TMT_GLYPHTRANSPARENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2206u32);
pub const TMT_GLYPHTRANSPARENTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3820u32);
pub const TMT_GLYPHTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4012u32);
pub const TMT_GRADIENTACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1628u32);
pub const TMT_GRADIENTCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3810u32);
pub const TMT_GRADIENTCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3811u32);
pub const TMT_GRADIENTCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3812u32);
pub const TMT_GRADIENTCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3813u32);
pub const TMT_GRADIENTCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3814u32);
pub const TMT_GRADIENTINACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1629u32);
pub const TMT_GRADIENTRATIO1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2406u32);
pub const TMT_GRADIENTRATIO2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2407u32);
pub const TMT_GRADIENTRATIO3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2408u32);
pub const TMT_GRADIENTRATIO4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2409u32);
pub const TMT_GRADIENTRATIO5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2410u32);
pub const TMT_GRAYTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1618u32);
pub const TMT_HALIGN: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4005u32);
pub const TMT_HBITMAP: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(212u32);
pub const TMT_HEADING1FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(807u32);
pub const TMT_HEADING1TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3825u32);
pub const TMT_HEADING2FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(808u32);
pub const TMT_HEADING2TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3826u32);
pub const TMT_HEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2417u32);
pub const TMT_HIGHLIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1614u32);
pub const TMT_HIGHLIGHTTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1615u32);
pub const TMT_HOTTRACKING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1627u32);
pub const TMT_ICONEFFECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4009u32);
pub const TMT_ICONTITLEFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(806u32);
pub const TMT_IMAGECOUNT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2401u32);
pub const TMT_IMAGEFILE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3001u32);
pub const TMT_IMAGEFILE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3002u32);
pub const TMT_IMAGEFILE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3003u32);
pub const TMT_IMAGEFILE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3004u32);
pub const TMT_IMAGEFILE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3005u32);
pub const TMT_IMAGEFILE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3006u32);
pub const TMT_IMAGEFILE6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3009u32);
pub const TMT_IMAGEFILE7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3010u32);
pub const TMT_IMAGELAYOUT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4011u32);
pub const TMT_IMAGESELECTTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4013u32);
pub const TMT_INACTIVEBORDER: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1612u32);
pub const TMT_INACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1604u32);
pub const TMT_INACTIVECAPTIONTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1620u32);
pub const TMT_INFOBK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1625u32);
pub const TMT_INFOTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1624u32);
pub const TMT_INT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(202u32);
pub const TMT_INTEGRALSIZING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2211u32);
pub const TMT_INTLIST: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(211u32);
pub const TMT_LASTBOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_LASTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1631u32);
pub const TMT_LASTFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(809u32);
pub const TMT_LASTINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_LASTSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1210u32);
pub const TMT_LASTSTRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1404u32);
pub const TMT_LASTUPDATED: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1403u32);
pub const TMT_LAST_RCSTRING_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(608u32);
pub const TMT_LIGHT3D: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1623u32);
pub const TMT_LOCALIZEDMIRRORIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2220u32);
pub const TMT_MARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(205u32);
pub const TMT_MENU: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1605u32);
pub const TMT_MENUBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1631u32);
pub const TMT_MENUBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1209u32);
pub const TMT_MENUBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1208u32);
pub const TMT_MENUFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(803u32);
pub const TMT_MENUHILIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1630u32);
pub const TMT_MENUTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1608u32);
pub const TMT_MINCOLORDEPTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_MINDPI1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2420u32);
pub const TMT_MINDPI2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2421u32);
pub const TMT_MINDPI3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2422u32);
pub const TMT_MINDPI4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2423u32);
pub const TMT_MINDPI5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2424u32);
pub const TMT_MINDPI6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2433u32);
pub const TMT_MINDPI7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2434u32);
pub const TMT_MINSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3403u32);
pub const TMT_MINSIZE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3404u32);
pub const TMT_MINSIZE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3405u32);
pub const TMT_MINSIZE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3406u32);
pub const TMT_MINSIZE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3407u32);
pub const TMT_MINSIZE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3408u32);
pub const TMT_MINSIZE6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3410u32);
pub const TMT_MINSIZE7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3411u32);
pub const TMT_MIRRORIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2209u32);
pub const TMT_MSGBOXFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(805u32);
pub const TMT_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(600u32);
pub const TMT_NOETCHEDEFFECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2215u32);
pub const TMT_NORMALSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3409u32);
pub const TMT_OFFSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3401u32);
pub const TMT_OFFSETTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4008u32);
pub const TMT_OPACITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2430u32);
pub const TMT_PADDEDBORDERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1210u32);
pub const TMT_PIXELSPERFRAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2427u32);
pub const TMT_POSITION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(208u32);
pub const TMT_PROGRESSCHUNKSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2411u32);
pub const TMT_PROGRESSSPACESIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2412u32);
pub const TMT_RECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(209u32);
pub const TMT_RESERVEDHIGH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(7999u32);
pub const TMT_RESERVEDLOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(0u32);
pub const TMT_ROUNDCORNERHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2405u32);
pub const TMT_ROUNDCORNERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2404u32);
pub const TMT_SATURATION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2413u32);
pub const TMT_SCALEDBACKGROUND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(7001u32);
pub const TMT_SCROLLBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1601u32);
pub const TMT_SCROLLBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1203u32);
pub const TMT_SCROLLBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1202u32);
pub const TMT_SHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3815u32);
pub const TMT_SIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(207u32);
pub const TMT_SIZES: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(402u32);
pub const TMT_SIZINGBORDERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1201u32);
pub const TMT_SIZINGMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3601u32);
pub const TMT_SIZINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4004u32);
pub const TMT_SMALLCAPTIONFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(802u32);
pub const TMT_SMCAPTIONBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1207u32);
pub const TMT_SMCAPTIONBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1206u32);
pub const TMT_SOURCEGROW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2212u32);
pub const TMT_SOURCESHRINK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2213u32);
pub const TMT_STATUSFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(804u32);
pub const TMT_STREAM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(214u32);
pub const TMT_STRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(201u32);
pub const TMT_TEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3201u32);
pub const TMT_TEXTAPPLYOVERLAY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2216u32);
pub const TMT_TEXTBORDERCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3817u32);
pub const TMT_TEXTBORDERSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2414u32);
pub const TMT_TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3803u32);
pub const TMT_TEXTCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3824u32);
pub const TMT_TEXTGLOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2217u32);
pub const TMT_TEXTGLOWSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2425u32);
pub const TMT_TEXTITALIC: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2218u32);
pub const TMT_TEXTSHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3818u32);
pub const TMT_TEXTSHADOWOFFSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3402u32);
pub const TMT_TEXTSHADOWTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4010u32);
pub const TMT_TOCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2006u32);
pub const TMT_TOCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2007u32);
pub const TMT_TOCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2008u32);
pub const TMT_TOCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2009u32);
pub const TMT_TOCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2010u32);
pub const TMT_TOHUE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1806u32);
pub const TMT_TOHUE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1807u32);
pub const TMT_TOHUE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1808u32);
pub const TMT_TOHUE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1809u32);
pub const TMT_TOHUE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1810u32);
pub const TMT_TOOLTIP: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(602u32);
pub const TMT_TRANSITIONDURATIONS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(6000u32);
pub const TMT_TRANSPARENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2201u32);
pub const TMT_TRANSPARENTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3809u32);
pub const TMT_TRUESIZESCALINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4015u32);
pub const TMT_TRUESIZESTRETCHMARK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2419u32);
pub const TMT_UNIFORMSIZING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2210u32);
pub const TMT_URL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(606u32);
pub const TMT_USERPICTURE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5001u32);
pub const TMT_VALIGN: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4007u32);
pub const TMT_VERSION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(607u32);
pub const TMT_WIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2416u32);
pub const TMT_WINDOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1606u32);
pub const TMT_WINDOWFRAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1607u32);
pub const TMT_WINDOWTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1609u32);
pub const TMT_XMLNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1402u32);
pub const TNP_ANIMBACKGROUND: TRAYNOTIFYPARTS = TRAYNOTIFYPARTS(2i32);
pub const TNP_BACKGROUND: TRAYNOTIFYPARTS = TRAYNOTIFYPARTS(1i32);
pub const TOOLBARCLASSNAME: windows_core::PCWSTR = windows_core::w!("ToolbarWindow32");
pub const TOOLBARCLASSNAMEA: windows_core::PCSTR = windows_core::s!("ToolbarWindow32");
pub const TOOLBARCLASSNAMEW: windows_core::PCWSTR = windows_core::w!("ToolbarWindow32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOOLBARPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOOLBARSTYLESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOOLTIPPARTS(pub i32);
pub const TOOLTIPS_CLASS: windows_core::PCWSTR = windows_core::w!("tooltips_class32");
pub const TOOLTIPS_CLASSA: windows_core::PCSTR = windows_core::s!("tooltips_class32");
pub const TOOLTIPS_CLASSW: windows_core::PCWSTR = windows_core::w!("tooltips_class32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOOLTIP_FLAGS(pub u32);
impl TOOLTIP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TOOLTIP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TOOLTIP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TOOLTIP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TOOLTIP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TOOLTIP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOPTABITEMBOTHEDGESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOPTABITEMLEFTEDGESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOPTABITEMRIGHTEDGESTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOPTABITEMSTATES(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOUCH_HIT_TESTING_INPUT {
    pub pointerId: u32,
    pub point: super::super::Foundation::POINT,
    pub boundingBox: super::super::Foundation::RECT,
    pub nonOccludedBoundingBox: super::super::Foundation::RECT,
    pub orientation: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    pub score: u16,
    pub adjustedPoint: super::super::Foundation::POINT,
}
pub const TP_BUTTON: TOOLBARPARTS = TOOLBARPARTS(1i32);
pub const TP_DROPDOWNBUTTON: TOOLBARPARTS = TOOLBARPARTS(2i32);
pub const TP_DROPDOWNBUTTONGLYPH: TOOLBARPARTS = TOOLBARPARTS(7i32);
pub const TP_SEPARATOR: TOOLBARPARTS = TOOLBARPARTS(5i32);
pub const TP_SEPARATORVERT: TOOLBARPARTS = TOOLBARPARTS(6i32);
pub const TP_SPLITBUTTON: TOOLBARPARTS = TOOLBARPARTS(3i32);
pub const TP_SPLITBUTTONDROPDOWN: TOOLBARPARTS = TOOLBARPARTS(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACKBARPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACKBARSTYLESTATES(pub i32);
pub const TRACKBAR_CLASS: windows_core::PCWSTR = windows_core::w!("msctls_trackbar32");
pub const TRACKBAR_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_trackbar32");
pub const TRACKBAR_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_trackbar32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACKSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACKVERTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRAILINGGRIDCELLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRAILINGGRIDCELLUPPERSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSPARENTBACKGROUNDSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSPARENTBARSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSPARENTBARVERTSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRAYNOTIFYPARTS(pub i32);
pub const TRBN_FIRST: u32 = 4294965795u32;
pub const TRBN_LAST: u32 = 4294965777u32;
pub const TRBN_THUMBPOSCHANGING: u32 = 4294965794u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TREEITEMSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TREEVIEWPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TREE_VIEW_ITEM_STATE_FLAGS(pub u32);
pub const TREIS_DISABLED: TREEITEMSTATES = TREEITEMSTATES(4i32);
pub const TREIS_HOT: TREEITEMSTATES = TREEITEMSTATES(2i32);
pub const TREIS_HOTSELECTED: TREEITEMSTATES = TREEITEMSTATES(6i32);
pub const TREIS_NORMAL: TREEITEMSTATES = TREEITEMSTATES(1i32);
pub const TREIS_SELECTED: TREEITEMSTATES = TREEITEMSTATES(3i32);
pub const TREIS_SELECTEDNOTFOCUS: TREEITEMSTATES = TREEITEMSTATES(5i32);
pub const TRS_NORMAL: TRACKSTATES = TRACKSTATES(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRUESIZESCALINGTYPE(pub i32);
pub const TRVS_NORMAL: TRACKVERTSTATES = TRACKVERTSTATES(1i32);
pub const TSGP_GRIPPER: TEXTSELECTIONGRIPPERPARTS = TEXTSELECTIONGRIPPERPARTS(1i32);
pub const TSGS_CENTERED: GRIPPERSTATES = GRIPPERSTATES(2i32);
pub const TSGS_NORMAL: GRIPPERSTATES = GRIPPERSTATES(1i32);
pub const TSST_DPI: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(2i32);
pub const TSST_NONE: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(0i32);
pub const TSST_SIZE: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(1i32);
pub const TSS_NORMAL: TICSSTATES = TICSSTATES(1i32);
pub const TST_CONTINUOUS: TEXTSHADOWTYPE = TEXTSHADOWTYPE(2i32);
pub const TST_NONE: TEXTSHADOWTYPE = TEXTSHADOWTYPE(0i32);
pub const TST_SINGLE: TEXTSHADOWTYPE = TEXTSHADOWTYPE(1i32);
pub const TSVS_NORMAL: TICSVERTSTATES = TICSVERTSTATES(1i32);
pub const TS_CHECKED: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(5i32);
pub const TS_CONTROLLABEL_DISABLED: CONTROLLABELSTATES = CONTROLLABELSTATES(2i32);
pub const TS_CONTROLLABEL_NORMAL: CONTROLLABELSTATES = CONTROLLABELSTATES(1i32);
pub const TS_DISABLED: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(4i32);
pub const TS_DRAW: THEMESIZE = THEMESIZE(2i32);
pub const TS_HOT: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(2i32);
pub const TS_HOTCHECKED: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(6i32);
pub const TS_HYPERLINK_DISABLED: HYPERLINKTEXTSTATES = HYPERLINKTEXTSTATES(4i32);
pub const TS_HYPERLINK_HOT: HYPERLINKTEXTSTATES = HYPERLINKTEXTSTATES(2i32);
pub const TS_HYPERLINK_NORMAL: HYPERLINKTEXTSTATES = HYPERLINKTEXTSTATES(1i32);
pub const TS_HYPERLINK_PRESSED: HYPERLINKTEXTSTATES = HYPERLINKTEXTSTATES(3i32);
pub const TS_MIN: THEMESIZE = THEMESIZE(0i32);
pub const TS_NEARHOT: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(7i32);
pub const TS_NORMAL: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(1i32);
pub const TS_OTHERSIDEHOT: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(8i32);
pub const TS_PRESSED: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(3i32);
pub const TS_TRUE: THEMESIZE = THEMESIZE(1i32);
pub const TTBSS_POINTINGDOWNCENTERED: BALLOONSTEMSTATES = BALLOONSTEMSTATES(5i32);
pub const TTBSS_POINTINGDOWNLEFTWALL: BALLOONSTEMSTATES = BALLOONSTEMSTATES(6i32);
pub const TTBSS_POINTINGDOWNRIGHTWALL: BALLOONSTEMSTATES = BALLOONSTEMSTATES(4i32);
pub const TTBSS_POINTINGUPCENTERED: BALLOONSTEMSTATES = BALLOONSTEMSTATES(2i32);
pub const TTBSS_POINTINGUPLEFTWALL: BALLOONSTEMSTATES = BALLOONSTEMSTATES(1i32);
pub const TTBSS_POINTINGUPRIGHTWALL: BALLOONSTEMSTATES = BALLOONSTEMSTATES(3i32);
pub const TTBS_LINK: BALLOONSTATES = BALLOONSTATES(2i32);
pub const TTBS_NORMAL: BALLOONSTATES = BALLOONSTATES(1i32);
pub const TTCS_HOT: CLOSESTATES = CLOSESTATES(2i32);
pub const TTCS_NORMAL: CLOSESTATES = CLOSESTATES(1i32);
pub const TTCS_PRESSED: CLOSESTATES = CLOSESTATES(3i32);
pub const TTDT_AUTOMATIC: u32 = 0u32;
pub const TTDT_AUTOPOP: u32 = 2u32;
pub const TTDT_INITIAL: u32 = 3u32;
pub const TTDT_RESHOW: u32 = 1u32;
pub const TTFT_CUBIC_BEZIER: TA_TIMINGFUNCTION_TYPE = TA_TIMINGFUNCTION_TYPE(1i32);
pub const TTFT_UNDEFINED: TA_TIMINGFUNCTION_TYPE = TA_TIMINGFUNCTION_TYPE(0i32);
pub const TTF_ABSOLUTE: TOOLTIP_FLAGS = TOOLTIP_FLAGS(128u32);
pub const TTF_CENTERTIP: TOOLTIP_FLAGS = TOOLTIP_FLAGS(2u32);
pub const TTF_DI_SETITEM: TOOLTIP_FLAGS = TOOLTIP_FLAGS(32768u32);
pub const TTF_IDISHWND: TOOLTIP_FLAGS = TOOLTIP_FLAGS(1u32);
pub const TTF_PARSELINKS: TOOLTIP_FLAGS = TOOLTIP_FLAGS(4096u32);
pub const TTF_RTLREADING: TOOLTIP_FLAGS = TOOLTIP_FLAGS(4u32);
pub const TTF_SUBCLASS: TOOLTIP_FLAGS = TOOLTIP_FLAGS(16u32);
pub const TTF_TRACK: TOOLTIP_FLAGS = TOOLTIP_FLAGS(32u32);
pub const TTF_TRANSPARENT: TOOLTIP_FLAGS = TOOLTIP_FLAGS(256u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TTGETTITLE {
    pub dwSize: u32,
    pub uTitleBitmap: u32,
    pub cch: u32,
    pub pszTitle: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TTHITTESTINFOA {
    pub hwnd: super::super::Foundation::HWND,
    pub pt: super::super::Foundation::POINT,
    pub ti: TTTOOLINFOA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TTHITTESTINFOW {
    pub hwnd: super::super::Foundation::HWND,
    pub pt: super::super::Foundation::POINT,
    pub ti: TTTOOLINFOW,
}
pub const TTIBES_DISABLED: TOPTABITEMBOTHEDGESTATES = TOPTABITEMBOTHEDGESTATES(4i32);
pub const TTIBES_FOCUSED: TOPTABITEMBOTHEDGESTATES = TOPTABITEMBOTHEDGESTATES(5i32);
pub const TTIBES_HOT: TOPTABITEMBOTHEDGESTATES = TOPTABITEMBOTHEDGESTATES(2i32);
pub const TTIBES_NORMAL: TOPTABITEMBOTHEDGESTATES = TOPTABITEMBOTHEDGESTATES(1i32);
pub const TTIBES_SELECTED: TOPTABITEMBOTHEDGESTATES = TOPTABITEMBOTHEDGESTATES(3i32);
pub const TTILES_DISABLED: TOPTABITEMLEFTEDGESTATES = TOPTABITEMLEFTEDGESTATES(4i32);
pub const TTILES_FOCUSED: TOPTABITEMLEFTEDGESTATES = TOPTABITEMLEFTEDGESTATES(5i32);
pub const TTILES_HOT: TOPTABITEMLEFTEDGESTATES = TOPTABITEMLEFTEDGESTATES(2i32);
pub const TTILES_NORMAL: TOPTABITEMLEFTEDGESTATES = TOPTABITEMLEFTEDGESTATES(1i32);
pub const TTILES_SELECTED: TOPTABITEMLEFTEDGESTATES = TOPTABITEMLEFTEDGESTATES(3i32);
pub const TTIRES_DISABLED: TOPTABITEMRIGHTEDGESTATES = TOPTABITEMRIGHTEDGESTATES(4i32);
pub const TTIRES_FOCUSED: TOPTABITEMRIGHTEDGESTATES = TOPTABITEMRIGHTEDGESTATES(5i32);
pub const TTIRES_HOT: TOPTABITEMRIGHTEDGESTATES = TOPTABITEMRIGHTEDGESTATES(2i32);
pub const TTIRES_NORMAL: TOPTABITEMRIGHTEDGESTATES = TOPTABITEMRIGHTEDGESTATES(1i32);
pub const TTIRES_SELECTED: TOPTABITEMRIGHTEDGESTATES = TOPTABITEMRIGHTEDGESTATES(3i32);
pub const TTIS_DISABLED: TOPTABITEMSTATES = TOPTABITEMSTATES(4i32);
pub const TTIS_FOCUSED: TOPTABITEMSTATES = TOPTABITEMSTATES(5i32);
pub const TTIS_HOT: TOPTABITEMSTATES = TOPTABITEMSTATES(2i32);
pub const TTIS_NORMAL: TOPTABITEMSTATES = TOPTABITEMSTATES(1i32);
pub const TTIS_SELECTED: TOPTABITEMSTATES = TOPTABITEMSTATES(3i32);
pub const TTI_ERROR: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(3i32);
pub const TTI_ERROR_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(6i32);
pub const TTI_INFO: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(1i32);
pub const TTI_INFO_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(4i32);
pub const TTI_NONE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(0i32);
pub const TTI_WARNING: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(2i32);
pub const TTI_WARNING_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(5i32);
pub const TTM_ACTIVATE: u32 = 1025u32;
pub const TTM_ADDTOOL: u32 = 1074u32;
pub const TTM_ADDTOOLA: u32 = 1028u32;
pub const TTM_ADDTOOLW: u32 = 1074u32;
pub const TTM_ADJUSTRECT: u32 = 1055u32;
pub const TTM_DELTOOL: u32 = 1075u32;
pub const TTM_DELTOOLA: u32 = 1029u32;
pub const TTM_DELTOOLW: u32 = 1075u32;
pub const TTM_ENUMTOOLS: u32 = 1082u32;
pub const TTM_ENUMTOOLSA: u32 = 1038u32;
pub const TTM_ENUMTOOLSW: u32 = 1082u32;
pub const TTM_GETBUBBLESIZE: u32 = 1054u32;
pub const TTM_GETCURRENTTOOL: u32 = 1083u32;
pub const TTM_GETCURRENTTOOLA: u32 = 1039u32;
pub const TTM_GETCURRENTTOOLW: u32 = 1083u32;
pub const TTM_GETDELAYTIME: u32 = 1045u32;
pub const TTM_GETMARGIN: u32 = 1051u32;
pub const TTM_GETMAXTIPWIDTH: u32 = 1049u32;
pub const TTM_GETTEXT: u32 = 1080u32;
pub const TTM_GETTEXTA: u32 = 1035u32;
pub const TTM_GETTEXTW: u32 = 1080u32;
pub const TTM_GETTIPBKCOLOR: u32 = 1046u32;
pub const TTM_GETTIPTEXTCOLOR: u32 = 1047u32;
pub const TTM_GETTITLE: u32 = 1059u32;
pub const TTM_GETTOOLCOUNT: u32 = 1037u32;
pub const TTM_GETTOOLINFO: u32 = 1077u32;
pub const TTM_GETTOOLINFOA: u32 = 1032u32;
pub const TTM_GETTOOLINFOW: u32 = 1077u32;
pub const TTM_HITTEST: u32 = 1079u32;
pub const TTM_HITTESTA: u32 = 1034u32;
pub const TTM_HITTESTW: u32 = 1079u32;
pub const TTM_NEWTOOLRECT: u32 = 1076u32;
pub const TTM_NEWTOOLRECTA: u32 = 1030u32;
pub const TTM_NEWTOOLRECTW: u32 = 1076u32;
pub const TTM_POP: u32 = 1052u32;
pub const TTM_POPUP: u32 = 1058u32;
pub const TTM_RELAYEVENT: u32 = 1031u32;
pub const TTM_SETDELAYTIME: u32 = 1027u32;
pub const TTM_SETMARGIN: u32 = 1050u32;
pub const TTM_SETMAXTIPWIDTH: u32 = 1048u32;
pub const TTM_SETTIPBKCOLOR: u32 = 1043u32;
pub const TTM_SETTIPTEXTCOLOR: u32 = 1044u32;
pub const TTM_SETTITLE: u32 = 1057u32;
pub const TTM_SETTITLEA: u32 = 1056u32;
pub const TTM_SETTITLEW: u32 = 1057u32;
pub const TTM_SETTOOLINFO: u32 = 1078u32;
pub const TTM_SETTOOLINFOA: u32 = 1033u32;
pub const TTM_SETTOOLINFOW: u32 = 1078u32;
pub const TTM_SETWINDOWTHEME: u32 = 8203u32;
pub const TTM_TRACKACTIVATE: u32 = 1041u32;
pub const TTM_TRACKPOSITION: u32 = 1042u32;
pub const TTM_UPDATE: u32 = 1053u32;
pub const TTM_UPDATETIPTEXT: u32 = 1081u32;
pub const TTM_UPDATETIPTEXTA: u32 = 1036u32;
pub const TTM_UPDATETIPTEXTW: u32 = 1081u32;
pub const TTM_WINDOWFROMPOINT: u32 = 1040u32;
pub const TTN_FIRST: u32 = 4294966776u32;
pub const TTN_GETDISPINFO: u32 = 4294966766u32;
pub const TTN_GETDISPINFOA: u32 = 4294966776u32;
pub const TTN_GETDISPINFOW: u32 = 4294966766u32;
pub const TTN_LAST: u32 = 4294966747u32;
pub const TTN_LINKCLICK: u32 = 4294966773u32;
pub const TTN_NEEDTEXT: u32 = 4294966766u32;
pub const TTN_NEEDTEXTA: u32 = 4294966776u32;
pub const TTN_NEEDTEXTW: u32 = 4294966766u32;
pub const TTN_POP: u32 = 4294966774u32;
pub const TTN_SHOW: u32 = 4294966775u32;
pub const TTP_BALLOON: TOOLTIPPARTS = TOOLTIPPARTS(3i32);
pub const TTP_BALLOONSTEM: TOOLTIPPARTS = TOOLTIPPARTS(6i32);
pub const TTP_BALLOONTITLE: TOOLTIPPARTS = TOOLTIPPARTS(4i32);
pub const TTP_CLOSE: TOOLTIPPARTS = TOOLTIPPARTS(5i32);
pub const TTP_STANDARD: TOOLTIPPARTS = TOOLTIPPARTS(1i32);
pub const TTP_STANDARDTITLE: TOOLTIPPARTS = TOOLTIPPARTS(2i32);
pub const TTP_WRENCH: TOOLTIPPARTS = TOOLTIPPARTS(7i32);
pub const TTSS_LINK: STANDARDSTATES = STANDARDSTATES(2i32);
pub const TTSS_NORMAL: STANDARDSTATES = STANDARDSTATES(1i32);
pub const TTS_ALWAYSTIP: u32 = 1u32;
pub const TTS_BALLOON: u32 = 64u32;
pub const TTS_CLOSE: u32 = 128u32;
pub const TTS_NOANIMATE: u32 = 16u32;
pub const TTS_NOFADE: u32 = 32u32;
pub const TTS_NOPREFIX: u32 = 2u32;
pub const TTS_USEVISUALSTYLE: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TTTOOLINFOA {
    pub cbSize: u32,
    pub uFlags: TOOLTIP_FLAGS,
    pub hwnd: super::super::Foundation::HWND,
    pub uId: usize,
    pub rect: super::super::Foundation::RECT,
    pub hinst: super::super::Foundation::HINSTANCE,
    pub lpszText: windows_core::PSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpReserved: *mut core::ffi::c_void,
}
impl Default for TTTOOLINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TTTOOLINFOW {
    pub cbSize: u32,
    pub uFlags: TOOLTIP_FLAGS,
    pub hwnd: super::super::Foundation::HWND,
    pub uId: usize,
    pub rect: super::super::Foundation::RECT,
    pub hinst: super::super::Foundation::HINSTANCE,
    pub lpszText: windows_core::PWSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpReserved: *mut core::ffi::c_void,
}
impl Default for TTTOOLINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TTWS_HOT: WRENCHSTATES = WRENCHSTATES(2i32);
pub const TTWS_NORMAL: WRENCHSTATES = WRENCHSTATES(1i32);
pub const TTWS_PRESSED: WRENCHSTATES = WRENCHSTATES(3i32);
pub const TUBS_DISABLED: THUMBBOTTOMSTATES = THUMBBOTTOMSTATES(5i32);
pub const TUBS_FOCUSED: THUMBBOTTOMSTATES = THUMBBOTTOMSTATES(4i32);
pub const TUBS_HOT: THUMBBOTTOMSTATES = THUMBBOTTOMSTATES(2i32);
pub const TUBS_NORMAL: THUMBBOTTOMSTATES = THUMBBOTTOMSTATES(1i32);
pub const TUBS_PRESSED: THUMBBOTTOMSTATES = THUMBBOTTOMSTATES(3i32);
pub const TUS_DISABLED: THUMBSTATES = THUMBSTATES(5i32);
pub const TUS_FOCUSED: THUMBSTATES = THUMBSTATES(4i32);
pub const TUS_HOT: THUMBSTATES = THUMBSTATES(2i32);
pub const TUS_NORMAL: THUMBSTATES = THUMBSTATES(1i32);
pub const TUS_PRESSED: THUMBSTATES = THUMBSTATES(3i32);
pub const TUTS_DISABLED: THUMBTOPSTATES = THUMBTOPSTATES(5i32);
pub const TUTS_FOCUSED: THUMBTOPSTATES = THUMBTOPSTATES(4i32);
pub const TUTS_HOT: THUMBTOPSTATES = THUMBTOPSTATES(2i32);
pub const TUTS_NORMAL: THUMBTOPSTATES = THUMBTOPSTATES(1i32);
pub const TUTS_PRESSED: THUMBTOPSTATES = THUMBTOPSTATES(3i32);
pub const TUVLS_DISABLED: THUMBLEFTSTATES = THUMBLEFTSTATES(5i32);
pub const TUVLS_FOCUSED: THUMBLEFTSTATES = THUMBLEFTSTATES(4i32);
pub const TUVLS_HOT: THUMBLEFTSTATES = THUMBLEFTSTATES(2i32);
pub const TUVLS_NORMAL: THUMBLEFTSTATES = THUMBLEFTSTATES(1i32);
pub const TUVLS_PRESSED: THUMBLEFTSTATES = THUMBLEFTSTATES(3i32);
pub const TUVRS_DISABLED: THUMBRIGHTSTATES = THUMBRIGHTSTATES(5i32);
pub const TUVRS_FOCUSED: THUMBRIGHTSTATES = THUMBRIGHTSTATES(4i32);
pub const TUVRS_HOT: THUMBRIGHTSTATES = THUMBRIGHTSTATES(2i32);
pub const TUVRS_NORMAL: THUMBRIGHTSTATES = THUMBRIGHTSTATES(1i32);
pub const TUVRS_PRESSED: THUMBRIGHTSTATES = THUMBRIGHTSTATES(3i32);
pub const TUVS_DISABLED: THUMBVERTSTATES = THUMBVERTSTATES(5i32);
pub const TUVS_FOCUSED: THUMBVERTSTATES = THUMBVERTSTATES(4i32);
pub const TUVS_HOT: THUMBVERTSTATES = THUMBVERTSTATES(2i32);
pub const TUVS_NORMAL: THUMBVERTSTATES = THUMBVERTSTATES(1i32);
pub const TUVS_PRESSED: THUMBVERTSTATES = THUMBVERTSTATES(3i32);
pub const TVCDRF_NOIMAGES: u32 = 65536u32;
pub const TVC_BYKEYBOARD: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(2u32);
pub const TVC_BYMOUSE: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(1u32);
pub const TVC_UNKNOWN: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(0u32);
pub const TVE_COLLAPSE: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(1u32);
pub const TVE_COLLAPSERESET: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(32768u32);
pub const TVE_EXPAND: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(2u32);
pub const TVE_EXPANDPARTIAL: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(16384u32);
pub const TVE_TOGGLE: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(3u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TVGETITEMPARTRECTINFO {
    pub hti: HTREEITEM,
    pub prc: *mut super::super::Foundation::RECT,
    pub partID: TVITEMPART,
}
impl Default for TVGETITEMPARTRECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TVGIPR_BUTTON: TVITEMPART = TVITEMPART(1i32);
pub const TVGN_CARET: u32 = 9u32;
pub const TVGN_CHILD: u32 = 4u32;
pub const TVGN_DROPHILITE: u32 = 8u32;
pub const TVGN_FIRSTVISIBLE: u32 = 5u32;
pub const TVGN_LASTVISIBLE: u32 = 10u32;
pub const TVGN_NEXT: u32 = 1u32;
pub const TVGN_NEXTSELECTED: u32 = 11u32;
pub const TVGN_NEXTVISIBLE: u32 = 6u32;
pub const TVGN_PARENT: u32 = 3u32;
pub const TVGN_PREVIOUS: u32 = 2u32;
pub const TVGN_PREVIOUSVISIBLE: u32 = 7u32;
pub const TVGN_ROOT: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TVHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: TVHITTESTINFO_FLAGS,
    pub hItem: HTREEITEM,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TVHITTESTINFO_FLAGS(pub u32);
impl TVHITTESTINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TVHITTESTINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TVHITTESTINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TVHT_ABOVE: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(256u32);
pub const TVHT_BELOW: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(512u32);
pub const TVHT_NOWHERE: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(1u32);
pub const TVHT_ONITEM: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(70u32);
pub const TVHT_ONITEMBUTTON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(16u32);
pub const TVHT_ONITEMICON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(2u32);
pub const TVHT_ONITEMINDENT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(8u32);
pub const TVHT_ONITEMLABEL: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(4u32);
pub const TVHT_ONITEMRIGHT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(32u32);
pub const TVHT_ONITEMSTATEICON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(64u32);
pub const TVHT_TOLEFT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(2048u32);
pub const TVHT_TORIGHT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(1024u32);
pub const TVIF_CHILDREN: TVITEM_MASK = TVITEM_MASK(64u32);
pub const TVIF_DI_SETITEM: TVITEM_MASK = TVITEM_MASK(4096u32);
pub const TVIF_EXPANDEDIMAGE: TVITEM_MASK = TVITEM_MASK(512u32);
pub const TVIF_HANDLE: TVITEM_MASK = TVITEM_MASK(16u32);
pub const TVIF_IMAGE: TVITEM_MASK = TVITEM_MASK(2u32);
pub const TVIF_INTEGRAL: TVITEM_MASK = TVITEM_MASK(128u32);
pub const TVIF_PARAM: TVITEM_MASK = TVITEM_MASK(4u32);
pub const TVIF_SELECTEDIMAGE: TVITEM_MASK = TVITEM_MASK(32u32);
pub const TVIF_STATE: TVITEM_MASK = TVITEM_MASK(8u32);
pub const TVIF_STATEEX: TVITEM_MASK = TVITEM_MASK(256u32);
pub const TVIF_TEXT: TVITEM_MASK = TVITEM_MASK(1u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TVINSERTSTRUCTA {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub Anonymous: TVINSERTSTRUCTA_0,
}
impl Default for TVINSERTSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TVINSERTSTRUCTA_0 {
    pub itemex: TVITEMEXA,
    pub item: TVITEMA,
}
impl Default for TVINSERTSTRUCTA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TVINSERTSTRUCTW {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub Anonymous: TVINSERTSTRUCTW_0,
}
impl Default for TVINSERTSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TVINSERTSTRUCTW_0 {
    pub itemex: TVITEMEXW,
    pub item: TVITEMW,
}
impl Default for TVINSERTSTRUCTW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TVIS_BOLD: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(16u32);
pub const TVIS_CUT: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(4u32);
pub const TVIS_DROPHILITED: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(8u32);
pub const TVIS_EXPANDED: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(32u32);
pub const TVIS_EXPANDEDONCE: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(64u32);
pub const TVIS_EXPANDPARTIAL: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(128u32);
pub const TVIS_EX_ALL: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(2u32);
pub const TVIS_EX_DISABLED: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(2u32);
pub const TVIS_EX_FLAT: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(1u32);
pub const TVIS_OVERLAYMASK: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(3840u32);
pub const TVIS_SELECTED: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(2u32);
pub const TVIS_STATEIMAGEMASK: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(61440u32);
pub const TVIS_USERMASK: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(61440u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TVITEMA {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: TREE_VIEW_ITEM_STATE_FLAGS,
    pub stateMask: TREE_VIEW_ITEM_STATE_FLAGS,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TVITEMEXA {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TVITEMEXW {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TVITEMEXW_CHILDREN(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TVITEMPART(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TVITEMW {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: TREE_VIEW_ITEM_STATE_FLAGS,
    pub stateMask: TREE_VIEW_ITEM_STATE_FLAGS,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TVITEM_MASK(pub u32);
impl TVITEM_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TVITEM_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TVITEM_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TVITEM_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TVITEM_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TVITEM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TVI_FIRST: HTREEITEM = HTREEITEM(-65535i32 as _);
pub const TVI_LAST: HTREEITEM = HTREEITEM(-65534i32 as _);
pub const TVI_ROOT: HTREEITEM = HTREEITEM(-65536i32 as _);
pub const TVI_SORT: HTREEITEM = HTREEITEM(-65533i32 as _);
pub const TVM_CREATEDRAGIMAGE: u32 = 4370u32;
pub const TVM_DELETEITEM: u32 = 4353u32;
pub const TVM_EDITLABEL: u32 = 4417u32;
pub const TVM_EDITLABELA: u32 = 4366u32;
pub const TVM_EDITLABELW: u32 = 4417u32;
pub const TVM_ENDEDITLABELNOW: u32 = 4374u32;
pub const TVM_ENSUREVISIBLE: u32 = 4372u32;
pub const TVM_EXPAND: u32 = 4354u32;
pub const TVM_GETBKCOLOR: u32 = 4383u32;
pub const TVM_GETCOUNT: u32 = 4357u32;
pub const TVM_GETEDITCONTROL: u32 = 4367u32;
pub const TVM_GETEXTENDEDSTYLE: u32 = 4397u32;
pub const TVM_GETIMAGELIST: u32 = 4360u32;
pub const TVM_GETINDENT: u32 = 4358u32;
pub const TVM_GETINSERTMARKCOLOR: u32 = 4390u32;
pub const TVM_GETISEARCHSTRING: u32 = 4416u32;
pub const TVM_GETISEARCHSTRINGA: u32 = 4375u32;
pub const TVM_GETISEARCHSTRINGW: u32 = 4416u32;
pub const TVM_GETITEM: u32 = 4414u32;
pub const TVM_GETITEMA: u32 = 4364u32;
pub const TVM_GETITEMHEIGHT: u32 = 4380u32;
pub const TVM_GETITEMPARTRECT: u32 = 4424u32;
pub const TVM_GETITEMRECT: u32 = 4356u32;
pub const TVM_GETITEMSTATE: u32 = 4391u32;
pub const TVM_GETITEMW: u32 = 4414u32;
pub const TVM_GETLINECOLOR: u32 = 4393u32;
pub const TVM_GETNEXTITEM: u32 = 4362u32;
pub const TVM_GETSCROLLTIME: u32 = 4386u32;
pub const TVM_GETSELECTEDCOUNT: u32 = 4422u32;
pub const TVM_GETTEXTCOLOR: u32 = 4384u32;
pub const TVM_GETTOOLTIPS: u32 = 4377u32;
pub const TVM_GETUNICODEFORMAT: u32 = 8198u32;
pub const TVM_GETVISIBLECOUNT: u32 = 4368u32;
pub const TVM_HITTEST: u32 = 4369u32;
pub const TVM_INSERTITEM: u32 = 4402u32;
pub const TVM_INSERTITEMA: u32 = 4352u32;
pub const TVM_INSERTITEMW: u32 = 4402u32;
pub const TVM_MAPACCIDTOHTREEITEM: u32 = 4394u32;
pub const TVM_MAPHTREEITEMTOACCID: u32 = 4395u32;
pub const TVM_SELECTITEM: u32 = 4363u32;
pub const TVM_SETAUTOSCROLLINFO: u32 = 4411u32;
pub const TVM_SETBKCOLOR: u32 = 4381u32;
pub const TVM_SETBORDER: u32 = 4387u32;
pub const TVM_SETEXTENDEDSTYLE: u32 = 4396u32;
pub const TVM_SETHOT: u32 = 4410u32;
pub const TVM_SETIMAGELIST: u32 = 4361u32;
pub const TVM_SETINDENT: u32 = 4359u32;
pub const TVM_SETINSERTMARK: u32 = 4378u32;
pub const TVM_SETINSERTMARKCOLOR: u32 = 4389u32;
pub const TVM_SETITEM: u32 = 4415u32;
pub const TVM_SETITEMA: u32 = 4365u32;
pub const TVM_SETITEMHEIGHT: u32 = 4379u32;
pub const TVM_SETITEMW: u32 = 4415u32;
pub const TVM_SETLINECOLOR: u32 = 4392u32;
pub const TVM_SETSCROLLTIME: u32 = 4385u32;
pub const TVM_SETTEXTCOLOR: u32 = 4382u32;
pub const TVM_SETTOOLTIPS: u32 = 4376u32;
pub const TVM_SETUNICODEFORMAT: u32 = 8197u32;
pub const TVM_SHOWINFOTIP: u32 = 4423u32;
pub const TVM_SORTCHILDREN: u32 = 4371u32;
pub const TVM_SORTCHILDRENCB: u32 = 4373u32;
pub const TVNRET_DEFAULT: u32 = 0u32;
pub const TVNRET_SKIPNEW: u32 = 2u32;
pub const TVNRET_SKIPOLD: u32 = 1u32;
pub const TVN_ASYNCDRAW: u32 = 4294966876u32;
pub const TVN_BEGINDRAG: u32 = 4294966840u32;
pub const TVN_BEGINDRAGA: u32 = 4294966889u32;
pub const TVN_BEGINDRAGW: u32 = 4294966840u32;
pub const TVN_BEGINLABELEDIT: u32 = 4294966837u32;
pub const TVN_BEGINLABELEDITA: u32 = 4294966886u32;
pub const TVN_BEGINLABELEDITW: u32 = 4294966837u32;
pub const TVN_BEGINRDRAG: u32 = 4294966839u32;
pub const TVN_BEGINRDRAGA: u32 = 4294966888u32;
pub const TVN_BEGINRDRAGW: u32 = 4294966839u32;
pub const TVN_DELETEITEM: u32 = 4294966838u32;
pub const TVN_DELETEITEMA: u32 = 4294966887u32;
pub const TVN_DELETEITEMW: u32 = 4294966838u32;
pub const TVN_ENDLABELEDIT: u32 = 4294966836u32;
pub const TVN_ENDLABELEDITA: u32 = 4294966885u32;
pub const TVN_ENDLABELEDITW: u32 = 4294966836u32;
pub const TVN_FIRST: u32 = 4294966896u32;
pub const TVN_GETDISPINFO: u32 = 4294966844u32;
pub const TVN_GETDISPINFOA: u32 = 4294966893u32;
pub const TVN_GETDISPINFOW: u32 = 4294966844u32;
pub const TVN_GETINFOTIP: u32 = 4294966882u32;
pub const TVN_GETINFOTIPA: u32 = 4294966883u32;
pub const TVN_GETINFOTIPW: u32 = 4294966882u32;
pub const TVN_ITEMCHANGED: u32 = 4294966877u32;
pub const TVN_ITEMCHANGEDA: u32 = 4294966878u32;
pub const TVN_ITEMCHANGEDW: u32 = 4294966877u32;
pub const TVN_ITEMCHANGING: u32 = 4294966879u32;
pub const TVN_ITEMCHANGINGA: u32 = 4294966880u32;
pub const TVN_ITEMCHANGINGW: u32 = 4294966879u32;
pub const TVN_ITEMEXPANDED: u32 = 4294966841u32;
pub const TVN_ITEMEXPANDEDA: u32 = 4294966890u32;
pub const TVN_ITEMEXPANDEDW: u32 = 4294966841u32;
pub const TVN_ITEMEXPANDING: u32 = 4294966842u32;
pub const TVN_ITEMEXPANDINGA: u32 = 4294966891u32;
pub const TVN_ITEMEXPANDINGW: u32 = 4294966842u32;
pub const TVN_KEYDOWN: u32 = 4294966884u32;
pub const TVN_LAST: u32 = 4294966797u32;
pub const TVN_SELCHANGED: u32 = 4294966845u32;
pub const TVN_SELCHANGEDA: u32 = 4294966894u32;
pub const TVN_SELCHANGEDW: u32 = 4294966845u32;
pub const TVN_SELCHANGING: u32 = 4294966846u32;
pub const TVN_SELCHANGINGA: u32 = 4294966895u32;
pub const TVN_SELCHANGINGW: u32 = 4294966846u32;
pub const TVN_SETDISPINFO: u32 = 4294966843u32;
pub const TVN_SETDISPINFOA: u32 = 4294966892u32;
pub const TVN_SETDISPINFOW: u32 = 4294966843u32;
pub const TVN_SINGLEEXPAND: u32 = 4294966881u32;
pub const TVP_BRANCH: TREEVIEWPARTS = TREEVIEWPARTS(3i32);
pub const TVP_GLYPH: TREEVIEWPARTS = TREEVIEWPARTS(2i32);
pub const TVP_HOTGLYPH: TREEVIEWPARTS = TREEVIEWPARTS(4i32);
pub const TVP_TREEITEM: TREEVIEWPARTS = TREEVIEWPARTS(1i32);
pub const TVSBF_XBORDER: u32 = 1u32;
pub const TVSBF_YBORDER: u32 = 2u32;
pub const TVSIL_NORMAL: u32 = 0u32;
pub const TVSIL_STATE: u32 = 2u32;
pub const TVSI_NOSINGLEEXPAND: u32 = 32768u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TVSORTCB {
    pub hParent: HTREEITEM,
    pub lpfnCompare: PFNTVCOMPARE,
    pub lParam: super::super::Foundation::LPARAM,
}
pub const TVS_CHECKBOXES: u32 = 256u32;
pub const TVS_DISABLEDRAGDROP: u32 = 16u32;
pub const TVS_EDITLABELS: u32 = 8u32;
pub const TVS_EX_AUTOHSCROLL: u32 = 32u32;
pub const TVS_EX_DIMMEDCHECKBOXES: u32 = 512u32;
pub const TVS_EX_DOUBLEBUFFER: u32 = 4u32;
pub const TVS_EX_DRAWIMAGEASYNC: u32 = 1024u32;
pub const TVS_EX_EXCLUSIONCHECKBOXES: u32 = 256u32;
pub const TVS_EX_FADEINOUTEXPANDOS: u32 = 64u32;
pub const TVS_EX_MULTISELECT: u32 = 2u32;
pub const TVS_EX_NOINDENTSTATE: u32 = 8u32;
pub const TVS_EX_NOSINGLECOLLAPSE: u32 = 1u32;
pub const TVS_EX_PARTIALCHECKBOXES: u32 = 128u32;
pub const TVS_EX_RICHTOOLTIP: u32 = 16u32;
pub const TVS_FULLROWSELECT: u32 = 4096u32;
pub const TVS_HASBUTTONS: u32 = 1u32;
pub const TVS_HASLINES: u32 = 2u32;
pub const TVS_INFOTIP: u32 = 2048u32;
pub const TVS_LINESATROOT: u32 = 4u32;
pub const TVS_NOHSCROLL: u32 = 32768u32;
pub const TVS_NONEVENHEIGHT: u32 = 16384u32;
pub const TVS_NOSCROLL: u32 = 8192u32;
pub const TVS_NOTOOLTIPS: u32 = 128u32;
pub const TVS_RTLREADING: u32 = 64u32;
pub const TVS_SHOWSELALWAYS: u32 = 32u32;
pub const TVS_SINGLEEXPAND: u32 = 1024u32;
pub const TVS_TRACKSELECT: u32 = 512u32;
pub const TV_FIRST: u32 = 4352u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UDACCEL {
    pub nSec: u32,
    pub nInc: u32,
}
pub const UDM_GETACCEL: u32 = 1132u32;
pub const UDM_GETBASE: u32 = 1134u32;
pub const UDM_GETBUDDY: u32 = 1130u32;
pub const UDM_GETPOS: u32 = 1128u32;
pub const UDM_GETPOS32: u32 = 1138u32;
pub const UDM_GETRANGE: u32 = 1126u32;
pub const UDM_GETRANGE32: u32 = 1136u32;
pub const UDM_GETUNICODEFORMAT: u32 = 8198u32;
pub const UDM_SETACCEL: u32 = 1131u32;
pub const UDM_SETBASE: u32 = 1133u32;
pub const UDM_SETBUDDY: u32 = 1129u32;
pub const UDM_SETPOS: u32 = 1127u32;
pub const UDM_SETPOS32: u32 = 1137u32;
pub const UDM_SETRANGE: u32 = 1125u32;
pub const UDM_SETRANGE32: u32 = 1135u32;
pub const UDM_SETUNICODEFORMAT: u32 = 8197u32;
pub const UDN_DELTAPOS: u32 = 4294966574u32;
pub const UDN_FIRST: u32 = 4294966575u32;
pub const UDN_LAST: u32 = 4294966567u32;
pub const UDS_ALIGNLEFT: u32 = 8u32;
pub const UDS_ALIGNRIGHT: u32 = 4u32;
pub const UDS_ARROWKEYS: u32 = 32u32;
pub const UDS_AUTOBUDDY: u32 = 16u32;
pub const UDS_HORZ: u32 = 64u32;
pub const UDS_HOTTRACK: u32 = 256u32;
pub const UDS_NOTHOUSANDS: u32 = 128u32;
pub const UDS_SETBUDDYINT: u32 = 2u32;
pub const UDS_WRAP: u32 = 1u32;
pub const UD_MAXVAL: u32 = 32767u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UPDATEMETADATASTATES(pub i32);
pub const UPDOWN_CLASS: windows_core::PCWSTR = windows_core::w!("msctls_updown32");
pub const UPDOWN_CLASSA: windows_core::PCSTR = windows_core::s!("msctls_updown32");
pub const UPDOWN_CLASSW: windows_core::PCWSTR = windows_core::w!("msctls_updown32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UPHORZSTATES(pub i32);
pub const UPHZS_DISABLED: UPHORZSTATES = UPHORZSTATES(4i32);
pub const UPHZS_HOT: UPHORZSTATES = UPHORZSTATES(2i32);
pub const UPHZS_NORMAL: UPHORZSTATES = UPHORZSTATES(1i32);
pub const UPHZS_PRESSED: UPHORZSTATES = UPHORZSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UPSTATES(pub i32);
pub const UPS_DISABLED: UPSTATES = UPSTATES(4i32);
pub const UPS_HOT: UPSTATES = UPSTATES(2i32);
pub const UPS_NORMAL: UPSTATES = UPSTATES(1i32);
pub const UPS_PRESSED: UPSTATES = UPSTATES(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USAGE_PROPERTIES {
    pub level: u16,
    pub page: u16,
    pub usage: u16,
    pub logicalMinimum: i32,
    pub logicalMaximum: i32,
    pub unit: u16,
    pub exponent: u16,
    pub count: u8,
    pub physicalMinimum: i32,
    pub physicalMaximum: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USERTILEPARTS(pub i32);
pub const UTP_HOVERBACKGROUND: USERTILEPARTS = USERTILEPARTS(2i32);
pub const UTP_STROKEBACKGROUND: USERTILEPARTS = USERTILEPARTS(1i32);
pub const UTS_HOT: HOVERBACKGROUNDSTATES = HOVERBACKGROUNDSTATES(2i32);
pub const UTS_NORMAL: HOVERBACKGROUNDSTATES = HOVERBACKGROUNDSTATES(1i32);
pub const UTS_PRESSED: HOVERBACKGROUNDSTATES = HOVERBACKGROUNDSTATES(3i32);
pub const VALIDBITS: SET_THEME_APP_PROPERTIES_FLAGS = SET_THEME_APP_PROPERTIES_FLAGS(7u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VALIGN(pub i32);
pub const VA_BOTTOM: VALIGN = VALIGN(2i32);
pub const VA_CENTER: VALIGN = VALIGN(1i32);
pub const VA_TOP: VALIGN = VALIGN(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VERTSCROLLSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VERTTHUMBSTATES(pub i32);
pub const VIEW_DETAILS: u32 = 3u32;
pub const VIEW_LARGEICONS: u32 = 0u32;
pub const VIEW_LIST: u32 = 2u32;
pub const VIEW_NETCONNECT: u32 = 9u32;
pub const VIEW_NETDISCONNECT: u32 = 10u32;
pub const VIEW_NEWFOLDER: u32 = 11u32;
pub const VIEW_PARENTFOLDER: u32 = 8u32;
pub const VIEW_SMALLICONS: u32 = 1u32;
pub const VIEW_SORTDATE: u32 = 6u32;
pub const VIEW_SORTNAME: u32 = 4u32;
pub const VIEW_SORTSIZE: u32 = 5u32;
pub const VIEW_SORTTYPE: u32 = 7u32;
pub const VIEW_VIEWMENU: u32 = 12u32;
pub const VSCLASS_AEROWIZARD: windows_core::PCWSTR = windows_core::w!("AEROWIZARD");
pub const VSCLASS_AEROWIZARDSTYLE: windows_core::PCWSTR = windows_core::w!("AEROWIZARDSTYLE");
pub const VSCLASS_BUTTON: windows_core::PCWSTR = windows_core::w!("BUTTON");
pub const VSCLASS_BUTTONSTYLE: windows_core::PCWSTR = windows_core::w!("BUTTONSTYLE");
pub const VSCLASS_CLOCK: windows_core::PCWSTR = windows_core::w!("CLOCK");
pub const VSCLASS_COMBOBOX: windows_core::PCWSTR = windows_core::w!("COMBOBOX");
pub const VSCLASS_COMBOBOXSTYLE: windows_core::PCWSTR = windows_core::w!("COMBOBOXSTYLE");
pub const VSCLASS_COMMUNICATIONS: windows_core::PCWSTR = windows_core::w!("COMMUNICATIONS");
pub const VSCLASS_COMMUNICATIONSSTYLE: windows_core::PCWSTR = windows_core::w!("COMMUNICATIONSSTYLE");
pub const VSCLASS_CONTROLPANEL: windows_core::PCWSTR = windows_core::w!("CONTROLPANEL");
pub const VSCLASS_CONTROLPANELSTYLE: windows_core::PCWSTR = windows_core::w!("CONTROLPANELSTYLE");
pub const VSCLASS_DATEPICKER: windows_core::PCWSTR = windows_core::w!("DATEPICKER");
pub const VSCLASS_DATEPICKERSTYLE: windows_core::PCWSTR = windows_core::w!("DATEPICKERSTYLE");
pub const VSCLASS_DRAGDROP: windows_core::PCWSTR = windows_core::w!("DRAGDROP");
pub const VSCLASS_DRAGDROPSTYLE: windows_core::PCWSTR = windows_core::w!("DRAGDROPSTYLE");
pub const VSCLASS_EDIT: windows_core::PCWSTR = windows_core::w!("EDIT");
pub const VSCLASS_EDITSTYLE: windows_core::PCWSTR = windows_core::w!("EDITSTYLE");
pub const VSCLASS_EMPTYMARKUP: windows_core::PCWSTR = windows_core::w!("EMPTYMARKUP");
pub const VSCLASS_EXPLORERBAR: windows_core::PCWSTR = windows_core::w!("EXPLORERBAR");
pub const VSCLASS_EXPLORERBARSTYLE: windows_core::PCWSTR = windows_core::w!("EXPLORERBARSTYLE");
pub const VSCLASS_FLYOUT: windows_core::PCWSTR = windows_core::w!("FLYOUT");
pub const VSCLASS_FLYOUTSTYLE: windows_core::PCWSTR = windows_core::w!("FLYOUTSTYLE");
pub const VSCLASS_HEADER: windows_core::PCWSTR = windows_core::w!("HEADER");
pub const VSCLASS_HEADERSTYLE: windows_core::PCWSTR = windows_core::w!("HEADERSTYLE");
pub const VSCLASS_LINK: windows_core::PCWSTR = windows_core::w!("LINK");
pub const VSCLASS_LISTBOX: windows_core::PCWSTR = windows_core::w!("LISTBOX");
pub const VSCLASS_LISTBOXSTYLE: windows_core::PCWSTR = windows_core::w!("LISTBOXSTYLE");
pub const VSCLASS_LISTVIEW: windows_core::PCWSTR = windows_core::w!("LISTVIEW");
pub const VSCLASS_LISTVIEWSTYLE: windows_core::PCWSTR = windows_core::w!("LISTVIEWSTYLE");
pub const VSCLASS_MENU: windows_core::PCWSTR = windows_core::w!("MENU");
pub const VSCLASS_MENUBAND: windows_core::PCWSTR = windows_core::w!("MENUBAND");
pub const VSCLASS_MENUSTYLE: windows_core::PCWSTR = windows_core::w!("MENUSTYLE");
pub const VSCLASS_MONTHCAL: windows_core::PCWSTR = windows_core::w!("MONTHCAL");
pub const VSCLASS_NAVIGATION: windows_core::PCWSTR = windows_core::w!("NAVIGATION");
pub const VSCLASS_PAGE: windows_core::PCWSTR = windows_core::w!("PAGE");
pub const VSCLASS_PROGRESS: windows_core::PCWSTR = windows_core::w!("PROGRESS");
pub const VSCLASS_PROGRESSSTYLE: windows_core::PCWSTR = windows_core::w!("PROGRESSSTYLE");
pub const VSCLASS_REBAR: windows_core::PCWSTR = windows_core::w!("REBAR");
pub const VSCLASS_REBARSTYLE: windows_core::PCWSTR = windows_core::w!("REBARSTYLE");
pub const VSCLASS_SCROLLBAR: windows_core::PCWSTR = windows_core::w!("SCROLLBAR");
pub const VSCLASS_SCROLLBARSTYLE: windows_core::PCWSTR = windows_core::w!("SCROLLBARSTYLE");
pub const VSCLASS_SPIN: windows_core::PCWSTR = windows_core::w!("SPIN");
pub const VSCLASS_SPINSTYLE: windows_core::PCWSTR = windows_core::w!("SPINSTYLE");
pub const VSCLASS_STARTPANEL: windows_core::PCWSTR = windows_core::w!("STARTPANEL");
pub const VSCLASS_STATIC: windows_core::PCWSTR = windows_core::w!("STATIC");
pub const VSCLASS_STATUS: windows_core::PCWSTR = windows_core::w!("STATUS");
pub const VSCLASS_STATUSSTYLE: windows_core::PCWSTR = windows_core::w!("STATUSSTYLE");
pub const VSCLASS_TAB: windows_core::PCWSTR = windows_core::w!("TAB");
pub const VSCLASS_TABSTYLE: windows_core::PCWSTR = windows_core::w!("TABSTYLE");
pub const VSCLASS_TASKBAND: windows_core::PCWSTR = windows_core::w!("TASKBAND");
pub const VSCLASS_TASKBAR: windows_core::PCWSTR = windows_core::w!("TASKBAR");
pub const VSCLASS_TASKDIALOG: windows_core::PCWSTR = windows_core::w!("TASKDIALOG");
pub const VSCLASS_TASKDIALOGSTYLE: windows_core::PCWSTR = windows_core::w!("TASKDIALOGSTYLE");
pub const VSCLASS_TEXTSELECTIONGRIPPER: windows_core::PCWSTR = windows_core::w!("TEXTSELECTIONGRIPPER");
pub const VSCLASS_TEXTSTYLE: windows_core::PCWSTR = windows_core::w!("TEXTSTYLE");
pub const VSCLASS_TOOLBAR: windows_core::PCWSTR = windows_core::w!("TOOLBAR");
pub const VSCLASS_TOOLBARSTYLE: windows_core::PCWSTR = windows_core::w!("TOOLBARSTYLE");
pub const VSCLASS_TOOLTIP: windows_core::PCWSTR = windows_core::w!("TOOLTIP");
pub const VSCLASS_TOOLTIPSTYLE: windows_core::PCWSTR = windows_core::w!("TOOLTIPSTYLE");
pub const VSCLASS_TRACKBAR: windows_core::PCWSTR = windows_core::w!("TRACKBAR");
pub const VSCLASS_TRACKBARSTYLE: windows_core::PCWSTR = windows_core::w!("TRACKBARSTYLE");
pub const VSCLASS_TRAYNOTIFY: windows_core::PCWSTR = windows_core::w!("TRAYNOTIFY");
pub const VSCLASS_TREEVIEW: windows_core::PCWSTR = windows_core::w!("TREEVIEW");
pub const VSCLASS_TREEVIEWSTYLE: windows_core::PCWSTR = windows_core::w!("TREEVIEWSTYLE");
pub const VSCLASS_USERTILE: windows_core::PCWSTR = windows_core::w!("USERTILE");
pub const VSCLASS_WINDOW: windows_core::PCWSTR = windows_core::w!("WINDOW");
pub const VSCLASS_WINDOWSTYLE: windows_core::PCWSTR = windows_core::w!("WINDOWSTYLE");
pub const VSS_DISABLED: VERTSCROLLSTATES = VERTSCROLLSTATES(4i32);
pub const VSS_HOT: VERTSCROLLSTATES = VERTSCROLLSTATES(2i32);
pub const VSS_NORMAL: VERTSCROLLSTATES = VERTSCROLLSTATES(1i32);
pub const VSS_PUSHED: VERTSCROLLSTATES = VERTSCROLLSTATES(3i32);
pub const VTS_DISABLED: VERTTHUMBSTATES = VERTTHUMBSTATES(4i32);
pub const VTS_HOT: VERTTHUMBSTATES = VERTTHUMBSTATES(2i32);
pub const VTS_NORMAL: VERTTHUMBSTATES = VERTTHUMBSTATES(1i32);
pub const VTS_PUSHED: VERTTHUMBSTATES = VERTTHUMBSTATES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WARNINGSTATES(pub i32);
pub const WB_CLASSIFY: WORD_BREAK_ACTION = WORD_BREAK_ACTION(3i32);
pub const WB_ISDELIMITER: WORD_BREAK_ACTION = WORD_BREAK_ACTION(2i32);
pub const WB_LEFT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(0i32);
pub const WB_LEFTBREAK: WORD_BREAK_ACTION = WORD_BREAK_ACTION(6i32);
pub const WB_MOVEWORDLEFT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(4i32);
pub const WB_MOVEWORDRIGHT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(5i32);
pub const WB_RIGHT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(1i32);
pub const WB_RIGHTBREAK: WORD_BREAK_ACTION = WORD_BREAK_ACTION(7i32);
pub const WC_BUTTON: windows_core::PCWSTR = windows_core::w!("Button");
pub const WC_BUTTONA: windows_core::PCSTR = windows_core::s!("Button");
pub const WC_BUTTONW: windows_core::PCWSTR = windows_core::w!("Button");
pub const WC_COMBOBOX: windows_core::PCWSTR = windows_core::w!("ComboBox");
pub const WC_COMBOBOXA: windows_core::PCSTR = windows_core::s!("ComboBox");
pub const WC_COMBOBOXEX: windows_core::PCWSTR = windows_core::w!("ComboBoxEx32");
pub const WC_COMBOBOXEXA: windows_core::PCSTR = windows_core::s!("ComboBoxEx32");
pub const WC_COMBOBOXEXW: windows_core::PCWSTR = windows_core::w!("ComboBoxEx32");
pub const WC_COMBOBOXW: windows_core::PCWSTR = windows_core::w!("ComboBox");
pub const WC_EDIT: windows_core::PCWSTR = windows_core::w!("Edit");
pub const WC_EDITA: windows_core::PCSTR = windows_core::s!("Edit");
pub const WC_EDITW: windows_core::PCWSTR = windows_core::w!("Edit");
pub const WC_HEADER: windows_core::PCWSTR = windows_core::w!("SysHeader32");
pub const WC_HEADERA: windows_core::PCSTR = windows_core::s!("SysHeader32");
pub const WC_HEADERW: windows_core::PCWSTR = windows_core::w!("SysHeader32");
pub const WC_IPADDRESS: windows_core::PCWSTR = windows_core::w!("SysIPAddress32");
pub const WC_IPADDRESSA: windows_core::PCSTR = windows_core::s!("SysIPAddress32");
pub const WC_IPADDRESSW: windows_core::PCWSTR = windows_core::w!("SysIPAddress32");
pub const WC_LINK: windows_core::PCWSTR = windows_core::w!("SysLink");
pub const WC_LISTBOX: windows_core::PCWSTR = windows_core::w!("ListBox");
pub const WC_LISTBOXA: windows_core::PCSTR = windows_core::s!("ListBox");
pub const WC_LISTBOXW: windows_core::PCWSTR = windows_core::w!("ListBox");
pub const WC_LISTVIEW: windows_core::PCWSTR = windows_core::w!("SysListView32");
pub const WC_LISTVIEWA: windows_core::PCSTR = windows_core::s!("SysListView32");
pub const WC_LISTVIEWW: windows_core::PCWSTR = windows_core::w!("SysListView32");
pub const WC_NATIVEFONTCTL: windows_core::PCWSTR = windows_core::w!("NativeFontCtl");
pub const WC_NATIVEFONTCTLA: windows_core::PCSTR = windows_core::s!("NativeFontCtl");
pub const WC_NATIVEFONTCTLW: windows_core::PCWSTR = windows_core::w!("NativeFontCtl");
pub const WC_PAGESCROLLER: windows_core::PCWSTR = windows_core::w!("SysPager");
pub const WC_PAGESCROLLERA: windows_core::PCSTR = windows_core::s!("SysPager");
pub const WC_PAGESCROLLERW: windows_core::PCWSTR = windows_core::w!("SysPager");
pub const WC_SCROLLBAR: windows_core::PCWSTR = windows_core::w!("ScrollBar");
pub const WC_SCROLLBARA: windows_core::PCSTR = windows_core::s!("ScrollBar");
pub const WC_SCROLLBARW: windows_core::PCWSTR = windows_core::w!("ScrollBar");
pub const WC_STATIC: windows_core::PCWSTR = windows_core::w!("Static");
pub const WC_STATICA: windows_core::PCSTR = windows_core::s!("Static");
pub const WC_STATICW: windows_core::PCWSTR = windows_core::w!("Static");
pub const WC_TABCONTROL: windows_core::PCWSTR = windows_core::w!("SysTabControl32");
pub const WC_TABCONTROLA: windows_core::PCSTR = windows_core::s!("SysTabControl32");
pub const WC_TABCONTROLW: windows_core::PCWSTR = windows_core::w!("SysTabControl32");
pub const WC_TREEVIEW: windows_core::PCWSTR = windows_core::w!("SysTreeView32");
pub const WC_TREEVIEWA: windows_core::PCSTR = windows_core::s!("SysTreeView32");
pub const WC_TREEVIEWW: windows_core::PCWSTR = windows_core::w!("SysTreeView32");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINDOWPARTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINDOWTHEMEATTRIBUTETYPE(pub i32);
pub const WIZ_BODYCX: u32 = 184u32;
pub const WIZ_BODYX: u32 = 92u32;
pub const WIZ_CXBMP: u32 = 80u32;
pub const WIZ_CXDLG: u32 = 276u32;
pub const WIZ_CYDLG: u32 = 140u32;
pub const WMN_FIRST: u32 = 4294966296u32;
pub const WMN_LAST: u32 = 4294966096u32;
pub const WM_CTLCOLOR: u32 = 25u32;
pub const WM_MOUSEHOVER: u32 = 673u32;
pub const WM_MOUSELEAVE: u32 = 675u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WORD_BREAK_ACTION(pub i32);
pub const WP_BORDER: WINDOWPARTS = WINDOWPARTS(39i32);
pub const WP_CAPTION: WINDOWPARTS = WINDOWPARTS(1i32);
pub const WP_CAPTIONSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(30i32);
pub const WP_CLOSEBUTTON: WINDOWPARTS = WINDOWPARTS(18i32);
pub const WP_DIALOG: WINDOWPARTS = WINDOWPARTS(29i32);
pub const WP_FRAME: WINDOWPARTS = WINDOWPARTS(38i32);
pub const WP_FRAMEBOTTOM: WINDOWPARTS = WINDOWPARTS(9i32);
pub const WP_FRAMEBOTTOMSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(36i32);
pub const WP_FRAMELEFT: WINDOWPARTS = WINDOWPARTS(7i32);
pub const WP_FRAMELEFTSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(32i32);
pub const WP_FRAMERIGHT: WINDOWPARTS = WINDOWPARTS(8i32);
pub const WP_FRAMERIGHTSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(34i32);
pub const WP_HELPBUTTON: WINDOWPARTS = WINDOWPARTS(23i32);
pub const WP_HORZSCROLL: WINDOWPARTS = WINDOWPARTS(25i32);
pub const WP_HORZTHUMB: WINDOWPARTS = WINDOWPARTS(26i32);
pub const WP_MAXBUTTON: WINDOWPARTS = WINDOWPARTS(17i32);
pub const WP_MAXCAPTION: WINDOWPARTS = WINDOWPARTS(5i32);
pub const WP_MDICLOSEBUTTON: WINDOWPARTS = WINDOWPARTS(20i32);
pub const WP_MDIHELPBUTTON: WINDOWPARTS = WINDOWPARTS(24i32);
pub const WP_MDIMINBUTTON: WINDOWPARTS = WINDOWPARTS(16i32);
pub const WP_MDIRESTOREBUTTON: WINDOWPARTS = WINDOWPARTS(22i32);
pub const WP_MDISYSBUTTON: WINDOWPARTS = WINDOWPARTS(14i32);
pub const WP_MINBUTTON: WINDOWPARTS = WINDOWPARTS(15i32);
pub const WP_MINCAPTION: WINDOWPARTS = WINDOWPARTS(3i32);
pub const WP_RESTOREBUTTON: WINDOWPARTS = WINDOWPARTS(21i32);
pub const WP_SMALLCAPTION: WINDOWPARTS = WINDOWPARTS(2i32);
pub const WP_SMALLCAPTIONSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(31i32);
pub const WP_SMALLCLOSEBUTTON: WINDOWPARTS = WINDOWPARTS(19i32);
pub const WP_SMALLFRAMEBOTTOM: WINDOWPARTS = WINDOWPARTS(12i32);
pub const WP_SMALLFRAMEBOTTOMSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(37i32);
pub const WP_SMALLFRAMELEFT: WINDOWPARTS = WINDOWPARTS(10i32);
pub const WP_SMALLFRAMELEFTSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(33i32);
pub const WP_SMALLFRAMERIGHT: WINDOWPARTS = WINDOWPARTS(11i32);
pub const WP_SMALLFRAMERIGHTSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(35i32);
pub const WP_SMALLMAXCAPTION: WINDOWPARTS = WINDOWPARTS(6i32);
pub const WP_SMALLMINCAPTION: WINDOWPARTS = WINDOWPARTS(4i32);
pub const WP_SYSBUTTON: WINDOWPARTS = WINDOWPARTS(13i32);
pub const WP_VERTSCROLL: WINDOWPARTS = WINDOWPARTS(27i32);
pub const WP_VERTTHUMB: WINDOWPARTS = WINDOWPARTS(28i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRENCHSTATES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSB_PROP(pub i32);
pub const WSB_PROP_CXHSCROLL: WSB_PROP = WSB_PROP(2i32);
pub const WSB_PROP_CXHTHUMB: WSB_PROP = WSB_PROP(16i32);
pub const WSB_PROP_CXVSCROLL: WSB_PROP = WSB_PROP(8i32);
pub const WSB_PROP_CYHSCROLL: WSB_PROP = WSB_PROP(4i32);
pub const WSB_PROP_CYVSCROLL: WSB_PROP = WSB_PROP(1i32);
pub const WSB_PROP_CYVTHUMB: WSB_PROP = WSB_PROP(32i32);
pub const WSB_PROP_HBKGCOLOR: WSB_PROP = WSB_PROP(128i32);
pub const WSB_PROP_HSTYLE: WSB_PROP = WSB_PROP(512i32);
pub const WSB_PROP_MASK: i32 = 4095i32;
pub const WSB_PROP_PALETTE: WSB_PROP = WSB_PROP(2048i32);
pub const WSB_PROP_VBKGCOLOR: WSB_PROP = WSB_PROP(64i32);
pub const WSB_PROP_VSTYLE: WSB_PROP = WSB_PROP(256i32);
pub const WSB_PROP_WINSTYLE: WSB_PROP = WSB_PROP(1024i32);
pub const WTA_NONCLIENT: WINDOWTHEMEATTRIBUTETYPE = WINDOWTHEMEATTRIBUTETYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTA_OPTIONS {
    pub dwFlags: u32,
    pub dwMask: u32,
}
pub const WTNCA_NODRAWCAPTION: u32 = 1u32;
pub const WTNCA_NODRAWICON: u32 = 2u32;
pub const WTNCA_NOMIRRORHELP: u32 = 8u32;
pub const WTNCA_NOSYSMENU: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _LI_METRIC(pub i32);
pub const chx1: u32 = 1040u32;
pub const chx10: u32 = 1049u32;
pub const chx11: u32 = 1050u32;
pub const chx12: u32 = 1051u32;
pub const chx13: u32 = 1052u32;
pub const chx14: u32 = 1053u32;
pub const chx15: u32 = 1054u32;
pub const chx16: u32 = 1055u32;
pub const chx2: u32 = 1041u32;
pub const chx3: u32 = 1042u32;
pub const chx4: u32 = 1043u32;
pub const chx5: u32 = 1044u32;
pub const chx6: u32 = 1045u32;
pub const chx7: u32 = 1046u32;
pub const chx8: u32 = 1047u32;
pub const chx9: u32 = 1048u32;
pub const cmb1: u32 = 1136u32;
pub const cmb10: u32 = 1145u32;
pub const cmb11: u32 = 1146u32;
pub const cmb12: u32 = 1147u32;
pub const cmb13: u32 = 1148u32;
pub const cmb14: u32 = 1149u32;
pub const cmb15: u32 = 1150u32;
pub const cmb16: u32 = 1151u32;
pub const cmb2: u32 = 1137u32;
pub const cmb3: u32 = 1138u32;
pub const cmb4: u32 = 1139u32;
pub const cmb5: u32 = 1140u32;
pub const cmb6: u32 = 1141u32;
pub const cmb7: u32 = 1142u32;
pub const cmb8: u32 = 1143u32;
pub const cmb9: u32 = 1144u32;
pub const ctl1: u32 = 1184u32;
pub const ctlFirst: u32 = 1024u32;
pub const ctlLast: u32 = 1279u32;
pub const edt1: u32 = 1152u32;
pub const edt10: u32 = 1161u32;
pub const edt11: u32 = 1162u32;
pub const edt12: u32 = 1163u32;
pub const edt13: u32 = 1164u32;
pub const edt14: u32 = 1165u32;
pub const edt15: u32 = 1166u32;
pub const edt16: u32 = 1167u32;
pub const edt2: u32 = 1153u32;
pub const edt3: u32 = 1154u32;
pub const edt4: u32 = 1155u32;
pub const edt5: u32 = 1156u32;
pub const edt6: u32 = 1157u32;
pub const edt7: u32 = 1158u32;
pub const edt8: u32 = 1159u32;
pub const edt9: u32 = 1160u32;
pub const frm1: u32 = 1076u32;
pub const frm2: u32 = 1077u32;
pub const frm3: u32 = 1078u32;
pub const frm4: u32 = 1079u32;
pub const grp1: u32 = 1072u32;
pub const grp2: u32 = 1073u32;
pub const grp3: u32 = 1074u32;
pub const grp4: u32 = 1075u32;
pub const ico1: u32 = 1084u32;
pub const ico2: u32 = 1085u32;
pub const ico3: u32 = 1086u32;
pub const ico4: u32 = 1087u32;
pub const lst1: u32 = 1120u32;
pub const lst10: u32 = 1129u32;
pub const lst11: u32 = 1130u32;
pub const lst12: u32 = 1131u32;
pub const lst13: u32 = 1132u32;
pub const lst14: u32 = 1133u32;
pub const lst15: u32 = 1134u32;
pub const lst16: u32 = 1135u32;
pub const lst2: u32 = 1121u32;
pub const lst3: u32 = 1122u32;
pub const lst4: u32 = 1123u32;
pub const lst5: u32 = 1124u32;
pub const lst6: u32 = 1125u32;
pub const lst7: u32 = 1126u32;
pub const lst8: u32 = 1127u32;
pub const lst9: u32 = 1128u32;
pub const psh1: u32 = 1024u32;
pub const psh10: u32 = 1033u32;
pub const psh11: u32 = 1034u32;
pub const psh12: u32 = 1035u32;
pub const psh13: u32 = 1036u32;
pub const psh14: u32 = 1037u32;
pub const psh15: u32 = 1038u32;
pub const psh16: u32 = 1039u32;
pub const psh2: u32 = 1025u32;
pub const psh3: u32 = 1026u32;
pub const psh4: u32 = 1027u32;
pub const psh5: u32 = 1028u32;
pub const psh6: u32 = 1029u32;
pub const psh7: u32 = 1030u32;
pub const psh8: u32 = 1031u32;
pub const psh9: u32 = 1032u32;
pub const pshHelp: u32 = 1038u32;
pub const rad1: u32 = 1056u32;
pub const rad10: u32 = 1065u32;
pub const rad11: u32 = 1066u32;
pub const rad12: u32 = 1067u32;
pub const rad13: u32 = 1068u32;
pub const rad14: u32 = 1069u32;
pub const rad15: u32 = 1070u32;
pub const rad16: u32 = 1071u32;
pub const rad2: u32 = 1057u32;
pub const rad3: u32 = 1058u32;
pub const rad4: u32 = 1059u32;
pub const rad5: u32 = 1060u32;
pub const rad6: u32 = 1061u32;
pub const rad7: u32 = 1062u32;
pub const rad8: u32 = 1063u32;
pub const rad9: u32 = 1064u32;
pub const rct1: u32 = 1080u32;
pub const rct2: u32 = 1081u32;
pub const rct3: u32 = 1082u32;
pub const rct4: u32 = 1083u32;
pub const scr1: u32 = 1168u32;
pub const scr2: u32 = 1169u32;
pub const scr3: u32 = 1170u32;
pub const scr4: u32 = 1171u32;
pub const scr5: u32 = 1172u32;
pub const scr6: u32 = 1173u32;
pub const scr7: u32 = 1174u32;
pub const scr8: u32 = 1175u32;
pub const stc1: u32 = 1088u32;
pub const stc10: u32 = 1097u32;
pub const stc11: u32 = 1098u32;
pub const stc12: u32 = 1099u32;
pub const stc13: u32 = 1100u32;
pub const stc14: u32 = 1101u32;
pub const stc15: u32 = 1102u32;
pub const stc16: u32 = 1103u32;
pub const stc17: u32 = 1104u32;
pub const stc18: u32 = 1105u32;
pub const stc19: u32 = 1106u32;
pub const stc2: u32 = 1089u32;
pub const stc20: u32 = 1107u32;
pub const stc21: u32 = 1108u32;
pub const stc22: u32 = 1109u32;
pub const stc23: u32 = 1110u32;
pub const stc24: u32 = 1111u32;
pub const stc25: u32 = 1112u32;
pub const stc26: u32 = 1113u32;
pub const stc27: u32 = 1114u32;
pub const stc28: u32 = 1115u32;
pub const stc29: u32 = 1116u32;
pub const stc3: u32 = 1090u32;
pub const stc30: u32 = 1117u32;
pub const stc31: u32 = 1118u32;
pub const stc32: u32 = 1119u32;
pub const stc4: u32 = 1091u32;
pub const stc5: u32 = 1092u32;
pub const stc6: u32 = 1093u32;
pub const stc7: u32 = 1094u32;
pub const stc8: u32 = 1095u32;
pub const stc9: u32 = 1096u32;
