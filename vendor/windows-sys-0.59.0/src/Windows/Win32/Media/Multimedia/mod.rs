windows_targets::link!("avifil32.dll" "system" fn AVIBuildFilterA(lpszfilter : windows_sys::core::PSTR, cbfilter : i32, fsaving : super::super::Foundation:: BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIBuildFilterW(lpszfilter : windows_sys::core::PWSTR, cbfilter : i32, fsaving : super::super::Foundation:: BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIClearClipboard() -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIFileAddRef(pfile : * mut core::ffi::c_void) -> u32);
windows_targets::link!("avifil32.dll" "system" fn AVIFileCreateStreamA(pfile : * mut core::ffi::c_void, ppavi : *mut * mut core::ffi::c_void, psi : *const AVISTREAMINFOA) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIFileCreateStreamW(pfile : * mut core::ffi::c_void, ppavi : *mut * mut core::ffi::c_void, psi : *const AVISTREAMINFOW) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIFileEndRecord(pfile : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIFileExit());
windows_targets::link!("avifil32.dll" "system" fn AVIFileGetStream(pfile : * mut core::ffi::c_void, ppavi : *mut * mut core::ffi::c_void, fcctype : u32, lparam : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIFileInfoA(pfile : * mut core::ffi::c_void, pfi : *mut AVIFILEINFOA, lsize : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIFileInfoW(pfile : * mut core::ffi::c_void, pfi : *mut AVIFILEINFOW, lsize : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIFileInit());
windows_targets::link!("avifil32.dll" "system" fn AVIFileOpenA(ppfile : *mut * mut core::ffi::c_void, szfile : windows_sys::core::PCSTR, umode : u32, lphandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIFileOpenW(ppfile : *mut * mut core::ffi::c_void, szfile : windows_sys::core::PCWSTR, umode : u32, lphandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIFileReadData(pfile : * mut core::ffi::c_void, ckid : u32, lpdata : *mut core::ffi::c_void, lpcbdata : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIFileRelease(pfile : * mut core::ffi::c_void) -> u32);
windows_targets::link!("avifil32.dll" "system" fn AVIFileWriteData(pfile : * mut core::ffi::c_void, ckid : u32, lpdata : *const core::ffi::c_void, cbdata : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIGetFromClipboard(lppf : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIMakeCompressedStream(ppscompressed : *mut * mut core::ffi::c_void, ppssource : * mut core::ffi::c_void, lpoptions : *const AVICOMPRESSOPTIONS, pclsidhandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIMakeFileFromStreams(ppfile : *mut * mut core::ffi::c_void, nstreams : i32, papstreams : *const * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIMakeStreamFromClipboard(cfformat : u32, hglobal : super::super::Foundation:: HANDLE, ppstream : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIPutFileOnClipboard(pf : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "cdecl" fn AVISaveA(szfile : windows_sys::core::PCSTR, pclsidhandler : *const windows_sys::core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, pfile : * mut core::ffi::c_void, lpoptions : *const AVICOMPRESSOPTIONS, ...) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVISaveOptions(hwnd : super::super::Foundation:: HWND, uiflags : u32, nstreams : i32, ppavi : *const * mut core::ffi::c_void, plpoptions : *mut *mut AVICOMPRESSOPTIONS) -> isize);
windows_targets::link!("avifil32.dll" "system" fn AVISaveOptionsFree(nstreams : i32, plpoptions : *const *const AVICOMPRESSOPTIONS) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVISaveVA(szfile : windows_sys::core::PCSTR, pclsidhandler : *const windows_sys::core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, ppavi : *const * mut core::ffi::c_void, plpoptions : *const *const AVICOMPRESSOPTIONS) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVISaveVW(szfile : windows_sys::core::PCWSTR, pclsidhandler : *const windows_sys::core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, ppavi : *const * mut core::ffi::c_void, plpoptions : *const *const AVICOMPRESSOPTIONS) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "cdecl" fn AVISaveW(szfile : windows_sys::core::PCWSTR, pclsidhandler : *const windows_sys::core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, pfile : * mut core::ffi::c_void, lpoptions : *const AVICOMPRESSOPTIONS, ...) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamAddRef(pavi : * mut core::ffi::c_void) -> u32);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamBeginStreaming(pavi : * mut core::ffi::c_void, lstart : i32, lend : i32, lrate : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamCreate(ppavi : *mut * mut core::ffi::c_void, lparam1 : i32, lparam2 : i32, pclsidhandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamEndStreaming(pavi : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamFindSample(pavi : * mut core::ffi::c_void, lpos : i32, lflags : i32) -> i32);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamGetFrame(pg : * mut core::ffi::c_void, lpos : i32) -> *mut core::ffi::c_void);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamGetFrameClose(pg : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("avifil32.dll" "system" fn AVIStreamGetFrameOpen(pavi : * mut core::ffi::c_void, lpbiwanted : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER) -> * mut core::ffi::c_void);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamInfoA(pavi : * mut core::ffi::c_void, psi : *mut AVISTREAMINFOA, lsize : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamInfoW(pavi : * mut core::ffi::c_void, psi : *mut AVISTREAMINFOW, lsize : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamLength(pavi : * mut core::ffi::c_void) -> i32);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamOpenFromFileA(ppavi : *mut * mut core::ffi::c_void, szfile : windows_sys::core::PCSTR, fcctype : u32, lparam : i32, mode : u32, pclsidhandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamOpenFromFileW(ppavi : *mut * mut core::ffi::c_void, szfile : windows_sys::core::PCWSTR, fcctype : u32, lparam : i32, mode : u32, pclsidhandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamRead(pavi : * mut core::ffi::c_void, lstart : i32, lsamples : i32, lpbuffer : *mut core::ffi::c_void, cbbuffer : i32, plbytes : *mut i32, plsamples : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamReadData(pavi : * mut core::ffi::c_void, fcc : u32, lp : *mut core::ffi::c_void, lpcb : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamReadFormat(pavi : * mut core::ffi::c_void, lpos : i32, lpformat : *mut core::ffi::c_void, lpcbformat : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamRelease(pavi : * mut core::ffi::c_void) -> u32);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamSampleToTime(pavi : * mut core::ffi::c_void, lsample : i32) -> i32);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamSetFormat(pavi : * mut core::ffi::c_void, lpos : i32, lpformat : *const core::ffi::c_void, cbformat : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamStart(pavi : * mut core::ffi::c_void) -> i32);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamTimeToSample(pavi : * mut core::ffi::c_void, ltime : i32) -> i32);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamWrite(pavi : * mut core::ffi::c_void, lstart : i32, lsamples : i32, lpbuffer : *const core::ffi::c_void, cbbuffer : i32, dwflags : u32, plsampwritten : *mut i32, plbyteswritten : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn AVIStreamWriteData(pavi : * mut core::ffi::c_void, fcc : u32, lp : *const core::ffi::c_void, cb : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("winmm.dll" "system" fn CloseDriver(hdriver : HDRVR, lparam1 : super::super::Foundation:: LPARAM, lparam2 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
windows_targets::link!("avifil32.dll" "system" fn CreateEditableStream(ppseditable : *mut * mut core::ffi::c_void, pssource : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("winmm.dll" "system" fn DefDriverProc(dwdriveridentifier : usize, hdrvr : HDRVR, umsg : u32, lparam1 : super::super::Foundation:: LPARAM, lparam2 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn DrawDibBegin(hdd : isize, hdc : super::super::Graphics::Gdi:: HDC, dxdst : i32, dydst : i32, lpbi : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, dxsrc : i32, dysrc : i32, wflags : u32) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn DrawDibChangePalette(hdd : isize, istart : i32, ilen : i32, lppe : *const super::super::Graphics::Gdi:: PALETTEENTRY) -> super::super::Foundation:: BOOL);
windows_targets::link!("msvfw32.dll" "system" fn DrawDibClose(hdd : isize) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn DrawDibDraw(hdd : isize, hdc : super::super::Graphics::Gdi:: HDC, xdst : i32, ydst : i32, dxdst : i32, dydst : i32, lpbi : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, lpbits : *const core::ffi::c_void, xsrc : i32, ysrc : i32, dxsrc : i32, dysrc : i32, wflags : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("msvfw32.dll" "system" fn DrawDibEnd(hdd : isize) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn DrawDibGetBuffer(hdd : isize, lpbi : *mut super::super::Graphics::Gdi:: BITMAPINFOHEADER, dwsize : u32, dwflags : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn DrawDibGetPalette(hdd : isize) -> super::super::Graphics::Gdi:: HPALETTE);
windows_targets::link!("msvfw32.dll" "system" fn DrawDibOpen() -> isize);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn DrawDibProfileDisplay(lpbi : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER) -> super::super::Foundation:: LRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn DrawDibRealize(hdd : isize, hdc : super::super::Graphics::Gdi:: HDC, fbackground : super::super::Foundation:: BOOL) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn DrawDibSetPalette(hdd : isize, hpal : super::super::Graphics::Gdi:: HPALETTE) -> super::super::Foundation:: BOOL);
windows_targets::link!("msvfw32.dll" "system" fn DrawDibStart(hdd : isize, rate : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("msvfw32.dll" "system" fn DrawDibStop(hdd : isize) -> super::super::Foundation:: BOOL);
windows_targets::link!("msvfw32.dll" "system" fn DrawDibTime(hdd : isize, lpddtime : *mut DRAWDIBTIME) -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn DriverCallback(dwcallback : usize, dwflags : u32, hdevice : HDRVR, dwmsg : u32, dwuser : usize, dwparam1 : usize, dwparam2 : usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn DrvGetModuleHandle(hdriver : HDRVR) -> super::super::Foundation:: HMODULE);
windows_targets::link!("avifil32.dll" "system" fn EditStreamClone(pavi : * mut core::ffi::c_void, ppresult : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn EditStreamCopy(pavi : * mut core::ffi::c_void, plstart : *mut i32, pllength : *mut i32, ppresult : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn EditStreamCut(pavi : * mut core::ffi::c_void, plstart : *mut i32, pllength : *mut i32, ppresult : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn EditStreamPaste(pavi : * mut core::ffi::c_void, plpos : *mut i32, pllength : *mut i32, pstream : * mut core::ffi::c_void, lstart : i32, lend : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn EditStreamSetInfoA(pavi : * mut core::ffi::c_void, lpinfo : *const AVISTREAMINFOA, cbinfo : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn EditStreamSetInfoW(pavi : * mut core::ffi::c_void, lpinfo : *const AVISTREAMINFOW, cbinfo : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn EditStreamSetNameA(pavi : * mut core::ffi::c_void, lpszname : windows_sys::core::PCSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("avifil32.dll" "system" fn EditStreamSetNameW(pavi : * mut core::ffi::c_void, lpszname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("winmm.dll" "system" fn GetDriverModuleHandle(hdriver : HDRVR) -> super::super::Foundation:: HMODULE);
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
windows_targets::link!("msvfw32.dll" "system" fn GetOpenFileNamePreviewA(lpofn : *mut super::super::UI::Controls::Dialogs:: OPENFILENAMEA) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
windows_targets::link!("msvfw32.dll" "system" fn GetOpenFileNamePreviewW(lpofn : *mut super::super::UI::Controls::Dialogs:: OPENFILENAMEW) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
windows_targets::link!("msvfw32.dll" "system" fn GetSaveFileNamePreviewA(lpofn : *mut super::super::UI::Controls::Dialogs:: OPENFILENAMEA) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
windows_targets::link!("msvfw32.dll" "system" fn GetSaveFileNamePreviewW(lpofn : *mut super::super::UI::Controls::Dialogs:: OPENFILENAMEW) -> super::super::Foundation:: BOOL);
windows_targets::link!("msvfw32.dll" "system" fn ICClose(hic : HIC) -> super::super::Foundation:: LRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "cdecl" fn ICCompress(hic : HIC, dwflags : u32, lpbioutput : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, lpdata : *mut core::ffi::c_void, lpbiinput : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, lpbits : *const core::ffi::c_void, lpckid : *mut u32, lpdwflags : *mut u32, lframenum : i32, dwframesize : u32, dwquality : u32, lpbiprev : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, lpprev : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn ICCompressorChoose(hwnd : super::super::Foundation:: HWND, uiflags : u32, pvin : *const core::ffi::c_void, lpdata : *const core::ffi::c_void, pc : *mut COMPVARS, lpsztitle : windows_sys::core::PCSTR) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn ICCompressorFree(pc : *const COMPVARS));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "cdecl" fn ICDecompress(hic : HIC, dwflags : u32, lpbiformat : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, lpdata : *const core::ffi::c_void, lpbi : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, lpbits : *mut core::ffi::c_void) -> u32);
windows_targets::link!("msvfw32.dll" "cdecl" fn ICDraw(hic : HIC, dwflags : u32, lpformat : *const core::ffi::c_void, lpdata : *const core::ffi::c_void, cbdata : u32, ltime : i32) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "cdecl" fn ICDrawBegin(hic : HIC, dwflags : u32, hpal : super::super::Graphics::Gdi:: HPALETTE, hwnd : super::super::Foundation:: HWND, hdc : super::super::Graphics::Gdi:: HDC, xdst : i32, ydst : i32, dxdst : i32, dydst : i32, lpbi : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, xsrc : i32, ysrc : i32, dxsrc : i32, dysrc : i32, dwrate : u32, dwscale : u32) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn ICGetDisplayFormat(hic : HIC, lpbiin : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, lpbiout : *mut super::super::Graphics::Gdi:: BITMAPINFOHEADER, bitdepth : i32, dx : i32, dy : i32) -> HIC);
windows_targets::link!("msvfw32.dll" "system" fn ICGetInfo(hic : HIC, picinfo : *mut ICINFO, cb : u32) -> super::super::Foundation:: LRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn ICImageCompress(hic : HIC, uiflags : u32, lpbiin : *const super::super::Graphics::Gdi:: BITMAPINFO, lpbits : *const core::ffi::c_void, lpbiout : *const super::super::Graphics::Gdi:: BITMAPINFO, lquality : i32, plsize : *mut i32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn ICImageDecompress(hic : HIC, uiflags : u32, lpbiin : *const super::super::Graphics::Gdi:: BITMAPINFO, lpbits : *const core::ffi::c_void, lpbiout : *const super::super::Graphics::Gdi:: BITMAPINFO) -> super::super::Foundation:: HANDLE);
windows_targets::link!("msvfw32.dll" "system" fn ICInfo(fcctype : u32, fcchandler : u32, lpicinfo : *mut ICINFO) -> super::super::Foundation:: BOOL);
windows_targets::link!("msvfw32.dll" "system" fn ICInstall(fcctype : u32, fcchandler : u32, lparam : super::super::Foundation:: LPARAM, szdesc : windows_sys::core::PCSTR, wflags : u32) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn ICLocate(fcctype : u32, fcchandler : u32, lpbiin : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, lpbiout : *const super::super::Graphics::Gdi:: BITMAPINFOHEADER, wflags : u16) -> HIC);
windows_targets::link!("msvfw32.dll" "system" fn ICOpen(fcctype : u32, fcchandler : u32, wmode : u32) -> HIC);
windows_targets::link!("msvfw32.dll" "system" fn ICOpenFunction(fcctype : u32, fcchandler : u32, wmode : u32, lpfnhandler : super::super::Foundation:: FARPROC) -> HIC);
windows_targets::link!("msvfw32.dll" "system" fn ICRemove(fcctype : u32, fcchandler : u32, wflags : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("msvfw32.dll" "system" fn ICSendMessage(hic : HIC, msg : u32, dw1 : usize, dw2 : usize) -> super::super::Foundation:: LRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn ICSeqCompressFrame(pc : *const COMPVARS, uiflags : u32, lpbits : *const core::ffi::c_void, pfkey : *mut super::super::Foundation:: BOOL, plsize : *mut i32) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn ICSeqCompressFrameEnd(pc : *const COMPVARS));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("msvfw32.dll" "system" fn ICSeqCompressFrameStart(pc : *const COMPVARS, lpbiin : *const super::super::Graphics::Gdi:: BITMAPINFO) -> super::super::Foundation:: BOOL);
windows_targets::link!("msvfw32.dll" "cdecl" fn MCIWndCreateA(hwndparent : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, dwstyle : u32, szfile : windows_sys::core::PCSTR) -> super::super::Foundation:: HWND);
windows_targets::link!("msvfw32.dll" "cdecl" fn MCIWndCreateW(hwndparent : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, dwstyle : u32, szfile : windows_sys::core::PCWSTR) -> super::super::Foundation:: HWND);
windows_targets::link!("msvfw32.dll" "cdecl" fn MCIWndRegisterClass() -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn OpenDriver(szdrivername : windows_sys::core::PCWSTR, szsectionname : windows_sys::core::PCWSTR, lparam2 : super::super::Foundation:: LPARAM) -> HDRVR);
windows_targets::link!("winmm.dll" "system" fn SendDriverMessage(hdriver : HDRVR, message : u32, lparam1 : super::super::Foundation:: LPARAM, lparam2 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
windows_targets::link!("msvfw32.dll" "system" fn VideoForWindowsVersion() -> u32);
windows_targets::link!("avicap32.dll" "system" fn capCreateCaptureWindowA(lpszwindowname : windows_sys::core::PCSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::super::Foundation:: HWND, nid : i32) -> super::super::Foundation:: HWND);
windows_targets::link!("avicap32.dll" "system" fn capCreateCaptureWindowW(lpszwindowname : windows_sys::core::PCWSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::super::Foundation:: HWND, nid : i32) -> super::super::Foundation:: HWND);
windows_targets::link!("avicap32.dll" "system" fn capGetDriverDescriptionA(wdriverindex : u32, lpszname : windows_sys::core::PSTR, cbname : i32, lpszver : windows_sys::core::PSTR, cbver : i32) -> super::super::Foundation:: BOOL);
windows_targets::link!("avicap32.dll" "system" fn capGetDriverDescriptionW(wdriverindex : u32, lpszname : windows_sys::core::PWSTR, cbname : i32, lpszver : windows_sys::core::PWSTR, cbver : i32) -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn joyGetDevCapsA(ujoyid : usize, pjc : *mut JOYCAPSA, cbjc : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn joyGetDevCapsW(ujoyid : usize, pjc : *mut JOYCAPSW, cbjc : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn joyGetNumDevs() -> u32);
windows_targets::link!("winmm.dll" "system" fn joyGetPos(ujoyid : u32, pji : *mut JOYINFO) -> u32);
windows_targets::link!("winmm.dll" "system" fn joyGetPosEx(ujoyid : u32, pji : *mut JOYINFOEX) -> u32);
windows_targets::link!("winmm.dll" "system" fn joyGetThreshold(ujoyid : u32, puthreshold : *mut u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn joyReleaseCapture(ujoyid : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn joySetCapture(hwnd : super::super::Foundation:: HWND, ujoyid : u32, uperiod : u32, fchanged : super::super::Foundation:: BOOL) -> u32);
windows_targets::link!("winmm.dll" "system" fn joySetThreshold(ujoyid : u32, uthreshold : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciDriverNotify(hwndcallback : super::super::Foundation:: HANDLE, wdeviceid : u32, ustatus : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn mciDriverYield(wdeviceid : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciFreeCommandResource(wtable : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn mciGetCreatorTask(mciid : u32) -> super:: HTASK);
windows_targets::link!("winmm.dll" "system" fn mciGetDeviceIDA(pszdevice : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciGetDeviceIDFromElementIDA(dwelementid : u32, lpstrtype : windows_sys::core::PCSTR) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciGetDeviceIDFromElementIDW(dwelementid : u32, lpstrtype : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciGetDeviceIDW(pszdevice : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciGetDriverData(wdeviceid : u32) -> usize);
windows_targets::link!("winmm.dll" "system" fn mciGetErrorStringA(mcierr : u32, psztext : windows_sys::core::PSTR, cchtext : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn mciGetErrorStringW(mcierr : u32, psztext : windows_sys::core::PWSTR, cchtext : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn mciGetYieldProc(mciid : u32, pdwyielddata : *const u32) -> YIELDPROC);
windows_targets::link!("winmm.dll" "system" fn mciLoadCommandResource(hinstance : super::super::Foundation:: HANDLE, lpresname : windows_sys::core::PCWSTR, wtype : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciSendCommandA(mciid : u32, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciSendCommandW(mciid : u32, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciSendStringA(lpstrcommand : windows_sys::core::PCSTR, lpstrreturnstring : windows_sys::core::PSTR, ureturnlength : u32, hwndcallback : super::super::Foundation:: HWND) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciSendStringW(lpstrcommand : windows_sys::core::PCWSTR, lpstrreturnstring : windows_sys::core::PWSTR, ureturnlength : u32, hwndcallback : super::super::Foundation:: HWND) -> u32);
windows_targets::link!("winmm.dll" "system" fn mciSetDriverData(wdeviceid : u32, dwdata : usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn mciSetYieldProc(mciid : u32, fpyieldproc : YIELDPROC, dwyielddata : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn mmDrvInstall(hdriver : HDRVR, wszdrventry : windows_sys::core::PCWSTR, drvmessage : DRIVERMSGPROC, wflags : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmGetCurrentTask() -> u32);
windows_targets::link!("winmm.dll" "system" fn mmTaskBlock(h : u32));
windows_targets::link!("winmm.dll" "system" fn mmTaskCreate(lpfn : LPTASKCALLBACK, lph : *mut super::super::Foundation:: HANDLE, dwinst : usize) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmTaskSignal(h : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("winmm.dll" "system" fn mmTaskYield());
windows_targets::link!("winmm.dll" "system" fn mmioAdvance(hmmio : HMMIO, pmmioinfo : *const MMIOINFO, fuadvance : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioAscend(hmmio : HMMIO, pmmcki : *const MMCKINFO, fuascend : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioClose(hmmio : HMMIO, fuclose : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioCreateChunk(hmmio : HMMIO, pmmcki : *const MMCKINFO, fucreate : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioDescend(hmmio : HMMIO, pmmcki : *mut MMCKINFO, pmmckiparent : *const MMCKINFO, fudescend : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioFlush(hmmio : HMMIO, fuflush : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioGetInfo(hmmio : HMMIO, pmmioinfo : *mut MMIOINFO, fuinfo : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioInstallIOProcA(fccioproc : u32, pioproc : LPMMIOPROC, dwflags : u32) -> LPMMIOPROC);
windows_targets::link!("winmm.dll" "system" fn mmioInstallIOProcW(fccioproc : u32, pioproc : LPMMIOPROC, dwflags : u32) -> LPMMIOPROC);
windows_targets::link!("winmm.dll" "system" fn mmioOpenA(pszfilename : windows_sys::core::PSTR, pmmioinfo : *mut MMIOINFO, fdwopen : u32) -> HMMIO);
windows_targets::link!("winmm.dll" "system" fn mmioOpenW(pszfilename : windows_sys::core::PWSTR, pmmioinfo : *mut MMIOINFO, fdwopen : u32) -> HMMIO);
windows_targets::link!("winmm.dll" "system" fn mmioRead(hmmio : HMMIO, pch : *mut i8, cch : i32) -> i32);
windows_targets::link!("winmm.dll" "system" fn mmioRenameA(pszfilename : windows_sys::core::PCSTR, psznewfilename : windows_sys::core::PCSTR, pmmioinfo : *const MMIOINFO, fdwrename : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioRenameW(pszfilename : windows_sys::core::PCWSTR, psznewfilename : windows_sys::core::PCWSTR, pmmioinfo : *const MMIOINFO, fdwrename : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioSeek(hmmio : HMMIO, loffset : i32, iorigin : i32) -> i32);
windows_targets::link!("winmm.dll" "system" fn mmioSendMessage(hmmio : HMMIO, umsg : u32, lparam1 : super::super::Foundation:: LPARAM, lparam2 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
windows_targets::link!("winmm.dll" "system" fn mmioSetBuffer(hmmio : HMMIO, pchbuffer : windows_sys::core::PSTR, cchbuffer : i32, fubuffer : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioSetInfo(hmmio : HMMIO, pmmioinfo : *const MMIOINFO, fuinfo : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioStringToFOURCCA(sz : windows_sys::core::PCSTR, uflags : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioStringToFOURCCW(sz : windows_sys::core::PCWSTR, uflags : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mmioWrite(hmmio : HMMIO, pch : windows_sys::core::PCSTR, cch : i32) -> i32);
windows_targets::link!("api-ms-win-mm-misc-l1-1-1.dll" "system" fn sndOpenSound(eventname : windows_sys::core::PCWSTR, appname : windows_sys::core::PCWSTR, flags : i32, filehandle : *mut super::super::Foundation:: HANDLE) -> i32);
pub const ACMDM_BASE: u32 = 24576u32;
pub const ACM_MPEG_COPYRIGHT: u32 = 2u32;
pub const ACM_MPEG_DUALCHANNEL: u32 = 4u32;
pub const ACM_MPEG_ID_MPEG1: u32 = 16u32;
pub const ACM_MPEG_JOINTSTEREO: u32 = 2u32;
pub const ACM_MPEG_LAYER1: u32 = 1u32;
pub const ACM_MPEG_LAYER2: u32 = 2u32;
pub const ACM_MPEG_LAYER3: u32 = 4u32;
pub const ACM_MPEG_ORIGINALHOME: u32 = 4u32;
pub const ACM_MPEG_PRIVATEBIT: u32 = 1u32;
pub const ACM_MPEG_PROTECTIONBIT: u32 = 8u32;
pub const ACM_MPEG_SINGLECHANNEL: u32 = 8u32;
pub const ACM_MPEG_STEREO: u32 = 1u32;
pub const AUXDM_GETDEVCAPS: u32 = 4u32;
pub const AUXDM_GETNUMDEVS: u32 = 3u32;
pub const AUXDM_GETVOLUME: u32 = 5u32;
pub const AUXDM_SETVOLUME: u32 = 6u32;
pub const AUXM_INIT: u32 = 100u32;
pub const AUXM_INIT_EX: u32 = 104u32;
pub const AVICOMPRESSF_DATARATE: u32 = 2u32;
pub const AVICOMPRESSF_INTERLEAVE: u32 = 1u32;
pub const AVICOMPRESSF_KEYFRAMES: u32 = 4u32;
pub const AVICOMPRESSF_VALID: u32 = 8u32;
pub const AVIERR_OK: i32 = 0i32;
pub const AVIFILECAPS_ALLKEYFRAMES: u32 = 16u32;
pub const AVIFILECAPS_CANREAD: u32 = 1u32;
pub const AVIFILECAPS_CANWRITE: u32 = 2u32;
pub const AVIFILECAPS_NOCOMPRESSION: u32 = 32u32;
pub const AVIFILEHANDLER_CANACCEPTNONRGB: u32 = 4u32;
pub const AVIFILEHANDLER_CANREAD: u32 = 1u32;
pub const AVIFILEHANDLER_CANWRITE: u32 = 2u32;
pub const AVIFILEINFO_COPYRIGHTED: u32 = 131072u32;
pub const AVIFILEINFO_HASINDEX: u32 = 16u32;
pub const AVIFILEINFO_ISINTERLEAVED: u32 = 256u32;
pub const AVIFILEINFO_MUSTUSEINDEX: u32 = 32u32;
pub const AVIFILEINFO_WASCAPTUREFILE: u32 = 65536u32;
pub const AVIGETFRAMEF_BESTDISPLAYFMT: u32 = 1u32;
pub const AVIIF_CONTROLFRAME: i32 = 512i32;
pub const AVIIF_TWOCC: i32 = 2i32;
pub const AVISTREAMINFO_DISABLED: u32 = 1u32;
pub const AVISTREAMINFO_FORMATCHANGES: u32 = 65536u32;
pub const AVISTREAMREAD_CONVENIENT: i32 = -1i32;
pub const AVSTREAMMASTER_AUDIO: u32 = 0u32;
pub const AVSTREAMMASTER_NONE: u32 = 1u32;
pub const BI_1632: u32 = 842217009u32;
pub const CLSID_AVIFile: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00020000_0000_0000_c000_000000000046);
pub const CLSID_AVISimpleUnMarshal: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00020009_0000_0000_c000_000000000046);
pub const CONTROLCALLBACK_CAPTURING: u32 = 2u32;
pub const CONTROLCALLBACK_PREROLL: u32 = 1u32;
pub const CRYSTAL_NET_SFM_CODEC: u32 = 1u32;
pub const DCB_EVENT: u32 = 5u32;
pub const DCB_FUNCTION: u32 = 3u32;
pub const DCB_NOSWITCH: u32 = 8u32;
pub const DCB_NULL: u32 = 0u32;
pub const DCB_TASK: u32 = 2u32;
pub const DCB_TYPEMASK: u32 = 7u32;
pub const DCB_WINDOW: u32 = 1u32;
pub const DDF_0001: u32 = 1u32;
pub const DDF_2000: u32 = 8192u32;
pub const DDF_ANIMATE: u32 = 32u32;
pub const DDF_BACKGROUNDPAL: u32 = 512u32;
pub const DDF_BUFFER: u32 = 64u32;
pub const DDF_DONTDRAW: u32 = 16u32;
pub const DDF_FULLSCREEN: u32 = 256u32;
pub const DDF_HALFTONE: u32 = 4096u32;
pub const DDF_HURRYUP: u32 = 2048u32;
pub const DDF_JUSTDRAWIT: u32 = 128u32;
pub const DDF_NOTKEYFRAME: u32 = 1024u32;
pub const DDF_PREROLL: u32 = 16u32;
pub const DDF_SAME_DIB: u32 = 8u32;
pub const DDF_SAME_DRAW: u32 = 8u32;
pub const DDF_SAME_HDC: u32 = 4u32;
pub const DDF_SAME_SIZE: u32 = 8u32;
pub const DDF_UPDATE: u32 = 2u32;
pub const DLG_ACMFILTERCHOOSE_ID: u32 = 71u32;
pub const DLG_ACMFORMATCHOOSE_ID: u32 = 70u32;
pub const DRIVERS_SECTION: windows_sys::core::PCWSTR = windows_sys::core::w!("DRIVERS32");
pub const DRVCNF_CANCEL: u32 = 0u32;
pub const DRVCNF_OK: u32 = 1u32;
pub const DRVCNF_RESTART: u32 = 2u32;
pub const DRVM_ADD_THRU: u32 = 257u32;
pub const DRVM_DISABLE: u32 = 102u32;
pub const DRVM_ENABLE: u32 = 103u32;
pub const DRVM_EXIT: u32 = 101u32;
pub const DRVM_INIT: u32 = 100u32;
pub const DRVM_INIT_EX: u32 = 104u32;
pub const DRVM_IOCTL: u32 = 256u32;
pub const DRVM_IOCTL_CMD_SYSTEM: i32 = -2147483648i32;
pub const DRVM_IOCTL_CMD_USER: i32 = 0i32;
pub const DRVM_IOCTL_LAST: u32 = 261u32;
pub const DRVM_MAPPER_CONSOLEVOICECOM_GET: u32 = 8215u32;
pub const DRVM_MAPPER_PREFERRED_FLAGS_PREFERREDONLY: u32 = 1u32;
pub const DRVM_MAPPER_PREFERRED_GET: u32 = 8213u32;
pub const DRVM_MAPPER_RECONFIGURE: u32 = 8193u32;
pub const DRVM_REMOVE_THRU: u32 = 258u32;
pub const DRVM_USER: u32 = 16384u32;
pub const DRV_CANCEL: u32 = 0u32;
pub const DRV_CLOSE: u32 = 4u32;
pub const DRV_CONFIGURE: u32 = 7u32;
pub const DRV_DISABLE: u32 = 5u32;
pub const DRV_ENABLE: u32 = 2u32;
pub const DRV_EXITSESSION: u32 = 11u32;
pub const DRV_FREE: u32 = 6u32;
pub const DRV_INSTALL: u32 = 9u32;
pub const DRV_LOAD: u32 = 1u32;
pub const DRV_MCI_FIRST: u32 = 2048u32;
pub const DRV_MCI_LAST: u32 = 6143u32;
pub const DRV_OK: u32 = 1u32;
pub const DRV_OPEN: u32 = 3u32;
pub const DRV_PNPINSTALL: u32 = 2059u32;
pub const DRV_POWER: u32 = 15u32;
pub const DRV_QUERYCONFIGURE: u32 = 8u32;
pub const DRV_QUERYDEVICEINTERFACE: u32 = 2060u32;
pub const DRV_QUERYDEVICEINTERFACESIZE: u32 = 2061u32;
pub const DRV_QUERYDEVNODE: u32 = 2050u32;
pub const DRV_QUERYFUNCTIONINSTANCEID: u32 = 2065u32;
pub const DRV_QUERYFUNCTIONINSTANCEIDSIZE: u32 = 2066u32;
pub const DRV_QUERYIDFROMSTRINGID: u32 = 2064u32;
pub const DRV_QUERYMAPPABLE: u32 = 2053u32;
pub const DRV_QUERYMODULE: u32 = 2057u32;
pub const DRV_QUERYSTRINGID: u32 = 2062u32;
pub const DRV_QUERYSTRINGIDSIZE: u32 = 2063u32;
pub const DRV_REMOVE: u32 = 10u32;
pub const DRV_RESERVED: u32 = 2048u32;
pub const DRV_RESTART: u32 = 2u32;
pub const DRV_USER: u32 = 16384u32;
pub const DVM_CONFIGURE_END: u32 = 8191u32;
pub const DVM_CONFIGURE_START: u32 = 4096u32;
pub const DVM_DST_RECT: u32 = 4101u32;
pub const DVM_FORMAT: u32 = 4098u32;
pub const DVM_PALETTE: u32 = 4097u32;
pub const DVM_PALETTERGB555: u32 = 4099u32;
pub const DVM_SRC_RECT: u32 = 4100u32;
pub const DVM_USER: u32 = 16384u32;
pub const DV_ERR_13: u32 = 16u32;
pub const DV_ERR_ALLOCATED: u32 = 19u32;
pub const DV_ERR_BADDEVICEID: u32 = 20u32;
pub const DV_ERR_BADERRNUM: u32 = 22u32;
pub const DV_ERR_BADFORMAT: u32 = 2u32;
pub const DV_ERR_BADINSTALL: u32 = 8u32;
pub const DV_ERR_BASE: u32 = 1u32;
pub const DV_ERR_CONFIG1: u32 = 13u32;
pub const DV_ERR_CONFIG2: u32 = 14u32;
pub const DV_ERR_CREATEPALETTE: u32 = 9u32;
pub const DV_ERR_DMA_CONFLICT: u32 = 26u32;
pub const DV_ERR_FLAGS: u32 = 15u32;
pub const DV_ERR_INT_CONFLICT: u32 = 27u32;
pub const DV_ERR_INVALHANDLE: u32 = 21u32;
pub const DV_ERR_IO_CONFLICT: u32 = 25u32;
pub const DV_ERR_LASTERROR: u32 = 28u32;
pub const DV_ERR_MEM_CONFLICT: u32 = 24u32;
pub const DV_ERR_NOMEM: u32 = 18u32;
pub const DV_ERR_NONSPECIFIC: u32 = 1u32;
pub const DV_ERR_NOTDETECTED: u32 = 7u32;
pub const DV_ERR_NOTSUPPORTED: u32 = 17u32;
pub const DV_ERR_NO_BUFFERS: u32 = 23u32;
pub const DV_ERR_OK: u32 = 0u32;
pub const DV_ERR_PARAM1: u32 = 11u32;
pub const DV_ERR_PARAM2: u32 = 12u32;
pub const DV_ERR_PROTECT_ONLY: u32 = 28u32;
pub const DV_ERR_SIZEFIELD: u32 = 10u32;
pub const DV_ERR_STILLPLAYING: u32 = 3u32;
pub const DV_ERR_SYNC: u32 = 5u32;
pub const DV_ERR_TOOMANYCHANNELS: u32 = 6u32;
pub const DV_ERR_UNPREPARED: u32 = 4u32;
pub const DV_ERR_USER_MSG: u32 = 1001u32;
pub const DV_VM_CLOSE: u32 = 977u32;
pub const DV_VM_DATA: u32 = 978u32;
pub const DV_VM_ERROR: u32 = 979u32;
pub const DV_VM_OPEN: u32 = 976u32;
pub const FACILITY_NS: u32 = 13u32;
pub const FACILITY_NS_WIN32: u32 = 7u32;
pub const FIND_ANY: i32 = 32i32;
pub const FIND_DIR: i32 = 15i32;
pub const FIND_FORMAT: i32 = 64i32;
pub const FIND_FROM_START: i32 = 8i32;
pub const FIND_INDEX: i32 = 16384i32;
pub const FIND_KEY: i32 = 16i32;
pub const FIND_LENGTH: i32 = 4096i32;
pub const FIND_NEXT: i32 = 1i32;
pub const FIND_OFFSET: i32 = 8192i32;
pub const FIND_POS: i32 = 0i32;
pub const FIND_PREV: i32 = 4i32;
pub const FIND_RET: i32 = 61440i32;
pub const FIND_SIZE: i32 = 12288i32;
pub const FIND_TYPE: i32 = 240i32;
pub const ICCOMPRESSFRAMES_PADDING: u32 = 1u32;
pub const ICCOMPRESS_KEYFRAME: i32 = 1i32;
pub const ICDECOMPRESS_HURRYUP: i32 = -2147483648i32;
pub const ICDECOMPRESS_NOTKEYFRAME: i32 = 134217728i32;
pub const ICDECOMPRESS_NULLFRAME: i32 = 268435456i32;
pub const ICDECOMPRESS_PREROLL: i32 = 536870912i32;
pub const ICDECOMPRESS_UPDATE: i32 = 1073741824i32;
pub const ICDRAW_ANIMATE: i32 = 8i32;
pub const ICDRAW_BUFFER: i32 = 256i32;
pub const ICDRAW_CONTINUE: i32 = 16i32;
pub const ICDRAW_FULLSCREEN: i32 = 2i32;
pub const ICDRAW_HDC: i32 = 4i32;
pub const ICDRAW_HURRYUP: i32 = -2147483648i32;
pub const ICDRAW_MEMORYDC: i32 = 32i32;
pub const ICDRAW_NOTKEYFRAME: i32 = 134217728i32;
pub const ICDRAW_NULLFRAME: i32 = 268435456i32;
pub const ICDRAW_PREROLL: i32 = 536870912i32;
pub const ICDRAW_QUERY: i32 = 1i32;
pub const ICDRAW_RENDER: i32 = 128i32;
pub const ICDRAW_UPDATE: i32 = 1073741824i32;
pub const ICDRAW_UPDATING: i32 = 64i32;
pub const ICERR_ABORT: i32 = -10i32;
pub const ICERR_BADBITDEPTH: i32 = -200i32;
pub const ICERR_BADFLAGS: i32 = -5i32;
pub const ICERR_BADFORMAT: i32 = -2i32;
pub const ICERR_BADHANDLE: i32 = -8i32;
pub const ICERR_BADIMAGESIZE: i32 = -201i32;
pub const ICERR_BADPARAM: i32 = -6i32;
pub const ICERR_BADSIZE: i32 = -7i32;
pub const ICERR_CANTUPDATE: i32 = -9i32;
pub const ICERR_CUSTOM: i32 = -400i32;
pub const ICERR_DONTDRAW: i32 = 1i32;
pub const ICERR_ERROR: i32 = -100i32;
pub const ICERR_GOTOKEYFRAME: i32 = 3i32;
pub const ICERR_INTERNAL: i32 = -4i32;
pub const ICERR_MEMORY: i32 = -3i32;
pub const ICERR_NEWPALETTE: i32 = 2i32;
pub const ICERR_OK: i32 = 0i32;
pub const ICERR_STOPDRAWING: i32 = 4i32;
pub const ICERR_UNSUPPORTED: i32 = -1i32;
pub const ICINSTALL_DRIVER: u32 = 2u32;
pub const ICINSTALL_DRIVERW: u32 = 32770u32;
pub const ICINSTALL_FUNCTION: u32 = 1u32;
pub const ICINSTALL_HDRV: u32 = 4u32;
pub const ICINSTALL_UNICODE: u32 = 32768u32;
pub const ICMF_ABOUT_QUERY: u32 = 1u32;
pub const ICMF_CHOOSE_ALLCOMPRESSORS: u32 = 8u32;
pub const ICMF_CHOOSE_DATARATE: u32 = 2u32;
pub const ICMF_CHOOSE_KEYFRAME: u32 = 1u32;
pub const ICMF_CHOOSE_PREVIEW: u32 = 4u32;
pub const ICMF_COMPVARS_VALID: u32 = 1u32;
pub const ICMF_CONFIGURE_QUERY: u32 = 1u32;
pub const ICMODE_COMPRESS: u32 = 1u32;
pub const ICMODE_DECOMPRESS: u32 = 2u32;
pub const ICMODE_DRAW: u32 = 8u32;
pub const ICMODE_FASTCOMPRESS: u32 = 5u32;
pub const ICMODE_FASTDECOMPRESS: u32 = 3u32;
pub const ICMODE_INTERNALF_FUNCTION32: u32 = 32768u32;
pub const ICMODE_INTERNALF_MASK: u32 = 32768u32;
pub const ICMODE_QUERY: u32 = 4u32;
pub const ICM_ABOUT: u32 = 20491u32;
pub const ICM_COMPRESS: u32 = 16392u32;
pub const ICM_COMPRESS_BEGIN: u32 = 16391u32;
pub const ICM_COMPRESS_END: u32 = 16393u32;
pub const ICM_COMPRESS_FRAMES: u32 = 16455u32;
pub const ICM_COMPRESS_FRAMES_INFO: u32 = 16454u32;
pub const ICM_COMPRESS_GET_FORMAT: u32 = 16388u32;
pub const ICM_COMPRESS_GET_SIZE: u32 = 16389u32;
pub const ICM_COMPRESS_QUERY: u32 = 16390u32;
pub const ICM_CONFIGURE: u32 = 20490u32;
pub const ICM_DECOMPRESS: u32 = 16397u32;
pub const ICM_DECOMPRESSEX: u32 = 16446u32;
pub const ICM_DECOMPRESSEX_BEGIN: u32 = 16444u32;
pub const ICM_DECOMPRESSEX_END: u32 = 16447u32;
pub const ICM_DECOMPRESSEX_QUERY: u32 = 16445u32;
pub const ICM_DECOMPRESS_BEGIN: u32 = 16396u32;
pub const ICM_DECOMPRESS_END: u32 = 16398u32;
pub const ICM_DECOMPRESS_GET_FORMAT: u32 = 16394u32;
pub const ICM_DECOMPRESS_GET_PALETTE: u32 = 16414u32;
pub const ICM_DECOMPRESS_QUERY: u32 = 16395u32;
pub const ICM_DECOMPRESS_SET_PALETTE: u32 = 16413u32;
pub const ICM_DRAW: u32 = 16417u32;
pub const ICM_DRAW_BEGIN: u32 = 16399u32;
pub const ICM_DRAW_BITS: u32 = 16404u32;
pub const ICM_DRAW_CHANGEPALETTE: u32 = 16435u32;
pub const ICM_DRAW_END: u32 = 16405u32;
pub const ICM_DRAW_FLUSH: u32 = 16421u32;
pub const ICM_DRAW_GETTIME: u32 = 16416u32;
pub const ICM_DRAW_GET_PALETTE: u32 = 16400u32;
pub const ICM_DRAW_IDLE: u32 = 16436u32;
pub const ICM_DRAW_QUERY: u32 = 16415u32;
pub const ICM_DRAW_REALIZE: u32 = 16420u32;
pub const ICM_DRAW_RENDERBUFFER: u32 = 16422u32;
pub const ICM_DRAW_SETTIME: u32 = 16419u32;
pub const ICM_DRAW_START: u32 = 16402u32;
pub const ICM_DRAW_START_PLAY: u32 = 16423u32;
pub const ICM_DRAW_STOP: u32 = 16403u32;
pub const ICM_DRAW_STOP_PLAY: u32 = 16424u32;
pub const ICM_DRAW_SUGGESTFORMAT: u32 = 16434u32;
pub const ICM_DRAW_UPDATE: u32 = 16401u32;
pub const ICM_DRAW_WINDOW: u32 = 16418u32;
pub const ICM_ENUMFORMATS: u32 = 20501u32;
pub const ICM_GET: u32 = 20521u32;
pub const ICM_GETBUFFERSWANTED: u32 = 16425u32;
pub const ICM_GETDEFAULTKEYFRAMERATE: u32 = 16426u32;
pub const ICM_GETDEFAULTQUALITY: u32 = 20510u32;
pub const ICM_GETERRORTEXT: u32 = 20492u32;
pub const ICM_GETFORMATNAME: u32 = 20500u32;
pub const ICM_GETINFO: u32 = 20482u32;
pub const ICM_GETQUALITY: u32 = 20511u32;
pub const ICM_GETSTATE: u32 = 20480u32;
pub const ICM_RESERVED: u32 = 20480u32;
pub const ICM_RESERVED_HIGH: u32 = 24576u32;
pub const ICM_RESERVED_LOW: u32 = 20480u32;
pub const ICM_SET: u32 = 20520u32;
pub const ICM_SETQUALITY: u32 = 20512u32;
pub const ICM_SETSTATE: u32 = 20481u32;
pub const ICM_SET_STATUS_PROC: u32 = 16456u32;
pub const ICM_USER: u32 = 16384u32;
pub const ICQUALITY_DEFAULT: i32 = -1i32;
pub const ICQUALITY_HIGH: u32 = 10000u32;
pub const ICQUALITY_LOW: u32 = 0u32;
pub const ICSTATUS_END: u32 = 2u32;
pub const ICSTATUS_ERROR: u32 = 3u32;
pub const ICSTATUS_START: u32 = 0u32;
pub const ICSTATUS_STATUS: u32 = 1u32;
pub const ICSTATUS_YIELD: u32 = 4u32;
pub const ICVERSION: u32 = 260u32;
pub const IDD_ACMFILTERCHOOSE_BTN_DELNAME: u32 = 104u32;
pub const IDD_ACMFILTERCHOOSE_BTN_HELP: u32 = 9u32;
pub const IDD_ACMFILTERCHOOSE_BTN_SETNAME: u32 = 103u32;
pub const IDD_ACMFILTERCHOOSE_CMB_CUSTOM: u32 = 100u32;
pub const IDD_ACMFILTERCHOOSE_CMB_FILTER: u32 = 102u32;
pub const IDD_ACMFILTERCHOOSE_CMB_FILTERTAG: u32 = 101u32;
pub const IDD_ACMFORMATCHOOSE_BTN_DELNAME: u32 = 104u32;
pub const IDD_ACMFORMATCHOOSE_BTN_HELP: u32 = 9u32;
pub const IDD_ACMFORMATCHOOSE_BTN_SETNAME: u32 = 103u32;
pub const IDD_ACMFORMATCHOOSE_CMB_CUSTOM: u32 = 100u32;
pub const IDD_ACMFORMATCHOOSE_CMB_FORMAT: u32 = 102u32;
pub const IDD_ACMFORMATCHOOSE_CMB_FORMATTAG: u32 = 101u32;
pub const IDS_CAP_AUDIO_DROP_COMPERROR: u32 = 442u32;
pub const IDS_CAP_AUDIO_DROP_ERROR: u32 = 441u32;
pub const IDS_CAP_AVI_DRAWDIB_ERROR: u32 = 439u32;
pub const IDS_CAP_AVI_INIT_ERROR: u32 = 433u32;
pub const IDS_CAP_BEGIN: u32 = 300u32;
pub const IDS_CAP_CANTOPEN: u32 = 409u32;
pub const IDS_CAP_COMPRESSOR_ERROR: u32 = 440u32;
pub const IDS_CAP_DEFAVIEXT: u32 = 407u32;
pub const IDS_CAP_DEFPALEXT: u32 = 408u32;
pub const IDS_CAP_DRIVER_ERROR: u32 = 418u32;
pub const IDS_CAP_END: u32 = 301u32;
pub const IDS_CAP_ERRORDIBSAVE: u32 = 406u32;
pub const IDS_CAP_ERRORPALOPEN: u32 = 404u32;
pub const IDS_CAP_ERRORPALSAVE: u32 = 405u32;
pub const IDS_CAP_FILEEXISTS: u32 = 403u32;
pub const IDS_CAP_FILE_OPEN_ERROR: u32 = 429u32;
pub const IDS_CAP_FILE_WRITE_ERROR: u32 = 430u32;
pub const IDS_CAP_INFO: u32 = 401u32;
pub const IDS_CAP_MCI_CANT_STEP_ERROR: u32 = 437u32;
pub const IDS_CAP_MCI_CONTROL_ERROR: u32 = 436u32;
pub const IDS_CAP_NODISKSPACE: u32 = 415u32;
pub const IDS_CAP_NO_AUDIO_CAP_ERROR: u32 = 438u32;
pub const IDS_CAP_NO_FRAME_CAP_ERROR: u32 = 434u32;
pub const IDS_CAP_NO_PALETTE_WARN: u32 = 435u32;
pub const IDS_CAP_OUTOFMEM: u32 = 402u32;
pub const IDS_CAP_READONLYFILE: u32 = 413u32;
pub const IDS_CAP_RECORDING_ERROR: u32 = 431u32;
pub const IDS_CAP_RECORDING_ERROR2: u32 = 432u32;
pub const IDS_CAP_SAVEASPERCENT: u32 = 417u32;
pub const IDS_CAP_SEQ_MSGSTART: u32 = 410u32;
pub const IDS_CAP_SEQ_MSGSTOP: u32 = 411u32;
pub const IDS_CAP_SETFILESIZE: u32 = 416u32;
pub const IDS_CAP_STAT_CAP_AUDIO: u32 = 509u32;
pub const IDS_CAP_STAT_CAP_FINI: u32 = 503u32;
pub const IDS_CAP_STAT_CAP_INIT: u32 = 502u32;
pub const IDS_CAP_STAT_CAP_L_FRAMES: u32 = 508u32;
pub const IDS_CAP_STAT_FRAMESDROPPED: u32 = 513u32;
pub const IDS_CAP_STAT_I_FRAMES: u32 = 506u32;
pub const IDS_CAP_STAT_LIVE_MODE: u32 = 500u32;
pub const IDS_CAP_STAT_L_FRAMES: u32 = 507u32;
pub const IDS_CAP_STAT_OPTPAL_BUILD: u32 = 505u32;
pub const IDS_CAP_STAT_OVERLAY_MODE: u32 = 501u32;
pub const IDS_CAP_STAT_PALETTE_BUILD: u32 = 504u32;
pub const IDS_CAP_STAT_VIDEOAUDIO: u32 = 511u32;
pub const IDS_CAP_STAT_VIDEOCURRENT: u32 = 510u32;
pub const IDS_CAP_STAT_VIDEOONLY: u32 = 512u32;
pub const IDS_CAP_VIDEDITERR: u32 = 412u32;
pub const IDS_CAP_VIDEO_ADD_ERROR: u32 = 427u32;
pub const IDS_CAP_VIDEO_ALLOC_ERROR: u32 = 425u32;
pub const IDS_CAP_VIDEO_OPEN_ERROR: u32 = 424u32;
pub const IDS_CAP_VIDEO_PREPARE_ERROR: u32 = 426u32;
pub const IDS_CAP_VIDEO_SIZE_ERROR: u32 = 428u32;
pub const IDS_CAP_WAVE_ADD_ERROR: u32 = 422u32;
pub const IDS_CAP_WAVE_ALLOC_ERROR: u32 = 420u32;
pub const IDS_CAP_WAVE_OPEN_ERROR: u32 = 419u32;
pub const IDS_CAP_WAVE_PREPARE_ERROR: u32 = 421u32;
pub const IDS_CAP_WAVE_SIZE_ERROR: u32 = 423u32;
pub const IDS_CAP_WRITEERROR: u32 = 414u32;
pub const JDD_CONFIGCHANGED: u32 = 2307u32;
pub const JDD_GETDEVCAPS: u32 = 2050u32;
pub const JDD_GETNUMDEVS: u32 = 2049u32;
pub const JDD_GETPOS: u32 = 2305u32;
pub const JDD_GETPOSEX: u32 = 2308u32;
pub const JDD_SETCALIBRATION: u32 = 2306u32;
pub const JIFMK_00: u32 = 65280u32;
pub const JIFMK_APP0: u32 = 65504u32;
pub const JIFMK_APP1: u32 = 65505u32;
pub const JIFMK_APP2: u32 = 65506u32;
pub const JIFMK_APP3: u32 = 65507u32;
pub const JIFMK_APP4: u32 = 65508u32;
pub const JIFMK_APP5: u32 = 65509u32;
pub const JIFMK_APP6: u32 = 65510u32;
pub const JIFMK_APP7: u32 = 65511u32;
pub const JIFMK_COM: u32 = 65534u32;
pub const JIFMK_DAC: u32 = 65484u32;
pub const JIFMK_DHP: u32 = 65502u32;
pub const JIFMK_DHT: u32 = 65476u32;
pub const JIFMK_DNL: u32 = 65500u32;
pub const JIFMK_DQT: u32 = 65499u32;
pub const JIFMK_DRI: u32 = 65501u32;
pub const JIFMK_EOI: u32 = 65497u32;
pub const JIFMK_EXP: u32 = 65503u32;
pub const JIFMK_FF: u32 = 65535u32;
pub const JIFMK_JPG: u32 = 65480u32;
pub const JIFMK_JPG0: u32 = 65520u32;
pub const JIFMK_JPG1: u32 = 65521u32;
pub const JIFMK_JPG10: u32 = 65530u32;
pub const JIFMK_JPG11: u32 = 65531u32;
pub const JIFMK_JPG12: u32 = 65532u32;
pub const JIFMK_JPG13: u32 = 65533u32;
pub const JIFMK_JPG2: u32 = 65522u32;
pub const JIFMK_JPG3: u32 = 65523u32;
pub const JIFMK_JPG4: u32 = 65524u32;
pub const JIFMK_JPG5: u32 = 65525u32;
pub const JIFMK_JPG6: u32 = 65526u32;
pub const JIFMK_JPG7: u32 = 65527u32;
pub const JIFMK_JPG8: u32 = 65528u32;
pub const JIFMK_JPG9: u32 = 65529u32;
pub const JIFMK_RES: u32 = 65282u32;
pub const JIFMK_RST0: u32 = 65488u32;
pub const JIFMK_RST1: u32 = 65489u32;
pub const JIFMK_RST2: u32 = 65490u32;
pub const JIFMK_RST3: u32 = 65491u32;
pub const JIFMK_RST4: u32 = 65492u32;
pub const JIFMK_RST5: u32 = 65493u32;
pub const JIFMK_RST6: u32 = 65494u32;
pub const JIFMK_RST7: u32 = 65495u32;
pub const JIFMK_SOF0: u32 = 65472u32;
pub const JIFMK_SOF1: u32 = 65473u32;
pub const JIFMK_SOF10: u32 = 65482u32;
pub const JIFMK_SOF11: u32 = 65483u32;
pub const JIFMK_SOF13: u32 = 65485u32;
pub const JIFMK_SOF14: u32 = 65486u32;
pub const JIFMK_SOF15: u32 = 65487u32;
pub const JIFMK_SOF2: u32 = 65474u32;
pub const JIFMK_SOF3: u32 = 65475u32;
pub const JIFMK_SOF5: u32 = 65477u32;
pub const JIFMK_SOF6: u32 = 65478u32;
pub const JIFMK_SOF7: u32 = 65479u32;
pub const JIFMK_SOF9: u32 = 65481u32;
pub const JIFMK_SOI: u32 = 65496u32;
pub const JIFMK_SOS: u32 = 65498u32;
pub const JIFMK_TEM: u32 = 65281u32;
pub const JOYCAPS_HASPOV: u32 = 16u32;
pub const JOYCAPS_HASR: u32 = 2u32;
pub const JOYCAPS_HASU: u32 = 4u32;
pub const JOYCAPS_HASV: u32 = 8u32;
pub const JOYCAPS_HASZ: u32 = 1u32;
pub const JOYCAPS_POV4DIR: u32 = 32u32;
pub const JOYCAPS_POVCTS: u32 = 64u32;
pub const JOYERR_NOCANDO: u32 = 166u32;
pub const JOYERR_NOERROR: u32 = 0u32;
pub const JOYERR_PARMS: u32 = 165u32;
pub const JOYERR_UNPLUGGED: u32 = 167u32;
pub const JOYSTICKID1: u32 = 0u32;
pub const JOYSTICKID2: u32 = 1u32;
pub const JOY_BUTTON1: u32 = 1u32;
pub const JOY_BUTTON10: i32 = 512i32;
pub const JOY_BUTTON11: i32 = 1024i32;
pub const JOY_BUTTON12: i32 = 2048i32;
pub const JOY_BUTTON13: i32 = 4096i32;
pub const JOY_BUTTON14: i32 = 8192i32;
pub const JOY_BUTTON15: i32 = 16384i32;
pub const JOY_BUTTON16: i32 = 32768i32;
pub const JOY_BUTTON17: i32 = 65536i32;
pub const JOY_BUTTON18: i32 = 131072i32;
pub const JOY_BUTTON19: i32 = 262144i32;
pub const JOY_BUTTON1CHG: u32 = 256u32;
pub const JOY_BUTTON2: u32 = 2u32;
pub const JOY_BUTTON20: i32 = 524288i32;
pub const JOY_BUTTON21: i32 = 1048576i32;
pub const JOY_BUTTON22: i32 = 2097152i32;
pub const JOY_BUTTON23: i32 = 4194304i32;
pub const JOY_BUTTON24: i32 = 8388608i32;
pub const JOY_BUTTON25: i32 = 16777216i32;
pub const JOY_BUTTON26: i32 = 33554432i32;
pub const JOY_BUTTON27: i32 = 67108864i32;
pub const JOY_BUTTON28: i32 = 134217728i32;
pub const JOY_BUTTON29: i32 = 268435456i32;
pub const JOY_BUTTON2CHG: u32 = 512u32;
pub const JOY_BUTTON3: u32 = 4u32;
pub const JOY_BUTTON30: i32 = 536870912i32;
pub const JOY_BUTTON31: i32 = 1073741824i32;
pub const JOY_BUTTON32: i32 = -2147483648i32;
pub const JOY_BUTTON3CHG: u32 = 1024u32;
pub const JOY_BUTTON4: u32 = 8u32;
pub const JOY_BUTTON4CHG: u32 = 2048u32;
pub const JOY_BUTTON5: i32 = 16i32;
pub const JOY_BUTTON6: i32 = 32i32;
pub const JOY_BUTTON7: i32 = 64i32;
pub const JOY_BUTTON8: i32 = 128i32;
pub const JOY_BUTTON9: i32 = 256i32;
pub const JOY_CAL_READ3: i32 = 262144i32;
pub const JOY_CAL_READ4: i32 = 524288i32;
pub const JOY_CAL_READ5: i32 = 4194304i32;
pub const JOY_CAL_READ6: i32 = 8388608i32;
pub const JOY_CAL_READALWAYS: i32 = 65536i32;
pub const JOY_CAL_READRONLY: i32 = 33554432i32;
pub const JOY_CAL_READUONLY: i32 = 67108864i32;
pub const JOY_CAL_READVONLY: i32 = 134217728i32;
pub const JOY_CAL_READXONLY: i32 = 1048576i32;
pub const JOY_CAL_READXYONLY: i32 = 131072i32;
pub const JOY_CAL_READYONLY: i32 = 2097152i32;
pub const JOY_CAL_READZONLY: i32 = 16777216i32;
pub const JOY_CONFIGCHANGED_MSGSTRING: windows_sys::core::PCSTR = windows_sys::core::s!("MSJSTICK_VJOYD_MSGSTR");
pub const JOY_POVBACKWARD: u32 = 18000u32;
pub const JOY_POVFORWARD: u32 = 0u32;
pub const JOY_POVLEFT: u32 = 27000u32;
pub const JOY_POVRIGHT: u32 = 9000u32;
pub const JOY_RETURNBUTTONS: i32 = 128i32;
pub const JOY_RETURNCENTERED: i32 = 1024i32;
pub const JOY_RETURNPOV: i32 = 64i32;
pub const JOY_RETURNPOVCTS: i32 = 512i32;
pub const JOY_RETURNR: i32 = 8i32;
pub const JOY_RETURNRAWDATA: i32 = 256i32;
pub const JOY_RETURNU: i32 = 16i32;
pub const JOY_RETURNV: i32 = 32i32;
pub const JOY_RETURNX: i32 = 1i32;
pub const JOY_RETURNY: i32 = 2i32;
pub const JOY_RETURNZ: i32 = 4i32;
pub const JOY_USEDEADZONE: i32 = 2048i32;
pub const JPEG_PROCESS_BASELINE: u32 = 0u32;
pub const JPEG_RGB: u32 = 3u32;
pub const JPEG_Y: u32 = 1u32;
pub const JPEG_YCbCr: u32 = 2u32;
pub const MCIERR_AVI_AUDIOERROR: u32 = 619u32;
pub const MCIERR_AVI_BADPALETTE: u32 = 620u32;
pub const MCIERR_AVI_CANTPLAYFULLSCREEN: u32 = 615u32;
pub const MCIERR_AVI_DISPLAYERROR: u32 = 618u32;
pub const MCIERR_AVI_NOCOMPRESSOR: u32 = 617u32;
pub const MCIERR_AVI_NODISPDIB: u32 = 614u32;
pub const MCIERR_AVI_NOTINTERLEAVED: u32 = 613u32;
pub const MCIERR_AVI_OLDAVIFORMAT: u32 = 612u32;
pub const MCIERR_AVI_TOOBIGFORVGA: u32 = 616u32;
pub const MCIERR_BAD_CONSTANT: u32 = 290u32;
pub const MCIERR_BAD_INTEGER: u32 = 270u32;
pub const MCIERR_BAD_TIME_FORMAT: u32 = 293u32;
pub const MCIERR_CANNOT_LOAD_DRIVER: u32 = 266u32;
pub const MCIERR_CANNOT_USE_ALL: u32 = 279u32;
pub const MCIERR_CREATEWINDOW: u32 = 347u32;
pub const MCIERR_CUSTOM_DRIVER_BASE: u32 = 512u32;
pub const MCIERR_DEVICE_LENGTH: u32 = 310u32;
pub const MCIERR_DEVICE_LOCKED: u32 = 288u32;
pub const MCIERR_DEVICE_NOT_INSTALLED: u32 = 306u32;
pub const MCIERR_DEVICE_NOT_READY: u32 = 276u32;
pub const MCIERR_DEVICE_OPEN: u32 = 265u32;
pub const MCIERR_DEVICE_ORD_LENGTH: u32 = 311u32;
pub const MCIERR_DEVICE_TYPE_REQUIRED: u32 = 287u32;
pub const MCIERR_DGV_BAD_CLIPBOARD_RANGE: u32 = 517u32;
pub const MCIERR_DGV_DEVICE_LIMIT: u32 = 512u32;
pub const MCIERR_DGV_DEVICE_MEMORY_FULL: u32 = 516u32;
pub const MCIERR_DGV_DISK_FULL: u32 = 515u32;
pub const MCIERR_DGV_IOERR: u32 = 513u32;
pub const MCIERR_DGV_WORKSPACE_EMPTY: u32 = 514u32;
pub const MCIERR_DRIVER: u32 = 278u32;
pub const MCIERR_DRIVER_INTERNAL: u32 = 272u32;
pub const MCIERR_DUPLICATE_ALIAS: u32 = 289u32;
pub const MCIERR_DUPLICATE_FLAGS: u32 = 295u32;
pub const MCIERR_EXTENSION_NOT_FOUND: u32 = 281u32;
pub const MCIERR_EXTRA_CHARACTERS: u32 = 305u32;
pub const MCIERR_FILENAME_REQUIRED: u32 = 304u32;
pub const MCIERR_FILE_NOT_FOUND: u32 = 275u32;
pub const MCIERR_FILE_NOT_SAVED: u32 = 286u32;
pub const MCIERR_FILE_READ: u32 = 348u32;
pub const MCIERR_FILE_WRITE: u32 = 349u32;
pub const MCIERR_FLAGS_NOT_COMPATIBLE: u32 = 284u32;
pub const MCIERR_GET_CD: u32 = 307u32;
pub const MCIERR_HARDWARE: u32 = 262u32;
pub const MCIERR_ILLEGAL_FOR_AUTO_OPEN: u32 = 303u32;
pub const MCIERR_INTERNAL: u32 = 277u32;
pub const MCIERR_INVALID_DEVICE_ID: u32 = 257u32;
pub const MCIERR_INVALID_DEVICE_NAME: u32 = 263u32;
pub const MCIERR_INVALID_FILE: u32 = 296u32;
pub const MCIERR_MISSING_COMMAND_STRING: u32 = 267u32;
pub const MCIERR_MISSING_DEVICE_NAME: u32 = 292u32;
pub const MCIERR_MISSING_PARAMETER: u32 = 273u32;
pub const MCIERR_MISSING_STRING_ARGUMENT: u32 = 269u32;
pub const MCIERR_MULTIPLE: u32 = 280u32;
pub const MCIERR_MUST_USE_SHAREABLE: u32 = 291u32;
pub const MCIERR_NEW_REQUIRES_ALIAS: u32 = 299u32;
pub const MCIERR_NONAPPLICABLE_FUNCTION: u32 = 302u32;
pub const MCIERR_NOTIFY_ON_AUTO_OPEN: u32 = 300u32;
pub const MCIERR_NO_CLOSING_QUOTE: u32 = 294u32;
pub const MCIERR_NO_ELEMENT_ALLOWED: u32 = 301u32;
pub const MCIERR_NO_IDENTITY: u32 = 350u32;
pub const MCIERR_NO_INTEGER: u32 = 312u32;
pub const MCIERR_NO_WINDOW: u32 = 346u32;
pub const MCIERR_NULL_PARAMETER_BLOCK: u32 = 297u32;
pub const MCIERR_OUTOFRANGE: u32 = 282u32;
pub const MCIERR_OUT_OF_MEMORY: u32 = 264u32;
pub const MCIERR_PARAM_OVERFLOW: u32 = 268u32;
pub const MCIERR_PARSER_INTERNAL: u32 = 271u32;
pub const MCIERR_SEQ_DIV_INCOMPATIBLE: u32 = 336u32;
pub const MCIERR_SEQ_NOMIDIPRESENT: u32 = 343u32;
pub const MCIERR_SEQ_PORTUNSPECIFIED: u32 = 342u32;
pub const MCIERR_SEQ_PORT_INUSE: u32 = 337u32;
pub const MCIERR_SEQ_PORT_MAPNODEVICE: u32 = 339u32;
pub const MCIERR_SEQ_PORT_MISCERROR: u32 = 340u32;
pub const MCIERR_SEQ_PORT_NONEXISTENT: u32 = 338u32;
pub const MCIERR_SEQ_TIMER: u32 = 341u32;
pub const MCIERR_SET_CD: u32 = 308u32;
pub const MCIERR_SET_DRIVE: u32 = 309u32;
pub const MCIERR_UNNAMED_RESOURCE: u32 = 298u32;
pub const MCIERR_UNRECOGNIZED_COMMAND: u32 = 261u32;
pub const MCIERR_UNRECOGNIZED_KEYWORD: u32 = 259u32;
pub const MCIERR_UNSUPPORTED_FUNCTION: u32 = 274u32;
pub const MCIERR_WAVE_INPUTSINUSE: u32 = 322u32;
pub const MCIERR_WAVE_INPUTSUNSUITABLE: u32 = 328u32;
pub const MCIERR_WAVE_INPUTUNSPECIFIED: u32 = 325u32;
pub const MCIERR_WAVE_OUTPUTSINUSE: u32 = 320u32;
pub const MCIERR_WAVE_OUTPUTSUNSUITABLE: u32 = 326u32;
pub const MCIERR_WAVE_OUTPUTUNSPECIFIED: u32 = 324u32;
pub const MCIERR_WAVE_SETINPUTINUSE: u32 = 323u32;
pub const MCIERR_WAVE_SETINPUTUNSUITABLE: u32 = 329u32;
pub const MCIERR_WAVE_SETOUTPUTINUSE: u32 = 321u32;
pub const MCIERR_WAVE_SETOUTPUTUNSUITABLE: u32 = 327u32;
pub const MCIWNDF_NOAUTOSIZEMOVIE: u32 = 4u32;
pub const MCIWNDF_NOAUTOSIZEWINDOW: u32 = 1u32;
pub const MCIWNDF_NOERRORDLG: u32 = 16384u32;
pub const MCIWNDF_NOMENU: u32 = 8u32;
pub const MCIWNDF_NOOPEN: u32 = 32768u32;
pub const MCIWNDF_NOPLAYBAR: u32 = 2u32;
pub const MCIWNDF_NOTIFYALL: u32 = 7936u32;
pub const MCIWNDF_NOTIFYANSI: u32 = 128u32;
pub const MCIWNDF_NOTIFYERROR: u32 = 4096u32;
pub const MCIWNDF_NOTIFYMEDIA: u32 = 2048u32;
pub const MCIWNDF_NOTIFYMEDIAA: u32 = 2176u32;
pub const MCIWNDF_NOTIFYMEDIAW: u32 = 2048u32;
pub const MCIWNDF_NOTIFYMODE: u32 = 256u32;
pub const MCIWNDF_NOTIFYPOS: u32 = 512u32;
pub const MCIWNDF_NOTIFYSIZE: u32 = 1024u32;
pub const MCIWNDF_RECORD: u32 = 8192u32;
pub const MCIWNDF_SHOWALL: u32 = 112u32;
pub const MCIWNDF_SHOWMODE: u32 = 64u32;
pub const MCIWNDF_SHOWNAME: u32 = 16u32;
pub const MCIWNDF_SHOWPOS: u32 = 32u32;
pub const MCIWNDM_CAN_CONFIG: u32 = 1173u32;
pub const MCIWNDM_CAN_EJECT: u32 = 1172u32;
pub const MCIWNDM_CAN_PLAY: u32 = 1168u32;
pub const MCIWNDM_CAN_RECORD: u32 = 1170u32;
pub const MCIWNDM_CAN_SAVE: u32 = 1171u32;
pub const MCIWNDM_CAN_WINDOW: u32 = 1169u32;
pub const MCIWNDM_CHANGESTYLES: u32 = 1159u32;
pub const MCIWNDM_EJECT: u32 = 1131u32;
pub const MCIWNDM_GETACTIVETIMER: u32 = 1156u32;
pub const MCIWNDM_GETALIAS: u32 = 1161u32;
pub const MCIWNDM_GETDEVICE: u32 = 1249u32;
pub const MCIWNDM_GETDEVICEA: u32 = 1149u32;
pub const MCIWNDM_GETDEVICEID: u32 = 1124u32;
pub const MCIWNDM_GETDEVICEW: u32 = 1249u32;
pub const MCIWNDM_GETEND: u32 = 1129u32;
pub const MCIWNDM_GETERROR: u32 = 1252u32;
pub const MCIWNDM_GETERRORA: u32 = 1152u32;
pub const MCIWNDM_GETERRORW: u32 = 1252u32;
pub const MCIWNDM_GETFILENAME: u32 = 1248u32;
pub const MCIWNDM_GETFILENAMEA: u32 = 1148u32;
pub const MCIWNDM_GETFILENAMEW: u32 = 1248u32;
pub const MCIWNDM_GETINACTIVETIMER: u32 = 1157u32;
pub const MCIWNDM_GETLENGTH: u32 = 1128u32;
pub const MCIWNDM_GETMODE: u32 = 1230u32;
pub const MCIWNDM_GETMODEA: u32 = 1130u32;
pub const MCIWNDM_GETMODEW: u32 = 1230u32;
pub const MCIWNDM_GETPALETTE: u32 = 1150u32;
pub const MCIWNDM_GETPOSITION: u32 = 1226u32;
pub const MCIWNDM_GETPOSITIONA: u32 = 1126u32;
pub const MCIWNDM_GETPOSITIONW: u32 = 1226u32;
pub const MCIWNDM_GETREPEAT: u32 = 1139u32;
pub const MCIWNDM_GETSPEED: u32 = 1137u32;
pub const MCIWNDM_GETSTART: u32 = 1127u32;
pub const MCIWNDM_GETSTYLES: u32 = 1160u32;
pub const MCIWNDM_GETTIMEFORMAT: u32 = 1244u32;
pub const MCIWNDM_GETTIMEFORMATA: u32 = 1144u32;
pub const MCIWNDM_GETTIMEFORMATW: u32 = 1244u32;
pub const MCIWNDM_GETVOLUME: u32 = 1135u32;
pub const MCIWNDM_GETZOOM: u32 = 1133u32;
pub const MCIWNDM_GET_DEST: u32 = 1166u32;
pub const MCIWNDM_GET_SOURCE: u32 = 1164u32;
pub const MCIWNDM_NEW: u32 = 1258u32;
pub const MCIWNDM_NEWA: u32 = 1158u32;
pub const MCIWNDM_NEWW: u32 = 1258u32;
pub const MCIWNDM_NOTIFYERROR: u32 = 1229u32;
pub const MCIWNDM_NOTIFYMEDIA: u32 = 1227u32;
pub const MCIWNDM_NOTIFYMODE: u32 = 1224u32;
pub const MCIWNDM_NOTIFYPOS: u32 = 1225u32;
pub const MCIWNDM_NOTIFYSIZE: u32 = 1226u32;
pub const MCIWNDM_OPEN: u32 = 1276u32;
pub const MCIWNDM_OPENA: u32 = 1177u32;
pub const MCIWNDM_OPENINTERFACE: u32 = 1175u32;
pub const MCIWNDM_OPENW: u32 = 1276u32;
pub const MCIWNDM_PALETTEKICK: u32 = 1174u32;
pub const MCIWNDM_PLAYFROM: u32 = 1146u32;
pub const MCIWNDM_PLAYREVERSE: u32 = 1163u32;
pub const MCIWNDM_PLAYTO: u32 = 1147u32;
pub const MCIWNDM_PUT_DEST: u32 = 1167u32;
pub const MCIWNDM_PUT_SOURCE: u32 = 1165u32;
pub const MCIWNDM_REALIZE: u32 = 1142u32;
pub const MCIWNDM_RETURNSTRING: u32 = 1262u32;
pub const MCIWNDM_RETURNSTRINGA: u32 = 1162u32;
pub const MCIWNDM_RETURNSTRINGW: u32 = 1262u32;
pub const MCIWNDM_SENDSTRING: u32 = 1225u32;
pub const MCIWNDM_SENDSTRINGA: u32 = 1125u32;
pub const MCIWNDM_SENDSTRINGW: u32 = 1225u32;
pub const MCIWNDM_SETACTIVETIMER: u32 = 1154u32;
pub const MCIWNDM_SETINACTIVETIMER: u32 = 1155u32;
pub const MCIWNDM_SETOWNER: u32 = 1176u32;
pub const MCIWNDM_SETPALETTE: u32 = 1151u32;
pub const MCIWNDM_SETREPEAT: u32 = 1138u32;
pub const MCIWNDM_SETSPEED: u32 = 1136u32;
pub const MCIWNDM_SETTIMEFORMAT: u32 = 1243u32;
pub const MCIWNDM_SETTIMEFORMATA: u32 = 1143u32;
pub const MCIWNDM_SETTIMEFORMATW: u32 = 1243u32;
pub const MCIWNDM_SETTIMERS: u32 = 1153u32;
pub const MCIWNDM_SETVOLUME: u32 = 1134u32;
pub const MCIWNDM_SETZOOM: u32 = 1132u32;
pub const MCIWNDM_VALIDATEMEDIA: u32 = 1145u32;
pub const MCIWNDOPENF_NEW: u32 = 1u32;
pub const MCIWND_END: i32 = -2i32;
pub const MCIWND_START: i32 = -1i32;
pub const MCIWND_WINDOW_CLASS: windows_sys::core::PCWSTR = windows_sys::core::w!("MCIWndClass");
pub const MCI_ANIM_GETDEVCAPS_CAN_REVERSE: i32 = 16385i32;
pub const MCI_ANIM_GETDEVCAPS_CAN_STRETCH: i32 = 16391i32;
pub const MCI_ANIM_GETDEVCAPS_FAST_RATE: i32 = 16386i32;
pub const MCI_ANIM_GETDEVCAPS_MAX_WINDOWS: i32 = 16392i32;
pub const MCI_ANIM_GETDEVCAPS_NORMAL_RATE: i32 = 16388i32;
pub const MCI_ANIM_GETDEVCAPS_PALETTES: i32 = 16390i32;
pub const MCI_ANIM_GETDEVCAPS_SLOW_RATE: i32 = 16387i32;
pub const MCI_ANIM_INFO_TEXT: i32 = 65536i32;
pub const MCI_ANIM_OPEN_NOSTATIC: i32 = 262144i32;
pub const MCI_ANIM_OPEN_PARENT: i32 = 131072i32;
pub const MCI_ANIM_OPEN_WS: i32 = 65536i32;
pub const MCI_ANIM_PLAY_FAST: i32 = 262144i32;
pub const MCI_ANIM_PLAY_REVERSE: i32 = 131072i32;
pub const MCI_ANIM_PLAY_SCAN: i32 = 1048576i32;
pub const MCI_ANIM_PLAY_SLOW: i32 = 524288i32;
pub const MCI_ANIM_PLAY_SPEED: i32 = 65536i32;
pub const MCI_ANIM_PUT_DESTINATION: i32 = 262144i32;
pub const MCI_ANIM_PUT_SOURCE: i32 = 131072i32;
pub const MCI_ANIM_REALIZE_BKGD: i32 = 131072i32;
pub const MCI_ANIM_REALIZE_NORM: i32 = 65536i32;
pub const MCI_ANIM_RECT: i32 = 65536i32;
pub const MCI_ANIM_STATUS_FORWARD: i32 = 16386i32;
pub const MCI_ANIM_STATUS_HPAL: i32 = 16388i32;
pub const MCI_ANIM_STATUS_HWND: i32 = 16387i32;
pub const MCI_ANIM_STATUS_SPEED: i32 = 16385i32;
pub const MCI_ANIM_STATUS_STRETCH: i32 = 16389i32;
pub const MCI_ANIM_STEP_FRAMES: i32 = 131072i32;
pub const MCI_ANIM_STEP_REVERSE: i32 = 65536i32;
pub const MCI_ANIM_UPDATE_HDC: i32 = 131072i32;
pub const MCI_ANIM_WHERE_DESTINATION: i32 = 262144i32;
pub const MCI_ANIM_WHERE_SOURCE: i32 = 131072i32;
pub const MCI_ANIM_WINDOW_DEFAULT: i32 = 0i32;
pub const MCI_ANIM_WINDOW_DISABLE_STRETCH: i32 = 2097152i32;
pub const MCI_ANIM_WINDOW_ENABLE_STRETCH: i32 = 1048576i32;
pub const MCI_ANIM_WINDOW_HWND: i32 = 65536i32;
pub const MCI_ANIM_WINDOW_STATE: i32 = 262144i32;
pub const MCI_ANIM_WINDOW_TEXT: i32 = 524288i32;
pub const MCI_AVI_SETVIDEO_DRAW_PROCEDURE: i32 = 32768i32;
pub const MCI_AVI_SETVIDEO_PALETTE_COLOR: i32 = 33024i32;
pub const MCI_AVI_SETVIDEO_PALETTE_HALFTONE: i32 = 65535i32;
pub const MCI_AVI_STATUS_AUDIO_BREAKS: i32 = 32771i32;
pub const MCI_AVI_STATUS_FRAMES_SKIPPED: i32 = 32769i32;
pub const MCI_AVI_STATUS_LAST_PLAY_SPEED: i32 = 32770i32;
pub const MCI_BREAK: u32 = 2065u32;
pub const MCI_BREAK_HWND: i32 = 512i32;
pub const MCI_BREAK_KEY: i32 = 256i32;
pub const MCI_BREAK_OFF: i32 = 1024i32;
pub const MCI_CAPTURE: u32 = 2160u32;
pub const MCI_CDA_STATUS_TYPE_TRACK: i32 = 16385i32;
pub const MCI_CDA_TRACK_AUDIO: u32 = 1088u32;
pub const MCI_CDA_TRACK_OTHER: u32 = 1089u32;
pub const MCI_CLOSE: u32 = 2052u32;
pub const MCI_CLOSE_DRIVER: u32 = 2050u32;
pub const MCI_COLONIZED3_RETURN: u32 = 131072u32;
pub const MCI_COLONIZED4_RETURN: u32 = 262144u32;
pub const MCI_COMMAND_HEAD: u32 = 0u32;
pub const MCI_CONFIGURE: u32 = 2170u32;
pub const MCI_CONSTANT: u32 = 8u32;
pub const MCI_COPY: u32 = 2130u32;
pub const MCI_CUE: u32 = 2096u32;
pub const MCI_CUT: u32 = 2129u32;
pub const MCI_DELETE: u32 = 2134u32;
pub const MCI_DEVTYPE_ANIMATION: u32 = 519u32;
pub const MCI_DEVTYPE_CD_AUDIO: u32 = 516u32;
pub const MCI_DEVTYPE_DAT: u32 = 517u32;
pub const MCI_DEVTYPE_DIGITAL_VIDEO: u32 = 520u32;
pub const MCI_DEVTYPE_FIRST: u32 = 513u32;
pub const MCI_DEVTYPE_FIRST_USER: u32 = 4096u32;
pub const MCI_DEVTYPE_LAST: u32 = 523u32;
pub const MCI_DEVTYPE_OTHER: u32 = 521u32;
pub const MCI_DEVTYPE_OVERLAY: u32 = 515u32;
pub const MCI_DEVTYPE_SCANNER: u32 = 518u32;
pub const MCI_DEVTYPE_SEQUENCER: u32 = 523u32;
pub const MCI_DEVTYPE_VCR: u32 = 513u32;
pub const MCI_DEVTYPE_VIDEODISC: u32 = 514u32;
pub const MCI_DEVTYPE_WAVEFORM_AUDIO: u32 = 522u32;
pub const MCI_DGV_CAPTURE_AS: i32 = 65536i32;
pub const MCI_DGV_CAPTURE_AT: i32 = 131072i32;
pub const MCI_DGV_COPY_AT: i32 = 65536i32;
pub const MCI_DGV_COPY_AUDIO_STREAM: i32 = 131072i32;
pub const MCI_DGV_COPY_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_CUE_INPUT: i32 = 65536i32;
pub const MCI_DGV_CUE_NOSHOW: i32 = 262144i32;
pub const MCI_DGV_CUE_OUTPUT: i32 = 131072i32;
pub const MCI_DGV_CUT_AT: i32 = 65536i32;
pub const MCI_DGV_CUT_AUDIO_STREAM: i32 = 131072i32;
pub const MCI_DGV_CUT_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_DELETE_AT: i32 = 65536i32;
pub const MCI_DGV_DELETE_AUDIO_STREAM: i32 = 131072i32;
pub const MCI_DGV_DELETE_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_FF_AVI: i32 = 16385i32;
pub const MCI_DGV_FF_AVSS: i32 = 16384i32;
pub const MCI_DGV_FF_DIB: i32 = 16386i32;
pub const MCI_DGV_FF_JFIF: i32 = 16390i32;
pub const MCI_DGV_FF_JPEG: i32 = 16388i32;
pub const MCI_DGV_FF_MPEG: i32 = 16391i32;
pub const MCI_DGV_FF_RDIB: i32 = 16387i32;
pub const MCI_DGV_FF_RJPEG: i32 = 16389i32;
pub const MCI_DGV_FILE_MODE_EDITING: u32 = 3u32;
pub const MCI_DGV_FILE_MODE_EDITING_S: i32 = 32774i32;
pub const MCI_DGV_FILE_MODE_IDLE: u32 = 4u32;
pub const MCI_DGV_FILE_MODE_IDLE_S: i32 = 32775i32;
pub const MCI_DGV_FILE_MODE_LOADING: u32 = 2u32;
pub const MCI_DGV_FILE_MODE_LOADING_S: i32 = 32773i32;
pub const MCI_DGV_FILE_MODE_SAVING: u32 = 1u32;
pub const MCI_DGV_FILE_MODE_SAVING_S: i32 = 32772i32;
pub const MCI_DGV_FILE_S: i32 = 32770i32;
pub const MCI_DGV_FREEZE_AT: i32 = 65536i32;
pub const MCI_DGV_FREEZE_OUTSIDE: i32 = 131072i32;
pub const MCI_DGV_GETDEVCAPS_CAN_FREEZE: i32 = 16386i32;
pub const MCI_DGV_GETDEVCAPS_CAN_LOCK: i32 = 16384i32;
pub const MCI_DGV_GETDEVCAPS_CAN_REVERSE: i32 = 16388i32;
pub const MCI_DGV_GETDEVCAPS_CAN_STRETCH: i32 = 16385i32;
pub const MCI_DGV_GETDEVCAPS_CAN_STR_IN: i32 = 16392i32;
pub const MCI_DGV_GETDEVCAPS_CAN_TEST: i32 = 16393i32;
pub const MCI_DGV_GETDEVCAPS_HAS_STILL: i32 = 16389i32;
pub const MCI_DGV_GETDEVCAPS_MAXIMUM_RATE: i32 = 16394i32;
pub const MCI_DGV_GETDEVCAPS_MAX_WINDOWS: i32 = 16387i32;
pub const MCI_DGV_GETDEVCAPS_MINIMUM_RATE: i32 = 16395i32;
pub const MCI_DGV_GETDEVCAPS_PALETTES: i32 = 16390i32;
pub const MCI_DGV_INFO_AUDIO_ALG: i32 = 16388i32;
pub const MCI_DGV_INFO_AUDIO_QUALITY: i32 = 16385i32;
pub const MCI_DGV_INFO_ITEM: i32 = 131072i32;
pub const MCI_DGV_INFO_STILL_ALG: i32 = 16389i32;
pub const MCI_DGV_INFO_STILL_QUALITY: i32 = 16386i32;
pub const MCI_DGV_INFO_TEXT: i32 = 65536i32;
pub const MCI_DGV_INFO_USAGE: i32 = 16384i32;
pub const MCI_DGV_INFO_VIDEO_ALG: i32 = 16390i32;
pub const MCI_DGV_INFO_VIDEO_QUALITY: i32 = 16387i32;
pub const MCI_DGV_INPUT_S: i32 = 32771i32;
pub const MCI_DGV_LIST_ALG: i32 = 524288i32;
pub const MCI_DGV_LIST_AUDIO_ALG: i32 = 16384i32;
pub const MCI_DGV_LIST_AUDIO_QUALITY: i32 = 16385i32;
pub const MCI_DGV_LIST_AUDIO_STREAM: i32 = 16386i32;
pub const MCI_DGV_LIST_COUNT: i32 = 131072i32;
pub const MCI_DGV_LIST_ITEM: i32 = 65536i32;
pub const MCI_DGV_LIST_NUMBER: i32 = 262144i32;
pub const MCI_DGV_LIST_STILL_ALG: i32 = 16387i32;
pub const MCI_DGV_LIST_STILL_QUALITY: i32 = 16388i32;
pub const MCI_DGV_LIST_VIDEO_ALG: i32 = 16389i32;
pub const MCI_DGV_LIST_VIDEO_QUALITY: i32 = 16390i32;
pub const MCI_DGV_LIST_VIDEO_SOURCE: i32 = 16392i32;
pub const MCI_DGV_LIST_VIDEO_STREAM: i32 = 16391i32;
pub const MCI_DGV_METHOD_DIRECT: i32 = 40962i32;
pub const MCI_DGV_METHOD_POST: i32 = 40961i32;
pub const MCI_DGV_METHOD_PRE: i32 = 40960i32;
pub const MCI_DGV_MONITOR_FILE: i32 = 16385i32;
pub const MCI_DGV_MONITOR_INPUT: i32 = 16384i32;
pub const MCI_DGV_MONITOR_METHOD: i32 = 65536i32;
pub const MCI_DGV_MONITOR_SOURCE: i32 = 131072i32;
pub const MCI_DGV_OPEN_16BIT: i32 = 524288i32;
pub const MCI_DGV_OPEN_32BIT: i32 = 1048576i32;
pub const MCI_DGV_OPEN_NOSTATIC: i32 = 262144i32;
pub const MCI_DGV_OPEN_PARENT: i32 = 131072i32;
pub const MCI_DGV_OPEN_WS: i32 = 65536i32;
pub const MCI_DGV_PASTE_AT: i32 = 65536i32;
pub const MCI_DGV_PASTE_AUDIO_STREAM: i32 = 131072i32;
pub const MCI_DGV_PASTE_INSERT: i32 = 524288i32;
pub const MCI_DGV_PASTE_OVERWRITE: i32 = 1048576i32;
pub const MCI_DGV_PASTE_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_PLAY_REPEAT: i32 = 65536i32;
pub const MCI_DGV_PLAY_REVERSE: i32 = 131072i32;
pub const MCI_DGV_PUT_CLIENT: i32 = 4194304i32;
pub const MCI_DGV_PUT_DESTINATION: i32 = 262144i32;
pub const MCI_DGV_PUT_FRAME: i32 = 524288i32;
pub const MCI_DGV_PUT_SOURCE: i32 = 131072i32;
pub const MCI_DGV_PUT_VIDEO: i32 = 1048576i32;
pub const MCI_DGV_PUT_WINDOW: i32 = 2097152i32;
pub const MCI_DGV_REALIZE_BKGD: i32 = 131072i32;
pub const MCI_DGV_REALIZE_NORM: i32 = 65536i32;
pub const MCI_DGV_RECORD_AUDIO_STREAM: i32 = 262144i32;
pub const MCI_DGV_RECORD_HOLD: i32 = 131072i32;
pub const MCI_DGV_RECORD_VIDEO_STREAM: i32 = 524288i32;
pub const MCI_DGV_RECT: i32 = 65536i32;
pub const MCI_DGV_RESERVE_IN: i32 = 65536i32;
pub const MCI_DGV_RESERVE_SIZE: i32 = 131072i32;
pub const MCI_DGV_RESTORE_AT: i32 = 131072i32;
pub const MCI_DGV_RESTORE_FROM: i32 = 65536i32;
pub const MCI_DGV_SAVE_ABORT: i32 = 131072i32;
pub const MCI_DGV_SAVE_KEEPRESERVE: i32 = 262144i32;
pub const MCI_DGV_SETAUDIO_ALG: i32 = 262144i32;
pub const MCI_DGV_SETAUDIO_AVGBYTESPERSEC: i32 = 16390i32;
pub const MCI_DGV_SETAUDIO_BASS: i32 = 16385i32;
pub const MCI_DGV_SETAUDIO_BITSPERSAMPLE: i32 = 16392i32;
pub const MCI_DGV_SETAUDIO_BLOCKALIGN: i32 = 16391i32;
pub const MCI_DGV_SETAUDIO_CLOCKTIME: i32 = 131072i32;
pub const MCI_DGV_SETAUDIO_INPUT: i32 = 33554432i32;
pub const MCI_DGV_SETAUDIO_ITEM: i32 = 8388608i32;
pub const MCI_DGV_SETAUDIO_LEFT: i32 = 2097152i32;
pub const MCI_DGV_SETAUDIO_OUTPUT: i32 = 67108864i32;
pub const MCI_DGV_SETAUDIO_OVER: i32 = 65536i32;
pub const MCI_DGV_SETAUDIO_QUALITY: i32 = 524288i32;
pub const MCI_DGV_SETAUDIO_RECORD: i32 = 1048576i32;
pub const MCI_DGV_SETAUDIO_RIGHT: i32 = 4194304i32;
pub const MCI_DGV_SETAUDIO_SAMPLESPERSEC: i32 = 16389i32;
pub const MCI_DGV_SETAUDIO_SOURCE: i32 = 16388i32;
pub const MCI_DGV_SETAUDIO_SOURCE_AVERAGE: i32 = 16384i32;
pub const MCI_DGV_SETAUDIO_SOURCE_LEFT: i32 = 1i32;
pub const MCI_DGV_SETAUDIO_SOURCE_RIGHT: i32 = 2i32;
pub const MCI_DGV_SETAUDIO_SOURCE_STEREO: i32 = 0i32;
pub const MCI_DGV_SETAUDIO_SRC_AVERAGE_S: i32 = 32802i32;
pub const MCI_DGV_SETAUDIO_SRC_LEFT_S: i32 = 32800i32;
pub const MCI_DGV_SETAUDIO_SRC_RIGHT_S: i32 = 32801i32;
pub const MCI_DGV_SETAUDIO_SRC_STEREO_S: i32 = 32803i32;
pub const MCI_DGV_SETAUDIO_STREAM: i32 = 16387i32;
pub const MCI_DGV_SETAUDIO_TREBLE: i32 = 16384i32;
pub const MCI_DGV_SETAUDIO_VALUE: i32 = 16777216i32;
pub const MCI_DGV_SETAUDIO_VOLUME: i32 = 16386i32;
pub const MCI_DGV_SETVIDEO_ALG: i32 = 131072i32;
pub const MCI_DGV_SETVIDEO_BITSPERPEL: i32 = 16396i32;
pub const MCI_DGV_SETVIDEO_BRIGHTNESS: i32 = 16384i32;
pub const MCI_DGV_SETVIDEO_CLOCKTIME: i32 = 262144i32;
pub const MCI_DGV_SETVIDEO_COLOR: i32 = 16385i32;
pub const MCI_DGV_SETVIDEO_CONTRAST: i32 = 16386i32;
pub const MCI_DGV_SETVIDEO_FRAME_RATE: i32 = 16392i32;
pub const MCI_DGV_SETVIDEO_GAMMA: i32 = 16389i32;
pub const MCI_DGV_SETVIDEO_INPUT: i32 = 33554432i32;
pub const MCI_DGV_SETVIDEO_ITEM: i32 = 1048576i32;
pub const MCI_DGV_SETVIDEO_KEY_COLOR: i32 = 16395i32;
pub const MCI_DGV_SETVIDEO_KEY_INDEX: i32 = 16394i32;
pub const MCI_DGV_SETVIDEO_OUTPUT: i32 = 67108864i32;
pub const MCI_DGV_SETVIDEO_OVER: i32 = 2097152i32;
pub const MCI_DGV_SETVIDEO_PALHANDLE: i32 = 16391i32;
pub const MCI_DGV_SETVIDEO_QUALITY: i32 = 65536i32;
pub const MCI_DGV_SETVIDEO_RECORD: i32 = 4194304i32;
pub const MCI_DGV_SETVIDEO_SHARPNESS: i32 = 16388i32;
pub const MCI_DGV_SETVIDEO_SOURCE: i32 = 16393i32;
pub const MCI_DGV_SETVIDEO_SRC_GENERIC: i32 = 16389i32;
pub const MCI_DGV_SETVIDEO_SRC_GENERIC_S: i32 = 32789i32;
pub const MCI_DGV_SETVIDEO_SRC_NTSC: i32 = 16384i32;
pub const MCI_DGV_SETVIDEO_SRC_NTSC_S: i32 = 32784i32;
pub const MCI_DGV_SETVIDEO_SRC_NUMBER: i32 = 524288i32;
pub const MCI_DGV_SETVIDEO_SRC_PAL: i32 = 16387i32;
pub const MCI_DGV_SETVIDEO_SRC_PAL_S: i32 = 32787i32;
pub const MCI_DGV_SETVIDEO_SRC_RGB: i32 = 16385i32;
pub const MCI_DGV_SETVIDEO_SRC_RGB_S: i32 = 32785i32;
pub const MCI_DGV_SETVIDEO_SRC_SECAM: i32 = 16388i32;
pub const MCI_DGV_SETVIDEO_SRC_SECAM_S: i32 = 32788i32;
pub const MCI_DGV_SETVIDEO_SRC_SVIDEO: i32 = 16386i32;
pub const MCI_DGV_SETVIDEO_SRC_SVIDEO_S: i32 = 32786i32;
pub const MCI_DGV_SETVIDEO_STILL: i32 = 8388608i32;
pub const MCI_DGV_SETVIDEO_STREAM: i32 = 16390i32;
pub const MCI_DGV_SETVIDEO_TINT: i32 = 16387i32;
pub const MCI_DGV_SETVIDEO_VALUE: i32 = 16777216i32;
pub const MCI_DGV_SET_FILEFORMAT: i32 = 524288i32;
pub const MCI_DGV_SET_SEEK_EXACTLY: i32 = 65536i32;
pub const MCI_DGV_SET_SPEED: i32 = 131072i32;
pub const MCI_DGV_SET_STILL: i32 = 262144i32;
pub const MCI_DGV_SIGNAL_AT: i32 = 65536i32;
pub const MCI_DGV_SIGNAL_CANCEL: i32 = 524288i32;
pub const MCI_DGV_SIGNAL_EVERY: i32 = 131072i32;
pub const MCI_DGV_SIGNAL_POSITION: i32 = 1048576i32;
pub const MCI_DGV_SIGNAL_USERVAL: i32 = 262144i32;
pub const MCI_DGV_STATUS_AUDIO: i32 = 16404i32;
pub const MCI_DGV_STATUS_AUDIO_INPUT: i32 = 16384i32;
pub const MCI_DGV_STATUS_AUDIO_RECORD: i32 = 16410i32;
pub const MCI_DGV_STATUS_AUDIO_SOURCE: i32 = 16393i32;
pub const MCI_DGV_STATUS_AUDIO_STREAM: i32 = 16429i32;
pub const MCI_DGV_STATUS_AVGBYTESPERSEC: i32 = 16424i32;
pub const MCI_DGV_STATUS_BASS: i32 = 16399i32;
pub const MCI_DGV_STATUS_BITSPERPEL: i32 = 16427i32;
pub const MCI_DGV_STATUS_BITSPERSAMPLE: i32 = 16426i32;
pub const MCI_DGV_STATUS_BLOCKALIGN: i32 = 16425i32;
pub const MCI_DGV_STATUS_BRIGHTNESS: i32 = 16389i32;
pub const MCI_DGV_STATUS_COLOR: i32 = 16390i32;
pub const MCI_DGV_STATUS_CONTRAST: i32 = 16391i32;
pub const MCI_DGV_STATUS_DISKSPACE: i32 = 2097152i32;
pub const MCI_DGV_STATUS_FILEFORMAT: i32 = 16392i32;
pub const MCI_DGV_STATUS_FILE_COMPLETION: i32 = 16416i32;
pub const MCI_DGV_STATUS_FILE_MODE: i32 = 16415i32;
pub const MCI_DGV_STATUS_FORWARD: i32 = 16428i32;
pub const MCI_DGV_STATUS_FRAME_RATE: i32 = 16398i32;
pub const MCI_DGV_STATUS_GAMMA: i32 = 16394i32;
pub const MCI_DGV_STATUS_HPAL: i32 = 16388i32;
pub const MCI_DGV_STATUS_HWND: i32 = 16385i32;
pub const MCI_DGV_STATUS_INPUT: i32 = 4194304i32;
pub const MCI_DGV_STATUS_KEY_COLOR: i32 = 16421i32;
pub const MCI_DGV_STATUS_KEY_INDEX: i32 = 16420i32;
pub const MCI_DGV_STATUS_LEFT: i32 = 524288i32;
pub const MCI_DGV_STATUS_MONITOR: i32 = 16395i32;
pub const MCI_DGV_STATUS_MONITOR_METHOD: i32 = 16396i32;
pub const MCI_DGV_STATUS_NOMINAL: i32 = 131072i32;
pub const MCI_DGV_STATUS_OUTPUT: i32 = 8388608i32;
pub const MCI_DGV_STATUS_PAUSE_MODE: i32 = 16422i32;
pub const MCI_DGV_STATUS_RECORD: i32 = 16777216i32;
pub const MCI_DGV_STATUS_REFERENCE: i32 = 262144i32;
pub const MCI_DGV_STATUS_RIGHT: i32 = 1048576i32;
pub const MCI_DGV_STATUS_SAMPLESPERSEC: i32 = 16423i32;
pub const MCI_DGV_STATUS_SEEK_EXACTLY: i32 = 16401i32;
pub const MCI_DGV_STATUS_SHARPNESS: i32 = 16402i32;
pub const MCI_DGV_STATUS_SIZE: i32 = 16400i32;
pub const MCI_DGV_STATUS_SMPTE: i32 = 16403i32;
pub const MCI_DGV_STATUS_SPEED: i32 = 16387i32;
pub const MCI_DGV_STATUS_STILL_FILEFORMAT: i32 = 16413i32;
pub const MCI_DGV_STATUS_TINT: i32 = 16405i32;
pub const MCI_DGV_STATUS_TREBLE: i32 = 16406i32;
pub const MCI_DGV_STATUS_UNSAVED: i32 = 16407i32;
pub const MCI_DGV_STATUS_VIDEO: i32 = 16408i32;
pub const MCI_DGV_STATUS_VIDEO_RECORD: i32 = 16412i32;
pub const MCI_DGV_STATUS_VIDEO_SOURCE: i32 = 16411i32;
pub const MCI_DGV_STATUS_VIDEO_SRC_NUM: i32 = 16414i32;
pub const MCI_DGV_STATUS_VIDEO_STREAM: i32 = 16430i32;
pub const MCI_DGV_STATUS_VOLUME: i32 = 16409i32;
pub const MCI_DGV_STATUS_WINDOW_MAXIMIZED: i32 = 16419i32;
pub const MCI_DGV_STATUS_WINDOW_MINIMIZED: i32 = 16418i32;
pub const MCI_DGV_STATUS_WINDOW_VISIBLE: i32 = 16417i32;
pub const MCI_DGV_STEP_FRAMES: i32 = 131072i32;
pub const MCI_DGV_STEP_REVERSE: i32 = 65536i32;
pub const MCI_DGV_STOP_HOLD: i32 = 65536i32;
pub const MCI_DGV_UPDATE_HDC: i32 = 131072i32;
pub const MCI_DGV_UPDATE_PAINT: i32 = 262144i32;
pub const MCI_DGV_WHERE_DESTINATION: i32 = 262144i32;
pub const MCI_DGV_WHERE_FRAME: i32 = 524288i32;
pub const MCI_DGV_WHERE_MAX: i32 = 4194304i32;
pub const MCI_DGV_WHERE_SOURCE: i32 = 131072i32;
pub const MCI_DGV_WHERE_VIDEO: i32 = 1048576i32;
pub const MCI_DGV_WHERE_WINDOW: i32 = 2097152i32;
pub const MCI_DGV_WINDOW_DEFAULT: i32 = 0i32;
pub const MCI_DGV_WINDOW_HWND: i32 = 65536i32;
pub const MCI_DGV_WINDOW_STATE: i32 = 262144i32;
pub const MCI_DGV_WINDOW_TEXT: i32 = 524288i32;
pub const MCI_END_COMMAND: u32 = 3u32;
pub const MCI_END_COMMAND_LIST: u32 = 6u32;
pub const MCI_END_CONSTANT: u32 = 9u32;
pub const MCI_ESCAPE: u32 = 2053u32;
pub const MCI_FALSE: u32 = 531u32;
pub const MCI_FIRST: u32 = 2048u32;
pub const MCI_FLAG: u32 = 5u32;
pub const MCI_FORMAT_BYTES: u32 = 8u32;
pub const MCI_FORMAT_BYTES_S: u32 = 541u32;
pub const MCI_FORMAT_FRAMES: u32 = 3u32;
pub const MCI_FORMAT_FRAMES_S: u32 = 536u32;
pub const MCI_FORMAT_HMS: u32 = 1u32;
pub const MCI_FORMAT_HMS_S: u32 = 534u32;
pub const MCI_FORMAT_MILLISECONDS: u32 = 0u32;
pub const MCI_FORMAT_MILLISECONDS_S: u32 = 533u32;
pub const MCI_FORMAT_MSF: u32 = 2u32;
pub const MCI_FORMAT_MSF_S: u32 = 535u32;
pub const MCI_FORMAT_SAMPLES: u32 = 9u32;
pub const MCI_FORMAT_SAMPLES_S: u32 = 542u32;
pub const MCI_FORMAT_SMPTE_24: u32 = 4u32;
pub const MCI_FORMAT_SMPTE_24_S: u32 = 537u32;
pub const MCI_FORMAT_SMPTE_25: u32 = 5u32;
pub const MCI_FORMAT_SMPTE_25_S: u32 = 538u32;
pub const MCI_FORMAT_SMPTE_30: u32 = 6u32;
pub const MCI_FORMAT_SMPTE_30DROP: u32 = 7u32;
pub const MCI_FORMAT_SMPTE_30DROP_S: u32 = 540u32;
pub const MCI_FORMAT_SMPTE_30_S: u32 = 539u32;
pub const MCI_FORMAT_TMSF: u32 = 10u32;
pub const MCI_FORMAT_TMSF_S: u32 = 543u32;
pub const MCI_FREEZE: u32 = 2116u32;
pub const MCI_FROM: i32 = 4i32;
pub const MCI_GETDEVCAPS: u32 = 2059u32;
pub const MCI_GETDEVCAPS_CAN_EJECT: i32 = 7i32;
pub const MCI_GETDEVCAPS_CAN_PLAY: i32 = 8i32;
pub const MCI_GETDEVCAPS_CAN_RECORD: i32 = 1i32;
pub const MCI_GETDEVCAPS_CAN_SAVE: i32 = 9i32;
pub const MCI_GETDEVCAPS_COMPOUND_DEVICE: i32 = 6i32;
pub const MCI_GETDEVCAPS_DEVICE_TYPE: i32 = 4i32;
pub const MCI_GETDEVCAPS_HAS_AUDIO: i32 = 2i32;
pub const MCI_GETDEVCAPS_HAS_VIDEO: i32 = 3i32;
pub const MCI_GETDEVCAPS_ITEM: i32 = 256i32;
pub const MCI_GETDEVCAPS_USES_FILES: i32 = 5i32;
pub const MCI_HDC: u32 = 12u32;
pub const MCI_HPAL: u32 = 11u32;
pub const MCI_HWND: u32 = 10u32;
pub const MCI_INFO: u32 = 2058u32;
pub const MCI_INFO_COPYRIGHT: i32 = 8192i32;
pub const MCI_INFO_FILE: i32 = 512i32;
pub const MCI_INFO_MEDIA_IDENTITY: i32 = 2048i32;
pub const MCI_INFO_MEDIA_UPC: i32 = 1024i32;
pub const MCI_INFO_NAME: i32 = 4096i32;
pub const MCI_INFO_PRODUCT: i32 = 256i32;
pub const MCI_INFO_VERSION: i32 = 1024i32;
pub const MCI_INTEGER: u32 = 2u32;
pub const MCI_INTEGER64: u32 = 13u32;
pub const MCI_INTEGER_RETURNED: u32 = 524288u32;
pub const MCI_LAST: u32 = 4095u32;
pub const MCI_LIST: u32 = 2168u32;
pub const MCI_LOAD: u32 = 2128u32;
pub const MCI_LOAD_FILE: i32 = 256i32;
pub const MCI_MAX_DEVICE_TYPE_LENGTH: u32 = 80u32;
pub const MCI_MCIAVI_PLAY_FULLBY2: i32 = 67108864i32;
pub const MCI_MCIAVI_PLAY_FULLSCREEN: i32 = 33554432i32;
pub const MCI_MCIAVI_PLAY_WINDOW: i32 = 16777216i32;
pub const MCI_MODE_NOT_READY: u32 = 524u32;
pub const MCI_MODE_OPEN: u32 = 530u32;
pub const MCI_MODE_PAUSE: u32 = 529u32;
pub const MCI_MODE_PLAY: u32 = 526u32;
pub const MCI_MODE_RECORD: u32 = 527u32;
pub const MCI_MODE_SEEK: u32 = 528u32;
pub const MCI_MODE_STOP: u32 = 525u32;
pub const MCI_MONITOR: u32 = 2161u32;
pub const MCI_NOTIFY: i32 = 1i32;
pub const MCI_NOTIFY_ABORTED: u32 = 4u32;
pub const MCI_NOTIFY_FAILURE: u32 = 8u32;
pub const MCI_NOTIFY_SUCCESSFUL: u32 = 1u32;
pub const MCI_NOTIFY_SUPERSEDED: u32 = 2u32;
pub const MCI_OFF: u32 = 0u32;
pub const MCI_OFF_S: i32 = 32769i32;
pub const MCI_ON: u32 = 1u32;
pub const MCI_ON_S: i32 = 32768i32;
pub const MCI_OPEN: u32 = 2051u32;
pub const MCI_OPEN_ALIAS: i32 = 1024i32;
pub const MCI_OPEN_DRIVER: u32 = 2049u32;
pub const MCI_OPEN_ELEMENT: i32 = 512i32;
pub const MCI_OPEN_ELEMENT_ID: i32 = 2048i32;
pub const MCI_OPEN_SHAREABLE: i32 = 256i32;
pub const MCI_OPEN_TYPE: i32 = 8192i32;
pub const MCI_OPEN_TYPE_ID: i32 = 4096i32;
pub const MCI_OVLY_GETDEVCAPS_CAN_FREEZE: i32 = 16386i32;
pub const MCI_OVLY_GETDEVCAPS_CAN_STRETCH: i32 = 16385i32;
pub const MCI_OVLY_GETDEVCAPS_MAX_WINDOWS: i32 = 16387i32;
pub const MCI_OVLY_INFO_TEXT: i32 = 65536i32;
pub const MCI_OVLY_OPEN_PARENT: i32 = 131072i32;
pub const MCI_OVLY_OPEN_WS: i32 = 65536i32;
pub const MCI_OVLY_PUT_DESTINATION: i32 = 262144i32;
pub const MCI_OVLY_PUT_FRAME: i32 = 524288i32;
pub const MCI_OVLY_PUT_SOURCE: i32 = 131072i32;
pub const MCI_OVLY_PUT_VIDEO: i32 = 1048576i32;
pub const MCI_OVLY_RECT: i32 = 65536i32;
pub const MCI_OVLY_STATUS_HWND: i32 = 16385i32;
pub const MCI_OVLY_STATUS_STRETCH: i32 = 16386i32;
pub const MCI_OVLY_WHERE_DESTINATION: i32 = 262144i32;
pub const MCI_OVLY_WHERE_FRAME: i32 = 524288i32;
pub const MCI_OVLY_WHERE_SOURCE: i32 = 131072i32;
pub const MCI_OVLY_WHERE_VIDEO: i32 = 1048576i32;
pub const MCI_OVLY_WINDOW_DEFAULT: i32 = 0i32;
pub const MCI_OVLY_WINDOW_DISABLE_STRETCH: i32 = 2097152i32;
pub const MCI_OVLY_WINDOW_ENABLE_STRETCH: i32 = 1048576i32;
pub const MCI_OVLY_WINDOW_HWND: i32 = 65536i32;
pub const MCI_OVLY_WINDOW_STATE: i32 = 262144i32;
pub const MCI_OVLY_WINDOW_TEXT: i32 = 524288i32;
pub const MCI_PASTE: u32 = 2131u32;
pub const MCI_PAUSE: u32 = 2057u32;
pub const MCI_PLAY: u32 = 2054u32;
pub const MCI_PUT: u32 = 2114u32;
pub const MCI_QUALITY: u32 = 2167u32;
pub const MCI_QUALITY_ALG: i32 = 262144i32;
pub const MCI_QUALITY_DIALOG: i32 = 524288i32;
pub const MCI_QUALITY_HANDLE: i32 = 1048576i32;
pub const MCI_QUALITY_ITEM: i32 = 65536i32;
pub const MCI_QUALITY_ITEM_AUDIO: i32 = 16384i32;
pub const MCI_QUALITY_ITEM_STILL: i32 = 16385i32;
pub const MCI_QUALITY_ITEM_VIDEO: i32 = 16386i32;
pub const MCI_QUALITY_NAME: i32 = 131072i32;
pub const MCI_REALIZE: u32 = 2112u32;
pub const MCI_RECORD: u32 = 2063u32;
pub const MCI_RECORD_INSERT: i32 = 256i32;
pub const MCI_RECORD_OVERWRITE: i32 = 512i32;
pub const MCI_RECT: u32 = 7u32;
pub const MCI_RESERVE: u32 = 2162u32;
pub const MCI_RESOURCE_DRIVER: u32 = 1048576u32;
pub const MCI_RESOURCE_RETURNED: u32 = 65536u32;
pub const MCI_RESTORE: u32 = 2171u32;
pub const MCI_RESUME: u32 = 2133u32;
pub const MCI_RETURN: u32 = 4u32;
pub const MCI_SAVE: u32 = 2067u32;
pub const MCI_SAVE_FILE: i32 = 256i32;
pub const MCI_SECTION: windows_sys::core::PCWSTR = windows_sys::core::w!("MCI32");
pub const MCI_SEEK: u32 = 2055u32;
pub const MCI_SEEK_TO_END: i32 = 512i32;
pub const MCI_SEEK_TO_START: i32 = 256i32;
pub const MCI_SEQ_FILE: u32 = 16386u32;
pub const MCI_SEQ_FILE_S: u32 = 1222u32;
pub const MCI_SEQ_FORMAT_SONGPTR: u32 = 16385u32;
pub const MCI_SEQ_FORMAT_SONGPTR_S: u32 = 1225u32;
pub const MCI_SEQ_MAPPER: u32 = 65535u32;
pub const MCI_SEQ_MAPPER_S: u32 = 1221u32;
pub const MCI_SEQ_MIDI: u32 = 16387u32;
pub const MCI_SEQ_MIDI_S: u32 = 1223u32;
pub const MCI_SEQ_NONE: u32 = 65533u32;
pub const MCI_SEQ_NONE_S: u32 = 1226u32;
pub const MCI_SEQ_SET_MASTER: i32 = 524288i32;
pub const MCI_SEQ_SET_OFFSET: i32 = 16777216i32;
pub const MCI_SEQ_SET_PORT: i32 = 131072i32;
pub const MCI_SEQ_SET_SLAVE: i32 = 262144i32;
pub const MCI_SEQ_SET_TEMPO: i32 = 65536i32;
pub const MCI_SEQ_SMPTE: u32 = 16388u32;
pub const MCI_SEQ_SMPTE_S: u32 = 1224u32;
pub const MCI_SEQ_STATUS_COPYRIGHT: i32 = 16396i32;
pub const MCI_SEQ_STATUS_DIVTYPE: i32 = 16394i32;
pub const MCI_SEQ_STATUS_MASTER: i32 = 16392i32;
pub const MCI_SEQ_STATUS_NAME: i32 = 16395i32;
pub const MCI_SEQ_STATUS_OFFSET: i32 = 16393i32;
pub const MCI_SEQ_STATUS_PORT: i32 = 16387i32;
pub const MCI_SEQ_STATUS_SLAVE: i32 = 16391i32;
pub const MCI_SEQ_STATUS_TEMPO: i32 = 16386i32;
pub const MCI_SET: u32 = 2061u32;
pub const MCI_SETAUDIO: u32 = 2163u32;
pub const MCI_SETVIDEO: u32 = 2166u32;
pub const MCI_SET_AUDIO: i32 = 2048i32;
pub const MCI_SET_AUDIO_ALL: i32 = 0i32;
pub const MCI_SET_AUDIO_LEFT: i32 = 1i32;
pub const MCI_SET_AUDIO_RIGHT: i32 = 2i32;
pub const MCI_SET_DOOR_CLOSED: i32 = 512i32;
pub const MCI_SET_DOOR_OPEN: i32 = 256i32;
pub const MCI_SET_OFF: i32 = 16384i32;
pub const MCI_SET_ON: i32 = 8192i32;
pub const MCI_SET_TIME_FORMAT: i32 = 1024i32;
pub const MCI_SET_VIDEO: i32 = 4096i32;
pub const MCI_SIGNAL: u32 = 2165u32;
pub const MCI_SPIN: u32 = 2060u32;
pub const MCI_STATUS: u32 = 2068u32;
pub const MCI_STATUS_CURRENT_TRACK: i32 = 8i32;
pub const MCI_STATUS_ITEM: i32 = 256i32;
pub const MCI_STATUS_LENGTH: i32 = 1i32;
pub const MCI_STATUS_MEDIA_PRESENT: i32 = 5i32;
pub const MCI_STATUS_MODE: i32 = 4i32;
pub const MCI_STATUS_NUMBER_OF_TRACKS: i32 = 3i32;
pub const MCI_STATUS_POSITION: i32 = 2i32;
pub const MCI_STATUS_READY: i32 = 7i32;
pub const MCI_STATUS_START: i32 = 512i32;
pub const MCI_STATUS_TIME_FORMAT: i32 = 6i32;
pub const MCI_STEP: u32 = 2062u32;
pub const MCI_STOP: u32 = 2056u32;
pub const MCI_STRING: u32 = 1u32;
pub const MCI_SYSINFO: u32 = 2064u32;
pub const MCI_SYSINFO_INSTALLNAME: i32 = 2048i32;
pub const MCI_SYSINFO_NAME: i32 = 1024i32;
pub const MCI_SYSINFO_OPEN: i32 = 512i32;
pub const MCI_SYSINFO_QUANTITY: i32 = 256i32;
pub const MCI_TEST: i32 = 32i32;
pub const MCI_TO: i32 = 8i32;
pub const MCI_TRACK: i32 = 16i32;
pub const MCI_TRUE: u32 = 532u32;
pub const MCI_UNDO: u32 = 2169u32;
pub const MCI_UNFREEZE: u32 = 2117u32;
pub const MCI_UPDATE: u32 = 2132u32;
pub const MCI_USER_MESSAGES: u32 = 3072u32;
pub const MCI_VD_ESCAPE_STRING: i32 = 256i32;
pub const MCI_VD_FORMAT_TRACK: u32 = 16385u32;
pub const MCI_VD_FORMAT_TRACK_S: u32 = 1029u32;
pub const MCI_VD_GETDEVCAPS_CAN_REVERSE: i32 = 16386i32;
pub const MCI_VD_GETDEVCAPS_CAV: i32 = 131072i32;
pub const MCI_VD_GETDEVCAPS_CLV: i32 = 65536i32;
pub const MCI_VD_GETDEVCAPS_FAST_RATE: i32 = 16387i32;
pub const MCI_VD_GETDEVCAPS_NORMAL_RATE: i32 = 16389i32;
pub const MCI_VD_GETDEVCAPS_SLOW_RATE: i32 = 16388i32;
pub const MCI_VD_MEDIA_CAV: u32 = 1027u32;
pub const MCI_VD_MEDIA_CLV: u32 = 1026u32;
pub const MCI_VD_MEDIA_OTHER: u32 = 1028u32;
pub const MCI_VD_MODE_PARK: u32 = 1025u32;
pub const MCI_VD_PLAY_FAST: i32 = 131072i32;
pub const MCI_VD_PLAY_REVERSE: i32 = 65536i32;
pub const MCI_VD_PLAY_SCAN: i32 = 524288i32;
pub const MCI_VD_PLAY_SLOW: i32 = 1048576i32;
pub const MCI_VD_PLAY_SPEED: i32 = 262144i32;
pub const MCI_VD_SEEK_REVERSE: i32 = 65536i32;
pub const MCI_VD_SPIN_DOWN: i32 = 131072i32;
pub const MCI_VD_SPIN_UP: i32 = 65536i32;
pub const MCI_VD_STATUS_DISC_SIZE: i32 = 16390i32;
pub const MCI_VD_STATUS_FORWARD: i32 = 16387i32;
pub const MCI_VD_STATUS_MEDIA_TYPE: i32 = 16388i32;
pub const MCI_VD_STATUS_SIDE: i32 = 16389i32;
pub const MCI_VD_STATUS_SPEED: i32 = 16386i32;
pub const MCI_VD_STEP_FRAMES: i32 = 65536i32;
pub const MCI_VD_STEP_REVERSE: i32 = 131072i32;
pub const MCI_WAIT: i32 = 2i32;
pub const MCI_WAVE_GETDEVCAPS_INPUTS: i32 = 16385i32;
pub const MCI_WAVE_GETDEVCAPS_OUTPUTS: i32 = 16386i32;
pub const MCI_WAVE_INPUT: i32 = 4194304i32;
pub const MCI_WAVE_MAPPER: u32 = 1153u32;
pub const MCI_WAVE_OPEN_BUFFER: i32 = 65536i32;
pub const MCI_WAVE_OUTPUT: i32 = 8388608i32;
pub const MCI_WAVE_PCM: u32 = 1152u32;
pub const MCI_WAVE_SET_ANYINPUT: i32 = 67108864i32;
pub const MCI_WAVE_SET_ANYOUTPUT: i32 = 134217728i32;
pub const MCI_WAVE_SET_AVGBYTESPERSEC: i32 = 524288i32;
pub const MCI_WAVE_SET_BITSPERSAMPLE: i32 = 2097152i32;
pub const MCI_WAVE_SET_BLOCKALIGN: i32 = 1048576i32;
pub const MCI_WAVE_SET_CHANNELS: i32 = 131072i32;
pub const MCI_WAVE_SET_FORMATTAG: i32 = 65536i32;
pub const MCI_WAVE_SET_SAMPLESPERSEC: i32 = 262144i32;
pub const MCI_WAVE_STATUS_AVGBYTESPERSEC: i32 = 16388i32;
pub const MCI_WAVE_STATUS_BITSPERSAMPLE: i32 = 16390i32;
pub const MCI_WAVE_STATUS_BLOCKALIGN: i32 = 16389i32;
pub const MCI_WAVE_STATUS_CHANNELS: i32 = 16386i32;
pub const MCI_WAVE_STATUS_FORMATTAG: i32 = 16385i32;
pub const MCI_WAVE_STATUS_LEVEL: i32 = 16391i32;
pub const MCI_WAVE_STATUS_SAMPLESPERSEC: i32 = 16387i32;
pub const MCI_WHERE: u32 = 2115u32;
pub const MCI_WINDOW: u32 = 2113u32;
pub const MCMADM_E_REGKEY_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D006A_u32 as _;
pub const MCMADM_I_NO_EVENTS: windows_sys::core::HRESULT = 0x400D0069_u32 as _;
pub const MIDIMAPPER_S: u32 = 1227u32;
pub const MIDI_IO_COOKED: i32 = 2i32;
pub const MIDI_IO_PACKED: i32 = 0i32;
pub const MIDM_ADDBUFFER: u32 = 59u32;
pub const MIDM_CLOSE: u32 = 56u32;
pub const MIDM_GETDEVCAPS: u32 = 54u32;
pub const MIDM_GETNUMDEVS: u32 = 53u32;
pub const MIDM_INIT: u32 = 100u32;
pub const MIDM_INIT_EX: u32 = 104u32;
pub const MIDM_MAPPER: u32 = 8192u32;
pub const MIDM_OPEN: u32 = 55u32;
pub const MIDM_PREPARE: u32 = 57u32;
pub const MIDM_RESET: u32 = 62u32;
pub const MIDM_START: u32 = 60u32;
pub const MIDM_STOP: u32 = 61u32;
pub const MIDM_UNPREPARE: u32 = 58u32;
pub const MIDM_USER: u32 = 16384u32;
pub const MIXERCONTROL_CONTROLTYPE_SRS_MTS: u32 = 536936454u32;
pub const MIXERCONTROL_CONTROLTYPE_SRS_ONOFF: u32 = 536936455u32;
pub const MIXERCONTROL_CONTROLTYPE_SRS_SYNTHSELECT: u32 = 536936456u32;
pub const MMIOERR_ACCESSDENIED: u32 = 268u32;
pub const MMIOERR_BASE: u32 = 256u32;
pub const MMIOERR_CANNOTCLOSE: u32 = 260u32;
pub const MMIOERR_CANNOTEXPAND: u32 = 264u32;
pub const MMIOERR_CANNOTOPEN: u32 = 259u32;
pub const MMIOERR_CANNOTREAD: u32 = 261u32;
pub const MMIOERR_CANNOTSEEK: u32 = 263u32;
pub const MMIOERR_CANNOTWRITE: u32 = 262u32;
pub const MMIOERR_CHUNKNOTFOUND: u32 = 265u32;
pub const MMIOERR_FILENOTFOUND: u32 = 257u32;
pub const MMIOERR_INVALIDFILE: u32 = 272u32;
pub const MMIOERR_NETWORKERROR: u32 = 270u32;
pub const MMIOERR_OUTOFMEMORY: u32 = 258u32;
pub const MMIOERR_PATHNOTFOUND: u32 = 267u32;
pub const MMIOERR_SHARINGVIOLATION: u32 = 269u32;
pub const MMIOERR_TOOMANYOPENFILES: u32 = 271u32;
pub const MMIOERR_UNBUFFERED: u32 = 266u32;
pub const MMIOM_CLOSE: u32 = 4u32;
pub const MMIOM_OPEN: u32 = 3u32;
pub const MMIOM_READ: u32 = 0u32;
pub const MMIOM_RENAME: u32 = 6u32;
pub const MMIOM_SEEK: u32 = 2u32;
pub const MMIOM_USER: u32 = 32768u32;
pub const MMIOM_WRITE: u32 = 1u32;
pub const MMIOM_WRITEFLUSH: u32 = 5u32;
pub const MMIO_ALLOCBUF: u32 = 65536u32;
pub const MMIO_COMPAT: u32 = 0u32;
pub const MMIO_CREATE: u32 = 4096u32;
pub const MMIO_CREATELIST: u32 = 64u32;
pub const MMIO_CREATERIFF: u32 = 32u32;
pub const MMIO_DEFAULTBUFFER: u32 = 8192u32;
pub const MMIO_DELETE: u32 = 512u32;
pub const MMIO_DENYNONE: u32 = 64u32;
pub const MMIO_DENYREAD: u32 = 48u32;
pub const MMIO_DENYWRITE: u32 = 32u32;
pub const MMIO_DIRTY: u32 = 268435456u32;
pub const MMIO_EMPTYBUF: u32 = 16u32;
pub const MMIO_EXCLUSIVE: u32 = 16u32;
pub const MMIO_EXIST: u32 = 16384u32;
pub const MMIO_FHOPEN: u32 = 16u32;
pub const MMIO_FINDCHUNK: u32 = 16u32;
pub const MMIO_FINDLIST: u32 = 64u32;
pub const MMIO_FINDPROC: u32 = 262144u32;
pub const MMIO_FINDRIFF: u32 = 32u32;
pub const MMIO_GETTEMP: u32 = 131072u32;
pub const MMIO_GLOBALPROC: u32 = 268435456u32;
pub const MMIO_INSTALLPROC: u32 = 65536u32;
pub const MMIO_PARSE: u32 = 256u32;
pub const MMIO_READ: u32 = 0u32;
pub const MMIO_READWRITE: u32 = 2u32;
pub const MMIO_REMOVEPROC: u32 = 131072u32;
pub const MMIO_RWMODE: u32 = 3u32;
pub const MMIO_SHAREMODE: u32 = 112u32;
pub const MMIO_TOUPPER: u32 = 16u32;
pub const MMIO_UNICODEPROC: u32 = 16777216u32;
pub const MMIO_WRITE: u32 = 1u32;
pub const MM_3COM: u32 = 260u32;
pub const MM_3COM_CB_MIXER: u32 = 1u32;
pub const MM_3COM_CB_WAVEIN: u32 = 2u32;
pub const MM_3COM_CB_WAVEOUT: u32 = 3u32;
pub const MM_3DFX: u32 = 262u32;
pub const MM_AARDVARK: u32 = 11u32;
pub const MM_AARDVARK_STUDIO12_WAVEIN: u32 = 2u32;
pub const MM_AARDVARK_STUDIO12_WAVEOUT: u32 = 1u32;
pub const MM_AARDVARK_STUDIO88_WAVEIN: u32 = 4u32;
pub const MM_AARDVARK_STUDIO88_WAVEOUT: u32 = 3u32;
pub const MM_ACTIVEVOICE: u32 = 225u32;
pub const MM_ACTIVEVOICE_ACM_VOXADPCM: u32 = 1u32;
pub const MM_ACULAB: u32 = 14u32;
pub const MM_ADDX: u32 = 118u32;
pub const MM_ADDX_PCTV_AUX_CD: u32 = 5u32;
pub const MM_ADDX_PCTV_AUX_LINE: u32 = 6u32;
pub const MM_ADDX_PCTV_DIGITALMIX: u32 = 1u32;
pub const MM_ADDX_PCTV_MIXER: u32 = 4u32;
pub const MM_ADDX_PCTV_WAVEIN: u32 = 2u32;
pub const MM_ADDX_PCTV_WAVEOUT: u32 = 3u32;
pub const MM_ADLACC: u32 = 91u32;
pub const MM_ADMOS: u32 = 235u32;
pub const MM_ADMOS_FM_SYNTH: u32 = 1u32;
pub const MM_ADMOS_QS3AMIDIIN: u32 = 3u32;
pub const MM_ADMOS_QS3AMIDIOUT: u32 = 2u32;
pub const MM_ADMOS_QS3AWAVEIN: u32 = 5u32;
pub const MM_ADMOS_QS3AWAVEOUT: u32 = 4u32;
pub const MM_AHEAD: u32 = 77u32;
pub const MM_AHEAD_GENERIC: u32 = 4u32;
pub const MM_AHEAD_MULTISOUND: u32 = 1u32;
pub const MM_AHEAD_PROAUDIO: u32 = 3u32;
pub const MM_AHEAD_SOUNDBLASTER: u32 = 2u32;
pub const MM_ALARIS: u32 = 174u32;
pub const MM_ALDIGITAL: u32 = 143u32;
pub const MM_ALESIS: u32 = 243u32;
pub const MM_ALGOVISION: u32 = 266u32;
pub const MM_ALGOVISION_VB80AUX: u32 = 4u32;
pub const MM_ALGOVISION_VB80AUX2: u32 = 5u32;
pub const MM_ALGOVISION_VB80MIXER: u32 = 3u32;
pub const MM_ALGOVISION_VB80WAVEIN: u32 = 2u32;
pub const MM_ALGOVISION_VB80WAVEOUT: u32 = 1u32;
pub const MM_AMD: u32 = 146u32;
pub const MM_AMD_INTERWAVE_AUX1: u32 = 10u32;
pub const MM_AMD_INTERWAVE_AUX2: u32 = 11u32;
pub const MM_AMD_INTERWAVE_AUX_CD: u32 = 13u32;
pub const MM_AMD_INTERWAVE_AUX_MIC: u32 = 12u32;
pub const MM_AMD_INTERWAVE_EX_CD: u32 = 7u32;
pub const MM_AMD_INTERWAVE_EX_TELEPHONY: u32 = 16u32;
pub const MM_AMD_INTERWAVE_JOYSTICK: u32 = 6u32;
pub const MM_AMD_INTERWAVE_MIDIIN: u32 = 8u32;
pub const MM_AMD_INTERWAVE_MIDIOUT: u32 = 9u32;
pub const MM_AMD_INTERWAVE_MIXER1: u32 = 4u32;
pub const MM_AMD_INTERWAVE_MIXER2: u32 = 5u32;
pub const MM_AMD_INTERWAVE_MONO_IN: u32 = 14u32;
pub const MM_AMD_INTERWAVE_MONO_OUT: u32 = 15u32;
pub const MM_AMD_INTERWAVE_STEREO_ENHANCED: u32 = 19u32;
pub const MM_AMD_INTERWAVE_SYNTH: u32 = 3u32;
pub const MM_AMD_INTERWAVE_WAVEIN: u32 = 1u32;
pub const MM_AMD_INTERWAVE_WAVEOUT: u32 = 2u32;
pub const MM_AMD_INTERWAVE_WAVEOUT_BASE: u32 = 17u32;
pub const MM_AMD_INTERWAVE_WAVEOUT_TREBLE: u32 = 18u32;
pub const MM_ANALOGDEVICES: u32 = 252u32;
pub const MM_ANTEX: u32 = 31u32;
pub const MM_ANTEX_AUDIOPORT22_FEEDTHRU: u32 = 9u32;
pub const MM_ANTEX_AUDIOPORT22_WAVEIN: u32 = 7u32;
pub const MM_ANTEX_AUDIOPORT22_WAVEOUT: u32 = 8u32;
pub const MM_ANTEX_SX12_WAVEIN: u32 = 1u32;
pub const MM_ANTEX_SX12_WAVEOUT: u32 = 2u32;
pub const MM_ANTEX_SX15_WAVEIN: u32 = 3u32;
pub const MM_ANTEX_SX15_WAVEOUT: u32 = 4u32;
pub const MM_ANTEX_VP625_WAVEIN: u32 = 5u32;
pub const MM_ANTEX_VP625_WAVEOUT: u32 = 6u32;
pub const MM_APICOM: u32 = 116u32;
pub const MM_APPLE: u32 = 99u32;
pub const MM_APPS: u32 = 42u32;
pub const MM_APT: u32 = 56u32;
pub const MM_APT_ACE100CD: u32 = 1u32;
pub const MM_ARRAY: u32 = 231u32;
pub const MM_ARTISOFT: u32 = 20u32;
pub const MM_ARTISOFT_SBWAVEIN: u32 = 1u32;
pub const MM_ARTISOFT_SBWAVEOUT: u32 = 2u32;
pub const MM_AST: u32 = 64u32;
pub const MM_AST_MODEMWAVE_WAVEIN: u32 = 13u32;
pub const MM_AST_MODEMWAVE_WAVEOUT: u32 = 14u32;
pub const MM_ATI: u32 = 27u32;
pub const MM_ATT: u32 = 185u32;
pub const MM_ATT_G729A: u32 = 1u32;
pub const MM_ATT_MICROELECTRONICS: u32 = 139u32;
pub const MM_AU8820_AUX: u32 = 21u32;
pub const MM_AU8820_MIDIIN: u32 = 23u32;
pub const MM_AU8820_MIDIOUT: u32 = 22u32;
pub const MM_AU8820_MIXER: u32 = 20u32;
pub const MM_AU8820_SYNTH: u32 = 17u32;
pub const MM_AU8820_WAVEIN: u32 = 19u32;
pub const MM_AU8820_WAVEOUT: u32 = 18u32;
pub const MM_AU8830_AUX: u32 = 37u32;
pub const MM_AU8830_MIDIIN: u32 = 39u32;
pub const MM_AU8830_MIDIOUT: u32 = 38u32;
pub const MM_AU8830_MIXER: u32 = 36u32;
pub const MM_AU8830_SYNTH: u32 = 33u32;
pub const MM_AU8830_WAVEIN: u32 = 35u32;
pub const MM_AU8830_WAVEOUT: u32 = 34u32;
pub const MM_AUDIOFILE: u32 = 47u32;
pub const MM_AUDIOPT: u32 = 74u32;
pub const MM_AUDIOSCIENCE: u32 = 217u32;
pub const MM_AURAVISION: u32 = 80u32;
pub const MM_AUREAL: u32 = 181u32;
pub const MM_AUREAL_AU8820: u32 = 16u32;
pub const MM_AUREAL_AU8830: u32 = 32u32;
pub const MM_AZTECH: u32 = 52u32;
pub const MM_AZTECH_AUX: u32 = 404u32;
pub const MM_AZTECH_AUX_CD: u32 = 401u32;
pub const MM_AZTECH_AUX_LINE: u32 = 402u32;
pub const MM_AZTECH_AUX_MIC: u32 = 403u32;
pub const MM_AZTECH_DSP16_FMSYNTH: u32 = 68u32;
pub const MM_AZTECH_DSP16_WAVEIN: u32 = 65u32;
pub const MM_AZTECH_DSP16_WAVEOUT: u32 = 66u32;
pub const MM_AZTECH_DSP16_WAVESYNTH: u32 = 70u32;
pub const MM_AZTECH_FMSYNTH: u32 = 20u32;
pub const MM_AZTECH_MIDIIN: u32 = 4u32;
pub const MM_AZTECH_MIDIOUT: u32 = 3u32;
pub const MM_AZTECH_MIXER: u32 = 21u32;
pub const MM_AZTECH_NOVA16_MIXER: u32 = 73u32;
pub const MM_AZTECH_NOVA16_WAVEIN: u32 = 71u32;
pub const MM_AZTECH_NOVA16_WAVEOUT: u32 = 72u32;
pub const MM_AZTECH_PRO16_FMSYNTH: u32 = 38u32;
pub const MM_AZTECH_PRO16_WAVEIN: u32 = 33u32;
pub const MM_AZTECH_PRO16_WAVEOUT: u32 = 34u32;
pub const MM_AZTECH_WASH16_MIXER: u32 = 76u32;
pub const MM_AZTECH_WASH16_WAVEIN: u32 = 74u32;
pub const MM_AZTECH_WASH16_WAVEOUT: u32 = 75u32;
pub const MM_AZTECH_WAVEIN: u32 = 17u32;
pub const MM_AZTECH_WAVEOUT: u32 = 18u32;
pub const MM_BCB: u32 = 192u32;
pub const MM_BCB_NETBOARD_10: u32 = 1u32;
pub const MM_BCB_TT75_10: u32 = 2u32;
pub const MM_BECUBED: u32 = 10u32;
pub const MM_BERCOS: u32 = 199u32;
pub const MM_BERCOS_MIXER: u32 = 2u32;
pub const MM_BERCOS_WAVEIN: u32 = 1u32;
pub const MM_BERCOS_WAVEOUT: u32 = 3u32;
pub const MM_BERKOM: u32 = 189u32;
pub const MM_BINTEC: u32 = 12u32;
pub const MM_BINTEC_TAPI_WAVE: u32 = 1u32;
pub const MM_BROOKTREE: u32 = 121u32;
pub const MM_BTV_AUX_CD: u32 = 8u32;
pub const MM_BTV_AUX_LINE: u32 = 6u32;
pub const MM_BTV_AUX_MIC: u32 = 7u32;
pub const MM_BTV_DIGITALIN: u32 = 9u32;
pub const MM_BTV_DIGITALOUT: u32 = 10u32;
pub const MM_BTV_MIDIIN: u32 = 3u32;
pub const MM_BTV_MIDIOUT: u32 = 4u32;
pub const MM_BTV_MIDISYNTH: u32 = 5u32;
pub const MM_BTV_MIDIWAVESTREAM: u32 = 11u32;
pub const MM_BTV_MIXER: u32 = 12u32;
pub const MM_BTV_WAVEIN: u32 = 1u32;
pub const MM_BTV_WAVEOUT: u32 = 2u32;
pub const MM_CANAM: u32 = 148u32;
pub const MM_CANAM_CBXWAVEIN: u32 = 2u32;
pub const MM_CANAM_CBXWAVEOUT: u32 = 1u32;
pub const MM_CANOPUS: u32 = 49u32;
pub const MM_CANOPUS_ACM_DVREX: u32 = 1u32;
pub const MM_CASIO: u32 = 162u32;
pub const MM_CASIO_LSG_MIDIOUT: u32 = 3u32;
pub const MM_CASIO_WP150_MIDIIN: u32 = 2u32;
pub const MM_CASIO_WP150_MIDIOUT: u32 = 1u32;
pub const MM_CAT: u32 = 41u32;
pub const MM_CAT_WAVEOUT: u32 = 1u32;
pub const MM_CDPC_AUX: u32 = 119u32;
pub const MM_CDPC_MIDIIN: u32 = 114u32;
pub const MM_CDPC_MIDIOUT: u32 = 113u32;
pub const MM_CDPC_MIXER: u32 = 118u32;
pub const MM_CDPC_SYNTH: u32 = 115u32;
pub const MM_CDPC_WAVEIN: u32 = 117u32;
pub const MM_CDPC_WAVEOUT: u32 = 116u32;
pub const MM_CHROMATIC: u32 = 155u32;
pub const MM_CHROMATIC_M1: u32 = 1u32;
pub const MM_CHROMATIC_M1_AUX: u32 = 6u32;
pub const MM_CHROMATIC_M1_AUX_CD: u32 = 7u32;
pub const MM_CHROMATIC_M1_FMSYNTH: u32 = 4u32;
pub const MM_CHROMATIC_M1_MIDIIN: u32 = 8u32;
pub const MM_CHROMATIC_M1_MIDIOUT: u32 = 9u32;
pub const MM_CHROMATIC_M1_MIXER: u32 = 5u32;
pub const MM_CHROMATIC_M1_MPEGWAVEIN: u32 = 17u32;
pub const MM_CHROMATIC_M1_MPEGWAVEOUT: u32 = 18u32;
pub const MM_CHROMATIC_M1_WAVEIN: u32 = 2u32;
pub const MM_CHROMATIC_M1_WAVEOUT: u32 = 3u32;
pub const MM_CHROMATIC_M1_WTSYNTH: u32 = 16u32;
pub const MM_CHROMATIC_M2: u32 = 19u32;
pub const MM_CHROMATIC_M2_AUX: u32 = 24u32;
pub const MM_CHROMATIC_M2_AUX_CD: u32 = 25u32;
pub const MM_CHROMATIC_M2_FMSYNTH: u32 = 22u32;
pub const MM_CHROMATIC_M2_MIDIIN: u32 = 32u32;
pub const MM_CHROMATIC_M2_MIDIOUT: u32 = 33u32;
pub const MM_CHROMATIC_M2_MIXER: u32 = 23u32;
pub const MM_CHROMATIC_M2_MPEGWAVEIN: u32 = 35u32;
pub const MM_CHROMATIC_M2_MPEGWAVEOUT: u32 = 36u32;
pub const MM_CHROMATIC_M2_WAVEIN: u32 = 20u32;
pub const MM_CHROMATIC_M2_WAVEOUT: u32 = 21u32;
pub const MM_CHROMATIC_M2_WTSYNTH: u32 = 34u32;
pub const MM_CIRRUSLOGIC: u32 = 105u32;
pub const MM_COLORGRAPH: u32 = 179u32;
pub const MM_COMPAQ: u32 = 92u32;
pub const MM_COMPAQ_BB_WAVEAUX: u32 = 3u32;
pub const MM_COMPAQ_BB_WAVEIN: u32 = 1u32;
pub const MM_COMPAQ_BB_WAVEOUT: u32 = 2u32;
pub const MM_COMPUSIC: u32 = 89u32;
pub const MM_COMPUTER_FRIENDS: u32 = 45u32;
pub const MM_CONCEPTS: u32 = 108u32;
pub const MM_CONNECTIX: u32 = 158u32;
pub const MM_CONNECTIX_VIDEC_CODEC: u32 = 1u32;
pub const MM_CONTROLRES: u32 = 84u32;
pub const MM_COREDYNAMICS: u32 = 147u32;
pub const MM_COREDYNAMICS_DYNAGRAFX_VGA: u32 = 9u32;
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_IN: u32 = 10u32;
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_OUT: u32 = 11u32;
pub const MM_COREDYNAMICS_DYNAMIXHR: u32 = 1u32;
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_IN: u32 = 7u32;
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_OUT: u32 = 8u32;
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_IN: u32 = 3u32;
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_OUT: u32 = 4u32;
pub const MM_COREDYNAMICS_DYNASONIX_SYNTH: u32 = 2u32;
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_IN: u32 = 5u32;
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_OUT: u32 = 6u32;
pub const MM_CREATIVE: u32 = 2u32;
pub const MM_CREATIVE_AUX_CD: u32 = 401u32;
pub const MM_CREATIVE_AUX_LINE: u32 = 402u32;
pub const MM_CREATIVE_AUX_MASTER: u32 = 404u32;
pub const MM_CREATIVE_AUX_MIC: u32 = 403u32;
pub const MM_CREATIVE_AUX_MIDI: u32 = 407u32;
pub const MM_CREATIVE_AUX_PCSPK: u32 = 405u32;
pub const MM_CREATIVE_AUX_WAVE: u32 = 406u32;
pub const MM_CREATIVE_FMSYNTH_MONO: u32 = 301u32;
pub const MM_CREATIVE_FMSYNTH_STEREO: u32 = 302u32;
pub const MM_CREATIVE_MIDIIN: u32 = 202u32;
pub const MM_CREATIVE_MIDIOUT: u32 = 201u32;
pub const MM_CREATIVE_MIDI_AWE32: u32 = 303u32;
pub const MM_CREATIVE_PHNBLST_WAVEIN: u32 = 5u32;
pub const MM_CREATIVE_PHNBLST_WAVEOUT: u32 = 105u32;
pub const MM_CREATIVE_SB15_WAVEIN: u32 = 1u32;
pub const MM_CREATIVE_SB15_WAVEOUT: u32 = 101u32;
pub const MM_CREATIVE_SB16_MIXER: u32 = 409u32;
pub const MM_CREATIVE_SB20_WAVEIN: u32 = 2u32;
pub const MM_CREATIVE_SB20_WAVEOUT: u32 = 102u32;
pub const MM_CREATIVE_SBP16_WAVEIN: u32 = 4u32;
pub const MM_CREATIVE_SBP16_WAVEOUT: u32 = 104u32;
pub const MM_CREATIVE_SBPRO_MIXER: u32 = 408u32;
pub const MM_CREATIVE_SBPRO_WAVEIN: u32 = 3u32;
pub const MM_CREATIVE_SBPRO_WAVEOUT: u32 = 103u32;
pub const MM_CRYSTAL: u32 = 132u32;
pub const MM_CRYSTAL_CS4232_INPUTGAIN_AUX1: u32 = 13u32;
pub const MM_CRYSTAL_CS4232_INPUTGAIN_LOOP: u32 = 14u32;
pub const MM_CRYSTAL_CS4232_MIDIIN: u32 = 9u32;
pub const MM_CRYSTAL_CS4232_MIDIOUT: u32 = 10u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX1: u32 = 4u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX2: u32 = 5u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_LINE: u32 = 6u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_MASTER: u32 = 8u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_MONO: u32 = 7u32;
pub const MM_CRYSTAL_CS4232_WAVEIN: u32 = 1u32;
pub const MM_CRYSTAL_CS4232_WAVEMIXER: u32 = 3u32;
pub const MM_CRYSTAL_CS4232_WAVEOUT: u32 = 2u32;
pub const MM_CRYSTAL_NET: u32 = 154u32;
pub const MM_CRYSTAL_SOUND_FUSION_JOYSTICK: u32 = 26u32;
pub const MM_CRYSTAL_SOUND_FUSION_MIDIIN: u32 = 24u32;
pub const MM_CRYSTAL_SOUND_FUSION_MIDIOUT: u32 = 25u32;
pub const MM_CRYSTAL_SOUND_FUSION_MIXER: u32 = 23u32;
pub const MM_CRYSTAL_SOUND_FUSION_WAVEIN: u32 = 21u32;
pub const MM_CRYSTAL_SOUND_FUSION_WAVEOUT: u32 = 22u32;
pub const MM_CS: u32 = 242u32;
pub const MM_CYRIX: u32 = 6u32;
pub const MM_CYRIX_XAAUX: u32 = 6u32;
pub const MM_CYRIX_XAMIDIIN: u32 = 2u32;
pub const MM_CYRIX_XAMIDIOUT: u32 = 3u32;
pub const MM_CYRIX_XAMIXER: u32 = 7u32;
pub const MM_CYRIX_XASYNTH: u32 = 1u32;
pub const MM_CYRIX_XAWAVEIN: u32 = 4u32;
pub const MM_CYRIX_XAWAVEOUT: u32 = 5u32;
pub const MM_DATAFUSION: u32 = 196u32;
pub const MM_DATARAN: u32 = 232u32;
pub const MM_DDD: u32 = 151u32;
pub const MM_DDD_MIDILINK_MIDIIN: u32 = 1u32;
pub const MM_DDD_MIDILINK_MIDIOUT: u32 = 2u32;
pub const MM_DF_ACM_G726: u32 = 1u32;
pub const MM_DF_ACM_GSM610: u32 = 2u32;
pub const MM_DIACOUSTICS: u32 = 129u32;
pub const MM_DIACOUSTICS_DRUM_ACTION: u32 = 1u32;
pub const MM_DIALOGIC: u32 = 93u32;
pub const MM_DIAMONDMM: u32 = 163u32;
pub const MM_DICTAPHONE: u32 = 214u32;
pub const MM_DICTAPHONE_G726: u32 = 1u32;
pub const MM_DIGIGRAM: u32 = 227u32;
pub const MM_DIGITAL: u32 = 100u32;
pub const MM_DIGITAL_ACM_G723: u32 = 3u32;
pub const MM_DIGITAL_AUDIO_LABS: u32 = 136u32;
pub const MM_DIGITAL_AUDIO_LABS_CDLX: u32 = 19u32;
pub const MM_DIGITAL_AUDIO_LABS_CPRO: u32 = 17u32;
pub const MM_DIGITAL_AUDIO_LABS_CTDIF: u32 = 20u32;
pub const MM_DIGITAL_AUDIO_LABS_DOC: u32 = 2u32;
pub const MM_DIGITAL_AUDIO_LABS_TC: u32 = 1u32;
pub const MM_DIGITAL_AUDIO_LABS_V8: u32 = 16u32;
pub const MM_DIGITAL_AUDIO_LABS_VP: u32 = 18u32;
pub const MM_DIGITAL_AV320_WAVEIN: u32 = 1u32;
pub const MM_DIGITAL_AV320_WAVEOUT: u32 = 2u32;
pub const MM_DIGITAL_ICM_H261: u32 = 5u32;
pub const MM_DIGITAL_ICM_H263: u32 = 4u32;
pub const MM_DIMD_AUX_LINE: u32 = 9u32;
pub const MM_DIMD_DIRSOUND: u32 = 1u32;
pub const MM_DIMD_MIDIIN: u32 = 7u32;
pub const MM_DIMD_MIDIOUT: u32 = 8u32;
pub const MM_DIMD_MIXER: u32 = 10u32;
pub const MM_DIMD_PLATFORM: u32 = 0u32;
pub const MM_DIMD_VIRTJOY: u32 = 4u32;
pub const MM_DIMD_VIRTMPU: u32 = 2u32;
pub const MM_DIMD_VIRTSB: u32 = 3u32;
pub const MM_DIMD_WAVEIN: u32 = 5u32;
pub const MM_DIMD_WAVEOUT: u32 = 6u32;
pub const MM_DIMD_WSS_AUX: u32 = 21u32;
pub const MM_DIMD_WSS_MIXER: u32 = 17u32;
pub const MM_DIMD_WSS_SYNTH: u32 = 76u32;
pub const MM_DIMD_WSS_WAVEIN: u32 = 14u32;
pub const MM_DIMD_WSS_WAVEOUT: u32 = 15u32;
pub const MM_DOLBY: u32 = 78u32;
pub const MM_DPSINC: u32 = 191u32;
pub const MM_DSP_GROUP: u32 = 43u32;
pub const MM_DSP_GROUP_TRUESPEECH: u32 = 1u32;
pub const MM_DSP_SOLUTIONS: u32 = 25u32;
pub const MM_DSP_SOLUTIONS_AUX: u32 = 4u32;
pub const MM_DSP_SOLUTIONS_SYNTH: u32 = 3u32;
pub const MM_DSP_SOLUTIONS_WAVEIN: u32 = 2u32;
pub const MM_DSP_SOLUTIONS_WAVEOUT: u32 = 1u32;
pub const MM_DTS: u32 = 226u32;
pub const MM_DTS_DS: u32 = 1u32;
pub const MM_DUCK: u32 = 197u32;
pub const MM_DVISION: u32 = 165u32;
pub const MM_ECHO: u32 = 39u32;
pub const MM_ECHO_AUX: u32 = 6u32;
pub const MM_ECHO_MIDIIN: u32 = 5u32;
pub const MM_ECHO_MIDIOUT: u32 = 4u32;
pub const MM_ECHO_SYNTH: u32 = 1u32;
pub const MM_ECHO_WAVEIN: u32 = 3u32;
pub const MM_ECHO_WAVEOUT: u32 = 2u32;
pub const MM_ECS: u32 = 145u32;
pub const MM_ECS_AADF_MIDI_IN: u32 = 10u32;
pub const MM_ECS_AADF_MIDI_OUT: u32 = 11u32;
pub const MM_ECS_AADF_WAVE2MIDI_IN: u32 = 12u32;
pub const MM_EES: u32 = 219u32;
pub const MM_EES_PCMIDI14: u32 = 1u32;
pub const MM_EES_PCMIDI14_IN: u32 = 2u32;
pub const MM_EES_PCMIDI14_OUT1: u32 = 3u32;
pub const MM_EES_PCMIDI14_OUT2: u32 = 4u32;
pub const MM_EES_PCMIDI14_OUT3: u32 = 5u32;
pub const MM_EES_PCMIDI14_OUT4: u32 = 6u32;
pub const MM_EMAGIC: u32 = 208u32;
pub const MM_EMAGIC_UNITOR8: u32 = 1u32;
pub const MM_EMU: u32 = 19u32;
pub const MM_EMU_APSMIDIIN: u32 = 2u32;
pub const MM_EMU_APSMIDIOUT: u32 = 3u32;
pub const MM_EMU_APSSYNTH: u32 = 1u32;
pub const MM_EMU_APSWAVEIN: u32 = 4u32;
pub const MM_EMU_APSWAVEOUT: u32 = 5u32;
pub const MM_ENET: u32 = 206u32;
pub const MM_ENET_T2000_HANDSETIN: u32 = 3u32;
pub const MM_ENET_T2000_HANDSETOUT: u32 = 4u32;
pub const MM_ENET_T2000_LINEIN: u32 = 1u32;
pub const MM_ENET_T2000_LINEOUT: u32 = 2u32;
pub const MM_ENSONIQ: u32 = 125u32;
pub const MM_ENSONIQ_SOUNDSCAPE: u32 = 16u32;
pub const MM_EPSON: u32 = 50u32;
pub const MM_EPS_FMSND: u32 = 1u32;
pub const MM_ESS: u32 = 46u32;
pub const MM_ESS_AMAUX: u32 = 3u32;
pub const MM_ESS_AMMIDIIN: u32 = 6u32;
pub const MM_ESS_AMMIDIOUT: u32 = 5u32;
pub const MM_ESS_AMSYNTH: u32 = 4u32;
pub const MM_ESS_AMWAVEIN: u32 = 2u32;
pub const MM_ESS_AMWAVEOUT: u32 = 1u32;
pub const MM_ESS_AUX_CD: u32 = 8u32;
pub const MM_ESS_ES1488_MIXER: u32 = 24u32;
pub const MM_ESS_ES1488_WAVEIN: u32 = 23u32;
pub const MM_ESS_ES1488_WAVEOUT: u32 = 22u32;
pub const MM_ESS_ES1688_MIXER: u32 = 27u32;
pub const MM_ESS_ES1688_WAVEIN: u32 = 26u32;
pub const MM_ESS_ES1688_WAVEOUT: u32 = 25u32;
pub const MM_ESS_ES1788_MIXER: u32 = 30u32;
pub const MM_ESS_ES1788_WAVEIN: u32 = 29u32;
pub const MM_ESS_ES1788_WAVEOUT: u32 = 28u32;
pub const MM_ESS_ES1868_MIXER: u32 = 36u32;
pub const MM_ESS_ES1868_WAVEIN: u32 = 35u32;
pub const MM_ESS_ES1868_WAVEOUT: u32 = 34u32;
pub const MM_ESS_ES1878_MIXER: u32 = 39u32;
pub const MM_ESS_ES1878_WAVEIN: u32 = 38u32;
pub const MM_ESS_ES1878_WAVEOUT: u32 = 37u32;
pub const MM_ESS_ES1888_MIXER: u32 = 33u32;
pub const MM_ESS_ES1888_WAVEIN: u32 = 32u32;
pub const MM_ESS_ES1888_WAVEOUT: u32 = 31u32;
pub const MM_ESS_ES488_MIXER: u32 = 18u32;
pub const MM_ESS_ES488_WAVEIN: u32 = 17u32;
pub const MM_ESS_ES488_WAVEOUT: u32 = 16u32;
pub const MM_ESS_ES688_MIXER: u32 = 21u32;
pub const MM_ESS_ES688_WAVEIN: u32 = 20u32;
pub const MM_ESS_ES688_WAVEOUT: u32 = 19u32;
pub const MM_ESS_MIXER: u32 = 7u32;
pub const MM_ESS_MPU401_MIDIIN: u32 = 10u32;
pub const MM_ESS_MPU401_MIDIOUT: u32 = 9u32;
pub const MM_ETEK: u32 = 241u32;
pub const MM_ETEK_KWIKMIDI_MIDIIN: u32 = 1u32;
pub const MM_ETEK_KWIKMIDI_MIDIOUT: u32 = 2u32;
pub const MM_EUPHONICS: u32 = 152u32;
pub const MM_EUPHONICS_AUX_CD: u32 = 1u32;
pub const MM_EUPHONICS_AUX_LINE: u32 = 2u32;
pub const MM_EUPHONICS_AUX_MASTER: u32 = 3u32;
pub const MM_EUPHONICS_AUX_MIC: u32 = 4u32;
pub const MM_EUPHONICS_AUX_MIDI: u32 = 5u32;
pub const MM_EUPHONICS_AUX_WAVE: u32 = 6u32;
pub const MM_EUPHONICS_EUSYNTH: u32 = 14u32;
pub const MM_EUPHONICS_FMSYNTH_MONO: u32 = 7u32;
pub const MM_EUPHONICS_FMSYNTH_STEREO: u32 = 8u32;
pub const MM_EUPHONICS_MIDIIN: u32 = 9u32;
pub const MM_EUPHONICS_MIDIOUT: u32 = 10u32;
pub const MM_EUPHONICS_MIXER: u32 = 11u32;
pub const MM_EUPHONICS_WAVEIN: u32 = 12u32;
pub const MM_EUPHONICS_WAVEOUT: u32 = 13u32;
pub const MM_EVEREX: u32 = 38u32;
pub const MM_EVEREX_CARRIER: u32 = 1u32;
pub const MM_EXAN: u32 = 63u32;
pub const MM_FAITH: u32 = 15u32;
pub const MM_FAST: u32 = 126u32;
pub const MM_FHGIIS_MPEGLAYER3: u32 = 10u32;
pub const MM_FHGIIS_MPEGLAYER3_ADVANCED: u32 = 12u32;
pub const MM_FHGIIS_MPEGLAYER3_ADVANCEDPLUS: u32 = 14u32;
pub const MM_FHGIIS_MPEGLAYER3_BASIC: u32 = 11u32;
pub const MM_FHGIIS_MPEGLAYER3_DECODE: u32 = 9u32;
pub const MM_FHGIIS_MPEGLAYER3_LITE: u32 = 10u32;
pub const MM_FHGIIS_MPEGLAYER3_PROFESSIONAL: u32 = 13u32;
pub const MM_FLEXION: u32 = 249u32;
pub const MM_FLEXION_X300_WAVEIN: u32 = 1u32;
pub const MM_FLEXION_X300_WAVEOUT: u32 = 2u32;
pub const MM_FORTEMEDIA: u32 = 229u32;
pub const MM_FORTEMEDIA_AUX: u32 = 5u32;
pub const MM_FORTEMEDIA_FMSYNC: u32 = 3u32;
pub const MM_FORTEMEDIA_MIXER: u32 = 4u32;
pub const MM_FORTEMEDIA_WAVEIN: u32 = 1u32;
pub const MM_FORTEMEDIA_WAVEOUT: u32 = 2u32;
pub const MM_FRAUNHOFER_IIS: u32 = 172u32;
pub const MM_FRONTIER: u32 = 160u32;
pub const MM_FRONTIER_WAVECENTER_MIDIIN: u32 = 1u32;
pub const MM_FRONTIER_WAVECENTER_MIDIOUT: u32 = 2u32;
pub const MM_FRONTIER_WAVECENTER_WAVEIN: u32 = 3u32;
pub const MM_FRONTIER_WAVECENTER_WAVEOUT: u32 = 4u32;
pub const MM_FTR: u32 = 198u32;
pub const MM_FTR_ACM: u32 = 2u32;
pub const MM_FTR_ENCODER_WAVEIN: u32 = 1u32;
pub const MM_FUJITSU: u32 = 4u32;
pub const MM_GADGETLABS: u32 = 159u32;
pub const MM_GADGETLABS_WAVE42_WAVEIN: u32 = 3u32;
pub const MM_GADGETLABS_WAVE42_WAVEOUT: u32 = 4u32;
pub const MM_GADGETLABS_WAVE44_WAVEIN: u32 = 1u32;
pub const MM_GADGETLABS_WAVE44_WAVEOUT: u32 = 2u32;
pub const MM_GADGETLABS_WAVE4_MIDIIN: u32 = 5u32;
pub const MM_GADGETLABS_WAVE4_MIDIOUT: u32 = 6u32;
pub const MM_GRANDE: u32 = 117u32;
pub const MM_GRAVIS: u32 = 34u32;
pub const MM_GUILLEMOT: u32 = 207u32;
pub const MM_GULBRANSEN: u32 = 130u32;
pub const MM_HAFTMANN: u32 = 220u32;
pub const MM_HAFTMANN_LPTDAC2: u32 = 1u32;
pub const MM_HEADSPACE: u32 = 222u32;
pub const MM_HEADSPACE_HAEMIXER: u32 = 4u32;
pub const MM_HEADSPACE_HAESYNTH: u32 = 1u32;
pub const MM_HEADSPACE_HAEWAVEIN: u32 = 3u32;
pub const MM_HEADSPACE_HAEWAVEOUT: u32 = 2u32;
pub const MM_HEWLETT_PACKARD: u32 = 13u32;
pub const MM_HEWLETT_PACKARD_CU_CODEC: u32 = 1u32;
pub const MM_HORIZONS: u32 = 107u32;
pub const MM_HP: u32 = 253u32;
pub const MM_HP_WAVEIN: u32 = 2u32;
pub const MM_HP_WAVEOUT: u32 = 1u32;
pub const MM_HYPERACTIVE: u32 = 246u32;
pub const MM_IBM: u32 = 22u32;
pub const MM_IBM_MWAVE_AUX: u32 = 23u32;
pub const MM_IBM_MWAVE_MIDIIN: u32 = 21u32;
pub const MM_IBM_MWAVE_MIDIOUT: u32 = 22u32;
pub const MM_IBM_MWAVE_MIXER: u32 = 20u32;
pub const MM_IBM_MWAVE_WAVEIN: u32 = 18u32;
pub const MM_IBM_MWAVE_WAVEOUT: u32 = 19u32;
pub const MM_IBM_PCMCIA_AUX: u32 = 16u32;
pub const MM_IBM_PCMCIA_MIDIIN: u32 = 14u32;
pub const MM_IBM_PCMCIA_MIDIOUT: u32 = 15u32;
pub const MM_IBM_PCMCIA_SYNTH: u32 = 13u32;
pub const MM_IBM_PCMCIA_WAVEIN: u32 = 11u32;
pub const MM_IBM_PCMCIA_WAVEOUT: u32 = 12u32;
pub const MM_IBM_THINKPAD200: u32 = 17u32;
pub const MM_IBM_WC_MIDIOUT: u32 = 30u32;
pub const MM_IBM_WC_MIXEROUT: u32 = 33u32;
pub const MM_IBM_WC_WAVEOUT: u32 = 31u32;
pub const MM_ICCC: u32 = 259u32;
pub const MM_ICCC_UNA3_AUX: u32 = 3u32;
pub const MM_ICCC_UNA3_MIXER: u32 = 4u32;
pub const MM_ICCC_UNA3_WAVEIN: u32 = 1u32;
pub const MM_ICCC_UNA3_WAVEOUT: u32 = 2u32;
pub const MM_ICE: u32 = 239u32;
pub const MM_ICE_AUX: u32 = 11u32;
pub const MM_ICE_MIDIIN1: u32 = 6u32;
pub const MM_ICE_MIDIIN2: u32 = 8u32;
pub const MM_ICE_MIDIOUT1: u32 = 5u32;
pub const MM_ICE_MIDIOUT2: u32 = 7u32;
pub const MM_ICE_MIXER: u32 = 10u32;
pub const MM_ICE_MTWAVEIN: u32 = 4u32;
pub const MM_ICE_MTWAVEOUT: u32 = 3u32;
pub const MM_ICE_SYNTH: u32 = 9u32;
pub const MM_ICE_WAVEIN: u32 = 2u32;
pub const MM_ICE_WAVEOUT: u32 = 1u32;
pub const MM_ICL_PS: u32 = 32u32;
pub const MM_ICOM_AUX: u32 = 6u32;
pub const MM_ICOM_LINE: u32 = 7u32;
pub const MM_ICOM_MIXER: u32 = 5u32;
pub const MM_ICOM_WAVEIN: u32 = 3u32;
pub const MM_ICOM_WAVEOUT: u32 = 4u32;
pub const MM_ICS: u32 = 57u32;
pub const MM_ICS_2115_LITE_MIDIOUT: u32 = 13u32;
pub const MM_ICS_2120_LITE_MIDIOUT: u32 = 14u32;
pub const MM_ICS_WAVEDECK_AUX: u32 = 4u32;
pub const MM_ICS_WAVEDECK_MIXER: u32 = 3u32;
pub const MM_ICS_WAVEDECK_SYNTH: u32 = 5u32;
pub const MM_ICS_WAVEDECK_WAVEIN: u32 = 2u32;
pub const MM_ICS_WAVEDECK_WAVEOUT: u32 = 1u32;
pub const MM_ICS_WAVEDEC_SB_AUX: u32 = 12u32;
pub const MM_ICS_WAVEDEC_SB_FM_MIDIOUT: u32 = 8u32;
pub const MM_ICS_WAVEDEC_SB_MIXER: u32 = 11u32;
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIIN: u32 = 10u32;
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIOUT: u32 = 9u32;
pub const MM_ICS_WAVEDEC_SB_WAVEIN: u32 = 7u32;
pub const MM_ICS_WAVEDEC_SB_WAVEOUT: u32 = 6u32;
pub const MM_INSOFT: u32 = 94u32;
pub const MM_INTEL: u32 = 33u32;
pub const MM_INTELOPD_AUX: u32 = 401u32;
pub const MM_INTELOPD_WAVEIN: u32 = 1u32;
pub const MM_INTELOPD_WAVEOUT: u32 = 101u32;
pub const MM_INTEL_NSPMODEMLINEIN: u32 = 501u32;
pub const MM_INTEL_NSPMODEMLINEOUT: u32 = 502u32;
pub const MM_INTERACTIVE: u32 = 36u32;
pub const MM_INTERACTIVE_WAVEIN: u32 = 69u32;
pub const MM_INTERACTIVE_WAVEOUT: u32 = 69u32;
pub const MM_INTERNET: u32 = 244u32;
pub const MM_INTERNET_SSW_MIDIIN: u32 = 11u32;
pub const MM_INTERNET_SSW_MIDIOUT: u32 = 10u32;
pub const MM_INTERNET_SSW_WAVEIN: u32 = 13u32;
pub const MM_INTERNET_SSW_WAVEOUT: u32 = 12u32;
pub const MM_INVISION: u32 = 188u32;
pub const MM_IODD: u32 = 258u32;
pub const MM_IOMAGIC: u32 = 82u32;
pub const MM_IOMAGIC_TEMPO_AUXOUT: u32 = 6u32;
pub const MM_IOMAGIC_TEMPO_MIDIOUT: u32 = 4u32;
pub const MM_IOMAGIC_TEMPO_MXDOUT: u32 = 5u32;
pub const MM_IOMAGIC_TEMPO_SYNTH: u32 = 3u32;
pub const MM_IOMAGIC_TEMPO_WAVEIN: u32 = 2u32;
pub const MM_IOMAGIC_TEMPO_WAVEOUT: u32 = 1u32;
pub const MM_IPI: u32 = 238u32;
pub const MM_IPI_ACM_HSX: u32 = 1u32;
pub const MM_IPI_ACM_RPELP: u32 = 2u32;
pub const MM_IPI_AT_MIXER: u32 = 6u32;
pub const MM_IPI_AT_WAVEIN: u32 = 5u32;
pub const MM_IPI_AT_WAVEOUT: u32 = 4u32;
pub const MM_IPI_WF_ASSS: u32 = 3u32;
pub const MM_ISOLUTION: u32 = 106u32;
pub const MM_ISOLUTION_PASCAL: u32 = 1u32;
pub const MM_ITERATEDSYS: u32 = 58u32;
pub const MM_ITERATEDSYS_FUFCODEC: u32 = 1u32;
pub const MM_I_LINK: u32 = 233u32;
pub const MM_I_LINK_VOICE_CODER: u32 = 1u32;
pub const MM_KAY_ELEMETRICS: u32 = 131u32;
pub const MM_KAY_ELEMETRICS_CSL: u32 = 17152u32;
pub const MM_KAY_ELEMETRICS_CSL_4CHANNEL: u32 = 17161u32;
pub const MM_KAY_ELEMETRICS_CSL_DAT: u32 = 17160u32;
pub const MM_KORG: u32 = 55u32;
pub const MM_KORG_1212IO_MSWAVEIN: u32 = 3u32;
pub const MM_KORG_1212IO_MSWAVEOUT: u32 = 4u32;
pub const MM_KORG_PCIF_MIDIIN: u32 = 2u32;
pub const MM_KORG_PCIF_MIDIOUT: u32 = 1u32;
pub const MM_LERNOUT_ANDHAUSPIE_LHCODECACM: u32 = 1u32;
pub const MM_LERNOUT_AND_HAUSPIE: u32 = 97u32;
pub const MM_LEXICON: u32 = 236u32;
pub const MM_LEXICON_STUDIO_WAVE_IN: u32 = 2u32;
pub const MM_LEXICON_STUDIO_WAVE_OUT: u32 = 1u32;
pub const MM_LOGITECH: u32 = 60u32;
pub const MM_LUCENT: u32 = 184u32;
pub const MM_LUCENT_ACM_G723: u32 = 0u32;
pub const MM_LUCID: u32 = 221u32;
pub const MM_LUCID_PCI24WAVEIN: u32 = 1u32;
pub const MM_LUCID_PCI24WAVEOUT: u32 = 2u32;
pub const MM_LUMINOSITI: u32 = 224u32;
pub const MM_LUMINOSITI_SCWAVEIN: u32 = 1u32;
pub const MM_LUMINOSITI_SCWAVEMIX: u32 = 3u32;
pub const MM_LUMINOSITI_SCWAVEOUT: u32 = 2u32;
pub const MM_LYNX: u32 = 212u32;
pub const MM_LYRRUS: u32 = 88u32;
pub const MM_LYRRUS_BRIDGE_GUITAR: u32 = 1u32;
pub const MM_MALDEN: u32 = 261u32;
pub const MM_MARIAN: u32 = 190u32;
pub const MM_MARIAN_ARC44WAVEIN: u32 = 1u32;
pub const MM_MARIAN_ARC44WAVEOUT: u32 = 2u32;
pub const MM_MARIAN_ARC88WAVEIN: u32 = 5u32;
pub const MM_MARIAN_ARC88WAVEOUT: u32 = 6u32;
pub const MM_MARIAN_PRODIF24WAVEIN: u32 = 3u32;
pub const MM_MARIAN_PRODIF24WAVEOUT: u32 = 4u32;
pub const MM_MATROX_DIV: u32 = 254u32;
pub const MM_MATSUSHITA: u32 = 83u32;
pub const MM_MATSUSHITA_AUX: u32 = 5u32;
pub const MM_MATSUSHITA_FMSYNTH_STEREO: u32 = 3u32;
pub const MM_MATSUSHITA_MIXER: u32 = 4u32;
pub const MM_MATSUSHITA_WAVEIN: u32 = 1u32;
pub const MM_MATSUSHITA_WAVEOUT: u32 = 2u32;
pub const MM_MEDIASONIC: u32 = 71u32;
pub const MM_MEDIASONIC_ACM_G723: u32 = 1u32;
pub const MM_MEDIASONIC_ICOM: u32 = 2u32;
pub const MM_MEDIATRIX: u32 = 141u32;
pub const MM_MEDIAVISION: u32 = 3u32;
pub const MM_MEDIAVISION_CDPC: u32 = 112u32;
pub const MM_MEDIAVISION_OPUS1208: u32 = 128u32;
pub const MM_MEDIAVISION_OPUS1216: u32 = 144u32;
pub const MM_MEDIAVISION_PROAUDIO: u32 = 16u32;
pub const MM_MEDIAVISION_PROAUDIO_16: u32 = 96u32;
pub const MM_MEDIAVISION_PROAUDIO_PLUS: u32 = 80u32;
pub const MM_MEDIAVISION_PROSTUDIO_16: u32 = 96u32;
pub const MM_MEDIAVISION_THUNDER: u32 = 32u32;
pub const MM_MEDIAVISION_TPORT: u32 = 64u32;
pub const MM_MELABS: u32 = 44u32;
pub const MM_MELABS_MIDI2GO: u32 = 1u32;
pub const MM_MERGING_MPEGL3: u32 = 1u32;
pub const MM_MERGING_TECHNOLOGIES: u32 = 177u32;
pub const MM_METHEUS: u32 = 59u32;
pub const MM_METHEUS_ZIPPER: u32 = 1u32;
pub const MM_MICRONAS: u32 = 251u32;
pub const MM_MICRONAS_CLP833: u32 = 2u32;
pub const MM_MICRONAS_SC4: u32 = 1u32;
pub const MM_MINDMAKER: u32 = 263u32;
pub const MM_MINDMAKER_GC_MIXER: u32 = 3u32;
pub const MM_MINDMAKER_GC_WAVEIN: u32 = 1u32;
pub const MM_MINDMAKER_GC_WAVEOUT: u32 = 2u32;
pub const MM_MIRO: u32 = 104u32;
pub const MM_MIRO_DC30_MIX: u32 = 7u32;
pub const MM_MIRO_DC30_WAVEIN: u32 = 6u32;
pub const MM_MIRO_DC30_WAVEOUT: u32 = 5u32;
pub const MM_MIRO_MOVIEPRO: u32 = 1u32;
pub const MM_MIRO_VIDEOD1: u32 = 2u32;
pub const MM_MIRO_VIDEODC1TV: u32 = 3u32;
pub const MM_MIRO_VIDEOTD: u32 = 4u32;
pub const MM_MITEL: u32 = 16u32;
pub const MM_MITEL_MEDIAPATH_WAVEIN: u32 = 301u32;
pub const MM_MITEL_MEDIAPATH_WAVEOUT: u32 = 300u32;
pub const MM_MITEL_MPA_HANDSET_WAVEIN: u32 = 201u32;
pub const MM_MITEL_MPA_HANDSET_WAVEOUT: u32 = 200u32;
pub const MM_MITEL_MPA_HANDSFREE_WAVEIN: u32 = 203u32;
pub const MM_MITEL_MPA_HANDSFREE_WAVEOUT: u32 = 202u32;
pub const MM_MITEL_MPA_LINE1_WAVEIN: u32 = 205u32;
pub const MM_MITEL_MPA_LINE1_WAVEOUT: u32 = 204u32;
pub const MM_MITEL_MPA_LINE2_WAVEIN: u32 = 207u32;
pub const MM_MITEL_MPA_LINE2_WAVEOUT: u32 = 206u32;
pub const MM_MITEL_TALKTO_BRIDGED_WAVEIN: u32 = 105u32;
pub const MM_MITEL_TALKTO_BRIDGED_WAVEOUT: u32 = 104u32;
pub const MM_MITEL_TALKTO_HANDSET_WAVEIN: u32 = 103u32;
pub const MM_MITEL_TALKTO_HANDSET_WAVEOUT: u32 = 102u32;
pub const MM_MITEL_TALKTO_LINE_WAVEIN: u32 = 101u32;
pub const MM_MITEL_TALKTO_LINE_WAVEOUT: u32 = 100u32;
pub const MM_MMOTION_WAVEAUX: u32 = 1u32;
pub const MM_MMOTION_WAVEIN: u32 = 3u32;
pub const MM_MMOTION_WAVEOUT: u32 = 2u32;
pub const MM_MOSCOM: u32 = 68u32;
pub const MM_MOSCOM_VPC2400_IN: u32 = 1u32;
pub const MM_MOSCOM_VPC2400_OUT: u32 = 2u32;
pub const MM_MOTIONPIXELS: u32 = 193u32;
pub const MM_MOTIONPIXELS_MVI2: u32 = 1u32;
pub const MM_MOTOROLA: u32 = 48u32;
pub const MM_MOTU: u32 = 101u32;
pub const MM_MOTU_DTX_MIDI_IN_A: u32 = 801u32;
pub const MM_MOTU_DTX_MIDI_IN_B: u32 = 802u32;
pub const MM_MOTU_DTX_MIDI_IN_SYNC: u32 = 800u32;
pub const MM_MOTU_DTX_MIDI_OUT_A: u32 = 801u32;
pub const MM_MOTU_DTX_MIDI_OUT_B: u32 = 802u32;
pub const MM_MOTU_FLYER_MIDI_IN_A: u32 = 601u32;
pub const MM_MOTU_FLYER_MIDI_IN_B: u32 = 602u32;
pub const MM_MOTU_FLYER_MIDI_IN_SYNC: u32 = 600u32;
pub const MM_MOTU_FLYER_MIDI_OUT_A: u32 = 601u32;
pub const MM_MOTU_FLYER_MIDI_OUT_B: u32 = 602u32;
pub const MM_MOTU_MTPAV_MIDIIN_1: u32 = 901u32;
pub const MM_MOTU_MTPAV_MIDIIN_2: u32 = 902u32;
pub const MM_MOTU_MTPAV_MIDIIN_3: u32 = 903u32;
pub const MM_MOTU_MTPAV_MIDIIN_4: u32 = 904u32;
pub const MM_MOTU_MTPAV_MIDIIN_5: u32 = 905u32;
pub const MM_MOTU_MTPAV_MIDIIN_6: u32 = 906u32;
pub const MM_MOTU_MTPAV_MIDIIN_7: u32 = 907u32;
pub const MM_MOTU_MTPAV_MIDIIN_8: u32 = 908u32;
pub const MM_MOTU_MTPAV_MIDIIN_ADAT: u32 = 917u32;
pub const MM_MOTU_MTPAV_MIDIIN_SYNC: u32 = 900u32;
pub const MM_MOTU_MTPAV_MIDIOUT_1: u32 = 901u32;
pub const MM_MOTU_MTPAV_MIDIOUT_2: u32 = 902u32;
pub const MM_MOTU_MTPAV_MIDIOUT_3: u32 = 903u32;
pub const MM_MOTU_MTPAV_MIDIOUT_4: u32 = 904u32;
pub const MM_MOTU_MTPAV_MIDIOUT_5: u32 = 905u32;
pub const MM_MOTU_MTPAV_MIDIOUT_6: u32 = 906u32;
pub const MM_MOTU_MTPAV_MIDIOUT_7: u32 = 907u32;
pub const MM_MOTU_MTPAV_MIDIOUT_8: u32 = 908u32;
pub const MM_MOTU_MTPAV_MIDIOUT_ADAT: u32 = 917u32;
pub const MM_MOTU_MTPAV_MIDIOUT_ALL: u32 = 900u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_1: u32 = 909u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_2: u32 = 910u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_3: u32 = 911u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_4: u32 = 912u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_5: u32 = 913u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_6: u32 = 914u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_7: u32 = 915u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_8: u32 = 916u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_1: u32 = 909u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_2: u32 = 910u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_3: u32 = 911u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_4: u32 = 912u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_5: u32 = 913u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_6: u32 = 914u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_7: u32 = 915u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_8: u32 = 916u32;
pub const MM_MOTU_MTPII_MIDIIN_1: u32 = 201u32;
pub const MM_MOTU_MTPII_MIDIIN_2: u32 = 202u32;
pub const MM_MOTU_MTPII_MIDIIN_3: u32 = 203u32;
pub const MM_MOTU_MTPII_MIDIIN_4: u32 = 204u32;
pub const MM_MOTU_MTPII_MIDIIN_5: u32 = 205u32;
pub const MM_MOTU_MTPII_MIDIIN_6: u32 = 206u32;
pub const MM_MOTU_MTPII_MIDIIN_7: u32 = 207u32;
pub const MM_MOTU_MTPII_MIDIIN_8: u32 = 208u32;
pub const MM_MOTU_MTPII_MIDIIN_SYNC: u32 = 200u32;
pub const MM_MOTU_MTPII_MIDIOUT_1: u32 = 201u32;
pub const MM_MOTU_MTPII_MIDIOUT_2: u32 = 202u32;
pub const MM_MOTU_MTPII_MIDIOUT_3: u32 = 203u32;
pub const MM_MOTU_MTPII_MIDIOUT_4: u32 = 204u32;
pub const MM_MOTU_MTPII_MIDIOUT_5: u32 = 205u32;
pub const MM_MOTU_MTPII_MIDIOUT_6: u32 = 206u32;
pub const MM_MOTU_MTPII_MIDIOUT_7: u32 = 207u32;
pub const MM_MOTU_MTPII_MIDIOUT_8: u32 = 208u32;
pub const MM_MOTU_MTPII_MIDIOUT_ALL: u32 = 200u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_1: u32 = 209u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_2: u32 = 210u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_3: u32 = 211u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_4: u32 = 212u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_5: u32 = 213u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_6: u32 = 214u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_7: u32 = 215u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_8: u32 = 216u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_1: u32 = 209u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_2: u32 = 210u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_3: u32 = 211u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_4: u32 = 212u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_5: u32 = 213u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_6: u32 = 214u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_7: u32 = 215u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_8: u32 = 216u32;
pub const MM_MOTU_MTP_MIDIIN_1: u32 = 101u32;
pub const MM_MOTU_MTP_MIDIIN_2: u32 = 102u32;
pub const MM_MOTU_MTP_MIDIIN_3: u32 = 103u32;
pub const MM_MOTU_MTP_MIDIIN_4: u32 = 104u32;
pub const MM_MOTU_MTP_MIDIIN_5: u32 = 105u32;
pub const MM_MOTU_MTP_MIDIIN_6: u32 = 106u32;
pub const MM_MOTU_MTP_MIDIIN_7: u32 = 107u32;
pub const MM_MOTU_MTP_MIDIIN_8: u32 = 108u32;
pub const MM_MOTU_MTP_MIDIOUT_1: u32 = 101u32;
pub const MM_MOTU_MTP_MIDIOUT_2: u32 = 102u32;
pub const MM_MOTU_MTP_MIDIOUT_3: u32 = 103u32;
pub const MM_MOTU_MTP_MIDIOUT_4: u32 = 104u32;
pub const MM_MOTU_MTP_MIDIOUT_5: u32 = 105u32;
pub const MM_MOTU_MTP_MIDIOUT_6: u32 = 106u32;
pub const MM_MOTU_MTP_MIDIOUT_7: u32 = 107u32;
pub const MM_MOTU_MTP_MIDIOUT_8: u32 = 108u32;
pub const MM_MOTU_MTP_MIDIOUT_ALL: u32 = 100u32;
pub const MM_MOTU_MXN_MIDIIN_1: u32 = 501u32;
pub const MM_MOTU_MXN_MIDIIN_2: u32 = 502u32;
pub const MM_MOTU_MXN_MIDIIN_3: u32 = 503u32;
pub const MM_MOTU_MXN_MIDIIN_4: u32 = 504u32;
pub const MM_MOTU_MXN_MIDIIN_SYNC: u32 = 500u32;
pub const MM_MOTU_MXN_MIDIOUT_1: u32 = 501u32;
pub const MM_MOTU_MXN_MIDIOUT_2: u32 = 502u32;
pub const MM_MOTU_MXN_MIDIOUT_3: u32 = 503u32;
pub const MM_MOTU_MXN_MIDIOUT_4: u32 = 504u32;
pub const MM_MOTU_MXN_MIDIOUT_ALL: u32 = 500u32;
pub const MM_MOTU_MXPMPU_MIDIIN_1: u32 = 401u32;
pub const MM_MOTU_MXPMPU_MIDIIN_2: u32 = 402u32;
pub const MM_MOTU_MXPMPU_MIDIIN_3: u32 = 403u32;
pub const MM_MOTU_MXPMPU_MIDIIN_4: u32 = 404u32;
pub const MM_MOTU_MXPMPU_MIDIIN_5: u32 = 405u32;
pub const MM_MOTU_MXPMPU_MIDIIN_6: u32 = 406u32;
pub const MM_MOTU_MXPMPU_MIDIIN_SYNC: u32 = 400u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_1: u32 = 401u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_2: u32 = 402u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_3: u32 = 403u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_4: u32 = 404u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_5: u32 = 405u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_6: u32 = 406u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_ALL: u32 = 400u32;
pub const MM_MOTU_MXPXT_MIDIIN_1: u32 = 1001u32;
pub const MM_MOTU_MXPXT_MIDIIN_2: u32 = 1002u32;
pub const MM_MOTU_MXPXT_MIDIIN_3: u32 = 1003u32;
pub const MM_MOTU_MXPXT_MIDIIN_4: u32 = 1004u32;
pub const MM_MOTU_MXPXT_MIDIIN_5: u32 = 1005u32;
pub const MM_MOTU_MXPXT_MIDIIN_6: u32 = 1006u32;
pub const MM_MOTU_MXPXT_MIDIIN_7: u32 = 1007u32;
pub const MM_MOTU_MXPXT_MIDIIN_8: u32 = 1008u32;
pub const MM_MOTU_MXPXT_MIDIIN_SYNC: u32 = 1000u32;
pub const MM_MOTU_MXPXT_MIDIOUT_1: u32 = 1001u32;
pub const MM_MOTU_MXPXT_MIDIOUT_2: u32 = 1002u32;
pub const MM_MOTU_MXPXT_MIDIOUT_3: u32 = 1003u32;
pub const MM_MOTU_MXPXT_MIDIOUT_4: u32 = 1004u32;
pub const MM_MOTU_MXPXT_MIDIOUT_5: u32 = 1005u32;
pub const MM_MOTU_MXPXT_MIDIOUT_6: u32 = 1006u32;
pub const MM_MOTU_MXPXT_MIDIOUT_7: u32 = 1007u32;
pub const MM_MOTU_MXPXT_MIDIOUT_8: u32 = 1008u32;
pub const MM_MOTU_MXPXT_MIDIOUT_ALL: u32 = 1000u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_1: u32 = 301u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_2: u32 = 302u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_3: u32 = 303u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_4: u32 = 304u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_5: u32 = 305u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_6: u32 = 306u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_1: u32 = 301u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_2: u32 = 302u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_3: u32 = 303u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_4: u32 = 304u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_5: u32 = 305u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_6: u32 = 306u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_ALL: u32 = 300u32;
pub const MM_MOTU_MXP_MIDIIN_SYNC: u32 = 300u32;
pub const MM_MOTU_PKX_MIDI_IN_A: u32 = 701u32;
pub const MM_MOTU_PKX_MIDI_IN_B: u32 = 702u32;
pub const MM_MOTU_PKX_MIDI_IN_SYNC: u32 = 700u32;
pub const MM_MOTU_PKX_MIDI_OUT_A: u32 = 701u32;
pub const MM_MOTU_PKX_MIDI_OUT_B: u32 = 702u32;
pub const MM_MPTUS: u32 = 95u32;
pub const MM_MPTUS_SPWAVEOUT: u32 = 1u32;
pub const MM_MSFT_ACM_G711: u32 = 37u32;
pub const MM_MSFT_ACM_GSM610: u32 = 36u32;
pub const MM_MSFT_ACM_IMAADPCM: u32 = 34u32;
pub const MM_MSFT_ACM_MSADPCM: u32 = 33u32;
pub const MM_MSFT_ACM_MSAUDIO1: u32 = 39u32;
pub const MM_MSFT_ACM_MSFILTER: u32 = 35u32;
pub const MM_MSFT_ACM_MSG723: u32 = 92u32;
pub const MM_MSFT_ACM_MSNAUDIO: u32 = 91u32;
pub const MM_MSFT_ACM_MSRT24: u32 = 93u32;
pub const MM_MSFT_ACM_PCM: u32 = 38u32;
pub const MM_MSFT_ACM_WMAUDIO: u32 = 39u32;
pub const MM_MSFT_ACM_WMAUDIO2: u32 = 101u32;
pub const MM_MSFT_GENERIC_AUX_CD: u32 = 30u32;
pub const MM_MSFT_GENERIC_AUX_LINE: u32 = 28u32;
pub const MM_MSFT_GENERIC_AUX_MIC: u32 = 29u32;
pub const MM_MSFT_GENERIC_MIDIIN: u32 = 25u32;
pub const MM_MSFT_GENERIC_MIDIOUT: u32 = 26u32;
pub const MM_MSFT_GENERIC_MIDISYNTH: u32 = 27u32;
pub const MM_MSFT_GENERIC_WAVEIN: u32 = 23u32;
pub const MM_MSFT_GENERIC_WAVEOUT: u32 = 24u32;
pub const MM_MSFT_MSACM: u32 = 32u32;
pub const MM_MSFT_MSOPL_SYNTH: u32 = 76u32;
pub const MM_MSFT_SB16_AUX_CD: u32 = 66u32;
pub const MM_MSFT_SB16_AUX_LINE: u32 = 65u32;
pub const MM_MSFT_SB16_MIDIIN: u32 = 62u32;
pub const MM_MSFT_SB16_MIDIOUT: u32 = 63u32;
pub const MM_MSFT_SB16_MIXER: u32 = 67u32;
pub const MM_MSFT_SB16_SYNTH: u32 = 64u32;
pub const MM_MSFT_SB16_WAVEIN: u32 = 60u32;
pub const MM_MSFT_SB16_WAVEOUT: u32 = 61u32;
pub const MM_MSFT_SBPRO_AUX_CD: u32 = 74u32;
pub const MM_MSFT_SBPRO_AUX_LINE: u32 = 73u32;
pub const MM_MSFT_SBPRO_MIDIIN: u32 = 70u32;
pub const MM_MSFT_SBPRO_MIDIOUT: u32 = 71u32;
pub const MM_MSFT_SBPRO_MIXER: u32 = 75u32;
pub const MM_MSFT_SBPRO_SYNTH: u32 = 72u32;
pub const MM_MSFT_SBPRO_WAVEIN: u32 = 68u32;
pub const MM_MSFT_SBPRO_WAVEOUT: u32 = 69u32;
pub const MM_MSFT_VMDMS_HANDSET_WAVEIN: u32 = 82u32;
pub const MM_MSFT_VMDMS_HANDSET_WAVEOUT: u32 = 83u32;
pub const MM_MSFT_VMDMS_LINE_WAVEIN: u32 = 80u32;
pub const MM_MSFT_VMDMS_LINE_WAVEOUT: u32 = 81u32;
pub const MM_MSFT_VMDMW_HANDSET_WAVEIN: u32 = 86u32;
pub const MM_MSFT_VMDMW_HANDSET_WAVEOUT: u32 = 87u32;
pub const MM_MSFT_VMDMW_LINE_WAVEIN: u32 = 84u32;
pub const MM_MSFT_VMDMW_LINE_WAVEOUT: u32 = 85u32;
pub const MM_MSFT_VMDMW_MIXER: u32 = 88u32;
pub const MM_MSFT_VMDM_GAME_WAVEIN: u32 = 90u32;
pub const MM_MSFT_VMDM_GAME_WAVEOUT: u32 = 89u32;
pub const MM_MSFT_WDMAUDIO_AUX: u32 = 105u32;
pub const MM_MSFT_WDMAUDIO_MIDIIN: u32 = 103u32;
pub const MM_MSFT_WDMAUDIO_MIDIOUT: u32 = 102u32;
pub const MM_MSFT_WDMAUDIO_MIXER: u32 = 104u32;
pub const MM_MSFT_WDMAUDIO_WAVEIN: u32 = 101u32;
pub const MM_MSFT_WDMAUDIO_WAVEOUT: u32 = 100u32;
pub const MM_MSFT_WSS_AUX: u32 = 21u32;
pub const MM_MSFT_WSS_FMSYNTH_STEREO: u32 = 16u32;
pub const MM_MSFT_WSS_MIXER: u32 = 17u32;
pub const MM_MSFT_WSS_NT_AUX: u32 = 59u32;
pub const MM_MSFT_WSS_NT_FMSYNTH_STEREO: u32 = 57u32;
pub const MM_MSFT_WSS_NT_MIXER: u32 = 58u32;
pub const MM_MSFT_WSS_NT_WAVEIN: u32 = 55u32;
pub const MM_MSFT_WSS_NT_WAVEOUT: u32 = 56u32;
pub const MM_MSFT_WSS_OEM_AUX: u32 = 22u32;
pub const MM_MSFT_WSS_OEM_FMSYNTH_STEREO: u32 = 20u32;
pub const MM_MSFT_WSS_OEM_MIXER: u32 = 31u32;
pub const MM_MSFT_WSS_OEM_WAVEIN: u32 = 18u32;
pub const MM_MSFT_WSS_OEM_WAVEOUT: u32 = 19u32;
pub const MM_MSFT_WSS_WAVEIN: u32 = 14u32;
pub const MM_MSFT_WSS_WAVEOUT: u32 = 15u32;
pub const MM_MWM: u32 = 209u32;
pub const MM_NCR: u32 = 62u32;
pub const MM_NCR_BA_AUX: u32 = 4u32;
pub const MM_NCR_BA_MIXER: u32 = 5u32;
pub const MM_NCR_BA_SYNTH: u32 = 3u32;
pub const MM_NCR_BA_WAVEIN: u32 = 1u32;
pub const MM_NCR_BA_WAVEOUT: u32 = 2u32;
pub const MM_NEC: u32 = 26u32;
pub const MM_NEC_26_SYNTH: u32 = 9u32;
pub const MM_NEC_73_86_SYNTH: u32 = 5u32;
pub const MM_NEC_73_86_WAVEIN: u32 = 7u32;
pub const MM_NEC_73_86_WAVEOUT: u32 = 6u32;
pub const MM_NEC_JOYSTICK: u32 = 12u32;
pub const MM_NEC_MPU401_MIDIIN: u32 = 11u32;
pub const MM_NEC_MPU401_MIDIOUT: u32 = 10u32;
pub const MM_NEOMAGIC: u32 = 176u32;
pub const MM_NEOMAGIC_AUX: u32 = 6u32;
pub const MM_NEOMAGIC_MIDIIN: u32 = 5u32;
pub const MM_NEOMAGIC_MIDIOUT: u32 = 4u32;
pub const MM_NEOMAGIC_MW3DX_AUX: u32 = 17u32;
pub const MM_NEOMAGIC_MW3DX_FMSYNTH: u32 = 14u32;
pub const MM_NEOMAGIC_MW3DX_GMSYNTH: u32 = 15u32;
pub const MM_NEOMAGIC_MW3DX_MIDIIN: u32 = 13u32;
pub const MM_NEOMAGIC_MW3DX_MIDIOUT: u32 = 12u32;
pub const MM_NEOMAGIC_MW3DX_MIXER: u32 = 16u32;
pub const MM_NEOMAGIC_MW3DX_WAVEIN: u32 = 11u32;
pub const MM_NEOMAGIC_MW3DX_WAVEOUT: u32 = 10u32;
pub const MM_NEOMAGIC_MWAVE_AUX: u32 = 25u32;
pub const MM_NEOMAGIC_MWAVE_MIDIIN: u32 = 23u32;
pub const MM_NEOMAGIC_MWAVE_MIDIOUT: u32 = 22u32;
pub const MM_NEOMAGIC_MWAVE_MIXER: u32 = 24u32;
pub const MM_NEOMAGIC_MWAVE_WAVEIN: u32 = 21u32;
pub const MM_NEOMAGIC_MWAVE_WAVEOUT: u32 = 20u32;
pub const MM_NEOMAGIC_SYNTH: u32 = 1u32;
pub const MM_NEOMAGIC_WAVEIN: u32 = 3u32;
pub const MM_NEOMAGIC_WAVEOUT: u32 = 2u32;
pub const MM_NETSCAPE: u32 = 166u32;
pub const MM_NETXL: u32 = 8u32;
pub const MM_NETXL_XLVIDEO: u32 = 1u32;
pub const MM_NEWMEDIA: u32 = 86u32;
pub const MM_NEWMEDIA_WAVJAMMER: u32 = 1u32;
pub const MM_NMP: u32 = 195u32;
pub const MM_NMP_ACM_AMR: u32 = 10u32;
pub const MM_NMP_CCP_WAVEIN: u32 = 1u32;
pub const MM_NMP_CCP_WAVEOUT: u32 = 2u32;
pub const MM_NMS: u32 = 87u32;
pub const MM_NOGATECH: u32 = 75u32;
pub const MM_NORRIS: u32 = 150u32;
pub const MM_NORRIS_VOICELINK: u32 = 1u32;
pub const MM_NORTEL_MPXAC_WAVEIN: u32 = 1u32;
pub const MM_NORTEL_MPXAC_WAVEOUT: u32 = 2u32;
pub const MM_NORTHERN_TELECOM: u32 = 115u32;
pub const MM_NVIDIA: u32 = 127u32;
pub const MM_NVIDIA_AUX: u32 = 7u32;
pub const MM_NVIDIA_GAMEPORT: u32 = 5u32;
pub const MM_NVIDIA_MIDIIN: u32 = 4u32;
pub const MM_NVIDIA_MIDIOUT: u32 = 3u32;
pub const MM_NVIDIA_MIXER: u32 = 6u32;
pub const MM_NVIDIA_WAVEIN: u32 = 2u32;
pub const MM_NVIDIA_WAVEOUT: u32 = 1u32;
pub const MM_OKI: u32 = 79u32;
pub const MM_OKSORI: u32 = 128u32;
pub const MM_OKSORI_BASE: u32 = 0u32;
pub const MM_OKSORI_EXT_MIC1: u32 = 15u32;
pub const MM_OKSORI_EXT_MIC2: u32 = 16u32;
pub const MM_OKSORI_FM_OPL4: u32 = 5u32;
pub const MM_OKSORI_MIDIIN: u32 = 18u32;
pub const MM_OKSORI_MIDIOUT: u32 = 17u32;
pub const MM_OKSORI_MIX_AUX1: u32 = 13u32;
pub const MM_OKSORI_MIX_CD: u32 = 10u32;
pub const MM_OKSORI_MIX_ECHO: u32 = 12u32;
pub const MM_OKSORI_MIX_FM: u32 = 8u32;
pub const MM_OKSORI_MIX_LINE: u32 = 9u32;
pub const MM_OKSORI_MIX_LINE1: u32 = 14u32;
pub const MM_OKSORI_MIX_MASTER: u32 = 6u32;
pub const MM_OKSORI_MIX_MIC: u32 = 11u32;
pub const MM_OKSORI_MIX_WAVE: u32 = 7u32;
pub const MM_OKSORI_MPEG_CDVISION: u32 = 19u32;
pub const MM_OKSORI_OSR16_WAVEIN: u32 = 4u32;
pub const MM_OKSORI_OSR16_WAVEOUT: u32 = 3u32;
pub const MM_OKSORI_OSR8_WAVEIN: u32 = 2u32;
pub const MM_OKSORI_OSR8_WAVEOUT: u32 = 1u32;
pub const MM_OLIVETTI: u32 = 81u32;
pub const MM_OLIVETTI_ACM_ADPCM: u32 = 10u32;
pub const MM_OLIVETTI_ACM_CELP: u32 = 11u32;
pub const MM_OLIVETTI_ACM_GSM: u32 = 9u32;
pub const MM_OLIVETTI_ACM_OPR: u32 = 13u32;
pub const MM_OLIVETTI_ACM_SBC: u32 = 12u32;
pub const MM_OLIVETTI_AUX: u32 = 4u32;
pub const MM_OLIVETTI_JOYSTICK: u32 = 8u32;
pub const MM_OLIVETTI_MIDIIN: u32 = 5u32;
pub const MM_OLIVETTI_MIDIOUT: u32 = 6u32;
pub const MM_OLIVETTI_MIXER: u32 = 3u32;
pub const MM_OLIVETTI_SYNTH: u32 = 7u32;
pub const MM_OLIVETTI_WAVEIN: u32 = 1u32;
pub const MM_OLIVETTI_WAVEOUT: u32 = 2u32;
pub const MM_ONLIVE: u32 = 200u32;
pub const MM_ONLIVE_MPCODEC: u32 = 1u32;
pub const MM_OPCODE: u32 = 113u32;
pub const MM_OPTI: u32 = 90u32;
pub const MM_OPTI_M16_AUX: u32 = 7u32;
pub const MM_OPTI_M16_FMSYNTH_STEREO: u32 = 1u32;
pub const MM_OPTI_M16_MIDIIN: u32 = 2u32;
pub const MM_OPTI_M16_MIDIOUT: u32 = 3u32;
pub const MM_OPTI_M16_MIXER: u32 = 6u32;
pub const MM_OPTI_M16_WAVEIN: u32 = 4u32;
pub const MM_OPTI_M16_WAVEOUT: u32 = 5u32;
pub const MM_OPTI_M32_AUX: u32 = 38u32;
pub const MM_OPTI_M32_MIDIIN: u32 = 34u32;
pub const MM_OPTI_M32_MIDIOUT: u32 = 35u32;
pub const MM_OPTI_M32_MIXER: u32 = 37u32;
pub const MM_OPTI_M32_SYNTH_STEREO: u32 = 36u32;
pub const MM_OPTI_M32_WAVEIN: u32 = 32u32;
pub const MM_OPTI_M32_WAVEOUT: u32 = 33u32;
pub const MM_OPTI_P16_AUX: u32 = 22u32;
pub const MM_OPTI_P16_FMSYNTH_STEREO: u32 = 16u32;
pub const MM_OPTI_P16_MIDIIN: u32 = 17u32;
pub const MM_OPTI_P16_MIDIOUT: u32 = 18u32;
pub const MM_OPTI_P16_MIXER: u32 = 21u32;
pub const MM_OPTI_P16_WAVEIN: u32 = 19u32;
pub const MM_OPTI_P16_WAVEOUT: u32 = 20u32;
pub const MM_OPUS1208_AUX: u32 = 135u32;
pub const MM_OPUS1208_MIXER: u32 = 134u32;
pub const MM_OPUS1208_SYNTH: u32 = 131u32;
pub const MM_OPUS1208_WAVEIN: u32 = 133u32;
pub const MM_OPUS1208_WAVEOUT: u32 = 132u32;
pub const MM_OPUS1216_AUX: u32 = 151u32;
pub const MM_OPUS1216_MIDIIN: u32 = 146u32;
pub const MM_OPUS1216_MIDIOUT: u32 = 145u32;
pub const MM_OPUS1216_MIXER: u32 = 150u32;
pub const MM_OPUS1216_SYNTH: u32 = 147u32;
pub const MM_OPUS1216_WAVEIN: u32 = 149u32;
pub const MM_OPUS1216_WAVEOUT: u32 = 148u32;
pub const MM_OPUS401_MIDIIN: u32 = 130u32;
pub const MM_OPUS401_MIDIOUT: u32 = 129u32;
pub const MM_OSITECH: u32 = 103u32;
pub const MM_OSITECH_TRUMPCARD: u32 = 1u32;
pub const MM_OSPREY: u32 = 140u32;
pub const MM_OSPREY_1000WAVEIN: u32 = 1u32;
pub const MM_OSPREY_1000WAVEOUT: u32 = 2u32;
pub const MM_OTI: u32 = 180u32;
pub const MM_OTI_611MIDIN: u32 = 18u32;
pub const MM_OTI_611MIDIOUT: u32 = 19u32;
pub const MM_OTI_611MIXER: u32 = 7u32;
pub const MM_OTI_611WAVEIN: u32 = 5u32;
pub const MM_OTI_611WAVEOUT: u32 = 6u32;
pub const MM_PACIFICRESEARCH: u32 = 210u32;
pub const MM_PCSPEAKER_WAVEOUT: u32 = 13u32;
pub const MM_PHILIPS_ACM_LPCBB: u32 = 1u32;
pub const MM_PHILIPS_SPEECH_PROCESSING: u32 = 7u32;
pub const MM_PHONET: u32 = 203u32;
pub const MM_PHONET_PP_MIXER: u32 = 3u32;
pub const MM_PHONET_PP_WAVEIN: u32 = 2u32;
pub const MM_PHONET_PP_WAVEOUT: u32 = 1u32;
pub const MM_PICTURETEL: u32 = 138u32;
pub const MM_PID_UNMAPPED: u32 = 65535u32;
pub const MM_PINNACLE: u32 = 218u32;
pub const MM_PRAGMATRAX: u32 = 5u32;
pub const MM_PRECEPT: u32 = 153u32;
pub const MM_PROAUD_16_AUX: u32 = 103u32;
pub const MM_PROAUD_16_MIDIIN: u32 = 98u32;
pub const MM_PROAUD_16_MIDIOUT: u32 = 97u32;
pub const MM_PROAUD_16_MIXER: u32 = 102u32;
pub const MM_PROAUD_16_SYNTH: u32 = 99u32;
pub const MM_PROAUD_16_WAVEIN: u32 = 101u32;
pub const MM_PROAUD_16_WAVEOUT: u32 = 100u32;
pub const MM_PROAUD_AUX: u32 = 23u32;
pub const MM_PROAUD_MIDIIN: u32 = 18u32;
pub const MM_PROAUD_MIDIOUT: u32 = 17u32;
pub const MM_PROAUD_MIXER: u32 = 22u32;
pub const MM_PROAUD_PLUS_AUX: u32 = 87u32;
pub const MM_PROAUD_PLUS_MIDIIN: u32 = 82u32;
pub const MM_PROAUD_PLUS_MIDIOUT: u32 = 81u32;
pub const MM_PROAUD_PLUS_MIXER: u32 = 86u32;
pub const MM_PROAUD_PLUS_SYNTH: u32 = 83u32;
pub const MM_PROAUD_PLUS_WAVEIN: u32 = 85u32;
pub const MM_PROAUD_PLUS_WAVEOUT: u32 = 84u32;
pub const MM_PROAUD_SYNTH: u32 = 19u32;
pub const MM_PROAUD_WAVEIN: u32 = 21u32;
pub const MM_PROAUD_WAVEOUT: u32 = 20u32;
pub const MM_QCIAR: u32 = 98u32;
pub const MM_QDESIGN: u32 = 194u32;
pub const MM_QDESIGN_ACM_MPEG: u32 = 1u32;
pub const MM_QDESIGN_ACM_QDESIGN_MUSIC: u32 = 2u32;
pub const MM_QTEAM: u32 = 169u32;
pub const MM_QUALCOMM: u32 = 215u32;
pub const MM_QUANTUM3D: u32 = 17u32;
pub const MM_QUARTERDECK: u32 = 134u32;
pub const MM_QUARTERDECK_LHWAVEIN: u32 = 0u32;
pub const MM_QUARTERDECK_LHWAVEOUT: u32 = 1u32;
pub const MM_QUICKAUDIO: u32 = 255u32;
pub const MM_QUICKAUDIO_MAXIMIDI: u32 = 2u32;
pub const MM_QUICKAUDIO_MINIMIDI: u32 = 1u32;
pub const MM_QUICKNET: u32 = 173u32;
pub const MM_QUICKNET_PJWAVEIN: u32 = 1u32;
pub const MM_QUICKNET_PJWAVEOUT: u32 = 2u32;
pub const MM_RADIUS: u32 = 110u32;
pub const MM_RHETOREX: u32 = 120u32;
pub const MM_RHETOREX_WAVEIN: u32 = 1u32;
pub const MM_RHETOREX_WAVEOUT: u32 = 2u32;
pub const MM_RICHMOND: u32 = 257u32;
pub const MM_ROCKWELL: u32 = 111u32;
pub const MM_ROLAND: u32 = 24u32;
pub const MM_ROLAND_MPU401_MIDIIN: u32 = 16u32;
pub const MM_ROLAND_MPU401_MIDIOUT: u32 = 15u32;
pub const MM_ROLAND_RAP10_MIDIIN: u32 = 11u32;
pub const MM_ROLAND_RAP10_MIDIOUT: u32 = 10u32;
pub const MM_ROLAND_RAP10_SYNTH: u32 = 12u32;
pub const MM_ROLAND_RAP10_WAVEIN: u32 = 14u32;
pub const MM_ROLAND_RAP10_WAVEOUT: u32 = 13u32;
pub const MM_ROLAND_SC7_MIDIIN: u32 = 22u32;
pub const MM_ROLAND_SC7_MIDIOUT: u32 = 21u32;
pub const MM_ROLAND_SCP_AUX: u32 = 48u32;
pub const MM_ROLAND_SCP_MIDIIN: u32 = 39u32;
pub const MM_ROLAND_SCP_MIDIOUT: u32 = 38u32;
pub const MM_ROLAND_SCP_MIXER: u32 = 42u32;
pub const MM_ROLAND_SCP_WAVEIN: u32 = 41u32;
pub const MM_ROLAND_SCP_WAVEOUT: u32 = 40u32;
pub const MM_ROLAND_SERIAL_MIDIIN: u32 = 24u32;
pub const MM_ROLAND_SERIAL_MIDIOUT: u32 = 23u32;
pub const MM_ROLAND_SMPU_MIDIINA: u32 = 19u32;
pub const MM_ROLAND_SMPU_MIDIINB: u32 = 20u32;
pub const MM_ROLAND_SMPU_MIDIOUTA: u32 = 17u32;
pub const MM_ROLAND_SMPU_MIDIOUTB: u32 = 18u32;
pub const MM_RZS: u32 = 216u32;
pub const MM_RZS_ACM_TUBGSM: u32 = 1u32;
pub const MM_S3: u32 = 164u32;
pub const MM_S3_AUX: u32 = 7u32;
pub const MM_S3_FMSYNTH: u32 = 5u32;
pub const MM_S3_MIDIIN: u32 = 4u32;
pub const MM_S3_MIDIOUT: u32 = 3u32;
pub const MM_S3_MIXER: u32 = 6u32;
pub const MM_S3_WAVEIN: u32 = 2u32;
pub const MM_S3_WAVEOUT: u32 = 1u32;
pub const MM_SANYO: u32 = 72u32;
pub const MM_SANYO_ACM_LD_ADPCM: u32 = 1u32;
pub const MM_SCALACS: u32 = 54u32;
pub const MM_SEERSYS: u32 = 137u32;
pub const MM_SEERSYS_REALITY: u32 = 6u32;
pub const MM_SEERSYS_SEERMIX: u32 = 3u32;
pub const MM_SEERSYS_SEERSYNTH: u32 = 1u32;
pub const MM_SEERSYS_SEERWAVE: u32 = 2u32;
pub const MM_SEERSYS_WAVESYNTH: u32 = 4u32;
pub const MM_SEERSYS_WAVESYNTH_WG: u32 = 5u32;
pub const MM_SELSIUS_SYSTEMS: u32 = 234u32;
pub const MM_SELSIUS_SYSTEMS_RTPWAVEIN: u32 = 2u32;
pub const MM_SELSIUS_SYSTEMS_RTPWAVEOUT: u32 = 1u32;
pub const MM_SGI: u32 = 237u32;
pub const MM_SGI_320_MIXER: u32 = 3u32;
pub const MM_SGI_320_WAVEIN: u32 = 1u32;
pub const MM_SGI_320_WAVEOUT: u32 = 2u32;
pub const MM_SGI_540_MIXER: u32 = 6u32;
pub const MM_SGI_540_WAVEIN: u32 = 4u32;
pub const MM_SGI_540_WAVEOUT: u32 = 5u32;
pub const MM_SGI_RAD_ADAT8CHAN_WAVEIN: u32 = 19u32;
pub const MM_SGI_RAD_ADAT8CHAN_WAVEOUT: u32 = 32u32;
pub const MM_SGI_RAD_ADATMONO1_WAVEIN: u32 = 7u32;
pub const MM_SGI_RAD_ADATMONO1_WAVEOUT: u32 = 20u32;
pub const MM_SGI_RAD_ADATMONO2_WAVEIN: u32 = 8u32;
pub const MM_SGI_RAD_ADATMONO2_WAVEOUT: u32 = 21u32;
pub const MM_SGI_RAD_ADATMONO3_WAVEIN: u32 = 9u32;
pub const MM_SGI_RAD_ADATMONO3_WAVEOUT: u32 = 22u32;
pub const MM_SGI_RAD_ADATMONO4_WAVEIN: u32 = 10u32;
pub const MM_SGI_RAD_ADATMONO4_WAVEOUT: u32 = 23u32;
pub const MM_SGI_RAD_ADATMONO5_WAVEIN: u32 = 11u32;
pub const MM_SGI_RAD_ADATMONO5_WAVEOUT: u32 = 24u32;
pub const MM_SGI_RAD_ADATMONO6_WAVEIN: u32 = 12u32;
pub const MM_SGI_RAD_ADATMONO6_WAVEOUT: u32 = 25u32;
pub const MM_SGI_RAD_ADATMONO7_WAVEIN: u32 = 13u32;
pub const MM_SGI_RAD_ADATMONO7_WAVEOUT: u32 = 26u32;
pub const MM_SGI_RAD_ADATMONO8_WAVEIN: u32 = 14u32;
pub const MM_SGI_RAD_ADATMONO8_WAVEOUT: u32 = 27u32;
pub const MM_SGI_RAD_ADATSTEREO12_WAVEIN: u32 = 15u32;
pub const MM_SGI_RAD_ADATSTEREO12_WAVEOUT: u32 = 28u32;
pub const MM_SGI_RAD_ADATSTEREO32_WAVEOUT: u32 = 29u32;
pub const MM_SGI_RAD_ADATSTEREO34_WAVEIN: u32 = 16u32;
pub const MM_SGI_RAD_ADATSTEREO56_WAVEIN: u32 = 17u32;
pub const MM_SGI_RAD_ADATSTEREO56_WAVEOUT: u32 = 30u32;
pub const MM_SGI_RAD_ADATSTEREO78_WAVEIN: u32 = 18u32;
pub const MM_SGI_RAD_ADATSTEREO78_WAVEOUT: u32 = 31u32;
pub const MM_SGI_RAD_AESMONO1_WAVEIN: u32 = 33u32;
pub const MM_SGI_RAD_AESMONO1_WAVEOUT: u32 = 36u32;
pub const MM_SGI_RAD_AESMONO2_WAVEIN: u32 = 34u32;
pub const MM_SGI_RAD_AESMONO2_WAVEOUT: u32 = 37u32;
pub const MM_SGI_RAD_AESSTEREO_WAVEIN: u32 = 35u32;
pub const MM_SGI_RAD_AESSTEREO_WAVEOUT: u32 = 38u32;
pub const MM_SHARP: u32 = 183u32;
pub const MM_SHARP_MDC_AUX: u32 = 6u32;
pub const MM_SHARP_MDC_AUX_BASS: u32 = 101u32;
pub const MM_SHARP_MDC_AUX_CHR: u32 = 109u32;
pub const MM_SHARP_MDC_AUX_MASTER: u32 = 100u32;
pub const MM_SHARP_MDC_AUX_MIDI_VOL: u32 = 103u32;
pub const MM_SHARP_MDC_AUX_RVB: u32 = 108u32;
pub const MM_SHARP_MDC_AUX_TREBLE: u32 = 102u32;
pub const MM_SHARP_MDC_AUX_VOL: u32 = 107u32;
pub const MM_SHARP_MDC_AUX_WAVE_CHR: u32 = 106u32;
pub const MM_SHARP_MDC_AUX_WAVE_RVB: u32 = 105u32;
pub const MM_SHARP_MDC_AUX_WAVE_VOL: u32 = 104u32;
pub const MM_SHARP_MDC_MIDI_IN: u32 = 2u32;
pub const MM_SHARP_MDC_MIDI_OUT: u32 = 3u32;
pub const MM_SHARP_MDC_MIDI_SYNTH: u32 = 1u32;
pub const MM_SHARP_MDC_MIXER: u32 = 10u32;
pub const MM_SHARP_MDC_WAVE_IN: u32 = 4u32;
pub const MM_SHARP_MDC_WAVE_OUT: u32 = 5u32;
pub const MM_SICRESOURCE: u32 = 175u32;
pub const MM_SICRESOURCE_SSO3D: u32 = 2u32;
pub const MM_SICRESOURCE_SSOW3DI: u32 = 3u32;
pub const MM_SIEMENS_SBC: u32 = 201u32;
pub const MM_SIERRA: u32 = 40u32;
pub const MM_SIERRA_ARIA_AUX: u32 = 25u32;
pub const MM_SIERRA_ARIA_AUX2: u32 = 32u32;
pub const MM_SIERRA_ARIA_MIDIIN: u32 = 21u32;
pub const MM_SIERRA_ARIA_MIDIOUT: u32 = 20u32;
pub const MM_SIERRA_ARIA_SYNTH: u32 = 22u32;
pub const MM_SIERRA_ARIA_WAVEIN: u32 = 24u32;
pub const MM_SIERRA_ARIA_WAVEOUT: u32 = 23u32;
pub const MM_SIERRA_QUARTET_AUX_CD: u32 = 85u32;
pub const MM_SIERRA_QUARTET_AUX_LINE: u32 = 86u32;
pub const MM_SIERRA_QUARTET_AUX_MODEM: u32 = 87u32;
pub const MM_SIERRA_QUARTET_MIDIIN: u32 = 82u32;
pub const MM_SIERRA_QUARTET_MIDIOUT: u32 = 83u32;
pub const MM_SIERRA_QUARTET_MIXER: u32 = 88u32;
pub const MM_SIERRA_QUARTET_SYNTH: u32 = 84u32;
pub const MM_SIERRA_QUARTET_WAVEIN: u32 = 80u32;
pub const MM_SIERRA_QUARTET_WAVEOUT: u32 = 81u32;
pub const MM_SILICONSOFT: u32 = 69u32;
pub const MM_SILICONSOFT_SC1_WAVEIN: u32 = 1u32;
pub const MM_SILICONSOFT_SC1_WAVEOUT: u32 = 2u32;
pub const MM_SILICONSOFT_SC2_WAVEIN: u32 = 3u32;
pub const MM_SILICONSOFT_SC2_WAVEOUT: u32 = 4u32;
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEIN: u32 = 6u32;
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEOUT: u32 = 7u32;
pub const MM_SILICONSOFT_SOUNDJR2_WAVEOUT: u32 = 5u32;
pub const MM_SILICONSOFT_SOUNDJR3_WAVEOUT: u32 = 8u32;
pub const MM_SIPROLAB: u32 = 211u32;
pub const MM_SIPROLAB_ACELPNET: u32 = 1u32;
pub const MM_SNI: u32 = 18u32;
pub const MM_SNI_ACM_G721: u32 = 1u32;
pub const MM_SOFTLAB_NSK: u32 = 228u32;
pub const MM_SOFTLAB_NSK_FRW_AUX: u32 = 4u32;
pub const MM_SOFTLAB_NSK_FRW_MIXER: u32 = 3u32;
pub const MM_SOFTLAB_NSK_FRW_WAVEIN: u32 = 1u32;
pub const MM_SOFTLAB_NSK_FRW_WAVEOUT: u32 = 2u32;
pub const MM_SOFTSOUND: u32 = 149u32;
pub const MM_SOFTSOUND_CODEC: u32 = 1u32;
pub const MM_SONICFOUNDRY: u32 = 66u32;
pub const MM_SONORUS: u32 = 230u32;
pub const MM_SONORUS_STUDIO: u32 = 1u32;
pub const MM_SONY: u32 = 245u32;
pub const MM_SONY_ACM_SCX: u32 = 1u32;
pub const MM_SORVIS: u32 = 187u32;
pub const MM_SOUNDESIGNS: u32 = 142u32;
pub const MM_SOUNDESIGNS_WAVEIN: u32 = 1u32;
pub const MM_SOUNDESIGNS_WAVEOUT: u32 = 2u32;
pub const MM_SOUNDSCAPE_AUX: u32 = 24u32;
pub const MM_SOUNDSCAPE_MIDIIN: u32 = 21u32;
pub const MM_SOUNDSCAPE_MIDIOUT: u32 = 20u32;
pub const MM_SOUNDSCAPE_MIXER: u32 = 23u32;
pub const MM_SOUNDSCAPE_SYNTH: u32 = 22u32;
pub const MM_SOUNDSCAPE_WAVEIN: u32 = 19u32;
pub const MM_SOUNDSCAPE_WAVEOUT: u32 = 17u32;
pub const MM_SOUNDSCAPE_WAVEOUT_AUX: u32 = 18u32;
pub const MM_SOUNDSPACE: u32 = 167u32;
pub const MM_SPECTRUM_PRODUCTIONS: u32 = 213u32;
pub const MM_SPECTRUM_SIGNAL_PROCESSING: u32 = 144u32;
pub const MM_SPEECHCOMP: u32 = 76u32;
pub const MM_SPLASH_STUDIOS: u32 = 133u32;
pub const MM_SSP_SNDFESAUX: u32 = 7u32;
pub const MM_SSP_SNDFESMIDIIN: u32 = 3u32;
pub const MM_SSP_SNDFESMIDIOUT: u32 = 4u32;
pub const MM_SSP_SNDFESMIX: u32 = 6u32;
pub const MM_SSP_SNDFESSYNTH: u32 = 5u32;
pub const MM_SSP_SNDFESWAVEIN: u32 = 1u32;
pub const MM_SSP_SNDFESWAVEOUT: u32 = 2u32;
pub const MM_STUDER: u32 = 171u32;
pub const MM_STUDIO_16_AUX: u32 = 103u32;
pub const MM_STUDIO_16_MIDIIN: u32 = 98u32;
pub const MM_STUDIO_16_MIDIOUT: u32 = 97u32;
pub const MM_STUDIO_16_MIXER: u32 = 102u32;
pub const MM_STUDIO_16_SYNTH: u32 = 99u32;
pub const MM_STUDIO_16_WAVEIN: u32 = 101u32;
pub const MM_STUDIO_16_WAVEOUT: u32 = 100u32;
pub const MM_ST_MICROELECTRONICS: u32 = 265u32;
pub const MM_SUNCOM: u32 = 186u32;
pub const MM_SUPERMAC: u32 = 73u32;
pub const MM_SYDEC_NV: u32 = 248u32;
pub const MM_SYDEC_NV_WAVEIN: u32 = 1u32;
pub const MM_SYDEC_NV_WAVEOUT: u32 = 2u32;
pub const MM_TANDY: u32 = 29u32;
pub const MM_TANDY_PSSJWAVEIN: u32 = 9u32;
pub const MM_TANDY_PSSJWAVEOUT: u32 = 10u32;
pub const MM_TANDY_SENS_MMAMIDIIN: u32 = 6u32;
pub const MM_TANDY_SENS_MMAMIDIOUT: u32 = 7u32;
pub const MM_TANDY_SENS_MMAWAVEIN: u32 = 4u32;
pub const MM_TANDY_SENS_MMAWAVEOUT: u32 = 5u32;
pub const MM_TANDY_SENS_VISWAVEOUT: u32 = 8u32;
pub const MM_TANDY_VISBIOSSYNTH: u32 = 3u32;
pub const MM_TANDY_VISWAVEIN: u32 = 1u32;
pub const MM_TANDY_VISWAVEOUT: u32 = 2u32;
pub const MM_TBS_TROPEZ_AUX1: u32 = 39u32;
pub const MM_TBS_TROPEZ_AUX2: u32 = 40u32;
pub const MM_TBS_TROPEZ_LINE: u32 = 41u32;
pub const MM_TBS_TROPEZ_WAVEIN: u32 = 37u32;
pub const MM_TBS_TROPEZ_WAVEOUT: u32 = 38u32;
pub const MM_TDK: u32 = 135u32;
pub const MM_TDK_MW_AUX: u32 = 6u32;
pub const MM_TDK_MW_AUX_BASS: u32 = 101u32;
pub const MM_TDK_MW_AUX_CHR: u32 = 109u32;
pub const MM_TDK_MW_AUX_MASTER: u32 = 100u32;
pub const MM_TDK_MW_AUX_MIDI_VOL: u32 = 103u32;
pub const MM_TDK_MW_AUX_RVB: u32 = 108u32;
pub const MM_TDK_MW_AUX_TREBLE: u32 = 102u32;
pub const MM_TDK_MW_AUX_VOL: u32 = 107u32;
pub const MM_TDK_MW_AUX_WAVE_CHR: u32 = 106u32;
pub const MM_TDK_MW_AUX_WAVE_RVB: u32 = 105u32;
pub const MM_TDK_MW_AUX_WAVE_VOL: u32 = 104u32;
pub const MM_TDK_MW_MIDI_IN: u32 = 2u32;
pub const MM_TDK_MW_MIDI_OUT: u32 = 3u32;
pub const MM_TDK_MW_MIDI_SYNTH: u32 = 1u32;
pub const MM_TDK_MW_MIXER: u32 = 10u32;
pub const MM_TDK_MW_WAVE_IN: u32 = 4u32;
pub const MM_TDK_MW_WAVE_OUT: u32 = 5u32;
pub const MM_TELEKOL: u32 = 264u32;
pub const MM_TELEKOL_WAVEIN: u32 = 2u32;
pub const MM_TELEKOL_WAVEOUT: u32 = 1u32;
pub const MM_TERALOGIC: u32 = 202u32;
pub const MM_TERRATEC: u32 = 70u32;
pub const MM_THUNDER_AUX: u32 = 39u32;
pub const MM_THUNDER_SYNTH: u32 = 35u32;
pub const MM_THUNDER_WAVEIN: u32 = 37u32;
pub const MM_THUNDER_WAVEOUT: u32 = 36u32;
pub const MM_TPORT_SYNTH: u32 = 67u32;
pub const MM_TPORT_WAVEIN: u32 = 66u32;
pub const MM_TPORT_WAVEOUT: u32 = 65u32;
pub const MM_TRUEVISION: u32 = 51u32;
pub const MM_TRUEVISION_WAVEIN1: u32 = 1u32;
pub const MM_TRUEVISION_WAVEOUT1: u32 = 2u32;
pub const MM_TTEWS_AUX: u32 = 9u32;
pub const MM_TTEWS_MIDIIN: u32 = 3u32;
pub const MM_TTEWS_MIDIMONITOR: u32 = 6u32;
pub const MM_TTEWS_MIDIOUT: u32 = 4u32;
pub const MM_TTEWS_MIDISYNTH: u32 = 5u32;
pub const MM_TTEWS_MIXER: u32 = 10u32;
pub const MM_TTEWS_VMIDIIN: u32 = 7u32;
pub const MM_TTEWS_VMIDIOUT: u32 = 8u32;
pub const MM_TTEWS_WAVEIN: u32 = 1u32;
pub const MM_TTEWS_WAVEOUT: u32 = 2u32;
pub const MM_TURTLE_BEACH: u32 = 21u32;
pub const MM_UHER_INFORMATIC: u32 = 247u32;
pub const MM_UH_ACM_ADPCM: u32 = 1u32;
pub const MM_UNISYS: u32 = 223u32;
pub const MM_UNISYS_ACM_NAP: u32 = 1u32;
pub const MM_UNMAPPED: u32 = 65535u32;
pub const MM_VAL: u32 = 35u32;
pub const MM_VAL_MICROKEY_AP_WAVEIN: u32 = 1u32;
pub const MM_VAL_MICROKEY_AP_WAVEOUT: u32 = 2u32;
pub const MM_VANKOEVERING: u32 = 168u32;
pub const MM_VIA: u32 = 250u32;
pub const MM_VIA_AUX: u32 = 4u32;
pub const MM_VIA_MIXER: u32 = 3u32;
pub const MM_VIA_MPU401_MIDIIN: u32 = 6u32;
pub const MM_VIA_MPU401_MIDIOUT: u32 = 5u32;
pub const MM_VIA_SWFM_SYNTH: u32 = 7u32;
pub const MM_VIA_WAVEIN: u32 = 2u32;
pub const MM_VIA_WAVEOUT: u32 = 1u32;
pub const MM_VIA_WDM_MIXER: u32 = 10u32;
pub const MM_VIA_WDM_MPU401_MIDIIN: u32 = 12u32;
pub const MM_VIA_WDM_MPU401_MIDIOUT: u32 = 11u32;
pub const MM_VIA_WDM_WAVEIN: u32 = 9u32;
pub const MM_VIA_WDM_WAVEOUT: u32 = 8u32;
pub const MM_VIDEOLOGIC: u32 = 53u32;
pub const MM_VIDEOLOGIC_MSWAVEIN: u32 = 1u32;
pub const MM_VIDEOLOGIC_MSWAVEOUT: u32 = 2u32;
pub const MM_VIENNASYS: u32 = 157u32;
pub const MM_VIENNASYS_TSP_WAVE_DRIVER: u32 = 1u32;
pub const MM_VIONA: u32 = 161u32;
pub const MM_VIONAQVINPCI_WAVEOUT: u32 = 3u32;
pub const MM_VIONA_BUSTER_MIXER: u32 = 4u32;
pub const MM_VIONA_CINEMASTER_MIXER: u32 = 5u32;
pub const MM_VIONA_CONCERTO_MIXER: u32 = 6u32;
pub const MM_VIONA_QVINPCI_MIXER: u32 = 1u32;
pub const MM_VIONA_QVINPCI_WAVEIN: u32 = 2u32;
pub const MM_VIRTUALMUSIC: u32 = 205u32;
pub const MM_VITEC: u32 = 67u32;
pub const MM_VITEC_VMAKER: u32 = 1u32;
pub const MM_VITEC_VMPRO: u32 = 2u32;
pub const MM_VIVO: u32 = 182u32;
pub const MM_VIVO_AUDIO_CODEC: u32 = 1u32;
pub const MM_VKC_MPU401_MIDIIN: u32 = 256u32;
pub const MM_VKC_MPU401_MIDIOUT: u32 = 512u32;
pub const MM_VKC_SERIAL_MIDIIN: u32 = 257u32;
pub const MM_VKC_SERIAL_MIDIOUT: u32 = 513u32;
pub const MM_VOCALTEC: u32 = 23u32;
pub const MM_VOCALTEC_WAVEIN: u32 = 2u32;
pub const MM_VOCALTEC_WAVEOUT: u32 = 1u32;
pub const MM_VOICEINFO: u32 = 156u32;
pub const MM_VOICEMIXER: u32 = 1u32;
pub const MM_VOXWARE: u32 = 114u32;
pub const MM_VOXWARE_CODEC: u32 = 1u32;
pub const MM_VOYETRA: u32 = 30u32;
pub const MM_VQST: u32 = 240u32;
pub const MM_VQST_VQC1: u32 = 1u32;
pub const MM_VQST_VQC2: u32 = 2u32;
pub const MM_VTG: u32 = 109u32;
pub const MM_WANGLABS: u32 = 28u32;
pub const MM_WANGLABS_WAVEIN1: u32 = 1u32;
pub const MM_WANGLABS_WAVEOUT1: u32 = 2u32;
pub const MM_WEITEK: u32 = 96u32;
pub const MM_WILDCAT: u32 = 119u32;
pub const MM_WILDCAT_AUTOSCOREMIDIIN: u32 = 1u32;
pub const MM_WILLOPOND_SNDCOMM_WAVEIN: u32 = 108u32;
pub const MM_WILLOWPOND: u32 = 65u32;
pub const MM_WILLOWPOND_FMSYNTH_STEREO: u32 = 20u32;
pub const MM_WILLOWPOND_GENERIC_AUX: u32 = 115u32;
pub const MM_WILLOWPOND_GENERIC_MIXER: u32 = 114u32;
pub const MM_WILLOWPOND_GENERIC_WAVEIN: u32 = 112u32;
pub const MM_WILLOWPOND_GENERIC_WAVEOUT: u32 = 113u32;
pub const MM_WILLOWPOND_MPU401: u32 = 21u32;
pub const MM_WILLOWPOND_PH_AUX: u32 = 107u32;
pub const MM_WILLOWPOND_PH_MIXER: u32 = 106u32;
pub const MM_WILLOWPOND_PH_WAVEIN: u32 = 104u32;
pub const MM_WILLOWPOND_PH_WAVEOUT: u32 = 105u32;
pub const MM_WILLOWPOND_SNDCOMM_AUX: u32 = 111u32;
pub const MM_WILLOWPOND_SNDCOMM_MIXER: u32 = 110u32;
pub const MM_WILLOWPOND_SNDCOMM_WAVEOUT: u32 = 109u32;
pub const MM_WILLOWPOND_SNDPORT_AUX: u32 = 103u32;
pub const MM_WILLOWPOND_SNDPORT_MIXER: u32 = 102u32;
pub const MM_WILLOWPOND_SNDPORT_WAVEIN: u32 = 100u32;
pub const MM_WILLOWPOND_SNDPORT_WAVEOUT: u32 = 101u32;
pub const MM_WINBOND: u32 = 204u32;
pub const MM_WINNOV: u32 = 61u32;
pub const MM_WINNOV_CAVIAR_CHAMPAGNE: u32 = 4u32;
pub const MM_WINNOV_CAVIAR_VIDC: u32 = 3u32;
pub const MM_WINNOV_CAVIAR_WAVEIN: u32 = 1u32;
pub const MM_WINNOV_CAVIAR_WAVEOUT: u32 = 2u32;
pub const MM_WINNOV_CAVIAR_YUV8: u32 = 5u32;
pub const MM_WORKBIT: u32 = 102u32;
pub const MM_WORKBIT_AUX: u32 = 7u32;
pub const MM_WORKBIT_FMSYNTH: u32 = 6u32;
pub const MM_WORKBIT_JOYSTICK: u32 = 8u32;
pub const MM_WORKBIT_MIDIIN: u32 = 4u32;
pub const MM_WORKBIT_MIDIOUT: u32 = 5u32;
pub const MM_WORKBIT_MIXER: u32 = 1u32;
pub const MM_WORKBIT_WAVEIN: u32 = 3u32;
pub const MM_WORKBIT_WAVEOUT: u32 = 2u32;
pub const MM_WSS_SB16_AUX_CD: u32 = 45u32;
pub const MM_WSS_SB16_AUX_LINE: u32 = 44u32;
pub const MM_WSS_SB16_MIDIIN: u32 = 41u32;
pub const MM_WSS_SB16_MIDIOUT: u32 = 42u32;
pub const MM_WSS_SB16_MIXER: u32 = 46u32;
pub const MM_WSS_SB16_SYNTH: u32 = 43u32;
pub const MM_WSS_SB16_WAVEIN: u32 = 39u32;
pub const MM_WSS_SB16_WAVEOUT: u32 = 40u32;
pub const MM_WSS_SBPRO_AUX_CD: u32 = 53u32;
pub const MM_WSS_SBPRO_AUX_LINE: u32 = 52u32;
pub const MM_WSS_SBPRO_MIDIIN: u32 = 49u32;
pub const MM_WSS_SBPRO_MIDIOUT: u32 = 50u32;
pub const MM_WSS_SBPRO_MIXER: u32 = 54u32;
pub const MM_WSS_SBPRO_SYNTH: u32 = 51u32;
pub const MM_WSS_SBPRO_WAVEIN: u32 = 47u32;
pub const MM_WSS_SBPRO_WAVEOUT: u32 = 48u32;
pub const MM_XEBEC: u32 = 85u32;
pub const MM_XIRLINK: u32 = 178u32;
pub const MM_XIRLINK_VISIONLINK: u32 = 1u32;
pub const MM_XYZ: u32 = 112u32;
pub const MM_YAMAHA: u32 = 37u32;
pub const MM_YAMAHA_ACXG_AUX: u32 = 41u32;
pub const MM_YAMAHA_ACXG_MIDIOUT: u32 = 39u32;
pub const MM_YAMAHA_ACXG_MIXER: u32 = 40u32;
pub const MM_YAMAHA_ACXG_WAVEIN: u32 = 37u32;
pub const MM_YAMAHA_ACXG_WAVEOUT: u32 = 38u32;
pub const MM_YAMAHA_GSS_AUX: u32 = 6u32;
pub const MM_YAMAHA_GSS_MIDIIN: u32 = 5u32;
pub const MM_YAMAHA_GSS_MIDIOUT: u32 = 4u32;
pub const MM_YAMAHA_GSS_SYNTH: u32 = 1u32;
pub const MM_YAMAHA_GSS_WAVEIN: u32 = 3u32;
pub const MM_YAMAHA_GSS_WAVEOUT: u32 = 2u32;
pub const MM_YAMAHA_OPL3SA_FMSYNTH: u32 = 18u32;
pub const MM_YAMAHA_OPL3SA_JOYSTICK: u32 = 24u32;
pub const MM_YAMAHA_OPL3SA_MIDIIN: u32 = 21u32;
pub const MM_YAMAHA_OPL3SA_MIDIOUT: u32 = 20u32;
pub const MM_YAMAHA_OPL3SA_MIXER: u32 = 23u32;
pub const MM_YAMAHA_OPL3SA_WAVEIN: u32 = 17u32;
pub const MM_YAMAHA_OPL3SA_WAVEOUT: u32 = 16u32;
pub const MM_YAMAHA_OPL3SA_YSYNTH: u32 = 19u32;
pub const MM_YAMAHA_SERIAL_MIDIIN: u32 = 8u32;
pub const MM_YAMAHA_SERIAL_MIDIOUT: u32 = 7u32;
pub const MM_YAMAHA_SXG_MIDIOUT: u32 = 34u32;
pub const MM_YAMAHA_SXG_MIXER: u32 = 36u32;
pub const MM_YAMAHA_SXG_WAVEOUT: u32 = 35u32;
pub const MM_YAMAHA_YMF724LEG_FMSYNTH: u32 = 32u32;
pub const MM_YAMAHA_YMF724LEG_MIDIIN: u32 = 26u32;
pub const MM_YAMAHA_YMF724LEG_MIDIOUT: u32 = 25u32;
pub const MM_YAMAHA_YMF724LEG_MIXER: u32 = 33u32;
pub const MM_YAMAHA_YMF724_AUX: u32 = 30u32;
pub const MM_YAMAHA_YMF724_MIDIOUT: u32 = 29u32;
pub const MM_YAMAHA_YMF724_MIXER: u32 = 31u32;
pub const MM_YAMAHA_YMF724_WAVEIN: u32 = 28u32;
pub const MM_YAMAHA_YMF724_WAVEOUT: u32 = 27u32;
pub const MM_YOUCOM: u32 = 256u32;
pub const MM_ZEFIRO: u32 = 170u32;
pub const MM_ZEFIRO_ZA2: u32 = 2u32;
pub const MM_ZYXEL: u32 = 9u32;
pub const MM_ZYXEL_ACM_ADPCM: u32 = 1u32;
pub const MODM_CACHEDRUMPATCHES: u32 = 13u32;
pub const MODM_CACHEPATCHES: u32 = 12u32;
pub const MODM_CLOSE: u32 = 4u32;
pub const MODM_DATA: u32 = 7u32;
pub const MODM_GETDEVCAPS: u32 = 2u32;
pub const MODM_GETNUMDEVS: u32 = 1u32;
pub const MODM_GETPOS: u32 = 17u32;
pub const MODM_GETVOLUME: u32 = 10u32;
pub const MODM_INIT: u32 = 100u32;
pub const MODM_INIT_EX: u32 = 104u32;
pub const MODM_LONGDATA: u32 = 8u32;
pub const MODM_MAPPER: u32 = 8192u32;
pub const MODM_OPEN: u32 = 3u32;
pub const MODM_PAUSE: u32 = 18u32;
pub const MODM_PREFERRED: u32 = 22u32;
pub const MODM_PREPARE: u32 = 5u32;
pub const MODM_PROPERTIES: u32 = 21u32;
pub const MODM_RECONFIGURE: u32 = 18280u32;
pub const MODM_RESET: u32 = 9u32;
pub const MODM_RESTART: u32 = 19u32;
pub const MODM_SETVOLUME: u32 = 11u32;
pub const MODM_STOP: u32 = 20u32;
pub const MODM_STRMDATA: u32 = 14u32;
pub const MODM_UNPREPARE: u32 = 6u32;
pub const MODM_USER: u32 = 16384u32;
pub const MPEGLAYER3_ID_CONSTANTFRAMESIZE: u32 = 2u32;
pub const MPEGLAYER3_ID_MPEG: u32 = 1u32;
pub const MPEGLAYER3_ID_UNKNOWN: u32 = 0u32;
pub const MPEGLAYER3_WFX_EXTRA_BYTES: u32 = 12u32;
pub const MSAUDIO1_BITS_PER_SAMPLE: u32 = 16u32;
pub const MSAUDIO1_MAX_CHANNELS: u32 = 2u32;
pub const MXDM_BASE: u32 = 1u32;
pub const MXDM_CLOSE: u32 = 4u32;
pub const MXDM_GETCONTROLDETAILS: u32 = 7u32;
pub const MXDM_GETDEVCAPS: u32 = 2u32;
pub const MXDM_GETLINECONTROLS: u32 = 6u32;
pub const MXDM_GETLINEINFO: u32 = 5u32;
pub const MXDM_GETNUMDEVS: u32 = 1u32;
pub const MXDM_INIT: u32 = 100u32;
pub const MXDM_INIT_EX: u32 = 104u32;
pub const MXDM_OPEN: u32 = 3u32;
pub const MXDM_SETCONTROLDETAILS: u32 = 8u32;
pub const MXDM_USER: u32 = 16384u32;
pub const NS_DRM_E_MIGRATION_IMAGE_ALREADY_EXISTS: windows_sys::core::HRESULT = 0xC00D278E_u32 as _;
pub const NS_DRM_E_MIGRATION_SOURCE_MACHINE_IN_USE: windows_sys::core::HRESULT = 0xC00D278C_u32 as _;
pub const NS_DRM_E_MIGRATION_TARGET_MACHINE_LESS_THAN_LH: windows_sys::core::HRESULT = 0xC00D278D_u32 as _;
pub const NS_DRM_E_MIGRATION_UPGRADE_WITH_DIFF_SID: windows_sys::core::HRESULT = 0xC00D278B_u32 as _;
pub const NS_E_8BIT_WAVE_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D0BCE_u32 as _;
pub const NS_E_ACTIVE_SG_DEVICE_CONTROL_DISCONNECTED: windows_sys::core::HRESULT = 0xC00D1BA6_u32 as _;
pub const NS_E_ACTIVE_SG_DEVICE_DISCONNECTED: windows_sys::core::HRESULT = 0xC00D1BA5_u32 as _;
pub const NS_E_ADVANCEDEDIT_TOO_MANY_PICTURES: windows_sys::core::HRESULT = 0xC00D136A_u32 as _;
pub const NS_E_ALLOCATE_FILE_FAIL: windows_sys::core::HRESULT = 0xC00D0061_u32 as _;
pub const NS_E_ALL_PROTOCOLS_DISABLED: windows_sys::core::HRESULT = 0xC00D2EEB_u32 as _;
pub const NS_E_ALREADY_CONNECTED: windows_sys::core::HRESULT = 0xC00D0010_u32 as _;
pub const NS_E_ANALOG_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D2907_u32 as _;
pub const NS_E_ARCHIVE_ABORT_DUE_TO_BCAST: windows_sys::core::HRESULT = 0xC00D158E_u32 as _;
pub const NS_E_ARCHIVE_FILENAME_NOTSET: windows_sys::core::HRESULT = 0xC00D1B79_u32 as _;
pub const NS_E_ARCHIVE_GAP_DETECTED: windows_sys::core::HRESULT = 0xC00D158F_u32 as _;
pub const NS_E_ARCHIVE_REACH_QUOTA: windows_sys::core::HRESULT = 0xC00D158D_u32 as _;
pub const NS_E_ARCHIVE_SAME_AS_INPUT: windows_sys::core::HRESULT = 0xC00D1B84_u32 as _;
pub const NS_E_ASSERT: windows_sys::core::HRESULT = 0xC00D00CB_u32 as _;
pub const NS_E_ASX_INVALIDFORMAT: windows_sys::core::HRESULT = 0xC00D1069_u32 as _;
pub const NS_E_ASX_INVALIDVERSION: windows_sys::core::HRESULT = 0xC00D106A_u32 as _;
pub const NS_E_ASX_INVALID_REPEAT_BLOCK: windows_sys::core::HRESULT = 0xC00D106B_u32 as _;
pub const NS_E_ASX_NOTHING_TO_WRITE: windows_sys::core::HRESULT = 0xC00D106C_u32 as _;
pub const NS_E_ATTRIBUTE_NOT_ALLOWED: windows_sys::core::HRESULT = 0xC00D0BD7_u32 as _;
pub const NS_E_ATTRIBUTE_READ_ONLY: windows_sys::core::HRESULT = 0xC00D0BD6_u32 as _;
pub const NS_E_AUDIENCE_CONTENTTYPE_MISMATCH: windows_sys::core::HRESULT = 0xC00D1B99_u32 as _;
pub const NS_E_AUDIENCE__LANGUAGE_CONTENTTYPE_MISMATCH: windows_sys::core::HRESULT = 0xC00D1BE3_u32 as _;
pub const NS_E_AUDIODEVICE_BADFORMAT: windows_sys::core::HRESULT = 0xC00D1B63_u32 as _;
pub const NS_E_AUDIODEVICE_BUSY: windows_sys::core::HRESULT = 0xC00D1B61_u32 as _;
pub const NS_E_AUDIODEVICE_UNEXPECTED: windows_sys::core::HRESULT = 0xC00D1B62_u32 as _;
pub const NS_E_AUDIO_BITRATE_STEPDOWN: windows_sys::core::HRESULT = 0xC00D1BB9_u32 as _;
pub const NS_E_AUDIO_CODEC_ERROR: windows_sys::core::HRESULT = 0xC00D0BC3_u32 as _;
pub const NS_E_AUDIO_CODEC_NOT_INSTALLED: windows_sys::core::HRESULT = 0xC00D0BC2_u32 as _;
pub const NS_E_AUTHORIZATION_FILE_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D1590_u32 as _;
pub const NS_E_BACKUP_RESTORE_BAD_DATA: windows_sys::core::HRESULT = 0xC00D2745_u32 as _;
pub const NS_E_BACKUP_RESTORE_BAD_REQUEST_ID: windows_sys::core::HRESULT = 0xC00D272E_u32 as _;
pub const NS_E_BACKUP_RESTORE_FAILURE: windows_sys::core::HRESULT = 0xC00D272D_u32 as _;
pub const NS_E_BACKUP_RESTORE_TOO_MANY_RESETS: windows_sys::core::HRESULT = 0xC00D2766_u32 as _;
pub const NS_E_BAD_ADAPTER_ADDRESS: windows_sys::core::HRESULT = 0xC00D0039_u32 as _;
pub const NS_E_BAD_ADAPTER_NAME: windows_sys::core::HRESULT = 0xC00D00CC_u32 as _;
pub const NS_E_BAD_BLOCK0_VERSION: windows_sys::core::HRESULT = 0xC00D0063_u32 as _;
pub const NS_E_BAD_CONTENTEDL: windows_sys::core::HRESULT = 0xC00D1BAA_u32 as _;
pub const NS_E_BAD_CONTROL_DATA: windows_sys::core::HRESULT = 0xC00D0032_u32 as _;
pub const NS_E_BAD_CUB_UID: windows_sys::core::HRESULT = 0xC00D0192_u32 as _;
pub const NS_E_BAD_DELIVERY_MODE: windows_sys::core::HRESULT = 0xC00D003A_u32 as _;
pub const NS_E_BAD_DISK_UID: windows_sys::core::HRESULT = 0xC00D0064_u32 as _;
pub const NS_E_BAD_FSMAJOR_VERSION: windows_sys::core::HRESULT = 0xC00D0065_u32 as _;
pub const NS_E_BAD_MARKIN: windows_sys::core::HRESULT = 0xC00D1B58_u32 as _;
pub const NS_E_BAD_MARKOUT: windows_sys::core::HRESULT = 0xC00D1B59_u32 as _;
pub const NS_E_BAD_MULTICAST_ADDRESS: windows_sys::core::HRESULT = 0xC00D0038_u32 as _;
pub const NS_E_BAD_REQUEST: windows_sys::core::HRESULT = 0xC00D2EE3_u32 as _;
pub const NS_E_BAD_STAMPNUMBER: windows_sys::core::HRESULT = 0xC00D0066_u32 as _;
pub const NS_E_BAD_SYNTAX_IN_SERVER_RESPONSE: windows_sys::core::HRESULT = 0xC00D2EFE_u32 as _;
pub const NS_E_BKGDOWNLOAD_CALLFUNCENDED: windows_sys::core::HRESULT = 0xC00D1267_u32 as _;
pub const NS_E_BKGDOWNLOAD_CALLFUNCFAILED: windows_sys::core::HRESULT = 0xC00D1265_u32 as _;
pub const NS_E_BKGDOWNLOAD_CALLFUNCTIMEOUT: windows_sys::core::HRESULT = 0xC00D1266_u32 as _;
pub const NS_E_BKGDOWNLOAD_CANCELCOMPLETEDJOB: windows_sys::core::HRESULT = 0xC00D125F_u32 as _;
pub const NS_E_BKGDOWNLOAD_COMPLETECANCELLEDJOB: windows_sys::core::HRESULT = 0xC00D125E_u32 as _;
pub const NS_E_BKGDOWNLOAD_FAILEDINITIALIZE: windows_sys::core::HRESULT = 0xC00D1269_u32 as _;
pub const NS_E_BKGDOWNLOAD_FAILED_TO_CREATE_TEMPFILE: windows_sys::core::HRESULT = 0xC00D1262_u32 as _;
pub const NS_E_BKGDOWNLOAD_INVALIDJOBSIGNATURE: windows_sys::core::HRESULT = 0xC00D1261_u32 as _;
pub const NS_E_BKGDOWNLOAD_INVALID_FILE_NAME: windows_sys::core::HRESULT = 0xC00D126B_u32 as _;
pub const NS_E_BKGDOWNLOAD_NOJOBPOINTER: windows_sys::core::HRESULT = 0xC00D1260_u32 as _;
pub const NS_E_BKGDOWNLOAD_PLUGIN_FAILEDINITIALIZE: windows_sys::core::HRESULT = 0xC00D1263_u32 as _;
pub const NS_E_BKGDOWNLOAD_PLUGIN_FAILEDTOMOVEFILE: windows_sys::core::HRESULT = 0xC00D1264_u32 as _;
pub const NS_E_BKGDOWNLOAD_WMDUNPACKFAILED: windows_sys::core::HRESULT = 0xC00D1268_u32 as _;
pub const NS_E_BKGDOWNLOAD_WRONG_NO_FILES: windows_sys::core::HRESULT = 0xC00D125D_u32 as _;
pub const NS_E_BUSY: windows_sys::core::HRESULT = 0xC00D0025_u32 as _;
pub const NS_E_CACHE_ARCHIVE_CONFLICT: windows_sys::core::HRESULT = 0xC00D13EC_u32 as _;
pub const NS_E_CACHE_CANNOT_BE_CACHED: windows_sys::core::HRESULT = 0xC00D13F0_u32 as _;
pub const NS_E_CACHE_NOT_BROADCAST: windows_sys::core::HRESULT = 0xC00D13EF_u32 as _;
pub const NS_E_CACHE_NOT_MODIFIED: windows_sys::core::HRESULT = 0xC00D13F1_u32 as _;
pub const NS_E_CACHE_ORIGIN_SERVER_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D13ED_u32 as _;
pub const NS_E_CACHE_ORIGIN_SERVER_TIMEOUT: windows_sys::core::HRESULT = 0xC00D13EE_u32 as _;
pub const NS_E_CANNOTCONNECT: windows_sys::core::HRESULT = 0xC00D0006_u32 as _;
pub const NS_E_CANNOTCONNECTEVENTS: windows_sys::core::HRESULT = 0xC00D006F_u32 as _;
pub const NS_E_CANNOTDESTROYTITLE: windows_sys::core::HRESULT = 0xC00D0007_u32 as _;
pub const NS_E_CANNOTOFFLINEDISK: windows_sys::core::HRESULT = 0xC00D0009_u32 as _;
pub const NS_E_CANNOTONLINEDISK: windows_sys::core::HRESULT = 0xC00D000A_u32 as _;
pub const NS_E_CANNOTRENAMETITLE: windows_sys::core::HRESULT = 0xC00D0008_u32 as _;
pub const NS_E_CANNOT_BUY_OR_DOWNLOAD_CONTENT: windows_sys::core::HRESULT = 0xC00D1358_u32 as _;
pub const NS_E_CANNOT_BUY_OR_DOWNLOAD_FROM_MULTIPLE_SERVICES: windows_sys::core::HRESULT = 0xC00D1357_u32 as _;
pub const NS_E_CANNOT_CONNECT_TO_PROXY: windows_sys::core::HRESULT = 0xC00D2EEE_u32 as _;
pub const NS_E_CANNOT_DELETE_ACTIVE_SOURCEGROUP: windows_sys::core::HRESULT = 0xC00D1B60_u32 as _;
pub const NS_E_CANNOT_GENERATE_BROADCAST_INFO_FOR_QUALITYVBR: windows_sys::core::HRESULT = 0xC00D1BDF_u32 as _;
pub const NS_E_CANNOT_PAUSE_LIVEBROADCAST: windows_sys::core::HRESULT = 0xC00D1B8E_u32 as _;
pub const NS_E_CANNOT_READ_PLAYLIST_FROM_MEDIASERVER: windows_sys::core::HRESULT = 0xC00D2EF2_u32 as _;
pub const NS_E_CANNOT_REMOVE_PLUGIN: windows_sys::core::HRESULT = 0xC00D1451_u32 as _;
pub const NS_E_CANNOT_REMOVE_PUBLISHING_POINT: windows_sys::core::HRESULT = 0xC00D1450_u32 as _;
pub const NS_E_CANNOT_SYNC_DRM_TO_NON_JANUS_DEVICE: windows_sys::core::HRESULT = 0xC00D1246_u32 as _;
pub const NS_E_CANNOT_SYNC_PREVIOUS_SYNC_RUNNING: windows_sys::core::HRESULT = 0xC00D1247_u32 as _;
pub const NS_E_CANT_READ_DIGITAL: windows_sys::core::HRESULT = 0xC00D0FA1_u32 as _;
pub const NS_E_CCLINK_DOWN: windows_sys::core::HRESULT = 0xC00D0023_u32 as _;
pub const NS_E_CD_COPYTO_CD: windows_sys::core::HRESULT = 0xC00D0FAE_u32 as _;
pub const NS_E_CD_DRIVER_PROBLEM: windows_sys::core::HRESULT = 0xC00D0FB2_u32 as _;
pub const NS_E_CD_EMPTY_TRACK_QUEUE: windows_sys::core::HRESULT = 0xC00D11F9_u32 as _;
pub const NS_E_CD_ISRC_INVALID: windows_sys::core::HRESULT = 0xC00D11FB_u32 as _;
pub const NS_E_CD_MEDIA_CATALOG_NUMBER_INVALID: windows_sys::core::HRESULT = 0xC00D11FC_u32 as _;
pub const NS_E_CD_NO_BUFFERS_READ: windows_sys::core::HRESULT = 0xC00D11F8_u32 as _;
pub const NS_E_CD_NO_READER: windows_sys::core::HRESULT = 0xC00D11FA_u32 as _;
pub const NS_E_CD_QUEUEING_DISABLED: windows_sys::core::HRESULT = 0xC00D11FF_u32 as _;
pub const NS_E_CD_READ_ERROR: windows_sys::core::HRESULT = 0xC00D0FAC_u32 as _;
pub const NS_E_CD_READ_ERROR_NO_CORRECTION: windows_sys::core::HRESULT = 0xC00D0FAB_u32 as _;
pub const NS_E_CD_REFRESH: windows_sys::core::HRESULT = 0xC00D0FB1_u32 as _;
pub const NS_E_CD_SLOW_COPY: windows_sys::core::HRESULT = 0xC00D0FAD_u32 as _;
pub const NS_E_CD_SPEEDDETECT_NOT_ENOUGH_READS: windows_sys::core::HRESULT = 0xC00D11FE_u32 as _;
pub const NS_E_CHANGING_PROXYBYPASS: windows_sys::core::HRESULT = 0xC00D10C3_u32 as _;
pub const NS_E_CHANGING_PROXY_EXCEPTIONLIST: windows_sys::core::HRESULT = 0xC00D10C2_u32 as _;
pub const NS_E_CHANGING_PROXY_NAME: windows_sys::core::HRESULT = 0xC00D10C0_u32 as _;
pub const NS_E_CHANGING_PROXY_PORT: windows_sys::core::HRESULT = 0xC00D10C1_u32 as _;
pub const NS_E_CHANGING_PROXY_PROTOCOL_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D10C4_u32 as _;
pub const NS_E_CLOSED_ON_SUSPEND: windows_sys::core::HRESULT = 0xC00D2EF1_u32 as _;
pub const NS_E_CODEC_DMO_ERROR: windows_sys::core::HRESULT = 0xC00D0BDA_u32 as _;
pub const NS_E_CODEC_UNAVAILABLE: windows_sys::core::HRESULT = 0xC00D1B83_u32 as _;
pub const NS_E_COMPRESSED_DIGITAL_AUDIO_PROTECTION_LEVEL_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D2908_u32 as _;
pub const NS_E_COMPRESSED_DIGITAL_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D2905_u32 as _;
pub const NS_E_CONNECTION_FAILURE: windows_sys::core::HRESULT = 0xC00D0029_u32 as _;
pub const NS_E_CONNECT_TIMEOUT: windows_sys::core::HRESULT = 0xC00D2F06_u32 as _;
pub const NS_E_CONTENT_PARTNER_STILL_INITIALIZING: windows_sys::core::HRESULT = 0xC00D1362_u32 as _;
pub const NS_E_CORECD_NOTAMEDIACD: windows_sys::core::HRESULT = 0xC00D10C7_u32 as _;
pub const NS_E_CRITICAL_ERROR: windows_sys::core::HRESULT = 0xC00D151C_u32 as _;
pub const NS_E_CUB_FAIL: windows_sys::core::HRESULT = 0xC00D0053_u32 as _;
pub const NS_E_CUB_FAIL_LINK: windows_sys::core::HRESULT = 0xC00D0190_u32 as _;
pub const NS_E_CURLHELPER_NOTADIRECTORY: windows_sys::core::HRESULT = 0xC00D132D_u32 as _;
pub const NS_E_CURLHELPER_NOTAFILE: windows_sys::core::HRESULT = 0xC00D132E_u32 as _;
pub const NS_E_CURLHELPER_NOTRELATIVE: windows_sys::core::HRESULT = 0xC00D1330_u32 as _;
pub const NS_E_CURL_CANTDECODE: windows_sys::core::HRESULT = 0xC00D132F_u32 as _;
pub const NS_E_CURL_CANTWALK: windows_sys::core::HRESULT = 0xC00D132B_u32 as _;
pub const NS_E_CURL_INVALIDBUFFERSIZE: windows_sys::core::HRESULT = 0xC00D1331_u32 as _;
pub const NS_E_CURL_INVALIDCHAR: windows_sys::core::HRESULT = 0xC00D1325_u32 as _;
pub const NS_E_CURL_INVALIDHOSTNAME: windows_sys::core::HRESULT = 0xC00D1326_u32 as _;
pub const NS_E_CURL_INVALIDPATH: windows_sys::core::HRESULT = 0xC00D1327_u32 as _;
pub const NS_E_CURL_INVALIDPORT: windows_sys::core::HRESULT = 0xC00D132C_u32 as _;
pub const NS_E_CURL_INVALIDSCHEME: windows_sys::core::HRESULT = 0xC00D1328_u32 as _;
pub const NS_E_CURL_INVALIDURL: windows_sys::core::HRESULT = 0xC00D1329_u32 as _;
pub const NS_E_CURL_NOTSAFE: windows_sys::core::HRESULT = 0xC00D1324_u32 as _;
pub const NS_E_DAMAGED_FILE: windows_sys::core::HRESULT = 0xC00D0FCB_u32 as _;
pub const NS_E_DATAPATH_NO_SINK: windows_sys::core::HRESULT = 0xC00D1518_u32 as _;
pub const NS_E_DATA_SOURCE_ENUMERATION_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC00D1580_u32 as _;
pub const NS_E_DATA_UNIT_EXTENSION_TOO_LARGE: windows_sys::core::HRESULT = 0xC00D0BD9_u32 as _;
pub const NS_E_DDRAW_GENERIC: windows_sys::core::HRESULT = 0xC00D10BD_u32 as _;
pub const NS_E_DEVCONTROL_FAILED_SEEK: windows_sys::core::HRESULT = 0xC00D1B94_u32 as _;
pub const NS_E_DEVICECONTROL_UNSTABLE: windows_sys::core::HRESULT = 0xC00D1BE1_u32 as _;
pub const NS_E_DEVICE_DISCONNECTED: windows_sys::core::HRESULT = 0xC00D0FA2_u32 as _;
pub const NS_E_DEVICE_IS_NOT_READY: windows_sys::core::HRESULT = 0xC00D1177_u32 as _;
pub const NS_E_DEVICE_NOT_READY: windows_sys::core::HRESULT = 0xC00D0FCA_u32 as _;
pub const NS_E_DEVICE_NOT_SUPPORT_FORMAT: windows_sys::core::HRESULT = 0xC00D0FA3_u32 as _;
pub const NS_E_DEVICE_NOT_WMDRM_DEVICE: windows_sys::core::HRESULT = 0xC00D277B_u32 as _;
pub const NS_E_DISK_FAIL: windows_sys::core::HRESULT = 0xC00D0055_u32 as _;
pub const NS_E_DISK_READ: windows_sys::core::HRESULT = 0xC00D0017_u32 as _;
pub const NS_E_DISK_WRITE: windows_sys::core::HRESULT = 0xC00D0016_u32 as _;
pub const NS_E_DISPLAY_MODE_CHANGE_FAILED: windows_sys::core::HRESULT = 0xC00D10BE_u32 as _;
pub const NS_E_DRMPROFILE_NOTFOUND: windows_sys::core::HRESULT = 0xC00D1BD5_u32 as _;
pub const NS_E_DRM_ACQUIRING_LICENSE: windows_sys::core::HRESULT = 0xC00D272B_u32 as _;
pub const NS_E_DRM_ACTION_NOT_QUERIED: windows_sys::core::HRESULT = 0xC00D272A_u32 as _;
pub const NS_E_DRM_ALREADY_INDIVIDUALIZED: windows_sys::core::HRESULT = 0xC00D2729_u32 as _;
pub const NS_E_DRM_APPCERT_REVOKED: windows_sys::core::HRESULT = 0xC00D2752_u32 as _;
pub const NS_E_DRM_ATTRIBUTE_TOO_LONG: windows_sys::core::HRESULT = 0xC00D28B2_u32 as _;
pub const NS_E_DRM_BACKUPRESTORE_BUSY: windows_sys::core::HRESULT = 0xC00D2744_u32 as _;
pub const NS_E_DRM_BACKUP_CORRUPT: windows_sys::core::HRESULT = 0xC00D2743_u32 as _;
pub const NS_E_DRM_BACKUP_EXISTS: windows_sys::core::HRESULT = 0xC00D2742_u32 as _;
pub const NS_E_DRM_BAD_REQUEST: windows_sys::core::HRESULT = 0xC00D28B0_u32 as _;
pub const NS_E_DRM_BB_UNABLE_TO_INITIALIZE: windows_sys::core::HRESULT = 0xC00D2780_u32 as _;
pub const NS_E_DRM_BUFFER_TOO_SMALL: windows_sys::core::HRESULT = 0xC00D275C_u32 as _;
pub const NS_E_DRM_BUSY: windows_sys::core::HRESULT = 0xC00D2841_u32 as _;
pub const NS_E_DRM_CACHED_CONTENT_ERROR: windows_sys::core::HRESULT = 0xC00D274B_u32 as _;
pub const NS_E_DRM_CERTIFICATE_REVOKED: windows_sys::core::HRESULT = 0xC00D28A1_u32 as _;
pub const NS_E_DRM_CERTIFICATE_SECURITY_LEVEL_INADEQUATE: windows_sys::core::HRESULT = 0xC00D28AE_u32 as _;
pub const NS_E_DRM_CHAIN_TOO_LONG: windows_sys::core::HRESULT = 0xC00D284C_u32 as _;
pub const NS_E_DRM_CHECKPOINT_CORRUPT: windows_sys::core::HRESULT = 0xC00D2797_u32 as _;
pub const NS_E_DRM_CHECKPOINT_FAILED: windows_sys::core::HRESULT = 0xC00D277F_u32 as _;
pub const NS_E_DRM_CHECKPOINT_MISMATCH: windows_sys::core::HRESULT = 0xC00D2796_u32 as _;
pub const NS_E_DRM_CLIENT_CODE_EXPIRED: windows_sys::core::HRESULT = 0xC00D2847_u32 as _;
pub const NS_E_DRM_DATASTORE_CORRUPT: windows_sys::core::HRESULT = 0xC00D2783_u32 as _;
pub const NS_E_DRM_DEBUGGING_NOT_ALLOWED: windows_sys::core::HRESULT = 0xC00D2767_u32 as _;
pub const NS_E_DRM_DECRYPT_ERROR: windows_sys::core::HRESULT = 0xC00D2723_u32 as _;
pub const NS_E_DRM_DEVICE_ACTIVATION_CANCELED: windows_sys::core::HRESULT = 0xC00D2765_u32 as _;
pub const NS_E_DRM_DEVICE_ALREADY_REGISTERED: windows_sys::core::HRESULT = 0xC00D28AB_u32 as _;
pub const NS_E_DRM_DEVICE_LIMIT_REACHED: windows_sys::core::HRESULT = 0xC00D28A3_u32 as _;
pub const NS_E_DRM_DEVICE_NOT_OPEN: windows_sys::core::HRESULT = 0xC00D28AA_u32 as _;
pub const NS_E_DRM_DEVICE_NOT_REGISTERED: windows_sys::core::HRESULT = 0xC00D27E2_u32 as _;
pub const NS_E_DRM_DRIVER_AUTH_FAILURE: windows_sys::core::HRESULT = 0xC00D274D_u32 as _;
pub const NS_E_DRM_DRIVER_DIGIOUT_FAILURE: windows_sys::core::HRESULT = 0xC00D2750_u32 as _;
pub const NS_E_DRM_DRMV2CLT_REVOKED: windows_sys::core::HRESULT = 0xC00D28B6_u32 as _;
pub const NS_E_DRM_ENCRYPT_ERROR: windows_sys::core::HRESULT = 0xC00D2722_u32 as _;
pub const NS_E_DRM_ENUM_LICENSE_FAILED: windows_sys::core::HRESULT = 0xC00D271B_u32 as _;
pub const NS_E_DRM_ERROR_BAD_NET_RESP: windows_sys::core::HRESULT = 0xC00D275E_u32 as _;
pub const NS_E_DRM_EXPIRED_LICENSEBLOB: windows_sys::core::HRESULT = 0xC00D28B3_u32 as _;
pub const NS_E_DRM_GET_CONTENTSTRING_ERROR: windows_sys::core::HRESULT = 0xC00D273D_u32 as _;
pub const NS_E_DRM_GET_LICENSESTRING_ERROR: windows_sys::core::HRESULT = 0xC00D273C_u32 as _;
pub const NS_E_DRM_GET_LICENSE_ERROR: windows_sys::core::HRESULT = 0xC00D2739_u32 as _;
pub const NS_E_DRM_HARDWAREID_MISMATCH: windows_sys::core::HRESULT = 0xC00D278F_u32 as _;
pub const NS_E_DRM_HARDWARE_INCONSISTENT: windows_sys::core::HRESULT = 0xC00D2754_u32 as _;
pub const NS_E_DRM_INCLUSION_LIST_REQUIRED: windows_sys::core::HRESULT = 0xC00D28B5_u32 as _;
pub const NS_E_DRM_INDIVIDUALIZATION_INCOMPLETE: windows_sys::core::HRESULT = 0xC00D274C_u32 as _;
pub const NS_E_DRM_INDIVIDUALIZE_ERROR: windows_sys::core::HRESULT = 0xC00D2736_u32 as _;
pub const NS_E_DRM_INDIVIDUALIZING: windows_sys::core::HRESULT = 0xC00D272C_u32 as _;
pub const NS_E_DRM_INDIV_FRAUD: windows_sys::core::HRESULT = 0xC00D2843_u32 as _;
pub const NS_E_DRM_INDIV_NO_CABS: windows_sys::core::HRESULT = 0xC00D2844_u32 as _;
pub const NS_E_DRM_INDIV_SERVICE_UNAVAILABLE: windows_sys::core::HRESULT = 0xC00D2845_u32 as _;
pub const NS_E_DRM_INVALID_APPCERT: windows_sys::core::HRESULT = 0xC00D277C_u32 as _;
pub const NS_E_DRM_INVALID_APPDATA: windows_sys::core::HRESULT = 0xC00D2740_u32 as _;
pub const NS_E_DRM_INVALID_APPDATA_VERSION: windows_sys::core::HRESULT = 0xC00D2741_u32 as _;
pub const NS_E_DRM_INVALID_APPLICATION: windows_sys::core::HRESULT = 0xC00D2711_u32 as _;
pub const NS_E_DRM_INVALID_CERTIFICATE: windows_sys::core::HRESULT = 0xC00D28A0_u32 as _;
pub const NS_E_DRM_INVALID_CONTENT: windows_sys::core::HRESULT = 0xC00D2716_u32 as _;
pub const NS_E_DRM_INVALID_CRL: windows_sys::core::HRESULT = 0xC00D28B1_u32 as _;
pub const NS_E_DRM_INVALID_DATA: windows_sys::core::HRESULT = 0xC00D2761_u32 as _;
pub const NS_E_DRM_INVALID_KID: windows_sys::core::HRESULT = 0xC00D2849_u32 as _;
pub const NS_E_DRM_INVALID_LICENSE: windows_sys::core::HRESULT = 0xC00D2718_u32 as _;
pub const NS_E_DRM_INVALID_LICENSEBLOB: windows_sys::core::HRESULT = 0xC00D28B4_u32 as _;
pub const NS_E_DRM_INVALID_LICENSE_ACQUIRED: windows_sys::core::HRESULT = 0xC00D271F_u32 as _;
pub const NS_E_DRM_INVALID_LICENSE_REQUEST: windows_sys::core::HRESULT = 0xC00D271C_u32 as _;
pub const NS_E_DRM_INVALID_MACHINE: windows_sys::core::HRESULT = 0xC00D2719_u32 as _;
pub const NS_E_DRM_INVALID_MIGRATION_IMAGE: windows_sys::core::HRESULT = 0xC00D2788_u32 as _;
pub const NS_E_DRM_INVALID_PROPERTY: windows_sys::core::HRESULT = 0xC00D2749_u32 as _;
pub const NS_E_DRM_INVALID_PROXIMITY_RESPONSE: windows_sys::core::HRESULT = 0xC00D28A8_u32 as _;
pub const NS_E_DRM_INVALID_SECURESTORE_PASSWORD: windows_sys::core::HRESULT = 0xC00D2751_u32 as _;
pub const NS_E_DRM_INVALID_SESSION: windows_sys::core::HRESULT = 0xC00D28A9_u32 as _;
pub const NS_E_DRM_KEY_ERROR: windows_sys::core::HRESULT = 0xC00D2721_u32 as _;
pub const NS_E_DRM_LICENSE_APPSECLOW: windows_sys::core::HRESULT = 0xC00D27DA_u32 as _;
pub const NS_E_DRM_LICENSE_APP_NOTALLOWED: windows_sys::core::HRESULT = 0xC00D27DD_u32 as _;
pub const NS_E_DRM_LICENSE_CERT_EXPIRED: windows_sys::core::HRESULT = 0xC00D27DF_u32 as _;
pub const NS_E_DRM_LICENSE_CLOSE_ERROR: windows_sys::core::HRESULT = 0xC00D2738_u32 as _;
pub const NS_E_DRM_LICENSE_CONTENT_REVOKED: windows_sys::core::HRESULT = 0xC00D27E1_u32 as _;
pub const NS_E_DRM_LICENSE_DELETION_ERROR: windows_sys::core::HRESULT = 0xC00D284E_u32 as _;
pub const NS_E_DRM_LICENSE_EXPIRED: windows_sys::core::HRESULT = 0xC00D27D8_u32 as _;
pub const NS_E_DRM_LICENSE_INITIALIZATION_ERROR: windows_sys::core::HRESULT = 0xC00D284A_u32 as _;
pub const NS_E_DRM_LICENSE_INVALID_XML: windows_sys::core::HRESULT = 0xC00D2725_u32 as _;
pub const NS_E_DRM_LICENSE_NOSAP: windows_sys::core::HRESULT = 0xC00D280A_u32 as _;
pub const NS_E_DRM_LICENSE_NOSVP: windows_sys::core::HRESULT = 0xC00D280B_u32 as _;
pub const NS_E_DRM_LICENSE_NOTACQUIRED: windows_sys::core::HRESULT = 0xC00D2759_u32 as _;
pub const NS_E_DRM_LICENSE_NOTENABLED: windows_sys::core::HRESULT = 0xC00D27D9_u32 as _;
pub const NS_E_DRM_LICENSE_NOTRUSTEDCODEC: windows_sys::core::HRESULT = 0xC00D280D_u32 as _;
pub const NS_E_DRM_LICENSE_NOWDM: windows_sys::core::HRESULT = 0xC00D280C_u32 as _;
pub const NS_E_DRM_LICENSE_OPEN_ERROR: windows_sys::core::HRESULT = 0xC00D2737_u32 as _;
pub const NS_E_DRM_LICENSE_SECLOW: windows_sys::core::HRESULT = 0xC00D27E0_u32 as _;
pub const NS_E_DRM_LICENSE_SERVER_INFO_MISSING: windows_sys::core::HRESULT = 0xC00D2840_u32 as _;
pub const NS_E_DRM_LICENSE_STORE_ERROR: windows_sys::core::HRESULT = 0xC00D2712_u32 as _;
pub const NS_E_DRM_LICENSE_STORE_SAVE_ERROR: windows_sys::core::HRESULT = 0xC00D2714_u32 as _;
pub const NS_E_DRM_LICENSE_UNAVAILABLE: windows_sys::core::HRESULT = 0xC00D28A2_u32 as _;
pub const NS_E_DRM_LICENSE_UNUSABLE: windows_sys::core::HRESULT = 0xC00D2748_u32 as _;
pub const NS_E_DRM_LIC_NEEDS_DEVICE_CLOCK_SET: windows_sys::core::HRESULT = 0xC00D2779_u32 as _;
pub const NS_E_DRM_MALFORMED_CONTENT_HEADER: windows_sys::core::HRESULT = 0xC00D279C_u32 as _;
pub const NS_E_DRM_MIGRATION_IMPORTER_NOT_AVAILABLE: windows_sys::core::HRESULT = 0xC00D278A_u32 as _;
pub const NS_E_DRM_MIGRATION_INVALID_LEGACYV2_DATA: windows_sys::core::HRESULT = 0xC00D2791_u32 as _;
pub const NS_E_DRM_MIGRATION_INVALID_LEGACYV2_SST_PASSWORD: windows_sys::core::HRESULT = 0xC00D2793_u32 as _;
pub const NS_E_DRM_MIGRATION_LICENSE_ALREADY_EXISTS: windows_sys::core::HRESULT = 0xC00D2792_u32 as _;
pub const NS_E_DRM_MIGRATION_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC00D2794_u32 as _;
pub const NS_E_DRM_MIGRATION_OBJECT_IN_USE: windows_sys::core::HRESULT = 0xC00D279B_u32 as _;
pub const NS_E_DRM_MIGRATION_OPERATION_CANCELLED: windows_sys::core::HRESULT = 0xC00D279A_u32 as _;
pub const NS_E_DRM_MIGRATION_TARGET_NOT_ONLINE: windows_sys::core::HRESULT = 0xC00D2787_u32 as _;
pub const NS_E_DRM_MIGRATION_TARGET_STATES_CORRUPTED: windows_sys::core::HRESULT = 0xC00D2789_u32 as _;
pub const NS_E_DRM_MONITOR_ERROR: windows_sys::core::HRESULT = 0xC00D273E_u32 as _;
pub const NS_E_DRM_MUST_APPROVE: windows_sys::core::HRESULT = 0xC00D28A6_u32 as _;
pub const NS_E_DRM_MUST_REGISTER: windows_sys::core::HRESULT = 0xC00D28A5_u32 as _;
pub const NS_E_DRM_MUST_REVALIDATE: windows_sys::core::HRESULT = 0xC00D28A7_u32 as _;
pub const NS_E_DRM_NEEDS_INDIVIDUALIZATION: windows_sys::core::HRESULT = 0xC00D2728_u32 as _;
pub const NS_E_DRM_NEEDS_UPGRADE_TEMPFILE: windows_sys::core::HRESULT = 0xC00D283D_u32 as _;
pub const NS_E_DRM_NEED_UPGRADE_MSSAP: windows_sys::core::HRESULT = 0xC00D274E_u32 as _;
pub const NS_E_DRM_NEED_UPGRADE_PD: windows_sys::core::HRESULT = 0xC00D283E_u32 as _;
pub const NS_E_DRM_NOT_CONFIGURED: windows_sys::core::HRESULT = 0xC00D2764_u32 as _;
pub const NS_E_DRM_NO_RIGHTS: windows_sys::core::HRESULT = 0xC00D2720_u32 as _;
pub const NS_E_DRM_NO_UPLINK_LICENSE: windows_sys::core::HRESULT = 0xC00D2848_u32 as _;
pub const NS_E_DRM_OPERATION_CANCELED: windows_sys::core::HRESULT = 0xC00D2768_u32 as _;
pub const NS_E_DRM_PARAMETERS_MISMATCHED: windows_sys::core::HRESULT = 0xC00D272F_u32 as _;
pub const NS_E_DRM_PASSWORD_TOO_LONG: windows_sys::core::HRESULT = 0xC00D1B93_u32 as _;
pub const NS_E_DRM_PD_TOO_MANY_DEVICES: windows_sys::core::HRESULT = 0xC00D2842_u32 as _;
pub const NS_E_DRM_POLICY_DISABLE_ONLINE: windows_sys::core::HRESULT = 0xC00D2762_u32 as _;
pub const NS_E_DRM_POLICY_METERING_DISABLED: windows_sys::core::HRESULT = 0xC00D2776_u32 as _;
pub const NS_E_DRM_PROFILE_NOT_SET: windows_sys::core::HRESULT = 0xC00D1B8F_u32 as _;
pub const NS_E_DRM_PROTOCOL_FORCEFUL_TERMINATION_ON_CHALLENGE: windows_sys::core::HRESULT = 0xC00D277E_u32 as _;
pub const NS_E_DRM_PROTOCOL_FORCEFUL_TERMINATION_ON_PETITION: windows_sys::core::HRESULT = 0xC00D277D_u32 as _;
pub const NS_E_DRM_QUERY_ERROR: windows_sys::core::HRESULT = 0xC00D273A_u32 as _;
pub const NS_E_DRM_REOPEN_CONTENT: windows_sys::core::HRESULT = 0xC00D274F_u32 as _;
pub const NS_E_DRM_REPORT_ERROR: windows_sys::core::HRESULT = 0xC00D273B_u32 as _;
pub const NS_E_DRM_RESTORE_FRAUD: windows_sys::core::HRESULT = 0xC00D2753_u32 as _;
pub const NS_E_DRM_RESTORE_SERVICE_UNAVAILABLE: windows_sys::core::HRESULT = 0xC00D2846_u32 as _;
pub const NS_E_DRM_RESTRICTIONS_NOT_RETRIEVED: windows_sys::core::HRESULT = 0xC00D2769_u32 as _;
pub const NS_E_DRM_RIV_TOO_SMALL: windows_sys::core::HRESULT = 0xC00D28B7_u32 as _;
pub const NS_E_DRM_SDK_VERSIONMISMATCH: windows_sys::core::HRESULT = 0xC00D2778_u32 as _;
pub const NS_E_DRM_SDMI_NOMORECOPIES: windows_sys::core::HRESULT = 0xC00D2756_u32 as _;
pub const NS_E_DRM_SDMI_TRIGGER: windows_sys::core::HRESULT = 0xC00D2755_u32 as _;
pub const NS_E_DRM_SECURE_STORE_ERROR: windows_sys::core::HRESULT = 0xC00D2713_u32 as _;
pub const NS_E_DRM_SECURE_STORE_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D274A_u32 as _;
pub const NS_E_DRM_SECURE_STORE_UNLOCK_ERROR: windows_sys::core::HRESULT = 0xC00D2715_u32 as _;
pub const NS_E_DRM_SECURITY_COMPONENT_SIGNATURE_INVALID: windows_sys::core::HRESULT = 0xC00D2760_u32 as _;
pub const NS_E_DRM_SIGNATURE_FAILURE: windows_sys::core::HRESULT = 0xC00D283F_u32 as _;
pub const NS_E_DRM_SOURCEID_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC00D280E_u32 as _;
pub const NS_E_DRM_STORE_NEEDINDI: windows_sys::core::HRESULT = 0xC00D27DB_u32 as _;
pub const NS_E_DRM_STORE_NOTALLOWED: windows_sys::core::HRESULT = 0xC00D27DC_u32 as _;
pub const NS_E_DRM_STORE_NOTALLSTORED: windows_sys::core::HRESULT = 0xC00D275F_u32 as _;
pub const NS_E_DRM_STUBLIB_REQUIRED: windows_sys::core::HRESULT = 0xC00D2785_u32 as _;
pub const NS_E_DRM_TRACK_EXCEEDED_PLAYLIST_RESTICTION: windows_sys::core::HRESULT = 0xC00D2770_u32 as _;
pub const NS_E_DRM_TRACK_EXCEEDED_TRACKBURN_RESTRICTION: windows_sys::core::HRESULT = 0xC00D2771_u32 as _;
pub const NS_E_DRM_TRANSFER_CHAINED_LICENSES_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D2777_u32 as _;
pub const NS_E_DRM_UNABLE_TO_ACQUIRE_LICENSE: windows_sys::core::HRESULT = 0xC00D271E_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_AUTHENTICATION_OBJECT: windows_sys::core::HRESULT = 0xC00D2763_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_BACKUP_OBJECT: windows_sys::core::HRESULT = 0xC00D2735_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_CERTIFICATE_OBJECT: windows_sys::core::HRESULT = 0xC00D2786_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_CODING_OBJECT: windows_sys::core::HRESULT = 0xC00D275A_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_DECRYPT_OBJECT: windows_sys::core::HRESULT = 0xC00D2733_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_DEVICE_REGISTRATION_OBJECT: windows_sys::core::HRESULT = 0xC00D276C_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_ENCRYPT_OBJECT: windows_sys::core::HRESULT = 0xC00D2732_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_HEADER_OBJECT: windows_sys::core::HRESULT = 0xC00D2757_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_INDI_OBJECT: windows_sys::core::HRESULT = 0xC00D2731_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_INMEMORYSTORE_OBJECT: windows_sys::core::HRESULT = 0xC00D2784_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_KEYS_OBJECT: windows_sys::core::HRESULT = 0xC00D2758_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_LICENSE_OBJECT: windows_sys::core::HRESULT = 0xC00D2730_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_METERING_OBJECT: windows_sys::core::HRESULT = 0xC00D276D_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_MIGRATION_IMPORTER_OBJECT: windows_sys::core::HRESULT = 0xC00D2795_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_PLAYLIST_BURN_OBJECT: windows_sys::core::HRESULT = 0xC00D276B_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_PLAYLIST_OBJECT: windows_sys::core::HRESULT = 0xC00D276A_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_PROPERTIES_OBJECT: windows_sys::core::HRESULT = 0xC00D2734_u32 as _;
pub const NS_E_DRM_UNABLE_TO_CREATE_STATE_DATA_OBJECT: windows_sys::core::HRESULT = 0xC00D275B_u32 as _;
pub const NS_E_DRM_UNABLE_TO_GET_DEVICE_CERT: windows_sys::core::HRESULT = 0xC00D2772_u32 as _;
pub const NS_E_DRM_UNABLE_TO_GET_SECURE_CLOCK: windows_sys::core::HRESULT = 0xC00D2773_u32 as _;
pub const NS_E_DRM_UNABLE_TO_GET_SECURE_CLOCK_FROM_SERVER: windows_sys::core::HRESULT = 0xC00D2775_u32 as _;
pub const NS_E_DRM_UNABLE_TO_INITIALIZE: windows_sys::core::HRESULT = 0xC00D271D_u32 as _;
pub const NS_E_DRM_UNABLE_TO_LOAD_HARDWARE_ID: windows_sys::core::HRESULT = 0xC00D2781_u32 as _;
pub const NS_E_DRM_UNABLE_TO_OPEN_DATA_STORE: windows_sys::core::HRESULT = 0xC00D2782_u32 as _;
pub const NS_E_DRM_UNABLE_TO_OPEN_LICENSE: windows_sys::core::HRESULT = 0xC00D2717_u32 as _;
pub const NS_E_DRM_UNABLE_TO_OPEN_PORT: windows_sys::core::HRESULT = 0xC00D28AF_u32 as _;
pub const NS_E_DRM_UNABLE_TO_SET_PARAMETER: windows_sys::core::HRESULT = 0xC00D273F_u32 as _;
pub const NS_E_DRM_UNABLE_TO_SET_SECURE_CLOCK: windows_sys::core::HRESULT = 0xC00D2774_u32 as _;
pub const NS_E_DRM_UNABLE_TO_VERIFY_PROXIMITY: windows_sys::core::HRESULT = 0xC00D28A4_u32 as _;
pub const NS_E_DRM_UNSUPPORTED_ACTION: windows_sys::core::HRESULT = 0xC00D28AD_u32 as _;
pub const NS_E_DRM_UNSUPPORTED_ALGORITHM: windows_sys::core::HRESULT = 0xC00D284D_u32 as _;
pub const NS_E_DRM_UNSUPPORTED_PROPERTY: windows_sys::core::HRESULT = 0xC00D275D_u32 as _;
pub const NS_E_DRM_UNSUPPORTED_PROTOCOL_VERSION: windows_sys::core::HRESULT = 0xC00D28AC_u32 as _;
pub const NS_E_DUPLICATE_ADDRESS: windows_sys::core::HRESULT = 0xC00D0037_u32 as _;
pub const NS_E_DUPLICATE_DRMPROFILE: windows_sys::core::HRESULT = 0xC00D1B90_u32 as _;
pub const NS_E_DUPLICATE_NAME: windows_sys::core::HRESULT = 0xC00D0036_u32 as _;
pub const NS_E_DUPLICATE_PACKET: windows_sys::core::HRESULT = 0xC00D0BD3_u32 as _;
pub const NS_E_DVD_AUTHORING_PROBLEM: windows_sys::core::HRESULT = 0xC00D1164_u32 as _;
pub const NS_E_DVD_CANNOT_COPY_PROTECTED: windows_sys::core::HRESULT = 0xC00D1172_u32 as _;
pub const NS_E_DVD_CANNOT_JUMP: windows_sys::core::HRESULT = 0xC00D116F_u32 as _;
pub const NS_E_DVD_COMPATIBLE_VIDEO_CARD: windows_sys::core::HRESULT = 0xC00D1166_u32 as _;
pub const NS_E_DVD_COPY_PROTECT: windows_sys::core::HRESULT = 0xC00D1163_u32 as _;
pub const NS_E_DVD_DEVICE_CONTENTION: windows_sys::core::HRESULT = 0xC00D1170_u32 as _;
pub const NS_E_DVD_DISC_COPY_PROTECT_OUTPUT_FAILED: windows_sys::core::HRESULT = 0xC00D1161_u32 as _;
pub const NS_E_DVD_DISC_COPY_PROTECT_OUTPUT_NS: windows_sys::core::HRESULT = 0xC00D1160_u32 as _;
pub const NS_E_DVD_DISC_DECODER_REGION: windows_sys::core::HRESULT = 0xC00D1169_u32 as _;
pub const NS_E_DVD_GRAPH_BUILDING: windows_sys::core::HRESULT = 0xC00D116C_u32 as _;
pub const NS_E_DVD_INVALID_DISC_REGION: windows_sys::core::HRESULT = 0xC00D1165_u32 as _;
pub const NS_E_DVD_INVALID_TITLE_CHAPTER: windows_sys::core::HRESULT = 0xC00D1174_u32 as _;
pub const NS_E_DVD_MACROVISION: windows_sys::core::HRESULT = 0xC00D1167_u32 as _;
pub const NS_E_DVD_NO_AUDIO_STREAM: windows_sys::core::HRESULT = 0xC00D116B_u32 as _;
pub const NS_E_DVD_NO_DECODER: windows_sys::core::HRESULT = 0xC00D116D_u32 as _;
pub const NS_E_DVD_NO_SUBPICTURE_STREAM: windows_sys::core::HRESULT = 0xC00D1162_u32 as _;
pub const NS_E_DVD_NO_VIDEO_MEMORY: windows_sys::core::HRESULT = 0xC00D1171_u32 as _;
pub const NS_E_DVD_NO_VIDEO_STREAM: windows_sys::core::HRESULT = 0xC00D116A_u32 as _;
pub const NS_E_DVD_PARENTAL: windows_sys::core::HRESULT = 0xC00D116E_u32 as _;
pub const NS_E_DVD_REQUIRED_PROPERTY_NOT_SET: windows_sys::core::HRESULT = 0xC00D1173_u32 as _;
pub const NS_E_DVD_SYSTEM_DECODER_REGION: windows_sys::core::HRESULT = 0xC00D1168_u32 as _;
pub const NS_E_EDL_REQUIRED_FOR_DEVICE_MULTIPASS: windows_sys::core::HRESULT = 0xC00D1BE7_u32 as _;
pub const NS_E_EMPTY_PLAYLIST: windows_sys::core::HRESULT = 0xC00D14B5_u32 as _;
pub const NS_E_EMPTY_PROGRAM_NAME: windows_sys::core::HRESULT = 0xC00D00D6_u32 as _;
pub const NS_E_ENACTPLAN_GIVEUP: windows_sys::core::HRESULT = 0xC00D0068_u32 as _;
pub const NS_E_END_OF_PLAYLIST: windows_sys::core::HRESULT = 0xC00D32C8_u32 as _;
pub const NS_E_END_OF_TAPE: windows_sys::core::HRESULT = 0xC00D1BAE_u32 as _;
pub const NS_E_ERROR_FROM_PROXY: windows_sys::core::HRESULT = 0xC00D2EE4_u32 as _;
pub const NS_E_EXCEED_MAX_DRM_PROFILE_LIMIT: windows_sys::core::HRESULT = 0xC00D1BE0_u32 as _;
pub const NS_E_EXPECT_MONO_WAV_INPUT: windows_sys::core::HRESULT = 0xC00D1BA1_u32 as _;
pub const NS_E_FAILED_DOWNLOAD_ABORT_BURN: windows_sys::core::HRESULT = 0xC00D10DC_u32 as _;
pub const NS_E_FAIL_LAUNCH_ROXIO_PLUGIN: windows_sys::core::HRESULT = 0xC00D1180_u32 as _;
pub const NS_E_FEATURE_DISABLED_BY_GROUP_POLICY: windows_sys::core::HRESULT = 0xC00D0BDC_u32 as _;
pub const NS_E_FEATURE_DISABLED_IN_SKU: windows_sys::core::HRESULT = 0xC00D0BDD_u32 as _;
pub const NS_E_FEATURE_REQUIRES_ENTERPRISE_SERVER: windows_sys::core::HRESULT = 0xC00D1583_u32 as _;
pub const NS_E_FILE_ALLOCATION_FAILED: windows_sys::core::HRESULT = 0xC00D001E_u32 as _;
pub const NS_E_FILE_BANDWIDTH_LIMIT: windows_sys::core::HRESULT = 0xC00D0030_u32 as _;
pub const NS_E_FILE_EXISTS: windows_sys::core::HRESULT = 0xC00D001B_u32 as _;
pub const NS_E_FILE_FAILED_CHECKS: windows_sys::core::HRESULT = 0xC00D0FCD_u32 as _;
pub const NS_E_FILE_INIT_FAILED: windows_sys::core::HRESULT = 0xC00D001F_u32 as _;
pub const NS_E_FILE_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D001A_u32 as _;
pub const NS_E_FILE_OPEN_FAILED: windows_sys::core::HRESULT = 0xC00D001D_u32 as _;
pub const NS_E_FILE_PLAY_FAILED: windows_sys::core::HRESULT = 0xC00D0020_u32 as _;
pub const NS_E_FILE_READ: windows_sys::core::HRESULT = 0xC00D0019_u32 as _;
pub const NS_E_FILE_WRITE: windows_sys::core::HRESULT = 0xC00D0018_u32 as _;
pub const NS_E_FIREWALL: windows_sys::core::HRESULT = 0xC00D2EF9_u32 as _;
pub const NS_E_FLASH_PLAYBACK_NOT_ALLOWED: windows_sys::core::HRESULT = 0xC00D10CF_u32 as _;
pub const NS_E_GLITCH_MODE: windows_sys::core::HRESULT = 0xC00D0195_u32 as _;
pub const NS_E_GRAPH_NOAUDIOLANGUAGE: windows_sys::core::HRESULT = 0xC00D10C5_u32 as _;
pub const NS_E_GRAPH_NOAUDIOLANGUAGESELECTED: windows_sys::core::HRESULT = 0xC00D10C6_u32 as _;
pub const NS_E_HDS_KEY_MISMATCH: windows_sys::core::HRESULT = 0xC00D2799_u32 as _;
pub const NS_E_HEADER_MISMATCH: windows_sys::core::HRESULT = 0xC00D151F_u32 as _;
pub const NS_E_HTTP_DISABLED: windows_sys::core::HRESULT = 0xC00D00D3_u32 as _;
pub const NS_E_HTTP_TEXT_DATACONTAINER_INVALID_SERVER_RESPONSE: windows_sys::core::HRESULT = 0xC00D158C_u32 as _;
pub const NS_E_HTTP_TEXT_DATACONTAINER_SIZE_LIMIT_EXCEEDED: windows_sys::core::HRESULT = 0xC00D1589_u32 as _;
pub const NS_E_ICMQUERYFORMAT: windows_sys::core::HRESULT = 0xC00D1B6C_u32 as _;
pub const NS_E_IE_DISALLOWS_ACTIVEX_CONTROLS: windows_sys::core::HRESULT = 0xC00D10CE_u32 as _;
pub const NS_E_IMAGE_DOWNLOAD_FAILED: windows_sys::core::HRESULT = 0xC00D128E_u32 as _;
pub const NS_E_IMAPI_LOSSOFSTREAMING: windows_sys::core::HRESULT = 0xC00D117E_u32 as _;
pub const NS_E_IMAPI_MEDIUM_INVALIDTYPE: windows_sys::core::HRESULT = 0xC00D1182_u32 as _;
pub const NS_E_INCOMPATIBLE_FORMAT: windows_sys::core::HRESULT = 0xC00D0041_u32 as _;
pub const NS_E_INCOMPATIBLE_PUSH_SERVER: windows_sys::core::HRESULT = 0xC00D2F0C_u32 as _;
pub const NS_E_INCOMPATIBLE_SERVER: windows_sys::core::HRESULT = 0xC00D2EE8_u32 as _;
pub const NS_E_INCOMPATIBLE_VERSION: windows_sys::core::HRESULT = 0xC00D0BC7_u32 as _;
pub const NS_E_INCOMPLETE_PLAYLIST: windows_sys::core::HRESULT = 0xC00D1242_u32 as _;
pub const NS_E_INCORRECTCLIPSETTINGS: windows_sys::core::HRESULT = 0xC00D1B7C_u32 as _;
pub const NS_E_INDUCED: windows_sys::core::HRESULT = 0xC00D0022_u32 as _;
pub const NS_E_INPUTSOURCE_PROBLEM: windows_sys::core::HRESULT = 0xC00D1B8A_u32 as _;
pub const NS_E_INPUT_DOESNOT_SUPPORT_SMPTE: windows_sys::core::HRESULT = 0xC00D1BA8_u32 as _;
pub const NS_E_INPUT_WAVFORMAT_MISMATCH: windows_sys::core::HRESULT = 0xC00D1BA2_u32 as _;
pub const NS_E_INSUFFICIENT_BANDWIDTH: windows_sys::core::HRESULT = 0xC00D002C_u32 as _;
pub const NS_E_INSUFFICIENT_DATA: windows_sys::core::HRESULT = 0xC00D00CA_u32 as _;
pub const NS_E_INTERFACE_NOT_REGISTERED_IN_GIT: windows_sys::core::HRESULT = 0xC00D126A_u32 as _;
pub const NS_E_INTERLACEMODE_MISMATCH: windows_sys::core::HRESULT = 0xC00D1BAB_u32 as _;
pub const NS_E_INTERLACE_REQUIRE_SAMESIZE: windows_sys::core::HRESULT = 0xC00D1B95_u32 as _;
pub const NS_E_INTERNAL: windows_sys::core::HRESULT = 0xC00D0024_u32 as _;
pub const NS_E_INTERNAL_SERVER_ERROR: windows_sys::core::HRESULT = 0xC00D2EE2_u32 as _;
pub const NS_E_INVALIDCALL_WHILE_ARCHIVAL_RUNNING: windows_sys::core::HRESULT = 0xC00D1B74_u32 as _;
pub const NS_E_INVALIDCALL_WHILE_ENCODER_RUNNING: windows_sys::core::HRESULT = 0xC00D1B66_u32 as _;
pub const NS_E_INVALIDCALL_WHILE_ENCODER_STOPPED: windows_sys::core::HRESULT = 0xC00D1B7F_u32 as _;
pub const NS_E_INVALIDINPUTFPS: windows_sys::core::HRESULT = 0xC00D1B81_u32 as _;
pub const NS_E_INVALIDPACKETSIZE: windows_sys::core::HRESULT = 0xC00D1B75_u32 as _;
pub const NS_E_INVALIDPROFILE: windows_sys::core::HRESULT = 0xC00D0BC6_u32 as _;
pub const NS_E_INVALID_ARCHIVE: windows_sys::core::HRESULT = 0xC00D003D_u32 as _;
pub const NS_E_INVALID_AUDIO_BUFFERMAX: windows_sys::core::HRESULT = 0xC00D1BBC_u32 as _;
pub const NS_E_INVALID_AUDIO_PEAKRATE: windows_sys::core::HRESULT = 0xC00D1BBA_u32 as _;
pub const NS_E_INVALID_AUDIO_PEAKRATE_2: windows_sys::core::HRESULT = 0xC00D1BBB_u32 as _;
pub const NS_E_INVALID_BLACKHOLE_ADDRESS: windows_sys::core::HRESULT = 0xC00D0040_u32 as _;
pub const NS_E_INVALID_CHANNEL: windows_sys::core::HRESULT = 0xC00D003B_u32 as _;
pub const NS_E_INVALID_CLIENT: windows_sys::core::HRESULT = 0xC00D003F_u32 as _;
pub const NS_E_INVALID_DATA: windows_sys::core::HRESULT = 0xC00D002F_u32 as _;
pub const NS_E_INVALID_DEVICE: windows_sys::core::HRESULT = 0xC00D1B91_u32 as _;
pub const NS_E_INVALID_DRMV2CLT_STUBLIB: windows_sys::core::HRESULT = 0xC00D2790_u32 as _;
pub const NS_E_INVALID_EDL: windows_sys::core::HRESULT = 0xC00D0BD8_u32 as _;
pub const NS_E_INVALID_FILE_BITRATE: windows_sys::core::HRESULT = 0xC00D1BD1_u32 as _;
pub const NS_E_INVALID_FOLDDOWN_COEFFICIENTS: windows_sys::core::HRESULT = 0xC00D1BD4_u32 as _;
pub const NS_E_INVALID_INDEX: windows_sys::core::HRESULT = 0xC00D0011_u32 as _;
pub const NS_E_INVALID_INDEX2: windows_sys::core::HRESULT = 0xC00D00D9_u32 as _;
pub const NS_E_INVALID_INPUT_AUDIENCE_INDEX: windows_sys::core::HRESULT = 0xC00D1B9E_u32 as _;
pub const NS_E_INVALID_INPUT_FORMAT: windows_sys::core::HRESULT = 0xC00D0BB8_u32 as _;
pub const NS_E_INVALID_INPUT_LANGUAGE: windows_sys::core::HRESULT = 0xC00D1B9F_u32 as _;
pub const NS_E_INVALID_INPUT_STREAM: windows_sys::core::HRESULT = 0xC00D1BA0_u32 as _;
pub const NS_E_INVALID_INTERLACEMODE: windows_sys::core::HRESULT = 0xC00D1BDB_u32 as _;
pub const NS_E_INVALID_INTERLACE_COMPAT: windows_sys::core::HRESULT = 0xC00D1BDC_u32 as _;
pub const NS_E_INVALID_KEY: windows_sys::core::HRESULT = 0xC00D0042_u32 as _;
pub const NS_E_INVALID_LOG_URL: windows_sys::core::HRESULT = 0xC00D1585_u32 as _;
pub const NS_E_INVALID_MTU_RANGE: windows_sys::core::HRESULT = 0xC00D1586_u32 as _;
pub const NS_E_INVALID_NAME: windows_sys::core::HRESULT = 0xC00D001C_u32 as _;
pub const NS_E_INVALID_NONSQUAREPIXEL_COMPAT: windows_sys::core::HRESULT = 0xC00D1BDD_u32 as _;
pub const NS_E_INVALID_NUM_PASSES: windows_sys::core::HRESULT = 0xC00D0BD5_u32 as _;
pub const NS_E_INVALID_OPERATING_SYSTEM_VERSION: windows_sys::core::HRESULT = 0xC00D1459_u32 as _;
pub const NS_E_INVALID_OUTPUT_FORMAT: windows_sys::core::HRESULT = 0xC00D0BBB_u32 as _;
pub const NS_E_INVALID_PIXEL_ASPECT_RATIO: windows_sys::core::HRESULT = 0xC00D1BE2_u32 as _;
pub const NS_E_INVALID_PLAY_STATISTICS: windows_sys::core::HRESULT = 0xC00D1587_u32 as _;
pub const NS_E_INVALID_PLUGIN_LOAD_TYPE_CONFIGURATION: windows_sys::core::HRESULT = 0xC00D1454_u32 as _;
pub const NS_E_INVALID_PORT: windows_sys::core::HRESULT = 0xC00D0043_u32 as _;
pub const NS_E_INVALID_PROFILE_CONTENTTYPE: windows_sys::core::HRESULT = 0xC00D1BE4_u32 as _;
pub const NS_E_INVALID_PUBLISHING_POINT_NAME: windows_sys::core::HRESULT = 0xC00D1455_u32 as _;
pub const NS_E_INVALID_PUSH_PUBLISHING_POINT: windows_sys::core::HRESULT = 0xC00D151B_u32 as _;
pub const NS_E_INVALID_PUSH_PUBLISHING_POINT_START_REQUEST: windows_sys::core::HRESULT = 0xC00D145B_u32 as _;
pub const NS_E_INVALID_PUSH_TEMPLATE: windows_sys::core::HRESULT = 0xC00D151A_u32 as _;
pub const NS_E_INVALID_QUERY_OPERATOR: windows_sys::core::HRESULT = 0xC00D32CF_u32 as _;
pub const NS_E_INVALID_QUERY_PROPERTY: windows_sys::core::HRESULT = 0xC00D32D0_u32 as _;
pub const NS_E_INVALID_REDIRECT: windows_sys::core::HRESULT = 0xC00D2EEA_u32 as _;
pub const NS_E_INVALID_REQUEST: windows_sys::core::HRESULT = 0xC00D002B_u32 as _;
pub const NS_E_INVALID_SAMPLING_RATE: windows_sys::core::HRESULT = 0xC00D0BD0_u32 as _;
pub const NS_E_INVALID_SCRIPT_BITRATE: windows_sys::core::HRESULT = 0xC00D1BCF_u32 as _;
pub const NS_E_INVALID_SOURCE_WITH_DEVICE_CONTROL: windows_sys::core::HRESULT = 0xC00D1BDE_u32 as _;
pub const NS_E_INVALID_STREAM: windows_sys::core::HRESULT = 0xC00D003C_u32 as _;
pub const NS_E_INVALID_TIMECODE: windows_sys::core::HRESULT = 0xC00D1BD6_u32 as _;
pub const NS_E_INVALID_TTL: windows_sys::core::HRESULT = 0xC00D0044_u32 as _;
pub const NS_E_INVALID_VBR_COMPAT: windows_sys::core::HRESULT = 0xC00D1BB2_u32 as _;
pub const NS_E_INVALID_VBR_WITH_UNCOMP: windows_sys::core::HRESULT = 0xC00D1BB4_u32 as _;
pub const NS_E_INVALID_VIDEO_BITRATE: windows_sys::core::HRESULT = 0xC00D1BBF_u32 as _;
pub const NS_E_INVALID_VIDEO_BUFFER: windows_sys::core::HRESULT = 0xC00D1BC9_u32 as _;
pub const NS_E_INVALID_VIDEO_BUFFERMAX: windows_sys::core::HRESULT = 0xC00D1BCA_u32 as _;
pub const NS_E_INVALID_VIDEO_BUFFERMAX_2: windows_sys::core::HRESULT = 0xC00D1BCB_u32 as _;
pub const NS_E_INVALID_VIDEO_CQUALITY: windows_sys::core::HRESULT = 0xC00D1BC8_u32 as _;
pub const NS_E_INVALID_VIDEO_FPS: windows_sys::core::HRESULT = 0xC00D1BC5_u32 as _;
pub const NS_E_INVALID_VIDEO_HEIGHT: windows_sys::core::HRESULT = 0xC00D1BC4_u32 as _;
pub const NS_E_INVALID_VIDEO_HEIGHT_ALIGN: windows_sys::core::HRESULT = 0xC00D1BCD_u32 as _;
pub const NS_E_INVALID_VIDEO_IQUALITY: windows_sys::core::HRESULT = 0xC00D1BC7_u32 as _;
pub const NS_E_INVALID_VIDEO_KEYFRAME: windows_sys::core::HRESULT = 0xC00D1BC6_u32 as _;
pub const NS_E_INVALID_VIDEO_PEAKRATE: windows_sys::core::HRESULT = 0xC00D1BC1_u32 as _;
pub const NS_E_INVALID_VIDEO_PEAKRATE_2: windows_sys::core::HRESULT = 0xC00D1BC2_u32 as _;
pub const NS_E_INVALID_VIDEO_WIDTH: windows_sys::core::HRESULT = 0xC00D1BC3_u32 as _;
pub const NS_E_INVALID_VIDEO_WIDTH_ALIGN: windows_sys::core::HRESULT = 0xC00D1BCC_u32 as _;
pub const NS_E_INVALID_VIDEO_WIDTH_FOR_INTERLACED_ENCODING: windows_sys::core::HRESULT = 0xC00D1BE8_u32 as _;
pub const NS_E_LANGUAGE_MISMATCH: windows_sys::core::HRESULT = 0xC00D1B9C_u32 as _;
pub const NS_E_LATE_OPERATION: windows_sys::core::HRESULT = 0xC00D002E_u32 as _;
pub const NS_E_LATE_PACKET: windows_sys::core::HRESULT = 0xC00D0BD2_u32 as _;
pub const NS_E_LICENSE_EXPIRED: windows_sys::core::HRESULT = 0xC00D00D4_u32 as _;
pub const NS_E_LICENSE_HEADER_MISSING_URL: windows_sys::core::HRESULT = 0xC00D277A_u32 as _;
pub const NS_E_LICENSE_INCORRECT_RIGHTS: windows_sys::core::HRESULT = 0xC00D0BC1_u32 as _;
pub const NS_E_LICENSE_OUTOFDATE: windows_sys::core::HRESULT = 0xC00D0BC0_u32 as _;
pub const NS_E_LICENSE_REQUIRED: windows_sys::core::HRESULT = 0xC00D0BBE_u32 as _;
pub const NS_E_LOGFILEPERIOD: windows_sys::core::HRESULT = 0xC00D0048_u32 as _;
pub const NS_E_LOG_FILE_SIZE: windows_sys::core::HRESULT = 0xC00D004A_u32 as _;
pub const NS_E_LOG_NEED_TO_BE_SKIPPED: windows_sys::core::HRESULT = 0xC00D1588_u32 as _;
pub const NS_E_MARKIN_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D1BE9_u32 as _;
pub const NS_E_MAX_BITRATE: windows_sys::core::HRESULT = 0xC00D0047_u32 as _;
pub const NS_E_MAX_CLIENTS: windows_sys::core::HRESULT = 0xC00D0049_u32 as _;
pub const NS_E_MAX_FILERATE: windows_sys::core::HRESULT = 0xC00D004B_u32 as _;
pub const NS_E_MAX_FUNNELS_ALERT: windows_sys::core::HRESULT = 0xC00D0060_u32 as _;
pub const NS_E_MAX_PACKET_SIZE_TOO_SMALL: windows_sys::core::HRESULT = 0xC00D0BD1_u32 as _;
pub const NS_E_MEDIACD_READ_ERROR: windows_sys::core::HRESULT = 0xC00D10CD_u32 as _;
pub const NS_E_MEDIA_LIBRARY_FAILED: windows_sys::core::HRESULT = 0xC00D0FCE_u32 as _;
pub const NS_E_MEDIA_PARSER_INVALID_FORMAT: windows_sys::core::HRESULT = 0xC00D1581_u32 as _;
pub const NS_E_MEMSTORAGE_BAD_DATA: windows_sys::core::HRESULT = 0xC00D117B_u32 as _;
pub const NS_E_METADATA_CACHE_DATA_NOT_AVAILABLE: windows_sys::core::HRESULT = 0xC00D32DB_u32 as _;
pub const NS_E_METADATA_CANNOT_RETRIEVE_FROM_OFFLINE_CACHE: windows_sys::core::HRESULT = 0xC00D32DE_u32 as _;
pub const NS_E_METADATA_CANNOT_SET_LOCALE: windows_sys::core::HRESULT = 0xC00D32D7_u32 as _;
pub const NS_E_METADATA_FORMAT_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC00D32D5_u32 as _;
pub const NS_E_METADATA_IDENTIFIER_NOT_AVAILABLE: windows_sys::core::HRESULT = 0xC00D32DD_u32 as _;
pub const NS_E_METADATA_INVALID_DOCUMENT_TYPE: windows_sys::core::HRESULT = 0xC00D32DC_u32 as _;
pub const NS_E_METADATA_LANGUAGE_NOT_SUPORTED: windows_sys::core::HRESULT = 0xC00D32D8_u32 as _;
pub const NS_E_METADATA_NOT_AVAILABLE: windows_sys::core::HRESULT = 0xC00D32DA_u32 as _;
pub const NS_E_METADATA_NO_EDITING_CAPABILITY: windows_sys::core::HRESULT = 0xC00D32D6_u32 as _;
pub const NS_E_METADATA_NO_RFC1766_NAME_FOR_LOCALE: windows_sys::core::HRESULT = 0xC00D32D9_u32 as _;
pub const NS_E_MISMATCHED_MEDIACONTENT: windows_sys::core::HRESULT = 0xC00D1B5F_u32 as _;
pub const NS_E_MISSING_AUDIENCE: windows_sys::core::HRESULT = 0xC00D1B98_u32 as _;
pub const NS_E_MISSING_CHANNEL: windows_sys::core::HRESULT = 0xC00D00D7_u32 as _;
pub const NS_E_MISSING_SOURCE_INDEX: windows_sys::core::HRESULT = 0xC00D1B9A_u32 as _;
pub const NS_E_MIXER_INVALID_CONTROL: windows_sys::core::HRESULT = 0xC00D0FA6_u32 as _;
pub const NS_E_MIXER_INVALID_LINE: windows_sys::core::HRESULT = 0xC00D0FA5_u32 as _;
pub const NS_E_MIXER_INVALID_VALUE: windows_sys::core::HRESULT = 0xC00D0FA7_u32 as _;
pub const NS_E_MIXER_NODRIVER: windows_sys::core::HRESULT = 0xC00D0FAF_u32 as _;
pub const NS_E_MIXER_UNKNOWN_MMRESULT: windows_sys::core::HRESULT = 0xC00D0FA8_u32 as _;
pub const NS_E_MLS_SMARTPLAYLIST_FILTER_NOT_REGISTERED: windows_sys::core::HRESULT = 0xC00D1075_u32 as _;
pub const NS_E_MMSAUTOSERVER_CANTFINDWALKER: windows_sys::core::HRESULT = 0xC00D0046_u32 as _;
pub const NS_E_MMS_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC00D2EFA_u32 as _;
pub const NS_E_MONITOR_GIVEUP: windows_sys::core::HRESULT = 0xC00D00C8_u32 as _;
pub const NS_E_MP3_FORMAT_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D0FAA_u32 as _;
pub const NS_E_MPDB_GENERIC: windows_sys::core::HRESULT = 0xC00D0FCC_u32 as _;
pub const NS_E_MSAUDIO_NOT_INSTALLED: windows_sys::core::HRESULT = 0xC00D0BB9_u32 as _;
pub const NS_E_MSBD_NO_LONGER_SUPPORTED: windows_sys::core::HRESULT = 0xC00D2EEC_u32 as _;
pub const NS_E_MULTICAST_DISABLED: windows_sys::core::HRESULT = 0xC00D2EE9_u32 as _;
pub const NS_E_MULTICAST_PLUGIN_NOT_ENABLED: windows_sys::core::HRESULT = 0xC00D1458_u32 as _;
pub const NS_E_MULTIPLE_AUDIO_CODECS: windows_sys::core::HRESULT = 0xC00D1BB7_u32 as _;
pub const NS_E_MULTIPLE_AUDIO_FORMATS: windows_sys::core::HRESULT = 0xC00D1BB8_u32 as _;
pub const NS_E_MULTIPLE_FILE_BITRATES: windows_sys::core::HRESULT = 0xC00D1BD0_u32 as _;
pub const NS_E_MULTIPLE_SCRIPT_BITRATES: windows_sys::core::HRESULT = 0xC00D1BCE_u32 as _;
pub const NS_E_MULTIPLE_VBR_AUDIENCES: windows_sys::core::HRESULT = 0xC00D1BB5_u32 as _;
pub const NS_E_MULTIPLE_VIDEO_CODECS: windows_sys::core::HRESULT = 0xC00D1BBD_u32 as _;
pub const NS_E_MULTIPLE_VIDEO_SIZES: windows_sys::core::HRESULT = 0xC00D1BBE_u32 as _;
pub const NS_E_NAMESPACE_BAD_NAME: windows_sys::core::HRESULT = 0xC00D1396_u32 as _;
pub const NS_E_NAMESPACE_BUFFER_TOO_SMALL: windows_sys::core::HRESULT = 0xC00D138E_u32 as _;
pub const NS_E_NAMESPACE_CALLBACK_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D1391_u32 as _;
pub const NS_E_NAMESPACE_DUPLICATE_CALLBACK: windows_sys::core::HRESULT = 0xC00D1390_u32 as _;
pub const NS_E_NAMESPACE_DUPLICATE_NAME: windows_sys::core::HRESULT = 0xC00D1393_u32 as _;
pub const NS_E_NAMESPACE_EMPTY_NAME: windows_sys::core::HRESULT = 0xC00D1394_u32 as _;
pub const NS_E_NAMESPACE_INDEX_TOO_LARGE: windows_sys::core::HRESULT = 0xC00D1395_u32 as _;
pub const NS_E_NAMESPACE_NAME_TOO_LONG: windows_sys::core::HRESULT = 0xC00D1392_u32 as _;
pub const NS_E_NAMESPACE_NODE_CONFLICT: windows_sys::core::HRESULT = 0xC00D138C_u32 as _;
pub const NS_E_NAMESPACE_NODE_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D138D_u32 as _;
pub const NS_E_NAMESPACE_TOO_MANY_CALLBACKS: windows_sys::core::HRESULT = 0xC00D138F_u32 as _;
pub const NS_E_NAMESPACE_WRONG_PERSIST: windows_sys::core::HRESULT = 0xC00D138A_u32 as _;
pub const NS_E_NAMESPACE_WRONG_SECURITY: windows_sys::core::HRESULT = 0xC00D1397_u32 as _;
pub const NS_E_NAMESPACE_WRONG_TYPE: windows_sys::core::HRESULT = 0xC00D138B_u32 as _;
pub const NS_E_NEED_CORE_REFERENCE: windows_sys::core::HRESULT = 0xC00D10CC_u32 as _;
pub const NS_E_NEED_TO_ASK_USER: windows_sys::core::HRESULT = 0xC00D0FDA_u32 as _;
pub const NS_E_NETWORK_BUSY: windows_sys::core::HRESULT = 0xC00D000E_u32 as _;
pub const NS_E_NETWORK_RESOURCE_FAILURE: windows_sys::core::HRESULT = 0xC00D0028_u32 as _;
pub const NS_E_NETWORK_SERVICE_FAILURE: windows_sys::core::HRESULT = 0xC00D0027_u32 as _;
pub const NS_E_NETWORK_SINK_WRITE: windows_sys::core::HRESULT = 0xC00D2EF8_u32 as _;
pub const NS_E_NET_READ: windows_sys::core::HRESULT = 0xC00D0015_u32 as _;
pub const NS_E_NET_WRITE: windows_sys::core::HRESULT = 0xC00D0014_u32 as _;
pub const NS_E_NOCONNECTION: windows_sys::core::HRESULT = 0xC00D0005_u32 as _;
pub const NS_E_NOFUNNEL: windows_sys::core::HRESULT = 0xC00D000C_u32 as _;
pub const NS_E_NOMATCHING_ELEMENT: windows_sys::core::HRESULT = 0xC00D1B5E_u32 as _;
pub const NS_E_NOMATCHING_MEDIASOURCE: windows_sys::core::HRESULT = 0xC00D1B5A_u32 as _;
pub const NS_E_NONSQUAREPIXELMODE_MISMATCH: windows_sys::core::HRESULT = 0xC00D1BAC_u32 as _;
pub const NS_E_NOREGISTEREDWALKER: windows_sys::core::HRESULT = 0xC00D000B_u32 as _;
pub const NS_E_NOSOURCEGROUPS: windows_sys::core::HRESULT = 0xC00D1B80_u32 as _;
pub const NS_E_NOSTATSAVAILABLE: windows_sys::core::HRESULT = 0xC00D1B7D_u32 as _;
pub const NS_E_NOTARCHIVING: windows_sys::core::HRESULT = 0xC00D1B7E_u32 as _;
pub const NS_E_NOTHING_TO_DO: windows_sys::core::HRESULT = 0xC00D07F1_u32 as _;
pub const NS_E_NOTITLES: windows_sys::core::HRESULT = 0xC00D003E_u32 as _;
pub const NS_E_NOT_CONFIGURED: windows_sys::core::HRESULT = 0xC00D0BBC_u32 as _;
pub const NS_E_NOT_CONNECTED: windows_sys::core::HRESULT = 0xC00D0BCB_u32 as _;
pub const NS_E_NOT_CONTENT_PARTNER_TRACK: windows_sys::core::HRESULT = 0xC00D135A_u32 as _;
pub const NS_E_NOT_LICENSED: windows_sys::core::HRESULT = 0xC00D00CD_u32 as _;
pub const NS_E_NOT_REBUILDING: windows_sys::core::HRESULT = 0xC00D002D_u32 as _;
pub const NS_E_NO_ACTIVE_SOURCEGROUP: windows_sys::core::HRESULT = 0xC00D1B72_u32 as _;
pub const NS_E_NO_AUDIENCES: windows_sys::core::HRESULT = 0xC00D1BB0_u32 as _;
pub const NS_E_NO_AUDIODATA: windows_sys::core::HRESULT = 0xC00D1B89_u32 as _;
pub const NS_E_NO_AUDIO_COMPAT: windows_sys::core::HRESULT = 0xC00D1BB1_u32 as _;
pub const NS_E_NO_AUDIO_TIMECOMPRESSION: windows_sys::core::HRESULT = 0xC00D1BD7_u32 as _;
pub const NS_E_NO_CD: windows_sys::core::HRESULT = 0xC00D0FA0_u32 as _;
pub const NS_E_NO_CD_BURNER: windows_sys::core::HRESULT = 0xC00D1176_u32 as _;
pub const NS_E_NO_CHANNELS: windows_sys::core::HRESULT = 0xC00D00D8_u32 as _;
pub const NS_E_NO_DATAVIEW_SUPPORT: windows_sys::core::HRESULT = 0xC00D1B82_u32 as _;
pub const NS_E_NO_DEVICE: windows_sys::core::HRESULT = 0xC00D0071_u32 as _;
pub const NS_E_NO_ERROR_STRING_FOUND: windows_sys::core::HRESULT = 0xC00D0FD0_u32 as _;
pub const NS_E_NO_EXISTING_PACKETIZER: windows_sys::core::HRESULT = 0xC00D2EFD_u32 as _;
pub const NS_E_NO_FORMATS: windows_sys::core::HRESULT = 0xC00D006B_u32 as _;
pub const NS_E_NO_FRAMES_SUBMITTED_TO_ANALYZER: windows_sys::core::HRESULT = 0xC00D1BA7_u32 as _;
pub const NS_E_NO_LOCALPLAY: windows_sys::core::HRESULT = 0xC00D000D_u32 as _;
pub const NS_E_NO_MBR_WITH_TIMECODE: windows_sys::core::HRESULT = 0xC00D1BDA_u32 as _;
pub const NS_E_NO_MEDIAFORMAT_IN_SOURCE: windows_sys::core::HRESULT = 0xC00D1B6F_u32 as _;
pub const NS_E_NO_MEDIA_IN_AUDIENCE: windows_sys::core::HRESULT = 0xC00D1BAF_u32 as _;
pub const NS_E_NO_MEDIA_PROTOCOL: windows_sys::core::HRESULT = 0xC00D019B_u32 as _;
pub const NS_E_NO_MORE_SAMPLES: windows_sys::core::HRESULT = 0xC00D0BCF_u32 as _;
pub const NS_E_NO_MULTICAST: windows_sys::core::HRESULT = 0xC00D07F2_u32 as _;
pub const NS_E_NO_MULTIPASS_FOR_LIVEDEVICE: windows_sys::core::HRESULT = 0xC00D1B97_u32 as _;
pub const NS_E_NO_NEW_CONNECTIONS: windows_sys::core::HRESULT = 0xC00D151D_u32 as _;
pub const NS_E_NO_PAL_INVERSE_TELECINE: windows_sys::core::HRESULT = 0xC00D1BA4_u32 as _;
pub const NS_E_NO_PDA: windows_sys::core::HRESULT = 0xC00D1179_u32 as _;
pub const NS_E_NO_PROFILE_IN_SOURCEGROUP: windows_sys::core::HRESULT = 0xC00D1B67_u32 as _;
pub const NS_E_NO_PROFILE_NAME: windows_sys::core::HRESULT = 0xC00D1BB3_u32 as _;
pub const NS_E_NO_REALTIME_PREPROCESS: windows_sys::core::HRESULT = 0xC00D1B8C_u32 as _;
pub const NS_E_NO_REALTIME_TIMECOMPRESSION: windows_sys::core::HRESULT = 0xC00D1B86_u32 as _;
pub const NS_E_NO_REFERENCES: windows_sys::core::HRESULT = 0xC00D006C_u32 as _;
pub const NS_E_NO_REPEAT_PREPROCESS: windows_sys::core::HRESULT = 0xC00D1B8D_u32 as _;
pub const NS_E_NO_SCRIPT_ENGINE: windows_sys::core::HRESULT = 0xC00D157C_u32 as _;
pub const NS_E_NO_SCRIPT_STREAM: windows_sys::core::HRESULT = 0xC00D1B73_u32 as _;
pub const NS_E_NO_SERVER_CONTACT: windows_sys::core::HRESULT = 0xC00D00CE_u32 as _;
pub const NS_E_NO_SMPTE_WITH_MULTIPLE_SOURCEGROUPS: windows_sys::core::HRESULT = 0xC00D1BA9_u32 as _;
pub const NS_E_NO_SPECIFIED_DEVICE: windows_sys::core::HRESULT = 0xC00D0072_u32 as _;
pub const NS_E_NO_STREAM: windows_sys::core::HRESULT = 0xC00D0033_u32 as _;
pub const NS_E_NO_TWOPASS_TIMECOMPRESSION: windows_sys::core::HRESULT = 0xC00D1BD8_u32 as _;
pub const NS_E_NO_VALID_OUTPUT_STREAM: windows_sys::core::HRESULT = 0xC00D1B70_u32 as _;
pub const NS_E_NO_VALID_SOURCE_PLUGIN: windows_sys::core::HRESULT = 0xC00D1B71_u32 as _;
pub const NS_E_NUM_LANGUAGE_MISMATCH: windows_sys::core::HRESULT = 0xC00D1B9B_u32 as _;
pub const NS_E_OFFLINE_MODE: windows_sys::core::HRESULT = 0xC00D0BCA_u32 as _;
pub const NS_E_OPEN_CONTAINING_FOLDER_FAILED: windows_sys::core::HRESULT = 0xC00D1363_u32 as _;
pub const NS_E_OPEN_FILE_LIMIT: windows_sys::core::HRESULT = 0xC00D0031_u32 as _;
pub const NS_E_OUTPUT_PROTECTION_LEVEL_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D2904_u32 as _;
pub const NS_E_OUTPUT_PROTECTION_SCHEME_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D290A_u32 as _;
pub const NS_E_PACKETSINK_UNKNOWN_FEC_STREAM: windows_sys::core::HRESULT = 0xC00D2F0A_u32 as _;
pub const NS_E_PAGING_ERROR: windows_sys::core::HRESULT = 0xC00D0062_u32 as _;
pub const NS_E_PARTIALLY_REBUILT_DISK: windows_sys::core::HRESULT = 0xC00D0067_u32 as _;
pub const NS_E_PDA_CANNOT_CREATE_ADDITIONAL_SYNC_RELATIONSHIP: windows_sys::core::HRESULT = 0xC00D1185_u32 as _;
pub const NS_E_PDA_CANNOT_SYNC_FROM_INTERNET: windows_sys::core::HRESULT = 0xC00D1234_u32 as _;
pub const NS_E_PDA_CANNOT_SYNC_FROM_LOCATION: windows_sys::core::HRESULT = 0xC00D1193_u32 as _;
pub const NS_E_PDA_CANNOT_SYNC_INVALID_PLAYLIST: windows_sys::core::HRESULT = 0xC00D1235_u32 as _;
pub const NS_E_PDA_CANNOT_TRANSCODE: windows_sys::core::HRESULT = 0xC00D1189_u32 as _;
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_AUDIO: windows_sys::core::HRESULT = 0xC00D123D_u32 as _;
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_IMAGE: windows_sys::core::HRESULT = 0xC00D123F_u32 as _;
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_VIDEO: windows_sys::core::HRESULT = 0xC00D123E_u32 as _;
pub const NS_E_PDA_CEWMDM_DRM_ERROR: windows_sys::core::HRESULT = 0xC00D1241_u32 as _;
pub const NS_E_PDA_DELETE_FAILED: windows_sys::core::HRESULT = 0xC00D1238_u32 as _;
pub const NS_E_PDA_DEVICESUPPORTDISABLED: windows_sys::core::HRESULT = 0xC00D1190_u32 as _;
pub const NS_E_PDA_DEVICE_FULL: windows_sys::core::HRESULT = 0xC00D117F_u32 as _;
pub const NS_E_PDA_DEVICE_FULL_IN_SESSION: windows_sys::core::HRESULT = 0xC00D1181_u32 as _;
pub const NS_E_PDA_DEVICE_NOT_RESPONDING: windows_sys::core::HRESULT = 0xC00D123A_u32 as _;
pub const NS_E_PDA_ENCODER_NOT_RESPONDING: windows_sys::core::HRESULT = 0xC00D1192_u32 as _;
pub const NS_E_PDA_FAILED_TO_BURN: windows_sys::core::HRESULT = 0xC00D10DA_u32 as _;
pub const NS_E_PDA_FAILED_TO_ENCRYPT_TRANSCODED_FILE: windows_sys::core::HRESULT = 0xC00D123C_u32 as _;
pub const NS_E_PDA_FAILED_TO_RETRIEVE_FILE: windows_sys::core::HRESULT = 0xC00D1239_u32 as _;
pub const NS_E_PDA_FAILED_TO_SYNCHRONIZE_FILE: windows_sys::core::HRESULT = 0xC00D1236_u32 as _;
pub const NS_E_PDA_FAILED_TO_TRANSCODE_PHOTO: windows_sys::core::HRESULT = 0xC00D123B_u32 as _;
pub const NS_E_PDA_FAIL_READ_WAVE_FILE: windows_sys::core::HRESULT = 0xC00D117D_u32 as _;
pub const NS_E_PDA_FAIL_SELECT_DEVICE: windows_sys::core::HRESULT = 0xC00D117C_u32 as _;
pub const NS_E_PDA_INITIALIZINGDEVICES: windows_sys::core::HRESULT = 0xC00D118D_u32 as _;
pub const NS_E_PDA_MANUALDEVICE: windows_sys::core::HRESULT = 0xC00D1183_u32 as _;
pub const NS_E_PDA_NO_LONGER_AVAILABLE: windows_sys::core::HRESULT = 0xC00D1191_u32 as _;
pub const NS_E_PDA_NO_TRANSCODE_OF_DRM: windows_sys::core::HRESULT = 0xC00D1186_u32 as _;
pub const NS_E_PDA_OBSOLETE_SP: windows_sys::core::HRESULT = 0xC00D118E_u32 as _;
pub const NS_E_PDA_PARTNERSHIPNOTEXIST: windows_sys::core::HRESULT = 0xC00D1184_u32 as _;
pub const NS_E_PDA_RETRIEVED_FILE_FILENAME_TOO_LONG: windows_sys::core::HRESULT = 0xC00D1240_u32 as _;
pub const NS_E_PDA_SYNC_FAILED: windows_sys::core::HRESULT = 0xC00D1237_u32 as _;
pub const NS_E_PDA_SYNC_LOGIN_ERROR: windows_sys::core::HRESULT = 0xC00D1244_u32 as _;
pub const NS_E_PDA_SYNC_RUNNING: windows_sys::core::HRESULT = 0xC00D1243_u32 as _;
pub const NS_E_PDA_TITLE_COLLISION: windows_sys::core::HRESULT = 0xC00D118F_u32 as _;
pub const NS_E_PDA_TOO_MANY_FILES_IN_DIRECTORY: windows_sys::core::HRESULT = 0xC00D118A_u32 as _;
pub const NS_E_PDA_TOO_MANY_FILE_COLLISIONS: windows_sys::core::HRESULT = 0xC00D1188_u32 as _;
pub const NS_E_PDA_TRANSCODECACHEFULL: windows_sys::core::HRESULT = 0xC00D1187_u32 as _;
pub const NS_E_PDA_TRANSCODE_CODEC_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D1245_u32 as _;
pub const NS_E_PDA_TRANSCODE_NOT_PERMITTED: windows_sys::core::HRESULT = 0xC00D118C_u32 as _;
pub const NS_E_PDA_UNSPECIFIED_ERROR: windows_sys::core::HRESULT = 0xC00D117A_u32 as _;
pub const NS_E_PDA_UNSUPPORTED_FORMAT: windows_sys::core::HRESULT = 0xC00D1178_u32 as _;
pub const NS_E_PLAYLIST_CONTAINS_ERRORS: windows_sys::core::HRESULT = 0xC00D10BF_u32 as _;
pub const NS_E_PLAYLIST_END_RECEDING: windows_sys::core::HRESULT = 0xC00D14BD_u32 as _;
pub const NS_E_PLAYLIST_ENTRY_ALREADY_PLAYING: windows_sys::core::HRESULT = 0xC00D14B4_u32 as _;
pub const NS_E_PLAYLIST_ENTRY_HAS_CHANGED: windows_sys::core::HRESULT = 0xC00D2EF5_u32 as _;
pub const NS_E_PLAYLIST_ENTRY_NOT_IN_PLAYLIST: windows_sys::core::HRESULT = 0xC00D14B8_u32 as _;
pub const NS_E_PLAYLIST_ENTRY_SEEK: windows_sys::core::HRESULT = 0xC00D14B9_u32 as _;
pub const NS_E_PLAYLIST_PARSE_FAILURE: windows_sys::core::HRESULT = 0xC00D14B6_u32 as _;
pub const NS_E_PLAYLIST_PLUGIN_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D157F_u32 as _;
pub const NS_E_PLAYLIST_RECURSIVE_PLAYLISTS: windows_sys::core::HRESULT = 0xC00D14BA_u32 as _;
pub const NS_E_PLAYLIST_SHUTDOWN: windows_sys::core::HRESULT = 0xC00D14BC_u32 as _;
pub const NS_E_PLAYLIST_TOO_MANY_NESTED_PLAYLISTS: windows_sys::core::HRESULT = 0xC00D14BB_u32 as _;
pub const NS_E_PLAYLIST_UNSUPPORTED_ENTRY: windows_sys::core::HRESULT = 0xC00D14B7_u32 as _;
pub const NS_E_PLUGIN_CLSID_INVALID: windows_sys::core::HRESULT = 0xC00D1B76_u32 as _;
pub const NS_E_PLUGIN_ERROR_REPORTED: windows_sys::core::HRESULT = 0xC00D157D_u32 as _;
pub const NS_E_PLUGIN_NOTSHUTDOWN: windows_sys::core::HRESULT = 0xC00D0FD6_u32 as _;
pub const NS_E_PORT_IN_USE: windows_sys::core::HRESULT = 0xC00D158A_u32 as _;
pub const NS_E_PORT_IN_USE_HTTP: windows_sys::core::HRESULT = 0xC00D158B_u32 as _;
pub const NS_E_PROCESSINGSHOWSYNCWIZARD: windows_sys::core::HRESULT = 0xC00D118B_u32 as _;
pub const NS_E_PROFILE_MISMATCH: windows_sys::core::HRESULT = 0xC00D1B7B_u32 as _;
pub const NS_E_PROPERTY_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D32CA_u32 as _;
pub const NS_E_PROPERTY_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC00D32D2_u32 as _;
pub const NS_E_PROPERTY_READ_ONLY: windows_sys::core::HRESULT = 0xC00D32CC_u32 as _;
pub const NS_E_PROTECTED_CONTENT: windows_sys::core::HRESULT = 0xC00D0BBD_u32 as _;
pub const NS_E_PROTOCOL_MISMATCH: windows_sys::core::HRESULT = 0xC00D0012_u32 as _;
pub const NS_E_PROXY_ACCESSDENIED: windows_sys::core::HRESULT = 0xC00D2EF6_u32 as _;
pub const NS_E_PROXY_CONNECT_TIMEOUT: windows_sys::core::HRESULT = 0xC00D2F07_u32 as _;
pub const NS_E_PROXY_DNS_TIMEOUT: windows_sys::core::HRESULT = 0xC00D2EF0_u32 as _;
pub const NS_E_PROXY_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D2EED_u32 as _;
pub const NS_E_PROXY_SOURCE_ACCESSDENIED: windows_sys::core::HRESULT = 0xC00D2EF7_u32 as _;
pub const NS_E_PROXY_TIMEOUT: windows_sys::core::HRESULT = 0xC00D2EE5_u32 as _;
pub const NS_E_PUBLISHING_POINT_INVALID_REQUEST_WHILE_STARTED: windows_sys::core::HRESULT = 0xC00D1457_u32 as _;
pub const NS_E_PUBLISHING_POINT_REMOVED: windows_sys::core::HRESULT = 0xC00D145A_u32 as _;
pub const NS_E_PUBLISHING_POINT_STOPPED: windows_sys::core::HRESULT = 0xC00D145E_u32 as _;
pub const NS_E_PUSH_CANNOTCONNECT: windows_sys::core::HRESULT = 0xC00D2F0B_u32 as _;
pub const NS_E_PUSH_DUPLICATE_PUBLISHING_POINT_NAME: windows_sys::core::HRESULT = 0xC00D1520_u32 as _;
pub const NS_E_REBOOT_RECOMMENDED: windows_sys::core::HRESULT = 0xC00D2AFA_u32 as _;
pub const NS_E_REBOOT_REQUIRED: windows_sys::core::HRESULT = 0xC00D2AFB_u32 as _;
pub const NS_E_RECORDQ_DISK_FULL: windows_sys::core::HRESULT = 0xC00D1BA3_u32 as _;
pub const NS_E_REDBOOK_ENABLED_WHILE_COPYING: windows_sys::core::HRESULT = 0xC00D0FB0_u32 as _;
pub const NS_E_REDIRECT: windows_sys::core::HRESULT = 0xC00D1388_u32 as _;
pub const NS_E_REDIRECT_TO_PROXY: windows_sys::core::HRESULT = 0xC00D2EE1_u32 as _;
pub const NS_E_REFUSED_BY_SERVER: windows_sys::core::HRESULT = 0xC00D2EE7_u32 as _;
pub const NS_E_REG_FLUSH_FAILURE: windows_sys::core::HRESULT = 0xC00D2798_u32 as _;
pub const NS_E_REMIRRORED_DISK: windows_sys::core::HRESULT = 0xC00D00C9_u32 as _;
pub const NS_E_REQUIRE_STREAMING_CLIENT: windows_sys::core::HRESULT = 0xC00D2EF4_u32 as _;
pub const NS_E_RESET_SOCKET_CONNECTION: windows_sys::core::HRESULT = 0xC00D2F00_u32 as _;
pub const NS_E_RESOURCE_GONE: windows_sys::core::HRESULT = 0xC00D2EFC_u32 as _;
pub const NS_E_SAME_AS_INPUT_COMBINATION: windows_sys::core::HRESULT = 0xC00D1BD2_u32 as _;
pub const NS_E_SCHEMA_CLASSIFY_FAILURE: windows_sys::core::HRESULT = 0xC00D32D4_u32 as _;
pub const NS_E_SCRIPT_DEBUGGER_NOT_INSTALLED: windows_sys::core::HRESULT = 0xC00D1582_u32 as _;
pub const NS_E_SDK_BUFFERTOOSMALL: windows_sys::core::HRESULT = 0xC00D0BD4_u32 as _;
pub const NS_E_SERVER_ACCESSDENIED: windows_sys::core::HRESULT = 0xC00D2EFB_u32 as _;
pub const NS_E_SERVER_DNS_TIMEOUT: windows_sys::core::HRESULT = 0xC00D2EEF_u32 as _;
pub const NS_E_SERVER_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D0035_u32 as _;
pub const NS_E_SERVER_UNAVAILABLE: windows_sys::core::HRESULT = 0xC00D2EE6_u32 as _;
pub const NS_E_SESSION_INVALID: windows_sys::core::HRESULT = 0xC00D2F08_u32 as _;
pub const NS_E_SESSION_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D2EF3_u32 as _;
pub const NS_E_SETUP_BLOCKED: windows_sys::core::HRESULT = 0xC00D2B00_u32 as _;
pub const NS_E_SETUP_DRM_MIGRATION_FAILED: windows_sys::core::HRESULT = 0xC00D2AFD_u32 as _;
pub const NS_E_SETUP_DRM_MIGRATION_FAILED_AND_IGNORABLE_FAILURE: windows_sys::core::HRESULT = 0xC00D2AFF_u32 as _;
pub const NS_E_SETUP_IGNORABLE_FAILURE: windows_sys::core::HRESULT = 0xC00D2AFE_u32 as _;
pub const NS_E_SETUP_INCOMPLETE: windows_sys::core::HRESULT = 0xC00D2AFC_u32 as _;
pub const NS_E_SET_DISK_UID_FAILED: windows_sys::core::HRESULT = 0xC00D0021_u32 as _;
pub const NS_E_SHARING_STATE_OUT_OF_SYNC: windows_sys::core::HRESULT = 0xC00D0FF4_u32 as _;
pub const NS_E_SHARING_VIOLATION: windows_sys::core::HRESULT = 0xC00D0FCF_u32 as _;
pub const NS_E_SHUTDOWN: windows_sys::core::HRESULT = 0xC00D002A_u32 as _;
pub const NS_E_SLOW_READ_DIGITAL: windows_sys::core::HRESULT = 0xC00D0FA4_u32 as _;
pub const NS_E_SLOW_READ_DIGITAL_WITH_ERRORCORRECTION: windows_sys::core::HRESULT = 0xC00D11FD_u32 as _;
pub const NS_E_SMPTEMODE_MISMATCH: windows_sys::core::HRESULT = 0xC00D1BAD_u32 as _;
pub const NS_E_SOURCEGROUP_NOTPREPARED: windows_sys::core::HRESULT = 0xC00D1B7A_u32 as _;
pub const NS_E_SOURCE_CANNOT_LOOP: windows_sys::core::HRESULT = 0xC00D1BD3_u32 as _;
pub const NS_E_SOURCE_NOTSPECIFIED: windows_sys::core::HRESULT = 0xC00D1B85_u32 as _;
pub const NS_E_SOURCE_PLUGIN_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D157E_u32 as _;
pub const NS_E_SPEECHEDL_ON_NON_MIXEDMODE: windows_sys::core::HRESULT = 0xC00D1B92_u32 as _;
pub const NS_E_STALE_PRESENTATION: windows_sys::core::HRESULT = 0xC00D1389_u32 as _;
pub const NS_E_STREAM_END: windows_sys::core::HRESULT = 0xC00D0034_u32 as _;
pub const NS_E_STRIDE_REFUSED: windows_sys::core::HRESULT = 0xC00D0045_u32 as _;
pub const NS_E_SUBSCRIPTIONSERVICE_DOWNLOAD_TIMEOUT: windows_sys::core::HRESULT = 0xC00D1360_u32 as _;
pub const NS_E_SUBSCRIPTIONSERVICE_LOGIN_FAILED: windows_sys::core::HRESULT = 0xC00D135F_u32 as _;
pub const NS_E_SUBSCRIPTIONSERVICE_PLAYBACK_DISALLOWED: windows_sys::core::HRESULT = 0xC00D1356_u32 as _;
pub const NS_E_SYNCWIZ_CANNOT_CHANGE_SETTINGS: windows_sys::core::HRESULT = 0xC00D11EF_u32 as _;
pub const NS_E_SYNCWIZ_DEVICE_FULL: windows_sys::core::HRESULT = 0xC00D11EE_u32 as _;
pub const NS_E_TABLE_KEY_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D32CD_u32 as _;
pub const NS_E_TAMPERED_CONTENT: windows_sys::core::HRESULT = 0xC00D0BBF_u32 as _;
pub const NS_E_TCP_DISABLED: windows_sys::core::HRESULT = 0xC00D00D2_u32 as _;
pub const NS_E_TIGER_FAIL: windows_sys::core::HRESULT = 0xC00D0050_u32 as _;
pub const NS_E_TIMECODE_REQUIRES_VIDEOSTREAM: windows_sys::core::HRESULT = 0xC00D1BD9_u32 as _;
pub const NS_E_TIMEOUT: windows_sys::core::HRESULT = 0xC00D0013_u32 as _;
pub const NS_E_TITLE_BITRATE: windows_sys::core::HRESULT = 0xC00D00D5_u32 as _;
pub const NS_E_TITLE_SIZE_EXCEEDED: windows_sys::core::HRESULT = 0xC00D00D0_u32 as _;
pub const NS_E_TOO_MANY_AUDIO: windows_sys::core::HRESULT = 0xC00D1B5C_u32 as _;
pub const NS_E_TOO_MANY_DEVICECONTROL: windows_sys::core::HRESULT = 0xC00D1B96_u32 as _;
pub const NS_E_TOO_MANY_HOPS: windows_sys::core::HRESULT = 0xC00D2F02_u32 as _;
pub const NS_E_TOO_MANY_MULTICAST_SINKS: windows_sys::core::HRESULT = 0xC00D1456_u32 as _;
pub const NS_E_TOO_MANY_SESS: windows_sys::core::HRESULT = 0xC00D000F_u32 as _;
pub const NS_E_TOO_MANY_TITLES: windows_sys::core::HRESULT = 0xC00D00CF_u32 as _;
pub const NS_E_TOO_MANY_VIDEO: windows_sys::core::HRESULT = 0xC00D1B5D_u32 as _;
pub const NS_E_TOO_MUCH_DATA: windows_sys::core::HRESULT = 0xC00D0BCC_u32 as _;
pub const NS_E_TOO_MUCH_DATA_FROM_SERVER: windows_sys::core::HRESULT = 0xC00D2F05_u32 as _;
pub const NS_E_TRACK_DOWNLOAD_REQUIRES_ALBUM_PURCHASE: windows_sys::core::HRESULT = 0xC00D135B_u32 as _;
pub const NS_E_TRACK_DOWNLOAD_REQUIRES_PURCHASE: windows_sys::core::HRESULT = 0xC00D135C_u32 as _;
pub const NS_E_TRACK_PURCHASE_MAXIMUM_EXCEEDED: windows_sys::core::HRESULT = 0xC00D135D_u32 as _;
pub const NS_E_TRANSCODE_DELETECACHEERROR: windows_sys::core::HRESULT = 0xC00D11F0_u32 as _;
pub const NS_E_TRANSFORM_PLUGIN_INVALID: windows_sys::core::HRESULT = 0xC00D1BE6_u32 as _;
pub const NS_E_TRANSFORM_PLUGIN_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D1BE5_u32 as _;
pub const NS_E_UDP_DISABLED: windows_sys::core::HRESULT = 0xC00D00D1_u32 as _;
pub const NS_E_UNABLE_TO_CREATE_RIP_LOCATION: windows_sys::core::HRESULT = 0xC00D10D0_u32 as _;
pub const NS_E_UNCOMPRESSED_DIGITAL_AUDIO_PROTECTION_LEVEL_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D2909_u32 as _;
pub const NS_E_UNCOMPRESSED_DIGITAL_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D2906_u32 as _;
pub const NS_E_UNCOMP_COMP_COMBINATION: windows_sys::core::HRESULT = 0xC00D1BB6_u32 as _;
pub const NS_E_UNEXPECTED_DISPLAY_SETTINGS: windows_sys::core::HRESULT = 0xC00D1B88_u32 as _;
pub const NS_E_UNEXPECTED_MSAUDIO_ERROR: windows_sys::core::HRESULT = 0xC00D0BBA_u32 as _;
pub const NS_E_UNKNOWN_PROTOCOL: windows_sys::core::HRESULT = 0xC00D2EE0_u32 as _;
pub const NS_E_UNRECOGNIZED_STREAM_TYPE: windows_sys::core::HRESULT = 0xC00D0026_u32 as _;
pub const NS_E_UNSUPPORTED_ARCHIVEOPERATION: windows_sys::core::HRESULT = 0xC00D1B78_u32 as _;
pub const NS_E_UNSUPPORTED_ARCHIVETYPE: windows_sys::core::HRESULT = 0xC00D1B77_u32 as _;
pub const NS_E_UNSUPPORTED_ENCODER_DEVICE: windows_sys::core::HRESULT = 0xC00D1B87_u32 as _;
pub const NS_E_UNSUPPORTED_LANGUAGE: windows_sys::core::HRESULT = 0xC00D145C_u32 as _;
pub const NS_E_UNSUPPORTED_LOAD_TYPE: windows_sys::core::HRESULT = 0xC00D1453_u32 as _;
pub const NS_E_UNSUPPORTED_PROPERTY: windows_sys::core::HRESULT = 0xC00D0BCD_u32 as _;
pub const NS_E_UNSUPPORTED_SOURCETYPE: windows_sys::core::HRESULT = 0xC00D1B5B_u32 as _;
pub const NS_E_URLLIST_INVALIDFORMAT: windows_sys::core::HRESULT = 0xC00D106D_u32 as _;
pub const NS_E_USER_STOP: windows_sys::core::HRESULT = 0xC00D0FA9_u32 as _;
pub const NS_E_USE_FILE_SOURCE: windows_sys::core::HRESULT = 0xC00D32C9_u32 as _;
pub const NS_E_VBRMODE_MISMATCH: windows_sys::core::HRESULT = 0xC00D1B9D_u32 as _;
pub const NS_E_VIDCAPCREATEWINDOW: windows_sys::core::HRESULT = 0xC00D1B6D_u32 as _;
pub const NS_E_VIDCAPDRVINUSE: windows_sys::core::HRESULT = 0xC00D1B6E_u32 as _;
pub const NS_E_VIDCAPSTARTFAILED: windows_sys::core::HRESULT = 0xC00D1B69_u32 as _;
pub const NS_E_VIDEODEVICE_BUSY: windows_sys::core::HRESULT = 0xC00D1B64_u32 as _;
pub const NS_E_VIDEODEVICE_UNEXPECTED: windows_sys::core::HRESULT = 0xC00D1B65_u32 as _;
pub const NS_E_VIDEODRIVER_UNSTABLE: windows_sys::core::HRESULT = 0xC00D1B68_u32 as _;
pub const NS_E_VIDEO_BITRATE_STEPDOWN: windows_sys::core::HRESULT = 0xC00D1BC0_u32 as _;
pub const NS_E_VIDEO_CODEC_ERROR: windows_sys::core::HRESULT = 0xC00D0BC5_u32 as _;
pub const NS_E_VIDEO_CODEC_NOT_INSTALLED: windows_sys::core::HRESULT = 0xC00D0BC4_u32 as _;
pub const NS_E_VIDSOURCECOMPRESSION: windows_sys::core::HRESULT = 0xC00D1B6A_u32 as _;
pub const NS_E_VIDSOURCESIZE: windows_sys::core::HRESULT = 0xC00D1B6B_u32 as _;
pub const NS_E_WALKER_SERVER: windows_sys::core::HRESULT = 0xC00D004D_u32 as _;
pub const NS_E_WALKER_UNKNOWN: windows_sys::core::HRESULT = 0xC00D004C_u32 as _;
pub const NS_E_WALKER_USAGE: windows_sys::core::HRESULT = 0xC00D004E_u32 as _;
pub const NS_E_WAVE_OPEN: windows_sys::core::HRESULT = 0xC00D006D_u32 as _;
pub const NS_E_WINSOCK_ERROR_STRING: windows_sys::core::HRESULT = 0xC00D1129_u32 as _;
pub const NS_E_WIZARD_RUNNING: windows_sys::core::HRESULT = 0xC00D1584_u32 as _;
pub const NS_E_WMDM_REVOKED: windows_sys::core::HRESULT = 0xC00D10BC_u32 as _;
pub const NS_E_WMDRM_DEPRECATED: windows_sys::core::HRESULT = 0xC00D0BDE_u32 as _;
pub const NS_E_WME_VERSION_MISMATCH: windows_sys::core::HRESULT = 0xC00D1B8B_u32 as _;
pub const NS_E_WMG_CANNOTQUEUE: windows_sys::core::HRESULT = 0xC00D104C_u32 as _;
pub const NS_E_WMG_COPP_SECURITY_INVALID: windows_sys::core::HRESULT = 0xC00D1052_u32 as _;
pub const NS_E_WMG_COPP_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D1053_u32 as _;
pub const NS_E_WMG_FILETRANSFERNOTALLOWED: windows_sys::core::HRESULT = 0xC00D1058_u32 as _;
pub const NS_E_WMG_INVALIDSTATE: windows_sys::core::HRESULT = 0xC00D1054_u32 as _;
pub const NS_E_WMG_INVALID_COPP_CERTIFICATE: windows_sys::core::HRESULT = 0xC00D1051_u32 as _;
pub const NS_E_WMG_LICENSE_TAMPERED: windows_sys::core::HRESULT = 0xC00D1064_u32 as _;
pub const NS_E_WMG_NOSDKINTERFACE: windows_sys::core::HRESULT = 0xC00D1056_u32 as _;
pub const NS_E_WMG_NOTALLOUTPUTSRENDERED: windows_sys::core::HRESULT = 0xC00D1057_u32 as _;
pub const NS_E_WMG_PLUGINUNAVAILABLE: windows_sys::core::HRESULT = 0xC00D104B_u32 as _;
pub const NS_E_WMG_PREROLLLICENSEACQUISITIONNOTALLOWED: windows_sys::core::HRESULT = 0xC00D104D_u32 as _;
pub const NS_E_WMG_RATEUNAVAILABLE: windows_sys::core::HRESULT = 0xC00D104A_u32 as _;
pub const NS_E_WMG_SINKALREADYEXISTS: windows_sys::core::HRESULT = 0xC00D1055_u32 as _;
pub const NS_E_WMG_UNEXPECTEDPREROLLSTATUS: windows_sys::core::HRESULT = 0xC00D104E_u32 as _;
pub const NS_E_WMPBR_BACKUPCANCEL: windows_sys::core::HRESULT = 0xC00D1131_u32 as _;
pub const NS_E_WMPBR_BACKUPRESTOREFAILED: windows_sys::core::HRESULT = 0xC00D1138_u32 as _;
pub const NS_E_WMPBR_DRIVE_INVALID: windows_sys::core::HRESULT = 0xC00D1137_u32 as _;
pub const NS_E_WMPBR_ERRORWITHURL: windows_sys::core::HRESULT = 0xC00D1133_u32 as _;
pub const NS_E_WMPBR_NAMECOLLISION: windows_sys::core::HRESULT = 0xC00D1134_u32 as _;
pub const NS_E_WMPBR_NOLISTENER: windows_sys::core::HRESULT = 0xC00D1130_u32 as _;
pub const NS_E_WMPBR_RESTORECANCEL: windows_sys::core::HRESULT = 0xC00D1132_u32 as _;
pub const NS_E_WMPCORE_BUFFERTOOSMALL: windows_sys::core::HRESULT = 0xC00D107F_u32 as _;
pub const NS_E_WMPCORE_BUSY: windows_sys::core::HRESULT = 0xC00D10B7_u32 as _;
pub const NS_E_WMPCORE_COCREATEFAILEDFORGITOBJECT: windows_sys::core::HRESULT = 0xC00D107D_u32 as _;
pub const NS_E_WMPCORE_CODEC_DOWNLOAD_NOT_ALLOWED: windows_sys::core::HRESULT = 0xC00D109C_u32 as _;
pub const NS_E_WMPCORE_CODEC_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D109B_u32 as _;
pub const NS_E_WMPCORE_CODEC_NOT_TRUSTED: windows_sys::core::HRESULT = 0xC00D109A_u32 as _;
pub const NS_E_WMPCORE_CURRENT_MEDIA_NOT_ACTIVE: windows_sys::core::HRESULT = 0xC00D10A9_u32 as _;
pub const NS_E_WMPCORE_DEVICE_DRIVERS_MISSING: windows_sys::core::HRESULT = 0xC00D10DD_u32 as _;
pub const NS_E_WMPCORE_ERRORMANAGERNOTAVAILABLE: windows_sys::core::HRESULT = 0xC00D108D_u32 as _;
pub const NS_E_WMPCORE_ERRORSINKNOTREGISTERED: windows_sys::core::HRESULT = 0xC00D108C_u32 as _;
pub const NS_E_WMPCORE_ERROR_DOWNLOADING_PLAYLIST: windows_sys::core::HRESULT = 0xC00D109D_u32 as _;
pub const NS_E_WMPCORE_FAILEDTOGETMARSHALLEDEVENTHANDLERINTERFACE: windows_sys::core::HRESULT = 0xC00D107E_u32 as _;
pub const NS_E_WMPCORE_FAILED_TO_BUILD_PLAYLIST: windows_sys::core::HRESULT = 0xC00D109E_u32 as _;
pub const NS_E_WMPCORE_FILE_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D10BA_u32 as _;
pub const NS_E_WMPCORE_GRAPH_NOT_IN_LIST: windows_sys::core::HRESULT = 0xC00D108A_u32 as _;
pub const NS_E_WMPCORE_INVALIDPLAYLISTMODE: windows_sys::core::HRESULT = 0xC00D1081_u32 as _;
pub const NS_E_WMPCORE_INVALID_PLAYLIST_URL: windows_sys::core::HRESULT = 0xC00D10AF_u32 as _;
pub const NS_E_WMPCORE_ITEMNOTINPLAYLIST: windows_sys::core::HRESULT = 0xC00D1086_u32 as _;
pub const NS_E_WMPCORE_LIST_ENTRY_NO_REF: windows_sys::core::HRESULT = 0xC00D1098_u32 as _;
pub const NS_E_WMPCORE_MEDIA_ALTERNATE_REF_EMPTY: windows_sys::core::HRESULT = 0xC00D10A4_u32 as _;
pub const NS_E_WMPCORE_MEDIA_CHILD_PLAYLIST_UNAVAILABLE: windows_sys::core::HRESULT = 0xC00D10B8_u32 as _;
pub const NS_E_WMPCORE_MEDIA_ERROR_RESUME_FAILED: windows_sys::core::HRESULT = 0xC00D108F_u32 as _;
pub const NS_E_WMPCORE_MEDIA_NO_CHILD_PLAYLIST: windows_sys::core::HRESULT = 0xC00D10B9_u32 as _;
pub const NS_E_WMPCORE_MEDIA_UNAVAILABLE: windows_sys::core::HRESULT = 0xC00D10B3_u32 as _;
pub const NS_E_WMPCORE_MEDIA_URL_TOO_LONG: windows_sys::core::HRESULT = 0xC00D10C8_u32 as _;
pub const NS_E_WMPCORE_MISMATCHED_RUNTIME: windows_sys::core::HRESULT = 0xC00D10B0_u32 as _;
pub const NS_E_WMPCORE_MISNAMED_FILE: windows_sys::core::HRESULT = 0xC00D1099_u32 as _;
pub const NS_E_WMPCORE_NOBROWSER: windows_sys::core::HRESULT = 0xC00D1088_u32 as _;
pub const NS_E_WMPCORE_NOSOURCEURLSTRING: windows_sys::core::HRESULT = 0xC00D107C_u32 as _;
pub const NS_E_WMPCORE_NO_PLAYABLE_MEDIA_IN_PLAYLIST: windows_sys::core::HRESULT = 0xC00D10B5_u32 as _;
pub const NS_E_WMPCORE_NO_REF_IN_ENTRY: windows_sys::core::HRESULT = 0xC00D1090_u32 as _;
pub const NS_E_WMPCORE_PLAYLISTEMPTY: windows_sys::core::HRESULT = 0xC00D1087_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_EMPTY_NESTED_PLAYLIST_SKIPPED_ITEMS: windows_sys::core::HRESULT = 0xC00D10B6_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_EMPTY_OR_SINGLE_MEDIA: windows_sys::core::HRESULT = 0xC00D108B_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_EVENT_ATTRIBUTE_ABSENT: windows_sys::core::HRESULT = 0xC00D10A6_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_EVENT_EMPTY: windows_sys::core::HRESULT = 0xC00D10A7_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_IMPORT_FAILED_NO_ITEMS: windows_sys::core::HRESULT = 0xC00D10B1_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_EXHAUSTED: windows_sys::core::HRESULT = 0xC00D10A0_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_INIT_FAILED: windows_sys::core::HRESULT = 0xC00D10A3_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_MORPH_FAILED: windows_sys::core::HRESULT = 0xC00D10A2_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_NAME_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D10A1_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_NONE: windows_sys::core::HRESULT = 0xC00D109F_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_NO_EVENT_NAME: windows_sys::core::HRESULT = 0xC00D10A5_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_EMPTY: windows_sys::core::HRESULT = 0xC00D10AC_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_END_MEDIA_NONE: windows_sys::core::HRESULT = 0xC00D10AE_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_START_MEDIA_NONE: windows_sys::core::HRESULT = 0xC00D10AD_u32 as _;
pub const NS_E_WMPCORE_PLAYLIST_STACK_EMPTY: windows_sys::core::HRESULT = 0xC00D10A8_u32 as _;
pub const NS_E_WMPCORE_SOME_CODECS_MISSING: windows_sys::core::HRESULT = 0xC00D10D1_u32 as _;
pub const NS_E_WMPCORE_TEMP_FILE_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D10BB_u32 as _;
pub const NS_E_WMPCORE_UNAVAILABLE: windows_sys::core::HRESULT = 0xC00D1080_u32 as _;
pub const NS_E_WMPCORE_UNRECOGNIZED_MEDIA_URL: windows_sys::core::HRESULT = 0xC00D1089_u32 as _;
pub const NS_E_WMPCORE_USER_CANCEL: windows_sys::core::HRESULT = 0xC00D10AB_u32 as _;
pub const NS_E_WMPCORE_VIDEO_TRANSFORM_FILTER_INSERTION: windows_sys::core::HRESULT = 0xC00D10B2_u32 as _;
pub const NS_E_WMPCORE_WEBHELPFAILED: windows_sys::core::HRESULT = 0xC00D108E_u32 as _;
pub const NS_E_WMPCORE_WMX_ENTRYREF_NO_REF: windows_sys::core::HRESULT = 0xC00D10B4_u32 as _;
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_NAME_EMPTY: windows_sys::core::HRESULT = 0xC00D1091_u32 as _;
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_NAME_ILLEGAL: windows_sys::core::HRESULT = 0xC00D1092_u32 as _;
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_VALUE_EMPTY: windows_sys::core::HRESULT = 0xC00D1093_u32 as _;
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_VALUE_ILLEGAL: windows_sys::core::HRESULT = 0xC00D1094_u32 as _;
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_NAME_EMPTY: windows_sys::core::HRESULT = 0xC00D1095_u32 as _;
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_NAME_ILLEGAL: windows_sys::core::HRESULT = 0xC00D1096_u32 as _;
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_VALUE_EMPTY: windows_sys::core::HRESULT = 0xC00D1097_u32 as _;
pub const NS_E_WMPFLASH_CANT_FIND_COM_SERVER: windows_sys::core::HRESULT = 0xC00D10C9_u32 as _;
pub const NS_E_WMPFLASH_INCOMPATIBLEVERSION: windows_sys::core::HRESULT = 0xC00D10CA_u32 as _;
pub const NS_E_WMPIM_DIALUPFAILED: windows_sys::core::HRESULT = 0xC00D1128_u32 as _;
pub const NS_E_WMPIM_USERCANCELED: windows_sys::core::HRESULT = 0xC00D1127_u32 as _;
pub const NS_E_WMPIM_USEROFFLINE: windows_sys::core::HRESULT = 0xC00D1126_u32 as _;
pub const NS_E_WMPOCXGRAPH_IE_DISALLOWS_ACTIVEX_CONTROLS: windows_sys::core::HRESULT = 0xC00D10CB_u32 as _;
pub const NS_E_WMPOCX_ERRORMANAGERNOTAVAILABLE: windows_sys::core::HRESULT = 0xC00D0FD5_u32 as _;
pub const NS_E_WMPOCX_NOT_RUNNING_REMOTELY: windows_sys::core::HRESULT = 0xC00D0FD3_u32 as _;
pub const NS_E_WMPOCX_NO_ACTIVE_CORE: windows_sys::core::HRESULT = 0xC00D0FD2_u32 as _;
pub const NS_E_WMPOCX_NO_REMOTE_CORE: windows_sys::core::HRESULT = 0xC00D0FD1_u32 as _;
pub const NS_E_WMPOCX_NO_REMOTE_WINDOW: windows_sys::core::HRESULT = 0xC00D0FD4_u32 as _;
pub const NS_E_WMPOCX_PLAYER_NOT_DOCKED: windows_sys::core::HRESULT = 0xC00D0FDB_u32 as _;
pub const NS_E_WMPOCX_REMOTE_PLAYER_ALREADY_RUNNING: windows_sys::core::HRESULT = 0xC00D0FFA_u32 as _;
pub const NS_E_WMPOCX_UNABLE_TO_LOAD_SKIN: windows_sys::core::HRESULT = 0xC00D0FEB_u32 as _;
pub const NS_E_WMPXML_ATTRIBUTENOTFOUND: windows_sys::core::HRESULT = 0xC00D0FB7_u32 as _;
pub const NS_E_WMPXML_EMPTYDOC: windows_sys::core::HRESULT = 0xC00D0FB9_u32 as _;
pub const NS_E_WMPXML_ENDOFDATA: windows_sys::core::HRESULT = 0xC00D0FB5_u32 as _;
pub const NS_E_WMPXML_NOERROR: windows_sys::core::HRESULT = 0xC00D0FB4_u32 as _;
pub const NS_E_WMPXML_PARSEERROR: windows_sys::core::HRESULT = 0xC00D0FB6_u32 as _;
pub const NS_E_WMPXML_PINOTFOUND: windows_sys::core::HRESULT = 0xC00D0FB8_u32 as _;
pub const NS_E_WMPZIP_CORRUPT: windows_sys::core::HRESULT = 0xC00D1019_u32 as _;
pub const NS_E_WMPZIP_FILENOTFOUND: windows_sys::core::HRESULT = 0xC00D101A_u32 as _;
pub const NS_E_WMPZIP_NOTAZIPFILE: windows_sys::core::HRESULT = 0xC00D1018_u32 as _;
pub const NS_E_WMP_ACCESS_DENIED: windows_sys::core::HRESULT = 0xC00D11D2_u32 as _;
pub const NS_E_WMP_ADDTOLIBRARY_FAILED: windows_sys::core::HRESULT = 0xC00D0FC7_u32 as _;
pub const NS_E_WMP_ALREADY_IN_USE: windows_sys::core::HRESULT = 0xC00D119E_u32 as _;
pub const NS_E_WMP_AUDIO_CODEC_NOT_INSTALLED: windows_sys::core::HRESULT = 0xC00D11C7_u32 as _;
pub const NS_E_WMP_AUDIO_DEVICE_LOST: windows_sys::core::HRESULT = 0xC00D11E5_u32 as _;
pub const NS_E_WMP_AUDIO_HW_PROBLEM: windows_sys::core::HRESULT = 0xC00D11BA_u32 as _;
pub const NS_E_WMP_AUTOPLAY_INVALID_STATE: windows_sys::core::HRESULT = 0xC00D12FC_u32 as _;
pub const NS_E_WMP_BAD_DRIVER: windows_sys::core::HRESULT = 0xC00D11D1_u32 as _;
pub const NS_E_WMP_BMP_BITMAP_NOT_CREATED: windows_sys::core::HRESULT = 0xC00D1030_u32 as _;
pub const NS_E_WMP_BMP_COMPRESSION_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D1031_u32 as _;
pub const NS_E_WMP_BMP_INVALID_BITMASK: windows_sys::core::HRESULT = 0xC00D102E_u32 as _;
pub const NS_E_WMP_BMP_INVALID_FORMAT: windows_sys::core::HRESULT = 0xC00D1032_u32 as _;
pub const NS_E_WMP_BMP_TOPDOWN_DIB_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D102F_u32 as _;
pub const NS_E_WMP_BSTR_TOO_LONG: windows_sys::core::HRESULT = 0xC00D12F2_u32 as _;
pub const NS_E_WMP_BURN_DISC_OVERFLOW: windows_sys::core::HRESULT = 0xC00D11D9_u32 as _;
pub const NS_E_WMP_CANNOT_BURN_NON_LOCAL_FILE: windows_sys::core::HRESULT = 0xC00D10D6_u32 as _;
pub const NS_E_WMP_CANNOT_FIND_FILE: windows_sys::core::HRESULT = 0xC00D1197_u32 as _;
pub const NS_E_WMP_CANNOT_FIND_FOLDER: windows_sys::core::HRESULT = 0xC00D0FD7_u32 as _;
pub const NS_E_WMP_CANT_PLAY_PROTECTED: windows_sys::core::HRESULT = 0xC00D0FF3_u32 as _;
pub const NS_E_WMP_CD_ANOTHER_USER: windows_sys::core::HRESULT = 0xC00D11CF_u32 as _;
pub const NS_E_WMP_CD_STASH_NO_SPACE: windows_sys::core::HRESULT = 0xC00D11D5_u32 as _;
pub const NS_E_WMP_CODEC_NEEDED_WITH_4CC: windows_sys::core::HRESULT = 0xC00D11A1_u32 as _;
pub const NS_E_WMP_CODEC_NEEDED_WITH_FORMATTAG: windows_sys::core::HRESULT = 0xC00D11A2_u32 as _;
pub const NS_E_WMP_COMPONENT_REVOKED: windows_sys::core::HRESULT = 0xC00D1306_u32 as _;
pub const NS_E_WMP_CONNECT_TIMEOUT: windows_sys::core::HRESULT = 0xC00D11C1_u32 as _;
pub const NS_E_WMP_CONVERT_FILE_CORRUPT: windows_sys::core::HRESULT = 0xC00D115B_u32 as _;
pub const NS_E_WMP_CONVERT_FILE_FAILED: windows_sys::core::HRESULT = 0xC00D1158_u32 as _;
pub const NS_E_WMP_CONVERT_NO_RIGHTS_ERRORURL: windows_sys::core::HRESULT = 0xC00D1159_u32 as _;
pub const NS_E_WMP_CONVERT_NO_RIGHTS_NOERRORURL: windows_sys::core::HRESULT = 0xC00D115A_u32 as _;
pub const NS_E_WMP_CONVERT_PLUGIN_UNAVAILABLE_ERRORURL: windows_sys::core::HRESULT = 0xC00D115C_u32 as _;
pub const NS_E_WMP_CONVERT_PLUGIN_UNAVAILABLE_NOERRORURL: windows_sys::core::HRESULT = 0xC00D115D_u32 as _;
pub const NS_E_WMP_CONVERT_PLUGIN_UNKNOWN_FILE_OWNER: windows_sys::core::HRESULT = 0xC00D115E_u32 as _;
pub const NS_E_WMP_CS_JPGPOSITIONIMAGE: windows_sys::core::HRESULT = 0xC00D100E_u32 as _;
pub const NS_E_WMP_CS_NOTEVENLYDIVISIBLE: windows_sys::core::HRESULT = 0xC00D100F_u32 as _;
pub const NS_E_WMP_DAI_SONGTOOSHORT: windows_sys::core::HRESULT = 0xC00D1049_u32 as _;
pub const NS_E_WMP_DRM_ACQUIRING_LICENSE: windows_sys::core::HRESULT = 0xC00D1202_u32 as _;
pub const NS_E_WMP_DRM_CANNOT_RESTORE: windows_sys::core::HRESULT = 0xC00D11D8_u32 as _;
pub const NS_E_WMP_DRM_COMPONENT_FAILURE: windows_sys::core::HRESULT = 0xC00D11E2_u32 as _;
pub const NS_E_WMP_DRM_CORRUPT_BACKUP: windows_sys::core::HRESULT = 0xC00D11B4_u32 as _;
pub const NS_E_WMP_DRM_DRIVER_AUTH_FAILURE: windows_sys::core::HRESULT = 0xC00D11CA_u32 as _;
pub const NS_E_WMP_DRM_GENERIC_LICENSE_FAILURE: windows_sys::core::HRESULT = 0xC00D11DA_u32 as _;
pub const NS_E_WMP_DRM_INDIV_FAILED: windows_sys::core::HRESULT = 0xC00D11DD_u32 as _;
pub const NS_E_WMP_DRM_INVALID_SIG: windows_sys::core::HRESULT = 0xC00D11D7_u32 as _;
pub const NS_E_WMP_DRM_LICENSE_CONTENT_REVOKED: windows_sys::core::HRESULT = 0xC00D1207_u32 as _;
pub const NS_E_WMP_DRM_LICENSE_EXPIRED: windows_sys::core::HRESULT = 0xC00D1203_u32 as _;
pub const NS_E_WMP_DRM_LICENSE_NOSAP: windows_sys::core::HRESULT = 0xC00D1208_u32 as _;
pub const NS_E_WMP_DRM_LICENSE_NOTACQUIRED: windows_sys::core::HRESULT = 0xC00D1204_u32 as _;
pub const NS_E_WMP_DRM_LICENSE_NOTENABLED: windows_sys::core::HRESULT = 0xC00D1205_u32 as _;
pub const NS_E_WMP_DRM_LICENSE_SERVER_UNAVAILABLE: windows_sys::core::HRESULT = 0xC00D11B5_u32 as _;
pub const NS_E_WMP_DRM_LICENSE_UNUSABLE: windows_sys::core::HRESULT = 0xC00D1206_u32 as _;
pub const NS_E_WMP_DRM_NEEDS_AUTHORIZATION: windows_sys::core::HRESULT = 0xC00D11D0_u32 as _;
pub const NS_E_WMP_DRM_NEW_HARDWARE: windows_sys::core::HRESULT = 0xC00D11D6_u32 as _;
pub const NS_E_WMP_DRM_NOT_ACQUIRING: windows_sys::core::HRESULT = 0xC00D12C1_u32 as _;
pub const NS_E_WMP_DRM_NO_DEVICE_CERT: windows_sys::core::HRESULT = 0xC00D11E3_u32 as _;
pub const NS_E_WMP_DRM_NO_RIGHTS: windows_sys::core::HRESULT = 0xC00D11DC_u32 as _;
pub const NS_E_WMP_DRM_NO_SECURE_CLOCK: windows_sys::core::HRESULT = 0xC00D11DB_u32 as _;
pub const NS_E_WMP_DRM_UNABLE_TO_ACQUIRE_LICENSE: windows_sys::core::HRESULT = 0xC00D1209_u32 as _;
pub const NS_E_WMP_DSHOW_UNSUPPORTED_FORMAT: windows_sys::core::HRESULT = 0xC00D119A_u32 as _;
pub const NS_E_WMP_ERASE_FAILED: windows_sys::core::HRESULT = 0xC00D10D4_u32 as _;
pub const NS_E_WMP_EXTERNAL_NOTREADY: windows_sys::core::HRESULT = 0xC00D0FDC_u32 as _;
pub const NS_E_WMP_FAILED_TO_OPEN_IMAGE: windows_sys::core::HRESULT = 0xC00D1044_u32 as _;
pub const NS_E_WMP_FAILED_TO_OPEN_WMD: windows_sys::core::HRESULT = 0xC00D0FF2_u32 as _;
pub const NS_E_WMP_FAILED_TO_RIP_TRACK: windows_sys::core::HRESULT = 0xC00D10D3_u32 as _;
pub const NS_E_WMP_FAILED_TO_SAVE_FILE: windows_sys::core::HRESULT = 0xC00D0FEF_u32 as _;
pub const NS_E_WMP_FAILED_TO_SAVE_PLAYLIST: windows_sys::core::HRESULT = 0xC00D0FF1_u32 as _;
pub const NS_E_WMP_FILESCANALREADYSTARTED: windows_sys::core::HRESULT = 0xC00D0FBE_u32 as _;
pub const NS_E_WMP_FILE_DOES_NOT_FIT_ON_CD: windows_sys::core::HRESULT = 0xC00D10D8_u32 as _;
pub const NS_E_WMP_FILE_NO_DURATION: windows_sys::core::HRESULT = 0xC00D10D9_u32 as _;
pub const NS_E_WMP_FILE_OPEN_FAILED: windows_sys::core::HRESULT = 0xC00D11B1_u32 as _;
pub const NS_E_WMP_FILE_TYPE_CANNOT_BURN_TO_AUDIO_CD: windows_sys::core::HRESULT = 0xC00D10D7_u32 as _;
pub const NS_E_WMP_FORMAT_FAILED: windows_sys::core::HRESULT = 0xC00D10D5_u32 as _;
pub const NS_E_WMP_GIF_BAD_VERSION_NUMBER: windows_sys::core::HRESULT = 0xC00D1026_u32 as _;
pub const NS_E_WMP_GIF_INVALID_FORMAT: windows_sys::core::HRESULT = 0xC00D1025_u32 as _;
pub const NS_E_WMP_GIF_NO_IMAGE_IN_FILE: windows_sys::core::HRESULT = 0xC00D1027_u32 as _;
pub const NS_E_WMP_GIF_UNEXPECTED_ENDOFFILE: windows_sys::core::HRESULT = 0xC00D1024_u32 as _;
pub const NS_E_WMP_GOFULLSCREEN_FAILED: windows_sys::core::HRESULT = 0xC00D11BF_u32 as _;
pub const NS_E_WMP_HME_INVALIDOBJECTID: windows_sys::core::HRESULT = 0xC00D0FBF_u32 as _;
pub const NS_E_WMP_HME_NOTSEARCHABLEFORITEMS: windows_sys::core::HRESULT = 0xC00D0FC1_u32 as _;
pub const NS_E_WMP_HME_STALEREQUEST: windows_sys::core::HRESULT = 0xC00D0FC2_u32 as _;
pub const NS_E_WMP_HWND_NOTFOUND: windows_sys::core::HRESULT = 0xC00D125C_u32 as _;
pub const NS_E_WMP_IMAGE_FILETYPE_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D1022_u32 as _;
pub const NS_E_WMP_IMAGE_INVALID_FORMAT: windows_sys::core::HRESULT = 0xC00D1023_u32 as _;
pub const NS_E_WMP_IMAPI2_ERASE_DEVICE_BUSY: windows_sys::core::HRESULT = 0xC00D11E1_u32 as _;
pub const NS_E_WMP_IMAPI2_ERASE_FAIL: windows_sys::core::HRESULT = 0xC00D11E0_u32 as _;
pub const NS_E_WMP_IMAPI_DEVICE_BUSY: windows_sys::core::HRESULT = 0xC00D11AE_u32 as _;
pub const NS_E_WMP_IMAPI_DEVICE_INVALIDTYPE: windows_sys::core::HRESULT = 0xC00D11C9_u32 as _;
pub const NS_E_WMP_IMAPI_DEVICE_NOTPRESENT: windows_sys::core::HRESULT = 0xC00D11AD_u32 as _;
pub const NS_E_WMP_IMAPI_FAILURE: windows_sys::core::HRESULT = 0xC00D119F_u32 as _;
pub const NS_E_WMP_IMAPI_GENERIC: windows_sys::core::HRESULT = 0xC00D11AB_u32 as _;
pub const NS_E_WMP_IMAPI_LOSS_OF_STREAMING: windows_sys::core::HRESULT = 0xC00D11AF_u32 as _;
pub const NS_E_WMP_IMAPI_MEDIA_INCOMPATIBLE: windows_sys::core::HRESULT = 0xC00D11E6_u32 as _;
pub const NS_E_WMP_INVALID_ASX: windows_sys::core::HRESULT = 0xC00D119D_u32 as _;
pub const NS_E_WMP_INVALID_KEY: windows_sys::core::HRESULT = 0xC00D11CE_u32 as _;
pub const NS_E_WMP_INVALID_LIBRARY_ADD: windows_sys::core::HRESULT = 0xC00D11BC_u32 as _;
pub const NS_E_WMP_INVALID_MAX_VAL: windows_sys::core::HRESULT = 0xC00D1009_u32 as _;
pub const NS_E_WMP_INVALID_MIN_VAL: windows_sys::core::HRESULT = 0xC00D100A_u32 as _;
pub const NS_E_WMP_INVALID_PROTOCOL: windows_sys::core::HRESULT = 0xC00D11BB_u32 as _;
pub const NS_E_WMP_INVALID_REQUEST: windows_sys::core::HRESULT = 0xC00D11D4_u32 as _;
pub const NS_E_WMP_INVALID_SKIN: windows_sys::core::HRESULT = 0xC00D0FEC_u32 as _;
pub const NS_E_WMP_JPGTRANSPARENCY: windows_sys::core::HRESULT = 0xC00D1005_u32 as _;
pub const NS_E_WMP_JPG_BAD_DCTSIZE: windows_sys::core::HRESULT = 0xC00D1035_u32 as _;
pub const NS_E_WMP_JPG_BAD_PRECISION: windows_sys::core::HRESULT = 0xC00D1037_u32 as _;
pub const NS_E_WMP_JPG_BAD_VERSION_NUMBER: windows_sys::core::HRESULT = 0xC00D1036_u32 as _;
pub const NS_E_WMP_JPG_CCIR601_NOTIMPL: windows_sys::core::HRESULT = 0xC00D1038_u32 as _;
pub const NS_E_WMP_JPG_FRACT_SAMPLE_NOTIMPL: windows_sys::core::HRESULT = 0xC00D103B_u32 as _;
pub const NS_E_WMP_JPG_IMAGE_TOO_BIG: windows_sys::core::HRESULT = 0xC00D103C_u32 as _;
pub const NS_E_WMP_JPG_INVALID_FORMAT: windows_sys::core::HRESULT = 0xC00D1034_u32 as _;
pub const NS_E_WMP_JPG_JERR_ARITHCODING_NOTIMPL: windows_sys::core::HRESULT = 0xC00D1033_u32 as _;
pub const NS_E_WMP_JPG_NO_IMAGE_IN_FILE: windows_sys::core::HRESULT = 0xC00D1039_u32 as _;
pub const NS_E_WMP_JPG_READ_ERROR: windows_sys::core::HRESULT = 0xC00D103A_u32 as _;
pub const NS_E_WMP_JPG_SOF_UNSUPPORTED: windows_sys::core::HRESULT = 0xC00D103E_u32 as _;
pub const NS_E_WMP_JPG_UNEXPECTED_ENDOFFILE: windows_sys::core::HRESULT = 0xC00D103D_u32 as _;
pub const NS_E_WMP_JPG_UNKNOWN_MARKER: windows_sys::core::HRESULT = 0xC00D103F_u32 as _;
pub const NS_E_WMP_LICENSE_REQUIRED: windows_sys::core::HRESULT = 0xC00D120A_u32 as _;
pub const NS_E_WMP_LICENSE_RESTRICTS: windows_sys::core::HRESULT = 0xC00D11D3_u32 as _;
pub const NS_E_WMP_LOCKEDINSKINMODE: windows_sys::core::HRESULT = 0xC00D0FEE_u32 as _;
pub const NS_E_WMP_LOGON_FAILURE: windows_sys::core::HRESULT = 0xC00D1196_u32 as _;
pub const NS_E_WMP_MF_CODE_EXPIRED: windows_sys::core::HRESULT = 0xC00D0FC0_u32 as _;
pub const NS_E_WMP_MLS_STALE_DATA: windows_sys::core::HRESULT = 0xC00D0FDD_u32 as _;
pub const NS_E_WMP_MMS_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC00D11BD_u32 as _;
pub const NS_E_WMP_MSSAP_NOT_AVAILABLE: windows_sys::core::HRESULT = 0xC00D11A3_u32 as _;
pub const NS_E_WMP_MULTICAST_DISABLED: windows_sys::core::HRESULT = 0xC00D11C2_u32 as _;
pub const NS_E_WMP_MULTIPLE_ERROR_IN_PLAYLIST: windows_sys::core::HRESULT = 0xC00D11DF_u32 as _;
pub const NS_E_WMP_NEED_UPGRADE: windows_sys::core::HRESULT = 0xC00D11B9_u32 as _;
pub const NS_E_WMP_NETWORK_ERROR: windows_sys::core::HRESULT = 0xC00D11C0_u32 as _;
pub const NS_E_WMP_NETWORK_FIREWALL: windows_sys::core::HRESULT = 0xC00D11B6_u32 as _;
pub const NS_E_WMP_NETWORK_RESOURCE_FAILURE: windows_sys::core::HRESULT = 0xC00D11CB_u32 as _;
pub const NS_E_WMP_NONMEDIA_FILES: windows_sys::core::HRESULT = 0xC00D119C_u32 as _;
pub const NS_E_WMP_NO_DISK_SPACE: windows_sys::core::HRESULT = 0xC00D1195_u32 as _;
pub const NS_E_WMP_NO_PROTOCOLS_SELECTED: windows_sys::core::HRESULT = 0xC00D11BE_u32 as _;
pub const NS_E_WMP_NO_REMOVABLE_MEDIA: windows_sys::core::HRESULT = 0xC00D11B7_u32 as _;
pub const NS_E_WMP_OUTOFMEMORY: windows_sys::core::HRESULT = 0xC00D11C6_u32 as _;
pub const NS_E_WMP_PATH_ALREADY_IN_LIBRARY: windows_sys::core::HRESULT = 0xC00D0FBA_u32 as _;
pub const NS_E_WMP_PLAYLIST_EXISTS: windows_sys::core::HRESULT = 0xC00D119B_u32 as _;
pub const NS_E_WMP_PLUGINDLL_NOTFOUND: windows_sys::core::HRESULT = 0xC00D0FD9_u32 as _;
pub const NS_E_WMP_PNG_INVALIDFORMAT: windows_sys::core::HRESULT = 0xC00D1028_u32 as _;
pub const NS_E_WMP_PNG_UNSUPPORTED_BAD_CRC: windows_sys::core::HRESULT = 0xC00D102D_u32 as _;
pub const NS_E_WMP_PNG_UNSUPPORTED_BITDEPTH: windows_sys::core::HRESULT = 0xC00D1029_u32 as _;
pub const NS_E_WMP_PNG_UNSUPPORTED_COMPRESSION: windows_sys::core::HRESULT = 0xC00D102A_u32 as _;
pub const NS_E_WMP_PNG_UNSUPPORTED_FILTER: windows_sys::core::HRESULT = 0xC00D102B_u32 as _;
pub const NS_E_WMP_PNG_UNSUPPORTED_INTERLACE: windows_sys::core::HRESULT = 0xC00D102C_u32 as _;
pub const NS_E_WMP_POLICY_VALUE_NOT_CONFIGURED: windows_sys::core::HRESULT = 0xC00D122A_u32 as _;
pub const NS_E_WMP_PROTECTED_CONTENT: windows_sys::core::HRESULT = 0xC00D120B_u32 as _;
pub const NS_E_WMP_PROTOCOL_PROBLEM: windows_sys::core::HRESULT = 0xC00D1194_u32 as _;
pub const NS_E_WMP_PROXY_CONNECT_TIMEOUT: windows_sys::core::HRESULT = 0xC00D11B8_u32 as _;
pub const NS_E_WMP_PROXY_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D11C4_u32 as _;
pub const NS_E_WMP_RBC_JPGMAPPINGIMAGE: windows_sys::core::HRESULT = 0xC00D1004_u32 as _;
pub const NS_E_WMP_RECORDING_NOT_ALLOWED: windows_sys::core::HRESULT = 0xC00D0FC9_u32 as _;
pub const NS_E_WMP_RIP_FAILED: windows_sys::core::HRESULT = 0xC00D10D2_u32 as _;
pub const NS_E_WMP_SAVEAS_READONLY: windows_sys::core::HRESULT = 0xC00D0FF0_u32 as _;
pub const NS_E_WMP_SENDMAILFAILED: windows_sys::core::HRESULT = 0xC00D0FED_u32 as _;
pub const NS_E_WMP_SERVER_DNS_TIMEOUT: windows_sys::core::HRESULT = 0xC00D11C3_u32 as _;
pub const NS_E_WMP_SERVER_INACCESSIBLE: windows_sys::core::HRESULT = 0xC00D1198_u32 as _;
pub const NS_E_WMP_SERVER_NONEWCONNECTIONS: windows_sys::core::HRESULT = 0xC00D11DE_u32 as _;
pub const NS_E_WMP_SERVER_NOT_RESPONDING: windows_sys::core::HRESULT = 0xC00D11B3_u32 as _;
pub const NS_E_WMP_SERVER_SECURITY_ERROR: windows_sys::core::HRESULT = 0xC00D11E4_u32 as _;
pub const NS_E_WMP_SERVER_UNAVAILABLE: windows_sys::core::HRESULT = 0xC00D11B0_u32 as _;
pub const NS_E_WMP_STREAMING_RECORDING_NOT_ALLOWED: windows_sys::core::HRESULT = 0xC00D0FD8_u32 as _;
pub const NS_E_WMP_TAMPERED_CONTENT: windows_sys::core::HRESULT = 0xC00D11C5_u32 as _;
pub const NS_E_WMP_UDRM_NOUSERLIST: windows_sys::core::HRESULT = 0xC00D12C0_u32 as _;
pub const NS_E_WMP_UI_NOSKININZIP: windows_sys::core::HRESULT = 0xC00D0FE7_u32 as _;
pub const NS_E_WMP_UI_NOTATHEMEFILE: windows_sys::core::HRESULT = 0xC00D0FE0_u32 as _;
pub const NS_E_WMP_UI_OBJECTNOTFOUND: windows_sys::core::HRESULT = 0xC00D0FE5_u32 as _;
pub const NS_E_WMP_UI_PASSTHROUGH: windows_sys::core::HRESULT = 0xC00D0FE4_u32 as _;
pub const NS_E_WMP_UI_SECONDHANDLER: windows_sys::core::HRESULT = 0xC00D0FE6_u32 as _;
pub const NS_E_WMP_UI_SUBCONTROLSNOTSUPPORTED: windows_sys::core::HRESULT = 0xC00D0FDE_u32 as _;
pub const NS_E_WMP_UI_SUBELEMENTNOTFOUND: windows_sys::core::HRESULT = 0xC00D0FE1_u32 as _;
pub const NS_E_WMP_UI_VERSIONMISMATCH: windows_sys::core::HRESULT = 0xC00D0FDF_u32 as _;
pub const NS_E_WMP_UI_VERSIONPARSE: windows_sys::core::HRESULT = 0xC00D0FE2_u32 as _;
pub const NS_E_WMP_UI_VIEWIDNOTFOUND: windows_sys::core::HRESULT = 0xC00D0FE3_u32 as _;
pub const NS_E_WMP_UNKNOWN_ERROR: windows_sys::core::HRESULT = 0xC00D11CD_u32 as _;
pub const NS_E_WMP_UNSUPPORTED_FORMAT: windows_sys::core::HRESULT = 0xC00D1199_u32 as _;
pub const NS_E_WMP_UPGRADE_APPLICATION: windows_sys::core::HRESULT = 0xC00D11CC_u32 as _;
pub const NS_E_WMP_URLDOWNLOADFAILED: windows_sys::core::HRESULT = 0xC00D0FEA_u32 as _;
pub const NS_E_WMP_VERIFY_ONLINE: windows_sys::core::HRESULT = 0xC00D11B2_u32 as _;
pub const NS_E_WMP_VIDEO_CODEC_NOT_INSTALLED: windows_sys::core::HRESULT = 0xC00D11C8_u32 as _;
pub const NS_E_WMP_WINDOWSAPIFAILURE: windows_sys::core::HRESULT = 0xC00D0FC8_u32 as _;
pub const NS_E_WMP_WMDM_BUSY: windows_sys::core::HRESULT = 0xC00D11A8_u32 as _;
pub const NS_E_WMP_WMDM_FAILURE: windows_sys::core::HRESULT = 0xC00D11A0_u32 as _;
pub const NS_E_WMP_WMDM_INCORRECT_RIGHTS: windows_sys::core::HRESULT = 0xC00D11AA_u32 as _;
pub const NS_E_WMP_WMDM_INTERFACEDEAD: windows_sys::core::HRESULT = 0xC00D11A4_u32 as _;
pub const NS_E_WMP_WMDM_LICENSE_EXPIRED: windows_sys::core::HRESULT = 0xC00D11A7_u32 as _;
pub const NS_E_WMP_WMDM_LICENSE_NOTEXIST: windows_sys::core::HRESULT = 0xC00D11A6_u32 as _;
pub const NS_E_WMP_WMDM_NORIGHTS: windows_sys::core::HRESULT = 0xC00D11A9_u32 as _;
pub const NS_E_WMP_WMDM_NOTCERTIFIED: windows_sys::core::HRESULT = 0xC00D11A5_u32 as _;
pub const NS_E_WMR_CANNOT_RENDER_BINARY_STREAM: windows_sys::core::HRESULT = 0xC00D1063_u32 as _;
pub const NS_E_WMR_NOCALLBACKAVAILABLE: windows_sys::core::HRESULT = 0xC00D105E_u32 as _;
pub const NS_E_WMR_NOSOURCEFILTER: windows_sys::core::HRESULT = 0xC00D105C_u32 as _;
pub const NS_E_WMR_PINNOTFOUND: windows_sys::core::HRESULT = 0xC00D105A_u32 as _;
pub const NS_E_WMR_PINTYPENOMATCH: windows_sys::core::HRESULT = 0xC00D105D_u32 as _;
pub const NS_E_WMR_SAMPLEPROPERTYNOTSET: windows_sys::core::HRESULT = 0xC00D1062_u32 as _;
pub const NS_E_WMR_UNSUPPORTEDSTREAM: windows_sys::core::HRESULT = 0xC00D1059_u32 as _;
pub const NS_E_WMR_WAITINGONFORMATSWITCH: windows_sys::core::HRESULT = 0xC00D105B_u32 as _;
pub const NS_E_WMR_WILLNOT_RENDER_BINARY_STREAM: windows_sys::core::HRESULT = 0xC00D1065_u32 as _;
pub const NS_E_WMX_ATTRIBUTE_ALREADY_EXISTS: windows_sys::core::HRESULT = 0xC00D106F_u32 as _;
pub const NS_E_WMX_ATTRIBUTE_DOES_NOT_EXIST: windows_sys::core::HRESULT = 0xC00D106E_u32 as _;
pub const NS_E_WMX_ATTRIBUTE_UNRETRIEVABLE: windows_sys::core::HRESULT = 0xC00D1070_u32 as _;
pub const NS_E_WMX_INVALID_FORMAT_OVER_NESTING: windows_sys::core::HRESULT = 0xC00D1076_u32 as _;
pub const NS_E_WMX_ITEM_DOES_NOT_EXIST: windows_sys::core::HRESULT = 0xC00D1071_u32 as _;
pub const NS_E_WMX_ITEM_TYPE_ILLEGAL: windows_sys::core::HRESULT = 0xC00D1072_u32 as _;
pub const NS_E_WMX_ITEM_UNSETTABLE: windows_sys::core::HRESULT = 0xC00D1073_u32 as _;
pub const NS_E_WMX_PLAYLIST_EMPTY: windows_sys::core::HRESULT = 0xC00D1074_u32 as _;
pub const NS_E_WMX_UNRECOGNIZED_PLAYLIST_FORMAT: windows_sys::core::HRESULT = 0xC00D1068_u32 as _;
pub const NS_E_WONT_DO_DIGITAL: windows_sys::core::HRESULT = 0xC00D0FB3_u32 as _;
pub const NS_E_WRONG_OS_VERSION: windows_sys::core::HRESULT = 0xC00D145D_u32 as _;
pub const NS_E_WRONG_PUBLISHING_POINT_TYPE: windows_sys::core::HRESULT = 0xC00D1452_u32 as _;
pub const NS_E_WSX_INVALID_VERSION: windows_sys::core::HRESULT = 0xC00D151E_u32 as _;
pub const NS_I_CATATONIC_AUTO_UNFAIL: windows_sys::core::HRESULT = 0x800D019A_u32 as _;
pub const NS_I_CATATONIC_FAILURE: windows_sys::core::HRESULT = 0x800D0199_u32 as _;
pub const NS_I_CUB_RUNNING: windows_sys::core::HRESULT = 0x400D0052_u32 as _;
pub const NS_I_CUB_START: windows_sys::core::HRESULT = 0x400D0051_u32 as _;
pub const NS_I_CUB_UNFAIL_LINK: windows_sys::core::HRESULT = 0x400D0191_u32 as _;
pub const NS_I_DISK_REBUILD_ABORTED: windows_sys::core::HRESULT = 0x400D0058_u32 as _;
pub const NS_I_DISK_REBUILD_FINISHED: windows_sys::core::HRESULT = 0x400D0057_u32 as _;
pub const NS_I_DISK_REBUILD_STARTED: windows_sys::core::HRESULT = 0x400D0056_u32 as _;
pub const NS_I_DISK_START: windows_sys::core::HRESULT = 0x400D0054_u32 as _;
pub const NS_I_DISK_STOP: windows_sys::core::HRESULT = 0x400D0198_u32 as _;
pub const NS_I_EXISTING_PACKETIZER: windows_sys::core::HRESULT = 0x400D2F03_u32 as _;
pub const NS_I_KILL_CONNECTION: windows_sys::core::HRESULT = 0x400D005E_u32 as _;
pub const NS_I_KILL_USERSESSION: windows_sys::core::HRESULT = 0x400D005D_u32 as _;
pub const NS_I_LIMIT_BANDWIDTH: windows_sys::core::HRESULT = 0x400D0070_u32 as _;
pub const NS_I_LIMIT_FUNNELS: windows_sys::core::HRESULT = 0x400D0059_u32 as _;
pub const NS_I_LOGGING_FAILED: windows_sys::core::HRESULT = 0x400D006E_u32 as _;
pub const NS_I_MANUAL_PROXY: windows_sys::core::HRESULT = 0x400D2F04_u32 as _;
pub const NS_I_NOLOG_STOP: windows_sys::core::HRESULT = 0x400D2F01_u32 as _;
pub const NS_I_PLAYLIST_CHANGE_RECEDING: windows_sys::core::HRESULT = 0x400D14BE_u32 as _;
pub const NS_I_REBUILD_DISK: windows_sys::core::HRESULT = 0x400D005F_u32 as _;
pub const NS_I_RECONNECTED: windows_sys::core::HRESULT = 0x400D2EFF_u32 as _;
pub const NS_I_RESTRIPE_CUB_OUT: windows_sys::core::HRESULT = 0x400D0197_u32 as _;
pub const NS_I_RESTRIPE_DISK_OUT: windows_sys::core::HRESULT = 0x400D0196_u32 as _;
pub const NS_I_RESTRIPE_DONE: windows_sys::core::HRESULT = 0x400D0194_u32 as _;
pub const NS_I_RESTRIPE_START: windows_sys::core::HRESULT = 0x400D0193_u32 as _;
pub const NS_I_START_DISK: windows_sys::core::HRESULT = 0x400D005A_u32 as _;
pub const NS_I_STOP_CUB: windows_sys::core::HRESULT = 0x400D005C_u32 as _;
pub const NS_I_STOP_DISK: windows_sys::core::HRESULT = 0x400D005B_u32 as _;
pub const NS_I_TIGER_START: windows_sys::core::HRESULT = 0x400D004F_u32 as _;
pub const NS_S_CALLABORTED: windows_sys::core::HRESULT = 0xD0001_u32 as _;
pub const NS_S_CALLPENDING: windows_sys::core::HRESULT = 0xD0000_u32 as _;
pub const NS_S_CHANGENOTICE: windows_sys::core::HRESULT = 0xD2F0D_u32 as _;
pub const NS_S_DEGRADING_QUALITY: windows_sys::core::HRESULT = 0xD0BC9_u32 as _;
pub const NS_S_DRM_ACQUIRE_CANCELLED: windows_sys::core::HRESULT = 0xD2747_u32 as _;
pub const NS_S_DRM_BURNABLE_TRACK: windows_sys::core::HRESULT = 0xD276E_u32 as _;
pub const NS_S_DRM_BURNABLE_TRACK_WITH_PLAYLIST_RESTRICTION: windows_sys::core::HRESULT = 0xD276F_u32 as _;
pub const NS_S_DRM_INDIVIDUALIZED: windows_sys::core::HRESULT = 0xD2727_u32 as _;
pub const NS_S_DRM_LICENSE_ACQUIRED: windows_sys::core::HRESULT = 0xD2726_u32 as _;
pub const NS_S_DRM_MONITOR_CANCELLED: windows_sys::core::HRESULT = 0xD2746_u32 as _;
pub const NS_S_DRM_NEEDS_INDIVIDUALIZATION: windows_sys::core::HRESULT = 0xD27DE_u32 as _;
pub const NS_S_EOSRECEDING: windows_sys::core::HRESULT = 0xD2F09_u32 as _;
pub const NS_S_NAVIGATION_COMPLETE_WITH_ERRORS: windows_sys::core::HRESULT = 0xD135E_u32 as _;
pub const NS_S_NEED_TO_BUY_BURN_RIGHTS: windows_sys::core::HRESULT = 0xD10DB_u32 as _;
pub const NS_S_OPERATION_PENDING: windows_sys::core::HRESULT = 0xD114E_u32 as _;
pub const NS_S_PUBLISHING_POINT_STARTED_WITH_FAILED_SINKS: windows_sys::core::HRESULT = 0xD1519_u32 as _;
pub const NS_S_REBOOT_RECOMMENDED: windows_sys::core::HRESULT = 0xD2AF8_u32 as _;
pub const NS_S_REBOOT_REQUIRED: windows_sys::core::HRESULT = 0xD2AF9_u32 as _;
pub const NS_S_REBUFFERING: windows_sys::core::HRESULT = 0xD0BC8_u32 as _;
pub const NS_S_STREAM_TRUNCATED: windows_sys::core::HRESULT = 0xD0002_u32 as _;
pub const NS_S_TRACK_ALREADY_DOWNLOADED: windows_sys::core::HRESULT = 0xD1361_u32 as _;
pub const NS_S_TRACK_BUY_REQUIRES_ALBUM_PURCHASE: windows_sys::core::HRESULT = 0xD1359_u32 as _;
pub const NS_S_TRANSCRYPTOR_EOF: windows_sys::core::HRESULT = 0xD0BDB_u32 as _;
pub const NS_S_WMG_ADVISE_DROP_FRAME: windows_sys::core::HRESULT = 0xD1066_u32 as _;
pub const NS_S_WMG_ADVISE_DROP_TO_KEYFRAME: windows_sys::core::HRESULT = 0xD1067_u32 as _;
pub const NS_S_WMG_FORCE_DROP_FRAME: windows_sys::core::HRESULT = 0xD104F_u32 as _;
pub const NS_S_WMPBR_PARTIALSUCCESS: windows_sys::core::HRESULT = 0xD1136_u32 as _;
pub const NS_S_WMPBR_SUCCESS: windows_sys::core::HRESULT = 0xD1135_u32 as _;
pub const NS_S_WMPCORE_COMMAND_NOT_AVAILABLE: windows_sys::core::HRESULT = 0xD1105_u32 as _;
pub const NS_S_WMPCORE_MEDIA_CHILD_PLAYLIST_OPEN_PENDING: windows_sys::core::HRESULT = 0xD1109_u32 as _;
pub const NS_S_WMPCORE_MEDIA_VALIDATION_PENDING: windows_sys::core::HRESULT = 0xD1103_u32 as _;
pub const NS_S_WMPCORE_MORE_NODES_AVAIABLE: windows_sys::core::HRESULT = 0xD110A_u32 as _;
pub const NS_S_WMPCORE_PLAYLISTCLEARABORT: windows_sys::core::HRESULT = 0xD10FE_u32 as _;
pub const NS_S_WMPCORE_PLAYLISTREMOVEITEMABORT: windows_sys::core::HRESULT = 0xD10FF_u32 as _;
pub const NS_S_WMPCORE_PLAYLIST_COLLAPSED_TO_SINGLE_MEDIA: windows_sys::core::HRESULT = 0xD1108_u32 as _;
pub const NS_S_WMPCORE_PLAYLIST_CREATION_PENDING: windows_sys::core::HRESULT = 0xD1102_u32 as _;
pub const NS_S_WMPCORE_PLAYLIST_IMPORT_MISSING_ITEMS: windows_sys::core::HRESULT = 0xD1107_u32 as _;
pub const NS_S_WMPCORE_PLAYLIST_NAME_AUTO_GENERATED: windows_sys::core::HRESULT = 0xD1106_u32 as _;
pub const NS_S_WMPCORE_PLAYLIST_REPEAT_SECONDARY_SEGMENTS_IGNORED: windows_sys::core::HRESULT = 0xD1104_u32 as _;
pub const NS_S_WMPEFFECT_OPAQUE: windows_sys::core::HRESULT = 0xD1145_u32 as _;
pub const NS_S_WMPEFFECT_TRANSPARENT: windows_sys::core::HRESULT = 0xD1144_u32 as _;
pub const NS_S_WMP_EXCEPTION: windows_sys::core::HRESULT = 0xD0FE9_u32 as _;
pub const NS_S_WMP_LOADED_BMP_IMAGE: windows_sys::core::HRESULT = 0xD1042_u32 as _;
pub const NS_S_WMP_LOADED_GIF_IMAGE: windows_sys::core::HRESULT = 0xD1040_u32 as _;
pub const NS_S_WMP_LOADED_JPG_IMAGE: windows_sys::core::HRESULT = 0xD1043_u32 as _;
pub const NS_S_WMP_LOADED_PNG_IMAGE: windows_sys::core::HRESULT = 0xD1041_u32 as _;
pub const NS_S_WMP_UI_VERSIONMISMATCH: windows_sys::core::HRESULT = 0xD0FE8_u32 as _;
pub const NS_S_WMR_ALREADYRENDERED: windows_sys::core::HRESULT = 0xD105F_u32 as _;
pub const NS_S_WMR_PINTYPEFULLMATCH: windows_sys::core::HRESULT = 0xD1061_u32 as _;
pub const NS_S_WMR_PINTYPEPARTIALMATCH: windows_sys::core::HRESULT = 0xD1060_u32 as _;
pub const NS_W_FILE_BANDWIDTH_LIMIT: windows_sys::core::HRESULT = 0x800D0004_u32 as _;
pub const NS_W_SERVER_BANDWIDTH_LIMIT: windows_sys::core::HRESULT = 0x800D0003_u32 as _;
pub const NS_W_UNKNOWN_EVENT: windows_sys::core::HRESULT = 0x800D0060_u32 as _;
pub const PD_CAN_DRAW_DIB: u32 = 1u32;
pub const PD_CAN_STRETCHDIB: u32 = 2u32;
pub const PD_STRETCHDIB_1_1_OK: u32 = 4u32;
pub const PD_STRETCHDIB_1_2_OK: u32 = 8u32;
pub const PD_STRETCHDIB_1_N_OK: u32 = 16u32;
pub const ROCKWELL_WA1_MIXER: u32 = 103u32;
pub const ROCKWELL_WA1_MPU401_IN: u32 = 104u32;
pub const ROCKWELL_WA1_MPU401_OUT: u32 = 105u32;
pub const ROCKWELL_WA1_SYNTH: u32 = 102u32;
pub const ROCKWELL_WA1_WAVEIN: u32 = 100u32;
pub const ROCKWELL_WA1_WAVEOUT: u32 = 101u32;
pub const ROCKWELL_WA2_MIXER: u32 = 203u32;
pub const ROCKWELL_WA2_MPU401_IN: u32 = 204u32;
pub const ROCKWELL_WA2_MPU401_OUT: u32 = 205u32;
pub const ROCKWELL_WA2_SYNTH: u32 = 202u32;
pub const ROCKWELL_WA2_WAVEIN: u32 = 200u32;
pub const ROCKWELL_WA2_WAVEOUT: u32 = 201u32;
pub const SEARCH_ANY: i32 = 32i32;
pub const SEARCH_BACKWARD: i32 = 4i32;
pub const SEARCH_FORWARD: i32 = 1i32;
pub const SEARCH_KEY: i32 = 16i32;
pub const SEARCH_NEAREST: i32 = 4i32;
pub const SEEK_CUR: u32 = 1u32;
pub const SEEK_END: u32 = 2u32;
pub const SEEK_SET: u32 = 0u32;
pub const TARGET_DEVICE_FRIENDLY_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("TargetDeviceFriendlyName");
pub const TARGET_DEVICE_OPEN_EXCLUSIVELY: windows_sys::core::PCSTR = windows_sys::core::s!("TargetDeviceOpenExclusively");
pub const TASKERR_NOTASKSUPPORT: u32 = 1u32;
pub const TASKERR_OUTOFMEMORY: u32 = 2u32;
pub const TDD_BEGINMINPERIOD: u32 = 2064u32;
pub const TDD_ENDMINPERIOD: u32 = 2068u32;
pub const TDD_GETDEVCAPS: u32 = 2060u32;
pub const TDD_GETSYSTEMTIME: u32 = 2056u32;
pub const TDD_KILLTIMEREVENT: u32 = 2048u32;
pub const TDD_SETTIMEREVENT: u32 = 2052u32;
pub const VADMAD_Device_ID: u32 = 1092u32;
pub const VCAPS_CAN_SCALE: u32 = 8u32;
pub const VCAPS_DST_CAN_CLIP: u32 = 4u32;
pub const VCAPS_OVERLAY: u32 = 1u32;
pub const VCAPS_SRC_CAN_CLIP: u32 = 2u32;
pub const VFW_HIDE_CAMERACONTROL_PAGE: u32 = 4u32;
pub const VFW_HIDE_SETTINGS_PAGE: u32 = 1u32;
pub const VFW_HIDE_VIDEOSRC_PAGE: u32 = 2u32;
pub const VFW_OEM_ADD_PAGE: u32 = 2147483648u32;
pub const VFW_QUERY_DEV_CHANGED: u32 = 256u32;
pub const VFW_USE_DEVICE_HANDLE: u32 = 1u32;
pub const VFW_USE_STREAM_HANDLE: u32 = 2u32;
pub const VHDR_DONE: u32 = 1u32;
pub const VHDR_INQUEUE: u32 = 4u32;
pub const VHDR_KEYFRAME: u32 = 8u32;
pub const VHDR_PREPARED: u32 = 2u32;
pub const VHDR_VALID: u32 = 15u32;
pub const VIDCF_COMPRESSFRAMES: u32 = 8u32;
pub const VIDCF_CRUNCH: u32 = 2u32;
pub const VIDCF_DRAW: u32 = 16u32;
pub const VIDCF_FASTTEMPORALC: u32 = 32u32;
pub const VIDCF_FASTTEMPORALD: u32 = 128u32;
pub const VIDCF_QUALITY: u32 = 1u32;
pub const VIDCF_TEMPORAL: u32 = 4u32;
pub const VIDEO_CONFIGURE_CURRENT: u32 = 16u32;
pub const VIDEO_CONFIGURE_GET: u32 = 8192u32;
pub const VIDEO_CONFIGURE_MAX: u32 = 128u32;
pub const VIDEO_CONFIGURE_MIN: u32 = 64u32;
pub const VIDEO_CONFIGURE_NOMINAL: u32 = 32u32;
pub const VIDEO_CONFIGURE_QUERY: u32 = 32768u32;
pub const VIDEO_CONFIGURE_QUERYSIZE: u32 = 1u32;
pub const VIDEO_CONFIGURE_SET: u32 = 4096u32;
pub const VIDEO_DLG_QUERY: u32 = 16u32;
pub const VIDEO_EXTERNALIN: u32 = 1u32;
pub const VIDEO_EXTERNALOUT: u32 = 2u32;
pub const VIDEO_IN: u32 = 4u32;
pub const VIDEO_OUT: u32 = 8u32;
pub const VP_COMMAND_GET: u32 = 1u32;
pub const VP_COMMAND_SET: u32 = 2u32;
pub const VP_CP_CMD_ACTIVATE: u32 = 1u32;
pub const VP_CP_CMD_CHANGE: u32 = 4u32;
pub const VP_CP_CMD_DEACTIVATE: u32 = 2u32;
pub const VP_CP_TYPE_APS_TRIGGER: u32 = 1u32;
pub const VP_CP_TYPE_MACROVISION: u32 = 2u32;
pub const VP_FLAGS_BRIGHTNESS: u32 = 64u32;
pub const VP_FLAGS_CONTRAST: u32 = 128u32;
pub const VP_FLAGS_COPYPROTECT: u32 = 256u32;
pub const VP_FLAGS_FLICKER: u32 = 4u32;
pub const VP_FLAGS_MAX_UNSCALED: u32 = 16u32;
pub const VP_FLAGS_OVERSCAN: u32 = 8u32;
pub const VP_FLAGS_POSITION: u32 = 32u32;
pub const VP_FLAGS_TV_MODE: u32 = 1u32;
pub const VP_FLAGS_TV_STANDARD: u32 = 2u32;
pub const VP_MODE_TV_PLAYBACK: u32 = 2u32;
pub const VP_MODE_WIN_GRAPHICS: u32 = 1u32;
pub const VP_TV_STANDARD_NTSC_433: u32 = 65536u32;
pub const VP_TV_STANDARD_NTSC_M: u32 = 1u32;
pub const VP_TV_STANDARD_NTSC_M_J: u32 = 2u32;
pub const VP_TV_STANDARD_PAL_60: u32 = 262144u32;
pub const VP_TV_STANDARD_PAL_B: u32 = 4u32;
pub const VP_TV_STANDARD_PAL_D: u32 = 8u32;
pub const VP_TV_STANDARD_PAL_G: u32 = 131072u32;
pub const VP_TV_STANDARD_PAL_H: u32 = 16u32;
pub const VP_TV_STANDARD_PAL_I: u32 = 32u32;
pub const VP_TV_STANDARD_PAL_M: u32 = 64u32;
pub const VP_TV_STANDARD_PAL_N: u32 = 128u32;
pub const VP_TV_STANDARD_SECAM_B: u32 = 256u32;
pub const VP_TV_STANDARD_SECAM_D: u32 = 512u32;
pub const VP_TV_STANDARD_SECAM_G: u32 = 1024u32;
pub const VP_TV_STANDARD_SECAM_H: u32 = 2048u32;
pub const VP_TV_STANDARD_SECAM_K: u32 = 4096u32;
pub const VP_TV_STANDARD_SECAM_K1: u32 = 8192u32;
pub const VP_TV_STANDARD_SECAM_L: u32 = 16384u32;
pub const VP_TV_STANDARD_SECAM_L1: u32 = 524288u32;
pub const VP_TV_STANDARD_WIN_VGA: u32 = 32768u32;
pub const WAVE_FILTER_DEVELOPMENT: u32 = 65535u32;
pub const WAVE_FILTER_ECHO: u32 = 2u32;
pub const WAVE_FILTER_UNKNOWN: u32 = 0u32;
pub const WAVE_FILTER_VOLUME: u32 = 1u32;
pub const WAVE_FORMAT_3COM_NBX: u32 = 28672u32;
pub const WAVE_FORMAT_ADPCM: u32 = 2u32;
pub const WAVE_FORMAT_ALAC: u32 = 27745u32;
pub const WAVE_FORMAT_ALAW: u32 = 6u32;
pub const WAVE_FORMAT_AMR_NB: u32 = 29537u32;
pub const WAVE_FORMAT_AMR_WB: u32 = 29538u32;
pub const WAVE_FORMAT_AMR_WP: u32 = 29539u32;
pub const WAVE_FORMAT_ANTEX_ADPCME: u32 = 51u32;
pub const WAVE_FORMAT_APTX: u32 = 37u32;
pub const WAVE_FORMAT_AUDIOFILE_AF10: u32 = 38u32;
pub const WAVE_FORMAT_AUDIOFILE_AF36: u32 = 36u32;
pub const WAVE_FORMAT_BTV_DIGITAL: u32 = 1024u32;
pub const WAVE_FORMAT_CANOPUS_ATRAC: u32 = 99u32;
pub const WAVE_FORMAT_CIRRUS: u32 = 96u32;
pub const WAVE_FORMAT_CODIAN: u32 = 41252u32;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_AVQSBC: u32 = 41217u32;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_G723_1: u32 = 41216u32;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_SBC: u32 = 41218u32;
pub const WAVE_FORMAT_CONGRUENCY: u32 = 141u32;
pub const WAVE_FORMAT_CONTROL_RES_CR10: u32 = 55u32;
pub const WAVE_FORMAT_CONTROL_RES_VQLPC: u32 = 52u32;
pub const WAVE_FORMAT_CONVEDIA_G729: u32 = 140u32;
pub const WAVE_FORMAT_CREATIVE_ADPCM: u32 = 512u32;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH10: u32 = 515u32;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH8: u32 = 514u32;
pub const WAVE_FORMAT_CS2: u32 = 608u32;
pub const WAVE_FORMAT_CS_IMAADPCM: u32 = 57u32;
pub const WAVE_FORMAT_CUSEEME: u32 = 7939u32;
pub const WAVE_FORMAT_CU_CODEC: u32 = 25u32;
pub const WAVE_FORMAT_DEVELOPMENT: u32 = 65535u32;
pub const WAVE_FORMAT_DF_G726: u32 = 133u32;
pub const WAVE_FORMAT_DF_GSM610: u32 = 134u32;
pub const WAVE_FORMAT_DIALOGIC_OKI_ADPCM: u32 = 23u32;
pub const WAVE_FORMAT_DICTAPHONE_CELP54: u32 = 322u32;
pub const WAVE_FORMAT_DICTAPHONE_CELP68: u32 = 321u32;
pub const WAVE_FORMAT_DIGIADPCM: u32 = 54u32;
pub const WAVE_FORMAT_DIGIFIX: u32 = 22u32;
pub const WAVE_FORMAT_DIGIREAL: u32 = 53u32;
pub const WAVE_FORMAT_DIGISTD: u32 = 21u32;
pub const WAVE_FORMAT_DIGITAL_G723: u32 = 291u32;
pub const WAVE_FORMAT_DIVIO_G726: u32 = 16963u32;
pub const WAVE_FORMAT_DIVIO_MPEG4_AAC: u32 = 16707u32;
pub const WAVE_FORMAT_DOLBY_AC2: u32 = 48u32;
pub const WAVE_FORMAT_DOLBY_AC3_SPDIF: u32 = 146u32;
pub const WAVE_FORMAT_DOLBY_AC4: u32 = 44096u32;
pub const WAVE_FORMAT_DRM: u32 = 9u32;
pub const WAVE_FORMAT_DSAT: u32 = 102u32;
pub const WAVE_FORMAT_DSAT_DISPLAY: u32 = 103u32;
pub const WAVE_FORMAT_DSPGROUP_TRUESPEECH: u32 = 34u32;
pub const WAVE_FORMAT_DTS: u32 = 8u32;
pub const WAVE_FORMAT_DTS2: u32 = 8193u32;
pub const WAVE_FORMAT_DTS_DS: u32 = 400u32;
pub const WAVE_FORMAT_DVI_ADPCM: u32 = 17u32;
pub const WAVE_FORMAT_DVM: u32 = 8192u32;
pub const WAVE_FORMAT_ECHOSC1: u32 = 35u32;
pub const WAVE_FORMAT_ECHOSC3: u32 = 58u32;
pub const WAVE_FORMAT_ENCORE_G726: u32 = 41223u32;
pub const WAVE_FORMAT_ESPCM: u32 = 97u32;
pub const WAVE_FORMAT_ESST_AC3: u32 = 577u32;
pub const WAVE_FORMAT_FAAD_AAC: u32 = 28781u32;
pub const WAVE_FORMAT_FLAC: u32 = 61868u32;
pub const WAVE_FORMAT_FM_TOWNS_SND: u32 = 768u32;
pub const WAVE_FORMAT_FRACE_TELECOM_G729: u32 = 41251u32;
pub const WAVE_FORMAT_FRAUNHOFER_IIS_MPEG2_AAC: u32 = 384u32;
pub const WAVE_FORMAT_G721_ADPCM: u32 = 64u32;
pub const WAVE_FORMAT_G722_ADPCM: u32 = 101u32;
pub const WAVE_FORMAT_G723_ADPCM: u32 = 20u32;
pub const WAVE_FORMAT_G726ADPCM: u32 = 320u32;
pub const WAVE_FORMAT_G726_ADPCM: u32 = 100u32;
pub const WAVE_FORMAT_G728_CELP: u32 = 65u32;
pub const WAVE_FORMAT_G729A: u32 = 131u32;
pub const WAVE_FORMAT_GENERIC_PASSTHRU: u32 = 585u32;
pub const WAVE_FORMAT_GLOBAL_IP_ILBC: u32 = 41238u32;
pub const WAVE_FORMAT_GSM610: u32 = 49u32;
pub const WAVE_FORMAT_GSM_610: u32 = 41229u32;
pub const WAVE_FORMAT_GSM_620: u32 = 41230u32;
pub const WAVE_FORMAT_GSM_660: u32 = 41231u32;
pub const WAVE_FORMAT_GSM_690: u32 = 41232u32;
pub const WAVE_FORMAT_GSM_ADAPTIVE_MULTIRATE_WB: u32 = 41233u32;
pub const WAVE_FORMAT_GSM_AMR_CBR: u32 = 31265u32;
pub const WAVE_FORMAT_GSM_AMR_VBR_SID: u32 = 31266u32;
pub const WAVE_FORMAT_HP_DYN_VOICE: u32 = 26u32;
pub const WAVE_FORMAT_IBM_CVSD: u32 = 5u32;
pub const WAVE_FORMAT_IEEE_FLOAT: u32 = 3u32;
pub const WAVE_FORMAT_ILINK_VC: u32 = 560u32;
pub const WAVE_FORMAT_IMA_ADPCM: u32 = 17u32;
pub const WAVE_FORMAT_INDEO_AUDIO: u32 = 1026u32;
pub const WAVE_FORMAT_INFOCOM_ITS_G721_ADPCM: u32 = 139u32;
pub const WAVE_FORMAT_INGENIENT_G726: u32 = 41221u32;
pub const WAVE_FORMAT_INNINGS_TELECOM_ADPCM: u32 = 6521u32;
pub const WAVE_FORMAT_INTEL_G723_1: u32 = 67u32;
pub const WAVE_FORMAT_INTEL_G729: u32 = 68u32;
pub const WAVE_FORMAT_INTEL_MUSIC_CODER: u32 = 1025u32;
pub const WAVE_FORMAT_IPI_HSX: u32 = 592u32;
pub const WAVE_FORMAT_IPI_RPELP: u32 = 593u32;
pub const WAVE_FORMAT_IRAT: u32 = 257u32;
pub const WAVE_FORMAT_ISIAUDIO: u32 = 136u32;
pub const WAVE_FORMAT_ISIAUDIO_2: u32 = 5121u32;
pub const WAVE_FORMAT_KNOWLEDGE_ADVENTURE_ADPCM: u32 = 376u32;
pub const WAVE_FORMAT_LEAD_SPEECH: u32 = 17228u32;
pub const WAVE_FORMAT_LEAD_VORBIS: u32 = 22092u32;
pub const WAVE_FORMAT_LH_CODEC: u32 = 4352u32;
pub const WAVE_FORMAT_LH_CODEC_CELP: u32 = 4353u32;
pub const WAVE_FORMAT_LH_CODEC_SBC12: u32 = 4355u32;
pub const WAVE_FORMAT_LH_CODEC_SBC16: u32 = 4356u32;
pub const WAVE_FORMAT_LH_CODEC_SBC8: u32 = 4354u32;
pub const WAVE_FORMAT_LIGHTWAVE_LOSSLESS: u32 = 2222u32;
pub const WAVE_FORMAT_LRC: u32 = 40u32;
pub const WAVE_FORMAT_LUCENT_G723: u32 = 89u32;
pub const WAVE_FORMAT_LUCENT_SX5363S: u32 = 7180u32;
pub const WAVE_FORMAT_LUCENT_SX8300P: u32 = 7175u32;
pub const WAVE_FORMAT_MAKEAVIS: u32 = 13075u32;
pub const WAVE_FORMAT_MALDEN_PHONYTALK: u32 = 160u32;
pub const WAVE_FORMAT_MEDIASONIC_G723: u32 = 147u32;
pub const WAVE_FORMAT_MEDIASPACE_ADPCM: u32 = 18u32;
pub const WAVE_FORMAT_MEDIAVISION_ADPCM: u32 = 24u32;
pub const WAVE_FORMAT_MICRONAS: u32 = 848u32;
pub const WAVE_FORMAT_MICRONAS_CELP833: u32 = 849u32;
pub const WAVE_FORMAT_MPEG: u32 = 80u32;
pub const WAVE_FORMAT_MPEG4_AAC: u32 = 41222u32;
pub const WAVE_FORMAT_MPEGLAYER3: u32 = 85u32;
pub const WAVE_FORMAT_MPEG_ADTS_AAC: u32 = 5632u32;
pub const WAVE_FORMAT_MPEG_HEAAC: u32 = 5648u32;
pub const WAVE_FORMAT_MPEG_LOAS: u32 = 5634u32;
pub const WAVE_FORMAT_MPEG_RAW_AAC: u32 = 5633u32;
pub const WAVE_FORMAT_MSAUDIO1: u32 = 352u32;
pub const WAVE_FORMAT_MSG723: u32 = 66u32;
pub const WAVE_FORMAT_MSNAUDIO: u32 = 50u32;
pub const WAVE_FORMAT_MSRT24: u32 = 130u32;
pub const WAVE_FORMAT_MULAW: u32 = 7u32;
pub const WAVE_FORMAT_MULTITUDE_FT_SX20: u32 = 138u32;
pub const WAVE_FORMAT_MVI_MVI2: u32 = 132u32;
pub const WAVE_FORMAT_NEC_AAC: u32 = 176u32;
pub const WAVE_FORMAT_NICE_ACA: u32 = 41240u32;
pub const WAVE_FORMAT_NICE_ADPCM: u32 = 41241u32;
pub const WAVE_FORMAT_NICE_G728: u32 = 41250u32;
pub const WAVE_FORMAT_NMS_VBXADPCM: u32 = 56u32;
pub const WAVE_FORMAT_NOKIA_ADAPTIVE_MULTIRATE: u32 = 16897u32;
pub const WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC: u32 = 5640u32;
pub const WAVE_FORMAT_NOKIA_MPEG_RAW_AAC: u32 = 5641u32;
pub const WAVE_FORMAT_NORCOM_VOICE_SYSTEMS_ADPCM: u32 = 645u32;
pub const WAVE_FORMAT_NORRIS: u32 = 5120u32;
pub const WAVE_FORMAT_NTCSOFT_ALF2CM_ACM: u32 = 8132u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1: u32 = 26447u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS: u32 = 26479u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2: u32 = 26448u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS: u32 = 26480u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3: u32 = 26449u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS: u32 = 26481u32;
pub const WAVE_FORMAT_OKI_ADPCM: u32 = 16u32;
pub const WAVE_FORMAT_OLIADPCM: u32 = 4097u32;
pub const WAVE_FORMAT_OLICELP: u32 = 4098u32;
pub const WAVE_FORMAT_OLIGSM: u32 = 4096u32;
pub const WAVE_FORMAT_OLIOPR: u32 = 4100u32;
pub const WAVE_FORMAT_OLISBC: u32 = 4099u32;
pub const WAVE_FORMAT_ON2_VP6_AUDIO: u32 = 1281u32;
pub const WAVE_FORMAT_ON2_VP7_AUDIO: u32 = 1280u32;
pub const WAVE_FORMAT_ONLIVE: u32 = 137u32;
pub const WAVE_FORMAT_OPUS: u32 = 28751u32;
pub const WAVE_FORMAT_PAC: u32 = 83u32;
pub const WAVE_FORMAT_PACKED: u32 = 153u32;
pub const WAVE_FORMAT_PCM_S: u32 = 1152u32;
pub const WAVE_FORMAT_PHILIPS_CELP: u32 = 288u32;
pub const WAVE_FORMAT_PHILIPS_GRUNDIG: u32 = 289u32;
pub const WAVE_FORMAT_PHILIPS_LPCBB: u32 = 152u32;
pub const WAVE_FORMAT_POLYCOM_G722: u32 = 41234u32;
pub const WAVE_FORMAT_POLYCOM_G728: u32 = 41235u32;
pub const WAVE_FORMAT_POLYCOM_G729_A: u32 = 41236u32;
pub const WAVE_FORMAT_POLYCOM_SIREN: u32 = 41237u32;
pub const WAVE_FORMAT_PROSODY_1612: u32 = 39u32;
pub const WAVE_FORMAT_PROSODY_8KBPS: u32 = 148u32;
pub const WAVE_FORMAT_QDESIGN_MUSIC: u32 = 1104u32;
pub const WAVE_FORMAT_QUALCOMM_HALFRATE: u32 = 337u32;
pub const WAVE_FORMAT_QUALCOMM_PUREVOICE: u32 = 336u32;
pub const WAVE_FORMAT_QUARTERDECK: u32 = 544u32;
pub const WAVE_FORMAT_RACAL_RECORDER_G720_A: u32 = 162u32;
pub const WAVE_FORMAT_RACAL_RECORDER_G723_1: u32 = 163u32;
pub const WAVE_FORMAT_RACAL_RECORDER_GSM: u32 = 161u32;
pub const WAVE_FORMAT_RACAL_RECORDER_TETRA_ACELP: u32 = 164u32;
pub const WAVE_FORMAT_RADIOTIME_TIME_SHIFT_RADIO: u32 = 41239u32;
pub const WAVE_FORMAT_RAW_AAC1: u32 = 255u32;
pub const WAVE_FORMAT_RAW_SPORT: u32 = 576u32;
pub const WAVE_FORMAT_RHETOREX_ADPCM: u32 = 256u32;
pub const WAVE_FORMAT_ROCKWELL_ADPCM: u32 = 59u32;
pub const WAVE_FORMAT_ROCKWELL_DIGITALK: u32 = 60u32;
pub const WAVE_FORMAT_RT24: u32 = 82u32;
pub const WAVE_FORMAT_SANYO_LD_ADPCM: u32 = 293u32;
pub const WAVE_FORMAT_SBC24: u32 = 145u32;
pub const WAVE_FORMAT_SHARP_G726: u32 = 69u32;
pub const WAVE_FORMAT_SIERRA_ADPCM: u32 = 19u32;
pub const WAVE_FORMAT_SIPROLAB_ACELP4800: u32 = 305u32;
pub const WAVE_FORMAT_SIPROLAB_ACELP8V3: u32 = 306u32;
pub const WAVE_FORMAT_SIPROLAB_ACEPLNET: u32 = 304u32;
pub const WAVE_FORMAT_SIPROLAB_G729: u32 = 307u32;
pub const WAVE_FORMAT_SIPROLAB_G729A: u32 = 308u32;
pub const WAVE_FORMAT_SIPROLAB_KELVIN: u32 = 309u32;
pub const WAVE_FORMAT_SOFTSOUND: u32 = 128u32;
pub const WAVE_FORMAT_SONARC: u32 = 33u32;
pub const WAVE_FORMAT_SONICFOUNDRY_LOSSLESS: u32 = 6513u32;
pub const WAVE_FORMAT_SONY_ATRAC3: u32 = 626u32;
pub const WAVE_FORMAT_SONY_SCX: u32 = 624u32;
pub const WAVE_FORMAT_SONY_SCY: u32 = 625u32;
pub const WAVE_FORMAT_SONY_SPC: u32 = 627u32;
pub const WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS: u32 = 5376u32;
pub const WAVE_FORMAT_SPEEX_VOICE: u32 = 41225u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC008: u32 = 372u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP54: u32 = 374u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP68: u32 = 375u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_G726L: u32 = 373u32;
pub const WAVE_FORMAT_SYMBOL_G729_A: u32 = 41219u32;
pub const WAVE_FORMAT_TELUM_AUDIO: u32 = 640u32;
pub const WAVE_FORMAT_TELUM_IA_AUDIO: u32 = 641u32;
pub const WAVE_FORMAT_TPC: u32 = 1665u32;
pub const WAVE_FORMAT_TUBGSM: u32 = 341u32;
pub const WAVE_FORMAT_UHER_ADPCM: u32 = 528u32;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO: u32 = 533u32;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO_1: u32 = 534u32;
pub const WAVE_FORMAT_UNISYS_NAP_16K: u32 = 371u32;
pub const WAVE_FORMAT_UNISYS_NAP_ADPCM: u32 = 368u32;
pub const WAVE_FORMAT_UNISYS_NAP_ALAW: u32 = 370u32;
pub const WAVE_FORMAT_UNISYS_NAP_ULAW: u32 = 369u32;
pub const WAVE_FORMAT_UNKNOWN: u32 = 0u32;
pub const WAVE_FORMAT_VIANIX_MASC: u32 = 41226u32;
pub const WAVE_FORMAT_VIVO_G723: u32 = 273u32;
pub const WAVE_FORMAT_VIVO_SIREN: u32 = 274u32;
pub const WAVE_FORMAT_VME_VMPCM: u32 = 1664u32;
pub const WAVE_FORMAT_VOCORD_G721: u32 = 41242u32;
pub const WAVE_FORMAT_VOCORD_G722_1: u32 = 41244u32;
pub const WAVE_FORMAT_VOCORD_G723_1: u32 = 41248u32;
pub const WAVE_FORMAT_VOCORD_G726: u32 = 41243u32;
pub const WAVE_FORMAT_VOCORD_G728: u32 = 41245u32;
pub const WAVE_FORMAT_VOCORD_G729: u32 = 41246u32;
pub const WAVE_FORMAT_VOCORD_G729_A: u32 = 41247u32;
pub const WAVE_FORMAT_VOCORD_LBC: u32 = 41249u32;
pub const WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC: u32 = 5642u32;
pub const WAVE_FORMAT_VODAFONE_MPEG_RAW_AAC: u32 = 5643u32;
pub const WAVE_FORMAT_VOICEAGE_AMR: u32 = 310u32;
pub const WAVE_FORMAT_VOICEAGE_AMR_WB: u32 = 41220u32;
pub const WAVE_FORMAT_VOXWARE: u32 = 98u32;
pub const WAVE_FORMAT_VOXWARE_AC10: u32 = 113u32;
pub const WAVE_FORMAT_VOXWARE_AC16: u32 = 114u32;
pub const WAVE_FORMAT_VOXWARE_AC20: u32 = 115u32;
pub const WAVE_FORMAT_VOXWARE_AC8: u32 = 112u32;
pub const WAVE_FORMAT_VOXWARE_BYTE_ALIGNED: u32 = 105u32;
pub const WAVE_FORMAT_VOXWARE_RT24: u32 = 116u32;
pub const WAVE_FORMAT_VOXWARE_RT24_SPEECH: u32 = 6172u32;
pub const WAVE_FORMAT_VOXWARE_RT29: u32 = 117u32;
pub const WAVE_FORMAT_VOXWARE_RT29HW: u32 = 118u32;
pub const WAVE_FORMAT_VOXWARE_SC3: u32 = 122u32;
pub const WAVE_FORMAT_VOXWARE_SC3_1: u32 = 123u32;
pub const WAVE_FORMAT_VOXWARE_TQ40: u32 = 121u32;
pub const WAVE_FORMAT_VOXWARE_TQ60: u32 = 129u32;
pub const WAVE_FORMAT_VOXWARE_VR12: u32 = 119u32;
pub const WAVE_FORMAT_VOXWARE_VR18: u32 = 120u32;
pub const WAVE_FORMAT_VSELP: u32 = 4u32;
pub const WAVE_FORMAT_WAVPACK_AUDIO: u32 = 22358u32;
pub const WAVE_FORMAT_WM9_SPECTRUM_ANALYZER: u32 = 41227u32;
pub const WAVE_FORMAT_WMASPDIF: u32 = 356u32;
pub const WAVE_FORMAT_WMAUDIO2: u32 = 353u32;
pub const WAVE_FORMAT_WMAUDIO3: u32 = 354u32;
pub const WAVE_FORMAT_WMAUDIO_LOSSLESS: u32 = 355u32;
pub const WAVE_FORMAT_WMAVOICE10: u32 = 11u32;
pub const WAVE_FORMAT_WMAVOICE9: u32 = 10u32;
pub const WAVE_FORMAT_WMF_SPECTRUM_ANAYZER: u32 = 41228u32;
pub const WAVE_FORMAT_XEBEC: u32 = 61u32;
pub const WAVE_FORMAT_YAMAHA_ADPCM: u32 = 32u32;
pub const WAVE_FORMAT_ZOLL_ASAO: u32 = 41224u32;
pub const WAVE_FORMAT_ZYXEL_ADPCM: u32 = 151u32;
pub const WAVE_MAPPER_S: u32 = 1153u32;
pub const WIDM_ADDBUFFER: u32 = 56u32;
pub const WIDM_CLOSE: u32 = 53u32;
pub const WIDM_GETDEVCAPS: u32 = 51u32;
pub const WIDM_GETNUMDEVS: u32 = 50u32;
pub const WIDM_GETPOS: u32 = 60u32;
pub const WIDM_INIT: u32 = 100u32;
pub const WIDM_INIT_EX: u32 = 104u32;
pub const WIDM_OPEN: u32 = 52u32;
pub const WIDM_PREFERRED: u32 = 61u32;
pub const WIDM_PREPARE: u32 = 54u32;
pub const WIDM_RESET: u32 = 59u32;
pub const WIDM_START: u32 = 57u32;
pub const WIDM_STOP: u32 = 58u32;
pub const WIDM_UNPREPARE: u32 = 55u32;
pub const WMAUDIO2_BITS_PER_SAMPLE: u32 = 16u32;
pub const WMAUDIO2_MAX_CHANNELS: u32 = 2u32;
pub const WMAUDIO_BITS_PER_SAMPLE: u32 = 16u32;
pub const WMAUDIO_MAX_CHANNELS: u32 = 2u32;
pub const WM_CAP_ABORT: u32 = 1093u32;
pub const WM_CAP_DLG_VIDEOCOMPRESSION: u32 = 1070u32;
pub const WM_CAP_DLG_VIDEODISPLAY: u32 = 1067u32;
pub const WM_CAP_DLG_VIDEOFORMAT: u32 = 1065u32;
pub const WM_CAP_DLG_VIDEOSOURCE: u32 = 1066u32;
pub const WM_CAP_DRIVER_CONNECT: u32 = 1034u32;
pub const WM_CAP_DRIVER_DISCONNECT: u32 = 1035u32;
pub const WM_CAP_DRIVER_GET_CAPS: u32 = 1038u32;
pub const WM_CAP_DRIVER_GET_NAME: u32 = 1136u32;
pub const WM_CAP_DRIVER_GET_NAMEA: u32 = 1036u32;
pub const WM_CAP_DRIVER_GET_NAMEW: u32 = 1136u32;
pub const WM_CAP_DRIVER_GET_VERSION: u32 = 1137u32;
pub const WM_CAP_DRIVER_GET_VERSIONA: u32 = 1037u32;
pub const WM_CAP_DRIVER_GET_VERSIONW: u32 = 1137u32;
pub const WM_CAP_EDIT_COPY: u32 = 1054u32;
pub const WM_CAP_END: u32 = 1205u32;
pub const WM_CAP_FILE_ALLOCATE: u32 = 1046u32;
pub const WM_CAP_FILE_GET_CAPTURE_FILE: u32 = 1145u32;
pub const WM_CAP_FILE_GET_CAPTURE_FILEA: u32 = 1045u32;
pub const WM_CAP_FILE_GET_CAPTURE_FILEW: u32 = 1145u32;
pub const WM_CAP_FILE_SAVEAS: u32 = 1147u32;
pub const WM_CAP_FILE_SAVEASA: u32 = 1047u32;
pub const WM_CAP_FILE_SAVEASW: u32 = 1147u32;
pub const WM_CAP_FILE_SAVEDIB: u32 = 1149u32;
pub const WM_CAP_FILE_SAVEDIBA: u32 = 1049u32;
pub const WM_CAP_FILE_SAVEDIBW: u32 = 1149u32;
pub const WM_CAP_FILE_SET_CAPTURE_FILE: u32 = 1144u32;
pub const WM_CAP_FILE_SET_CAPTURE_FILEA: u32 = 1044u32;
pub const WM_CAP_FILE_SET_CAPTURE_FILEW: u32 = 1144u32;
pub const WM_CAP_FILE_SET_INFOCHUNK: u32 = 1048u32;
pub const WM_CAP_GET_AUDIOFORMAT: u32 = 1060u32;
pub const WM_CAP_GET_CAPSTREAMPTR: u32 = 1025u32;
pub const WM_CAP_GET_MCI_DEVICE: u32 = 1191u32;
pub const WM_CAP_GET_MCI_DEVICEA: u32 = 1091u32;
pub const WM_CAP_GET_MCI_DEVICEW: u32 = 1191u32;
pub const WM_CAP_GET_SEQUENCE_SETUP: u32 = 1089u32;
pub const WM_CAP_GET_STATUS: u32 = 1078u32;
pub const WM_CAP_GET_USER_DATA: u32 = 1032u32;
pub const WM_CAP_GET_VIDEOFORMAT: u32 = 1068u32;
pub const WM_CAP_GRAB_FRAME: u32 = 1084u32;
pub const WM_CAP_GRAB_FRAME_NOSTOP: u32 = 1085u32;
pub const WM_CAP_PAL_AUTOCREATE: u32 = 1107u32;
pub const WM_CAP_PAL_MANUALCREATE: u32 = 1108u32;
pub const WM_CAP_PAL_OPEN: u32 = 1204u32;
pub const WM_CAP_PAL_OPENA: u32 = 1104u32;
pub const WM_CAP_PAL_OPENW: u32 = 1204u32;
pub const WM_CAP_PAL_PASTE: u32 = 1106u32;
pub const WM_CAP_PAL_SAVE: u32 = 1205u32;
pub const WM_CAP_PAL_SAVEA: u32 = 1105u32;
pub const WM_CAP_PAL_SAVEW: u32 = 1205u32;
pub const WM_CAP_SEQUENCE: u32 = 1086u32;
pub const WM_CAP_SEQUENCE_NOFILE: u32 = 1087u32;
pub const WM_CAP_SET_AUDIOFORMAT: u32 = 1059u32;
pub const WM_CAP_SET_CALLBACK_CAPCONTROL: u32 = 1109u32;
pub const WM_CAP_SET_CALLBACK_ERROR: u32 = 1126u32;
pub const WM_CAP_SET_CALLBACK_ERRORA: u32 = 1026u32;
pub const WM_CAP_SET_CALLBACK_ERRORW: u32 = 1126u32;
pub const WM_CAP_SET_CALLBACK_FRAME: u32 = 1029u32;
pub const WM_CAP_SET_CALLBACK_STATUS: u32 = 1127u32;
pub const WM_CAP_SET_CALLBACK_STATUSA: u32 = 1027u32;
pub const WM_CAP_SET_CALLBACK_STATUSW: u32 = 1127u32;
pub const WM_CAP_SET_CALLBACK_VIDEOSTREAM: u32 = 1030u32;
pub const WM_CAP_SET_CALLBACK_WAVESTREAM: u32 = 1031u32;
pub const WM_CAP_SET_CALLBACK_YIELD: u32 = 1028u32;
pub const WM_CAP_SET_MCI_DEVICE: u32 = 1190u32;
pub const WM_CAP_SET_MCI_DEVICEA: u32 = 1090u32;
pub const WM_CAP_SET_MCI_DEVICEW: u32 = 1190u32;
pub const WM_CAP_SET_OVERLAY: u32 = 1075u32;
pub const WM_CAP_SET_PREVIEW: u32 = 1074u32;
pub const WM_CAP_SET_PREVIEWRATE: u32 = 1076u32;
pub const WM_CAP_SET_SCALE: u32 = 1077u32;
pub const WM_CAP_SET_SCROLL: u32 = 1079u32;
pub const WM_CAP_SET_SEQUENCE_SETUP: u32 = 1088u32;
pub const WM_CAP_SET_USER_DATA: u32 = 1033u32;
pub const WM_CAP_SET_VIDEOFORMAT: u32 = 1069u32;
pub const WM_CAP_SINGLE_FRAME: u32 = 1096u32;
pub const WM_CAP_SINGLE_FRAME_CLOSE: u32 = 1095u32;
pub const WM_CAP_SINGLE_FRAME_OPEN: u32 = 1094u32;
pub const WM_CAP_START: u32 = 1024u32;
pub const WM_CAP_STOP: u32 = 1092u32;
pub const WM_CAP_UNICODE_END: u32 = 1205u32;
pub const WM_CAP_UNICODE_START: u32 = 1124u32;
pub const WODM_BREAKLOOP: u32 = 20u32;
pub const WODM_BUSY: u32 = 21u32;
pub const WODM_CLOSE: u32 = 6u32;
pub const WODM_GETDEVCAPS: u32 = 4u32;
pub const WODM_GETNUMDEVS: u32 = 3u32;
pub const WODM_GETPITCH: u32 = 14u32;
pub const WODM_GETPLAYBACKRATE: u32 = 18u32;
pub const WODM_GETPOS: u32 = 13u32;
pub const WODM_GETVOLUME: u32 = 16u32;
pub const WODM_INIT: u32 = 100u32;
pub const WODM_INIT_EX: u32 = 104u32;
pub const WODM_OPEN: u32 = 5u32;
pub const WODM_PAUSE: u32 = 10u32;
pub const WODM_PREFERRED: u32 = 21u32;
pub const WODM_PREPARE: u32 = 7u32;
pub const WODM_RESET: u32 = 12u32;
pub const WODM_RESTART: u32 = 11u32;
pub const WODM_SETPITCH: u32 = 15u32;
pub const WODM_SETPLAYBACKRATE: u32 = 19u32;
pub const WODM_SETVOLUME: u32 = 17u32;
pub const WODM_UNPREPARE: u32 = 8u32;
pub const WODM_WRITE: u32 = 9u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ADPCMCOEFSET {
    pub iCoef1: i16,
    pub iCoef2: i16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct ADPCMEWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wNumCoef: u16,
    pub aCoef: [ADPCMCOEFSET; 1],
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct APTXWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct AUDIOFILE_AF10WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct AUDIOFILE_AF36WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AVICOMPRESSOPTIONS {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwKeyFrameEvery: u32,
    pub dwQuality: u32,
    pub dwBytesPerSecond: u32,
    pub dwFlags: u32,
    pub lpFormat: *mut core::ffi::c_void,
    pub cbFormat: u32,
    pub lpParms: *mut core::ffi::c_void,
    pub cbParms: u32,
    pub dwInterleaveEvery: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AVIFILEINFOA {
    pub dwMaxBytesPerSec: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwLength: u32,
    pub dwEditCount: u32,
    pub szFileType: [i8; 64],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AVIFILEINFOW {
    pub dwMaxBytesPerSec: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwLength: u32,
    pub dwEditCount: u32,
    pub szFileType: [u16; 64],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AVISTREAMINFOA {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwInitialFrames: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: super::super::Foundation::RECT,
    pub dwEditCount: u32,
    pub dwFormatChangeCount: u32,
    pub szName: [i8; 64],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AVISTREAMINFOW {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwInitialFrames: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: super::super::Foundation::RECT,
    pub dwEditCount: u32,
    pub dwFormatChangeCount: u32,
    pub szName: [u16; 64],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAPDRIVERCAPS {
    pub wDeviceIndex: u32,
    pub fHasOverlay: super::super::Foundation::BOOL,
    pub fHasDlgVideoSource: super::super::Foundation::BOOL,
    pub fHasDlgVideoFormat: super::super::Foundation::BOOL,
    pub fHasDlgVideoDisplay: super::super::Foundation::BOOL,
    pub fCaptureInitialized: super::super::Foundation::BOOL,
    pub fDriverSuppliesPalettes: super::super::Foundation::BOOL,
    pub hVideoIn: super::super::Foundation::HANDLE,
    pub hVideoOut: super::super::Foundation::HANDLE,
    pub hVideoExtIn: super::super::Foundation::HANDLE,
    pub hVideoExtOut: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAPINFOCHUNK {
    pub fccInfoID: u32,
    pub lpData: *mut core::ffi::c_void,
    pub cbData: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct CAPSTATUS {
    pub uiImageWidth: u32,
    pub uiImageHeight: u32,
    pub fLiveWindow: super::super::Foundation::BOOL,
    pub fOverlayWindow: super::super::Foundation::BOOL,
    pub fScale: super::super::Foundation::BOOL,
    pub ptScroll: super::super::Foundation::POINT,
    pub fUsingDefaultPalette: super::super::Foundation::BOOL,
    pub fAudioHardware: super::super::Foundation::BOOL,
    pub fCapFileExists: super::super::Foundation::BOOL,
    pub dwCurrentVideoFrame: u32,
    pub dwCurrentVideoFramesDropped: u32,
    pub dwCurrentWaveSamples: u32,
    pub dwCurrentTimeElapsedMS: u32,
    pub hPalCurrent: super::super::Graphics::Gdi::HPALETTE,
    pub fCapturingNow: super::super::Foundation::BOOL,
    pub dwReturn: u32,
    pub wNumVideoAllocated: u32,
    pub wNumAudioAllocated: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAPTUREPARMS {
    pub dwRequestMicroSecPerFrame: u32,
    pub fMakeUserHitOKToCapture: super::super::Foundation::BOOL,
    pub wPercentDropForError: u32,
    pub fYield: super::super::Foundation::BOOL,
    pub dwIndexSize: u32,
    pub wChunkGranularity: u32,
    pub fUsingDOSMemory: super::super::Foundation::BOOL,
    pub wNumVideoRequested: u32,
    pub fCaptureAudio: super::super::Foundation::BOOL,
    pub wNumAudioRequested: u32,
    pub vKeyAbort: u32,
    pub fAbortLeftMouse: super::super::Foundation::BOOL,
    pub fAbortRightMouse: super::super::Foundation::BOOL,
    pub fLimitEnabled: super::super::Foundation::BOOL,
    pub wTimeLimit: u32,
    pub fMCIControl: super::super::Foundation::BOOL,
    pub fStepMCIDevice: super::super::Foundation::BOOL,
    pub dwMCIStartTime: u32,
    pub dwMCIStopTime: u32,
    pub fStepCaptureAt2x: super::super::Foundation::BOOL,
    pub wStepCaptureAverageFrames: u32,
    pub dwAudioBufferSize: u32,
    pub fDisableWriteCache: super::super::Foundation::BOOL,
    pub AVStreamMaster: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHANNEL_CAPS {
    pub dwFlags: u32,
    pub dwSrcRectXMod: u32,
    pub dwSrcRectYMod: u32,
    pub dwSrcRectWidthMod: u32,
    pub dwSrcRectHeightMod: u32,
    pub dwDstRectXMod: u32,
    pub dwDstRectYMod: u32,
    pub dwDstRectWidthMod: u32,
    pub dwDstRectHeightMod: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct COMPVARS {
    pub cbSize: i32,
    pub dwFlags: u32,
    pub hic: HIC,
    pub fccType: u32,
    pub fccHandler: u32,
    pub lpbiIn: *mut super::super::Graphics::Gdi::BITMAPINFO,
    pub lpbiOut: *mut super::super::Graphics::Gdi::BITMAPINFO,
    pub lpBitsOut: *mut core::ffi::c_void,
    pub lpBitsPrev: *mut core::ffi::c_void,
    pub lFrame: i32,
    pub lKey: i32,
    pub lDataRate: i32,
    pub lQ: i32,
    pub lKeyCount: i32,
    pub lpState: *mut core::ffi::c_void,
    pub cbState: i32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct CONTRESCR10WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct CONTRESVQLPCWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct CREATIVEADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct CREATIVEFASTSPEECH10WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct CREATIVEFASTSPEECH8WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct CSIMAADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct DIALOGICOKIADPCMWAVEFORMAT {
    pub ewf: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct DIGIADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct DIGIFIXWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct DIGIREALWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct DIGISTDWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct DOLBYAC2WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub nAuxBitsCode: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRAWDIBTIME {
    pub timeCount: i32,
    pub timeDraw: i32,
    pub timeDecompress: i32,
    pub timeDither: i32,
    pub timeStretch: i32,
    pub timeBlt: i32,
    pub timeSetDIBits: i32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct DRMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wReserved: u16,
    pub ulContentId: u32,
    pub wfxSecure: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DRVCONFIGINFO {
    pub dwDCISize: u32,
    pub lpszDCISectionName: windows_sys::core::PCWSTR,
    pub lpszDCIAliasName: windows_sys::core::PCWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DRVCONFIGINFOEX {
    pub dwDCISize: u32,
    pub lpszDCISectionName: windows_sys::core::PCWSTR,
    pub lpszDCIAliasName: windows_sys::core::PCWSTR,
    pub dnDevNode: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DRVM_IOCTL_DATA {
    pub dwSize: u32,
    pub dwCmd: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct DVIADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct ECHOSC1WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct EXBMINFOHEADER {
    pub bmi: super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub biExtDataOffset: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct FMTOWNS_SND_WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct G721_ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub nAuxBlockSize: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct G723_ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub cbExtraSize: u16,
    pub nAuxBlockSize: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct GSM610WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
pub type HDRVR = *mut core::ffi::c_void;
pub type HIC = *mut core::ffi::c_void;
pub type HMMIO = *mut core::ffi::c_void;
pub type HVIDEO = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct ICCOMPRESS {
    pub dwFlags: u32,
    pub lpbiOutput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpOutput: *mut core::ffi::c_void,
    pub lpbiInput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpInput: *mut core::ffi::c_void,
    pub lpckid: *mut u32,
    pub lpdwFlags: *mut u32,
    pub lFrameNum: i32,
    pub dwFrameSize: u32,
    pub dwQuality: u32,
    pub lpbiPrev: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpPrev: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct ICCOMPRESSFRAMES {
    pub dwFlags: u32,
    pub lpbiOutput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lOutput: super::super::Foundation::LPARAM,
    pub lpbiInput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lInput: super::super::Foundation::LPARAM,
    pub lStartFrame: i32,
    pub lFrameCount: i32,
    pub lQuality: i32,
    pub lDataRate: i32,
    pub lKeyRate: i32,
    pub dwRate: u32,
    pub dwScale: u32,
    pub dwOverheadPerFrame: u32,
    pub dwReserved2: u32,
    pub GetData: isize,
    pub PutData: isize,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct ICDECOMPRESS {
    pub dwFlags: u32,
    pub lpbiInput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpInput: *mut core::ffi::c_void,
    pub lpbiOutput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpOutput: *mut core::ffi::c_void,
    pub ckid: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct ICDECOMPRESSEX {
    pub dwFlags: u32,
    pub lpbiSrc: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpSrc: *mut core::ffi::c_void,
    pub lpbiDst: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpDst: *mut core::ffi::c_void,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ICDRAW {
    pub dwFlags: u32,
    pub lpFormat: *mut core::ffi::c_void,
    pub lpData: *mut core::ffi::c_void,
    pub cbData: u32,
    pub lTime: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct ICDRAWBEGIN {
    pub dwFlags: u32,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
    pub hwnd: super::super::Foundation::HWND,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub lpbi: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dwRate: u32,
    pub dwScale: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct ICDRAWSUGGEST {
    pub lpbiIn: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpbiSuggest: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub hicDecompressor: HIC,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ICINFO {
    pub dwSize: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwVersion: u32,
    pub dwVersionICM: u32,
    pub szName: [u16; 16],
    pub szDescription: [u16; 128],
    pub szDriver: [u16; 128],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ICOPEN {
    pub dwSize: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: super::super::Foundation::LRESULT,
    pub pV1Reserved: *mut core::ffi::c_void,
    pub pV2Reserved: *mut core::ffi::c_void,
    pub dnDevNode: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct ICPALETTE {
    pub dwFlags: u32,
    pub iStart: i32,
    pub iLen: i32,
    pub lppe: *mut super::super::Graphics::Gdi::PALETTEENTRY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ICSETSTATUSPROC {
    pub dwFlags: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub Status: isize,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct IMAADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JOYCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [i8; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [i8; 32],
    pub szOEMVxD: [i8; 260],
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JOYCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [u16; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [u16; 32],
    pub szOEMVxD: [u16; 260],
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JOYCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [i8; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [i8; 32],
    pub szOEMVxD: [i8; 260],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JOYCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [u16; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [u16; 32],
    pub szOEMVxD: [u16; 260],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JOYINFO {
    pub wXpos: u32,
    pub wYpos: u32,
    pub wZpos: u32,
    pub wButtons: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JOYINFOEX {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwXpos: u32,
    pub dwYpos: u32,
    pub dwZpos: u32,
    pub dwRpos: u32,
    pub dwUpos: u32,
    pub dwVpos: u32,
    pub dwButtons: u32,
    pub dwButtonNumber: u32,
    pub dwPOV: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JPEGINFOHEADER {
    pub JPEGSize: u32,
    pub JPEGProcess: u32,
    pub JPEGColorSpaceID: u32,
    pub JPEGBitsPerSample: u32,
    pub JPEGHSubSampling: u32,
    pub JPEGVSubSampling: u32,
}
pub const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000003_0000_0010_8000_00aa00389b71);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: windows_sys::core::PCSTR,
    pub lpstrElementName: windows_sys::core::PCSTR,
    pub lpstrAlias: windows_sys::core::PCSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: windows_sys::core::PCWSTR,
    pub lpstrElementName: windows_sys::core::PCWSTR,
    pub lpstrAlias: windows_sys::core::PCWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub dwSpeed: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_UPDATE_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
    pub hDC: super::super::Graphics::Gdi::HDC,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PCSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_WINDOW_PARMSW {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PCWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_BREAK_PARMS {
    pub dwCallback: usize,
    pub nVirtKey: i32,
    pub hwndBreak: super::super::Foundation::HWND,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_CAPTURE_PARMSA {
    pub dwCallback: usize,
    pub lpstrFileName: windows_sys::core::PSTR,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_CAPTURE_PARMSW {
    pub dwCallback: usize,
    pub lpstrFileName: windows_sys::core::PWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_COPY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_CUE_PARMS {
    pub dwCallback: usize,
    pub dwTo: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_CUT_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_DELETE_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_INFO_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PSTR,
    pub dwRetSize: u32,
    pub dwItem: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_INFO_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PWSTR,
    pub dwRetSize: u32,
    pub dwItem: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_LIST_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PSTR,
    pub dwLength: u32,
    pub dwNumber: u32,
    pub dwItem: u32,
    pub lpstrAlgorithm: windows_sys::core::PSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_LIST_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PWSTR,
    pub dwLength: u32,
    pub dwNumber: u32,
    pub dwItem: u32,
    pub lpstrAlgorithm: windows_sys::core::PWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_MONITOR_PARMS {
    pub dwCallback: usize,
    pub dwSource: u32,
    pub dwMethod: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: windows_sys::core::PSTR,
    pub lpstrElementName: windows_sys::core::PSTR,
    pub lpstrAlias: windows_sys::core::PSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: windows_sys::core::PWSTR,
    pub lpstrElementName: windows_sys::core::PWSTR,
    pub lpstrAlias: windows_sys::core::PWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_PASTE_PARMS {
    pub dwCallback: usize,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_QUALITY_PARMSA {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub lpstrName: windows_sys::core::PSTR,
    pub lpstrAlgorithm: u32,
    pub dwHandle: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_QUALITY_PARMSW {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub lpstrName: windows_sys::core::PWSTR,
    pub lpstrAlgorithm: u32,
    pub dwHandle: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_RECORD_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_RESERVE_PARMSA {
    pub dwCallback: usize,
    pub lpstrPath: windows_sys::core::PSTR,
    pub dwSize: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_RESERVE_PARMSW {
    pub dwCallback: usize,
    pub lpstrPath: windows_sys::core::PWSTR,
    pub dwSize: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_RESTORE_PARMSA {
    pub dwCallback: usize,
    pub lpstrFileName: windows_sys::core::PSTR,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_RESTORE_PARMSW {
    pub dwCallback: usize,
    pub lpstrFileName: windows_sys::core::PWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_SAVE_PARMSA {
    pub dwCallback: usize,
    pub lpstrFileName: windows_sys::core::PSTR,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_SAVE_PARMSW {
    pub dwCallback: usize,
    pub lpstrFileName: windows_sys::core::PWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_SETAUDIO_PARMSA {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: windows_sys::core::PSTR,
    pub lpstrQuality: windows_sys::core::PSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_SETAUDIO_PARMSW {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: windows_sys::core::PWSTR,
    pub lpstrQuality: windows_sys::core::PWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_SETVIDEO_PARMSA {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: windows_sys::core::PSTR,
    pub lpstrQuality: windows_sys::core::PSTR,
    pub dwSourceNumber: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_SETVIDEO_PARMSW {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: windows_sys::core::PWSTR,
    pub lpstrQuality: windows_sys::core::PWSTR,
    pub dwSourceNumber: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub dwFileFormat: u32,
    pub dwSpeed: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_SIGNAL_PARMS {
    pub dwCallback: usize,
    pub dwPosition: u32,
    pub dwPeriod: u32,
    pub dwUserParm: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_STATUS_PARMSA {
    pub dwCallback: usize,
    pub dwReturn: usize,
    pub dwItem: u32,
    pub dwTrack: u32,
    pub lpstrDrive: windows_sys::core::PSTR,
    pub dwReference: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_STATUS_PARMSW {
    pub dwCallback: usize,
    pub dwReturn: usize,
    pub dwItem: u32,
    pub dwTrack: u32,
    pub lpstrDrive: windows_sys::core::PWSTR,
    pub dwReference: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct MCI_DGV_UPDATE_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
    pub hDC: super::super::Graphics::Gdi::HDC,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_DGV_WINDOW_PARMSW {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_GENERIC_PARMS {
    pub dwCallback: usize,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_GETDEVCAPS_PARMS {
    pub dwCallback: usize,
    pub dwReturn: u32,
    pub dwItem: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_INFO_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PSTR,
    pub dwRetSize: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_INFO_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PWSTR,
    pub dwRetSize: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_LOAD_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_LOAD_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OPEN_DRIVER_PARMS {
    pub wDeviceID: u32,
    pub lpstrParams: windows_sys::core::PCWSTR,
    pub wCustomCommandTable: u32,
    pub wType: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: windows_sys::core::PCSTR,
    pub lpstrElementName: windows_sys::core::PCSTR,
    pub lpstrAlias: windows_sys::core::PCSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: windows_sys::core::PCWSTR,
    pub lpstrElementName: windows_sys::core::PCWSTR,
    pub lpstrAlias: windows_sys::core::PCWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_LOAD_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCSTR,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_LOAD_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: windows_sys::core::PCSTR,
    pub lpstrElementName: windows_sys::core::PCSTR,
    pub lpstrAlias: windows_sys::core::PCSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: windows_sys::core::PCWSTR,
    pub lpstrElementName: windows_sys::core::PCWSTR,
    pub lpstrAlias: windows_sys::core::PCWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_SAVE_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCSTR,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_SAVE_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PCSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_WINDOW_PARMSW {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PCWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_RECORD_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SAVE_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SAVE_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SEEK_PARMS {
    pub dwCallback: usize,
    pub dwTo: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SEQ_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub dwTempo: u32,
    pub dwPort: u32,
    pub dwSlave: u32,
    pub dwMaster: u32,
    pub dwOffset: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_STATUS_PARMS {
    pub dwCallback: usize,
    pub dwReturn: usize,
    pub dwItem: u32,
    pub dwTrack: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SYSINFO_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PSTR,
    pub dwRetSize: u32,
    pub dwNumber: u32,
    pub wDeviceType: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SYSINFO_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PWSTR,
    pub dwRetSize: u32,
    pub dwNumber: u32,
    pub wDeviceType: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_VD_ESCAPE_PARMSA {
    pub dwCallback: usize,
    pub lpstrCommand: windows_sys::core::PCSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_VD_ESCAPE_PARMSW {
    pub dwCallback: usize,
    pub lpstrCommand: windows_sys::core::PCWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_VD_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub dwSpeed: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_VD_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_WAVE_DELETE_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_WAVE_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: windows_sys::core::PCSTR,
    pub lpstrElementName: windows_sys::core::PCSTR,
    pub lpstrAlias: windows_sys::core::PCSTR,
    pub dwBufferSeconds: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_WAVE_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: windows_sys::core::PCWSTR,
    pub lpstrElementName: windows_sys::core::PCWSTR,
    pub lpstrAlias: windows_sys::core::PCWSTR,
    pub dwBufferSeconds: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_WAVE_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub wInput: u32,
    pub wOutput: u32,
    pub wFormatTag: u16,
    pub wReserved2: u16,
    pub nChannels: u16,
    pub wReserved3: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wReserved4: u16,
    pub wBitsPerSample: u16,
    pub wReserved5: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct MEDIASPACEADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIOPENSTRMID {
    pub dwStreamID: u32,
    pub uDeviceID: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct MIXEROPENDESC {
    pub hmx: super::Audio::HMIXER,
    pub pReserved0: *mut core::ffi::c_void,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub dnDevNode: usize,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MMCKINFO {
    pub ckid: u32,
    pub cksize: u32,
    pub fccType: u32,
    pub dwDataOffset: u32,
    pub dwFlags: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MMIOINFO {
    pub dwFlags: u32,
    pub fccIOProc: u32,
    pub pIOProc: LPMMIOPROC,
    pub wErrorRet: u32,
    pub htask: super::HTASK,
    pub cchBuffer: i32,
    pub pchBuffer: *mut i8,
    pub pchNext: *mut i8,
    pub pchEndRead: *mut i8,
    pub pchEndWrite: *mut i8,
    pub lBufOffset: i32,
    pub lDiskOffset: i32,
    pub adwInfo: [u32; 3],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub hmmio: HMMIO,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct MSAUDIO1WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wEncodeOptions: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct NMS_VBXADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct OLIADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct OLICELPWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct OLIGSMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct OLIOPRWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct OLISBCWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct SIERRAADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct SONARCWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wCompType: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct TIMEREVENT {
    pub wDelay: u16,
    pub wResolution: u16,
    pub lpFunction: super::LPTIMECALLBACK,
    pub dwUser: u32,
    pub wFlags: u16,
    pub wReserved1: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct TRUESPEECHWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
    pub nSamplesPerBlock: u16,
    pub abReserved: [u8; 28],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIDEOHDR {
    pub lpData: *mut u8,
    pub dwBufferLength: u32,
    pub dwBytesUsed: u32,
    pub dwTimeCaptured: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub dwReserved: [usize; 4],
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct WAVEOPENDESC {
    pub hWave: super::Audio::HWAVE,
    pub lpFormat: *mut super::Audio::WAVEFORMAT,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub uMappedDeviceID: u32,
    pub dnDevNode: usize,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct WMAUDIO2WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub dwSamplesPerBlock: u32,
    pub wEncodeOptions: u16,
    pub dwSuperBlockAlign: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct WMAUDIO3WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wValidBitsPerSample: u16,
    pub dwChannelMask: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub wEncodeOptions: u16,
    pub wReserved3: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy)]
pub struct YAMAHA_ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct s_RIFFWAVE_inst {
    pub bUnshiftedNote: u8,
    pub chFineTune: i8,
    pub chGain: i8,
    pub bLowNote: u8,
    pub bHighNote: u8,
    pub bLowVelocity: u8,
    pub bHighVelocity: u8,
}
pub type AVISAVECALLBACK = Option<unsafe extern "system" fn(param0: i32) -> super::super::Foundation::BOOL>;
pub type CAPCONTROLCALLBACK = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, nstate: i32) -> super::super::Foundation::LRESULT>;
pub type CAPERRORCALLBACKA = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, nid: i32, lpsz: windows_sys::core::PCSTR) -> super::super::Foundation::LRESULT>;
pub type CAPERRORCALLBACKW = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, nid: i32, lpsz: windows_sys::core::PCWSTR) -> super::super::Foundation::LRESULT>;
pub type CAPSTATUSCALLBACKA = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, nid: i32, lpsz: windows_sys::core::PCSTR) -> super::super::Foundation::LRESULT>;
pub type CAPSTATUSCALLBACKW = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, nid: i32, lpsz: windows_sys::core::PCWSTR) -> super::super::Foundation::LRESULT>;
pub type CAPVIDEOCALLBACK = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, lpvhdr: *const VIDEOHDR) -> super::super::Foundation::LRESULT>;
#[cfg(feature = "Win32_Media_Audio")]
pub type CAPWAVECALLBACK = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, lpwhdr: *const super::Audio::WAVEHDR) -> super::super::Foundation::LRESULT>;
pub type CAPYIELDCALLBACK = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::LRESULT>;
pub type DRIVERMSGPROC = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: usize, param3: usize, param4: usize) -> u32>;
pub type DRIVERPROC = Option<unsafe extern "system" fn(param0: usize, param1: HDRVR, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[cfg(feature = "Win32_System_IO")]
pub type LPFNEXTDEVIO = Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, dwflags: u32, dwiocontrolcode: u32, lpinbuffer: *mut core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL>;
pub type LPMMIOPROC = Option<unsafe extern "system" fn(lpmmioinfo: windows_sys::core::PCSTR, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
pub type LPTASKCALLBACK = Option<unsafe extern "system" fn(dwinst: usize)>;
#[cfg(feature = "Win32_UI_Controls")]
pub type VFWWDMExtensionProc = Option<unsafe extern "system" fn(pfndeviceiocontrol: *mut core::ffi::c_void, pfnaddpropertypage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> u32>;
pub type YIELDPROC = Option<unsafe extern "system" fn(mciid: u32, dwyielddata: u32) -> u32>;
