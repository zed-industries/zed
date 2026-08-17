#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn BstrFromVector(psa : *const super::Com:: SAFEARRAY, pbstr : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn ClearCustData(pcustdata : *mut super::Com:: CUSTDATA));
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn CreateDispTypeInfo(pidata : *mut INTERFACEDATA, lcid : u32, pptinfo : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn CreateErrorInfo(pperrinfo : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CreateOleAdviseHolder(ppoaholder : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn CreateStdDispatch(punkouter : * mut core::ffi::c_void, pvthis : *mut core::ffi::c_void, ptinfo : * mut core::ffi::c_void, ppunkstddisp : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn CreateTypeLib(syskind : super::Com:: SYSKIND, szfile : windows_sys::core::PCWSTR, ppctlib : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn CreateTypeLib2(syskind : super::Com:: SYSKIND, szfile : windows_sys::core::PCWSTR, ppctlib : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn DispCallFunc(pvinstance : *const core::ffi::c_void, ovft : usize, cc : super::Com:: CALLCONV, vtreturn : super::Variant:: VARENUM, cactuals : u32, prgvt : *const u16, prgpvarg : *const *const super::Variant:: VARIANT, pvargresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn DispGetIDsOfNames(ptinfo : * mut core::ffi::c_void, rgsznames : *const windows_sys::core::PCWSTR, cnames : u32, rgdispid : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn DispGetParam(pdispparams : *const super::Com:: DISPPARAMS, position : u32, vttarg : super::Variant:: VARENUM, pvarresult : *mut super::Variant:: VARIANT, puargerr : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn DispInvoke(_this : *mut core::ffi::c_void, ptinfo : * mut core::ffi::c_void, dispidmember : i32, wflags : u16, pparams : *mut super::Com:: DISPPARAMS, pvarresult : *mut super::Variant:: VARIANT, pexcepinfo : *mut super::Com:: EXCEPINFO, puargerr : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn DoDragDrop(pdataobj : * mut core::ffi::c_void, pdropsource : * mut core::ffi::c_void, dwokeffects : DROPEFFECT, pdweffect : *mut DROPEFFECT) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn GetActiveObject(rclsid : *const windows_sys::core::GUID, pvreserved : *mut core::ffi::c_void, ppunk : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn GetAltMonthNames(lcid : u32, prgp : *mut *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn GetRecordInfoFromGuids(rguidtypelib : *const windows_sys::core::GUID, uvermajor : u32, uverminor : u32, lcid : u32, rguidtypeinfo : *const windows_sys::core::GUID, pprecinfo : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn GetRecordInfoFromTypeInfo(ptypeinfo : * mut core::ffi::c_void, pprecinfo : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HRGN_UserFree(param0 : *const u32, param1 : *const super::super::Graphics::Gdi:: HRGN));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserFree64(param0 : *const u32, param1 : *const super::super::Graphics::Gdi:: HRGN));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HRGN_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::Graphics::Gdi:: HRGN) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::Graphics::Gdi:: HRGN) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HRGN_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::Graphics::Gdi:: HRGN) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::Graphics::Gdi:: HRGN) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HRGN_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::Graphics::Gdi:: HRGN) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::Graphics::Gdi:: HRGN) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn IsAccelerator(haccel : super::super::UI::WindowsAndMessaging:: HACCEL, caccelentries : i32, lpmsg : *const super::super::UI::WindowsAndMessaging:: MSG, lpwcmd : *mut u16) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn LHashValOfNameSys(syskind : super::Com:: SYSKIND, lcid : u32, szname : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn LHashValOfNameSysA(syskind : super::Com:: SYSKIND, lcid : u32, szname : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn LoadRegTypeLib(rguid : *const windows_sys::core::GUID, wvermajor : u16, wverminor : u16, lcid : u32, pptlib : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn LoadTypeLib(szfile : windows_sys::core::PCWSTR, pptlib : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn LoadTypeLibEx(szfile : windows_sys::core::PCWSTR, regkind : REGKIND, pptlib : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn OaBuildVersion() -> u32);
windows_targets::link!("oleaut32.dll" "system" fn OaEnablePerUserTLibRegistration());
windows_targets::link!("ole32.dll" "system" fn OleBuildVersion() -> u32);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage2(lpolestream : *const super::Com::StructuredStorage:: OLESTREAM, pstg : * mut core::ffi::c_void, ptd : *const super::Com:: DVTARGETDEVICE, opt : u32, pvcallbackcontext : *const core::ffi::c_void, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
windows_targets::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx2(polestm : *const super::Com::StructuredStorage:: OLESTREAM, pstg : * mut core::ffi::c_void, pcfformat : *mut u16, plwwidth : *mut i32, plheight : *mut i32, pdwsize : *mut u32, pmedium : *mut super::Com:: STGMEDIUM, opt : u32, pvcallbackcontext : *const core::ffi::c_void, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreate(rclsid : *const windows_sys::core::GUID, riid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleCreateDefaultHandler(clsid : *const windows_sys::core::GUID, punkouter : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, lplpobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn OleCreateEmbeddingHelper(clsid : *const windows_sys::core::GUID, punkouter : * mut core::ffi::c_void, flags : EMBDHLP_FLAGS, pcf : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, lplpobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateEx(rclsid : *const windows_sys::core::GUID, riid : *const windows_sys::core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn OleCreateFontIndirect(lpfontdesc : *const FONTDESC, riid : *const windows_sys::core::GUID, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateFromData(psrcdataobj : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateFromDataEx(psrcdataobj : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateFromFile(rclsid : *const windows_sys::core::GUID, lpszfilename : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, renderopt : u32, lpformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateFromFileEx(rclsid : *const windows_sys::core::GUID, lpszfilename : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateLink(pmklinksrc : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, renderopt : u32, lpformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateLinkEx(pmklinksrc : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateLinkFromData(psrcdataobj : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateLinkFromDataEx(psrcdataobj : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateLinkToFile(lpszfilename : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, renderopt : u32, lpformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateLinkToFileEx(lpszfilename : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn OleCreateMenuDescriptor(hmenucombined : super::super::UI::WindowsAndMessaging:: HMENU, lpmenuwidths : *const OLEMENUGROUPWIDTHS) -> isize);
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
windows_targets::link!("oleaut32.dll" "system" fn OleCreatePictureIndirect(lppictdesc : *const PICTDESC, riid : *const windows_sys::core::GUID, fown : windows_sys::core::BOOL, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn OleCreatePropertyFrame(hwndowner : super::super::Foundation:: HWND, x : u32, y : u32, lpszcaption : windows_sys::core::PCWSTR, cobjects : u32, ppunk : *const * mut core::ffi::c_void, cpages : u32, ppageclsid : *const windows_sys::core::GUID, lcid : u32, dwreserved : u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn OleCreatePropertyFrameIndirect(lpparams : *const OCPFIPARAMS) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleCreateStaticFromData(psrcdataobj : * mut core::ffi::c_void, iid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleDestroyMenuDescriptor(holemenu : isize) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleDoAutoConvert(pstg : * mut core::ffi::c_void, pclsidnew : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn OleDraw(punknown : * mut core::ffi::c_void, dwaspect : u32, hdcdraw : super::super::Graphics::Gdi:: HDC, lprcbounds : *const super::super::Foundation:: RECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Memory")]
windows_targets::link!("ole32.dll" "system" fn OleDuplicateData(hsrc : super::super::Foundation:: HANDLE, cfformat : CLIPBOARD_FORMAT, uiflags : super::Memory:: GLOBAL_ALLOC_FLAGS) -> super::super::Foundation:: HANDLE);
windows_targets::link!("ole32.dll" "system" fn OleFlushClipboard() -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleGetAutoConvert(clsidold : *const windows_sys::core::GUID, pclsidnew : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn OleGetClipboard(ppdataobj : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn OleGetClipboardWithEnterpriseInfo(dataobject : *mut * mut core::ffi::c_void, dataenterpriseid : *mut windows_sys::core::PWSTR, sourcedescription : *mut windows_sys::core::PWSTR, targetdescription : *mut windows_sys::core::PWSTR, datadescription : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleGetIconOfClass(rclsid : *const windows_sys::core::GUID, lpszlabel : windows_sys::core::PCWSTR, fusetypeaslabel : windows_sys::core::BOOL) -> super::super::Foundation:: HGLOBAL);
windows_targets::link!("ole32.dll" "system" fn OleGetIconOfFile(lpszpath : windows_sys::core::PCWSTR, fusefileaslabel : windows_sys::core::BOOL) -> super::super::Foundation:: HGLOBAL);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("oleaut32.dll" "system" fn OleIconToCursor(hinstexe : super::super::Foundation:: HINSTANCE, hicon : super::super::UI::WindowsAndMessaging:: HICON) -> super::super::UI::WindowsAndMessaging:: HCURSOR);
windows_targets::link!("ole32.dll" "system" fn OleInitialize(pvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn OleIsCurrentClipboard(pdataobj : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleIsRunning(pobject : * mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleLoad(pstg : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, pclientsite : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn OleLoadFromStream(pstm : * mut core::ffi::c_void, iidinterface : *const windows_sys::core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn OleLoadPicture(lpstream : * mut core::ffi::c_void, lsize : i32, frunmode : windows_sys::core::BOOL, riid : *const windows_sys::core::GUID, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn OleLoadPictureEx(lpstream : * mut core::ffi::c_void, lsize : i32, frunmode : windows_sys::core::BOOL, riid : *const windows_sys::core::GUID, xsizedesired : u32, ysizedesired : u32, dwflags : LOAD_PICTURE_FLAGS, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn OleLoadPictureFile(varfilename : super::Variant:: VARIANT, lplpdisppicture : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn OleLoadPictureFileEx(varfilename : super::Variant:: VARIANT, xsizedesired : u32, ysizedesired : u32, dwflags : LOAD_PICTURE_FLAGS, lplpdisppicture : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn OleLoadPicturePath(szurlorpath : windows_sys::core::PCWSTR, punkcaller : * mut core::ffi::c_void, dwreserved : u32, clrreserved : u32, riid : *const windows_sys::core::GUID, ppvret : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleLockRunning(punknown : * mut core::ffi::c_void, flock : windows_sys::core::BOOL, flastunlockcloses : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn OleMetafilePictFromIconAndLabel(hicon : super::super::UI::WindowsAndMessaging:: HICON, lpszlabel : windows_sys::core::PCWSTR, lpszsourcefile : windows_sys::core::PCWSTR, iiconindex : u32) -> super::super::Foundation:: HGLOBAL);
windows_targets::link!("ole32.dll" "system" fn OleNoteObjectVisible(punknown : * mut core::ffi::c_void, fvisible : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn OleQueryCreateFromData(psrcdataobject : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn OleQueryLinkFromData(psrcdataobject : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn OleRegEnumFormatEtc(clsid : *const windows_sys::core::GUID, dwdirection : u32, ppenum : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleRegEnumVerbs(clsid : *const windows_sys::core::GUID, ppenum : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleRegGetMiscStatus(clsid : *const windows_sys::core::GUID, dwaspect : u32, pdwstatus : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleRegGetUserType(clsid : *const windows_sys::core::GUID, dwformoftype : u32, pszusertype : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleRun(punknown : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("ole32.dll" "system" fn OleSave(pps : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, fsameasload : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn OleSavePictureFile(lpdisppicture : * mut core::ffi::c_void, bstrfilename : windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn OleSaveToStream(ppstm : * mut core::ffi::c_void, pstm : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleSetAutoConvert(clsidold : *const windows_sys::core::GUID, clsidnew : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("ole32.dll" "system" fn OleSetClipboard(pdataobj : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleSetContainedObject(punknown : * mut core::ffi::c_void, fcontained : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn OleSetMenuDescriptor(holemenu : isize, hwndframe : super::super::Foundation:: HWND, hwndactiveobject : super::super::Foundation:: HWND, lpframe : * mut core::ffi::c_void, lpactiveobj : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn OleTranslateAccelerator(lpframe : * mut core::ffi::c_void, lpframeinfo : *const OLEINPLACEFRAMEINFO, lpmsg : *const super::super::UI::WindowsAndMessaging:: MSG) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("oleaut32.dll" "system" fn OleTranslateColor(clr : u32, hpal : super::super::Graphics::Gdi:: HPALETTE, lpcolorref : *mut super::super::Foundation:: COLORREF) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("oledlg.dll" "system" fn OleUIAddVerbMenuA(lpoleobj : * mut core::ffi::c_void, lpszshorttype : windows_sys::core::PCSTR, hmenu : super::super::UI::WindowsAndMessaging:: HMENU, upos : u32, uidverbmin : u32, uidverbmax : u32, baddconvert : windows_sys::core::BOOL, idconvert : u32, lphmenu : *mut super::super::UI::WindowsAndMessaging:: HMENU) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("oledlg.dll" "system" fn OleUIAddVerbMenuW(lpoleobj : * mut core::ffi::c_void, lpszshorttype : windows_sys::core::PCWSTR, hmenu : super::super::UI::WindowsAndMessaging:: HMENU, upos : u32, uidverbmin : u32, uidverbmax : u32, baddconvert : windows_sys::core::BOOL, idconvert : u32, lphmenu : *mut super::super::UI::WindowsAndMessaging:: HMENU) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Media")]
windows_targets::link!("oledlg.dll" "system" fn OleUIBusyA(param0 : *const OLEUIBUSYA) -> u32);
#[cfg(feature = "Win32_Media")]
windows_targets::link!("oledlg.dll" "system" fn OleUIBusyW(param0 : *const OLEUIBUSYW) -> u32);
windows_targets::link!("oledlg.dll" "system" fn OleUICanConvertOrActivateAs(rclsid : *const windows_sys::core::GUID, fislinkedobject : windows_sys::core::BOOL, wformat : u16) -> windows_sys::core::BOOL);
windows_targets::link!("oledlg.dll" "system" fn OleUIChangeIconA(param0 : *const OLEUICHANGEICONA) -> u32);
windows_targets::link!("oledlg.dll" "system" fn OleUIChangeIconW(param0 : *const OLEUICHANGEICONW) -> u32);
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
windows_targets::link!("oledlg.dll" "system" fn OleUIChangeSourceA(param0 : *const OLEUICHANGESOURCEA) -> u32);
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
windows_targets::link!("oledlg.dll" "system" fn OleUIChangeSourceW(param0 : *const OLEUICHANGESOURCEW) -> u32);
windows_targets::link!("oledlg.dll" "system" fn OleUIConvertA(param0 : *const OLEUICONVERTA) -> u32);
windows_targets::link!("oledlg.dll" "system" fn OleUIConvertW(param0 : *const OLEUICONVERTW) -> u32);
windows_targets::link!("oledlg.dll" "system" fn OleUIEditLinksA(param0 : *const OLEUIEDITLINKSA) -> u32);
windows_targets::link!("oledlg.dll" "system" fn OleUIEditLinksW(param0 : *const OLEUIEDITLINKSW) -> u32);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("oledlg.dll" "system" fn OleUIInsertObjectA(param0 : *const OLEUIINSERTOBJECTA) -> u32);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_targets::link!("oledlg.dll" "system" fn OleUIInsertObjectW(param0 : *const OLEUIINSERTOBJECTW) -> u32);
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
windows_targets::link!("oledlg.dll" "system" fn OleUIObjectPropertiesA(param0 : *const OLEUIOBJECTPROPSA) -> u32);
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
windows_targets::link!("oledlg.dll" "system" fn OleUIObjectPropertiesW(param0 : *const OLEUIOBJECTPROPSW) -> u32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oledlg.dll" "system" fn OleUIPasteSpecialA(param0 : *const OLEUIPASTESPECIALA) -> u32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oledlg.dll" "system" fn OleUIPasteSpecialW(param0 : *const OLEUIPASTESPECIALW) -> u32);
windows_targets::link!("oledlg.dll" "C" fn OleUIPromptUserA(ntemplate : i32, hwndparent : super::super::Foundation:: HWND, ...) -> i32);
windows_targets::link!("oledlg.dll" "C" fn OleUIPromptUserW(ntemplate : i32, hwndparent : super::super::Foundation:: HWND, ...) -> i32);
windows_targets::link!("oledlg.dll" "system" fn OleUIUpdateLinksA(lpoleuilinkcntr : * mut core::ffi::c_void, hwndparent : super::super::Foundation:: HWND, lpsztitle : windows_sys::core::PCSTR, clinks : i32) -> windows_sys::core::BOOL);
windows_targets::link!("oledlg.dll" "system" fn OleUIUpdateLinksW(lpoleuilinkcntr : * mut core::ffi::c_void, hwndparent : super::super::Foundation:: HWND, lpsztitle : windows_sys::core::PCWSTR, clinks : i32) -> windows_sys::core::BOOL);
windows_targets::link!("ole32.dll" "system" fn OleUninitialize());
windows_targets::link!("oleaut32.dll" "system" fn QueryPathOfRegTypeLib(guid : *const windows_sys::core::GUID, wmaj : u16, wmin : u16, lcid : u32, lpbstrpathname : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn RegisterActiveObject(punk : * mut core::ffi::c_void, rclsid : *const windows_sys::core::GUID, dwflags : ACTIVEOBJECT_FLAGS, pdwregister : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn RegisterDragDrop(hwnd : super::super::Foundation:: HWND, pdroptarget : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn RegisterTypeLib(ptlib : * mut core::ffi::c_void, szfullpath : windows_sys::core::PCWSTR, szhelpdir : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn RegisterTypeLibForUser(ptlib : * mut core::ffi::c_void, szfullpath : windows_sys::core::PCWSTR, szhelpdir : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
windows_targets::link!("ole32.dll" "system" fn ReleaseStgMedium(param0 : *mut super::Com:: STGMEDIUM));
windows_targets::link!("oleaut32.dll" "system" fn RevokeActiveObject(dwregister : u32, pvreserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn RevokeDragDrop(hwnd : super::super::Foundation:: HWND) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayAccessData(psa : *const super::Com:: SAFEARRAY, ppvdata : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayAddRef(psa : *const super::Com:: SAFEARRAY, ppdatatorelease : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayAllocData(psa : *const super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptor(cdims : u32, ppsaout : *mut *mut super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptorEx(vt : super::Variant:: VARENUM, cdims : u32, ppsaout : *mut *mut super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCopy(psa : *const super::Com:: SAFEARRAY, ppsaout : *mut *mut super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCopyData(psasource : *const super::Com:: SAFEARRAY, psatarget : *const super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCreate(vt : super::Variant:: VARENUM, cdims : u32, rgsabound : *const super::Com:: SAFEARRAYBOUND) -> *mut super::Com:: SAFEARRAY);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCreateEx(vt : super::Variant:: VARENUM, cdims : u32, rgsabound : *const super::Com:: SAFEARRAYBOUND, pvextra : *const core::ffi::c_void) -> *mut super::Com:: SAFEARRAY);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCreateVector(vt : super::Variant:: VARENUM, llbound : i32, celements : u32) -> *mut super::Com:: SAFEARRAY);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCreateVectorEx(vt : super::Variant:: VARENUM, llbound : i32, celements : u32, pvextra : *const core::ffi::c_void) -> *mut super::Com:: SAFEARRAY);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayDestroy(psa : *const super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayDestroyData(psa : *const super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayDestroyDescriptor(psa : *const super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetDim(psa : *const super::Com:: SAFEARRAY) -> u32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetElement(psa : *const super::Com:: SAFEARRAY, rgindices : *const i32, pv : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetElemsize(psa : *const super::Com:: SAFEARRAY) -> u32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetIID(psa : *const super::Com:: SAFEARRAY, pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetLBound(psa : *const super::Com:: SAFEARRAY, ndim : u32, pllbound : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetRecordInfo(psa : *const super::Com:: SAFEARRAY, prinfo : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetUBound(psa : *const super::Com:: SAFEARRAY, ndim : u32, plubound : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetVartype(psa : *const super::Com:: SAFEARRAY, pvt : *mut super::Variant:: VARENUM) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayLock(psa : *const super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayPtrOfIndex(psa : *const super::Com:: SAFEARRAY, rgindices : *const i32, ppvdata : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayPutElement(psa : *const super::Com:: SAFEARRAY, rgindices : *const i32, pv : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayRedim(psa : *mut super::Com:: SAFEARRAY, psaboundnew : *const super::Com:: SAFEARRAYBOUND) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayReleaseData(pdata : *const core::ffi::c_void));
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayReleaseDescriptor(psa : *const super::Com:: SAFEARRAY));
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArraySetIID(psa : *const super::Com:: SAFEARRAY, guid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArraySetRecordInfo(psa : *const super::Com:: SAFEARRAY, prinfo : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayUnaccessData(psa : *const super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn SafeArrayUnlock(psa : *const super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn UnRegisterTypeLib(libid : *const windows_sys::core::GUID, wvermajor : u16, wverminor : u16, lcid : u32, syskind : super::Com:: SYSKIND) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn UnRegisterTypeLibForUser(libid : *const windows_sys::core::GUID, wmajorvernum : u16, wminorvernum : u16, lcid : u32, syskind : super::Com:: SYSKIND) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarAbs(pvarin : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarAdd(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarAnd(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromCy(cyin : super::Com:: CY, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromDate(datein : f64, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromDec(pdecin : *const super::super::Foundation:: DECIMAL, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromI1(cin : i8, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromI2(sin : i16, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromI4(lin : i32, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromI8(i64in : i64, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromR4(fltin : f32, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromR8(dblin : f64, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromUI1(bin : u8, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromUI2(uiin : u16, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromUI4(ulin : u32, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromUI8(i64in : u64, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrCat(bstrleft : windows_sys::core::BSTR, bstrright : windows_sys::core::BSTR, pbstrresult : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrCmp(bstrleft : windows_sys::core::BSTR, bstrright : windows_sys::core::BSTR, lcid : u32, dwflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromBool(boolin : super::super::Foundation:: VARIANT_BOOL, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromCy(cyin : super::Com:: CY, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromDate(datein : f64, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromDec(pdecin : *const super::super::Foundation:: DECIMAL, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromI1(cin : i8, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromI2(ival : i16, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromI4(lin : i32, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromI8(i64in : i64, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromR4(fltin : f32, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromR8(dblin : f64, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromUI1(bval : u8, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromUI2(uiin : u16, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromUI4(ulin : u32, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromUI8(ui64in : u64, lcid : u32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarCat(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarCmp(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, lcid : u32, dwflags : u32) -> VARCMP);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyAbs(cyin : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyAdd(cyleft : super::Com:: CY, cyright : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyCmp(cyleft : super::Com:: CY, cyright : super::Com:: CY) -> VARCMP);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyCmpR8(cyleft : super::Com:: CY, dblright : f64) -> VARCMP);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFix(cyin : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromDate(datein : f64, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromDec(pdecin : *const super::super::Foundation:: DECIMAL, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromI1(cin : i8, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromI2(sin : i16, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromI4(lin : i32, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromI8(i64in : i64, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromR4(fltin : f32, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromR8(dblin : f64, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromUI1(bin : u8, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromUI2(uiin : u16, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromUI4(ulin : u32, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyFromUI8(ui64in : u64, pcyout : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyInt(cyin : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyMul(cyleft : super::Com:: CY, cyright : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyMulI4(cyleft : super::Com:: CY, lright : i32, pcyresult : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyMulI8(cyleft : super::Com:: CY, lright : i64, pcyresult : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyNeg(cyin : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCyRound(cyin : super::Com:: CY, cdecimals : i32, pcyresult : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarCySub(cyleft : super::Com:: CY, cyright : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pdateout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromCy(cyin : super::Com:: CY, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromDec(pdecin : *const super::super::Foundation:: DECIMAL, pdateout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromI1(cin : i8, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromI2(sin : i16, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromI4(lin : i32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromI8(i64in : i64, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromR4(fltin : f32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromR8(dblin : f64, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUI1(bin : u8, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUI2(uiin : u16, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUI4(ulin : u32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUI8(ui64in : u64, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUdate(pudatein : *const UDATE, dwflags : u32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUdateEx(pudatein : *const UDATE, lcid : u32, dwflags : u32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecAbs(pdecin : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecAdd(pdecleft : *const super::super::Foundation:: DECIMAL, pdecright : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecCmp(pdecleft : *const super::super::Foundation:: DECIMAL, pdecright : *const super::super::Foundation:: DECIMAL) -> VARCMP);
windows_targets::link!("oleaut32.dll" "system" fn VarDecCmpR8(pdecleft : *const super::super::Foundation:: DECIMAL, dblright : f64) -> VARCMP);
windows_targets::link!("oleaut32.dll" "system" fn VarDecDiv(pdecleft : *const super::super::Foundation:: DECIMAL, pdecright : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFix(pdecin : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromCy(cyin : super::Com:: CY, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromDate(datein : f64, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromI1(cin : i8, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromI2(uiin : i16, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromI4(lin : i32, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromI8(i64in : i64, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromR4(fltin : f32, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromR8(dblin : f64, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromUI1(bin : u8, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromUI2(uiin : u16, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromUI4(ulin : u32, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecFromUI8(ui64in : u64, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecInt(pdecin : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecMul(pdecleft : *const super::super::Foundation:: DECIMAL, pdecright : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecNeg(pdecin : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecRound(pdecin : *const super::super::Foundation:: DECIMAL, cdecimals : i32, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarDecSub(pdecleft : *const super::super::Foundation:: DECIMAL, pdecright : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarDiv(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarEqv(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarFix(pvarin : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarFormat(pvarin : *const super::Variant:: VARIANT, pstrformat : windows_sys::core::PCWSTR, ifirstday : VARFORMAT_FIRST_DAY, ifirstweek : VARFORMAT_FIRST_WEEK, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarFormatCurrency(pvarin : *const super::Variant:: VARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarFormatDateTime(pvarin : *const super::Variant:: VARIANT, inamedformat : VARFORMAT_NAMED_FORMAT, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarFormatFromTokens(pvarin : *const super::Variant:: VARIANT, pstrformat : windows_sys::core::PCWSTR, pbtokcur : *const u8, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR, lcid : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarFormatNumber(pvarin : *const super::Variant:: VARIANT, inumdig : i32, iinclead : VARFORMAT_LEADING_DIGIT, iuseparens : VARFORMAT_PARENTHESES, igroup : VARFORMAT_GROUP, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarFormatPercent(pvarin : *const super::Variant:: VARIANT, inumdig : i32, iinclead : VARFORMAT_LEADING_DIGIT, iuseparens : VARFORMAT_PARENTHESES, igroup : VARFORMAT_GROUP, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromCy(cyin : super::Com:: CY, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromDate(datein : f64, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromI2(uiin : i16, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromI4(lin : i32, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromI8(i64in : i64, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromR4(fltin : f32, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromR8(dblin : f64, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromUI1(bin : u8, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromUI2(uiin : u16, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromUI4(ulin : u32, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI1FromUI8(i64in : u64, pcout : windows_sys::core::PSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, psout : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromCy(cyin : super::Com:: CY, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromDate(datein : f64, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromDec(pdecin : *const super::super::Foundation:: DECIMAL, psout : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromI1(cin : i8, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromI4(lin : i32, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromI8(i64in : i64, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromR4(fltin : f32, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromR8(dblin : f64, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromUI1(bin : u8, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromUI2(uiin : u16, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromUI4(ulin : u32, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI2FromUI8(ui64in : u64, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, plout : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromCy(cyin : super::Com:: CY, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromDate(datein : f64, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromDec(pdecin : *const super::super::Foundation:: DECIMAL, plout : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromI1(cin : i8, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromI2(sin : i16, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromI8(i64in : i64, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromR4(fltin : f32, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromR8(dblin : f64, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromUI1(bin : u8, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromUI2(uiin : u16, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromUI4(ulin : u32, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI4FromUI8(ui64in : u64, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pi64out : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromCy(cyin : super::Com:: CY, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromDate(datein : f64, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pi64out : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromI1(cin : i8, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromI2(sin : i16, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromR4(fltin : f32, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromR8(dblin : f64, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromUI1(bin : u8, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromUI2(uiin : u16, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromUI4(ulin : u32, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarI8FromUI8(ui64in : u64, pi64out : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarIdiv(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarImp(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarInt(pvarin : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarMod(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarMonthName(imonth : i32, fabbrev : i32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarMul(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarNeg(pvarin : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarNot(pvarin : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarNumFromParseNum(pnumprs : *const NUMPARSE, rgbdig : *const u8, dwvtbits : u32, pvar : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarOr(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarParseNumFromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pnumprs : *mut NUMPARSE, rgbdig : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarPow(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4CmpR8(fltleft : f32, dblright : f64) -> VARCMP);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pfltout : *mut f32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromCy(cyin : super::Com:: CY, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromDate(datein : f64, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pfltout : *mut f32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromI1(cin : i8, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromI2(sin : i16, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromI4(lin : i32, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromI8(i64in : i64, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromR8(dblin : f64, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromUI1(bin : u8, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromUI2(uiin : u16, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromUI4(ulin : u32, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR4FromUI8(ui64in : u64, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pdblout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromCy(cyin : super::Com:: CY, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromDate(datein : f64, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pdblout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromI1(cin : i8, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromI2(sin : i16, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromI4(lin : i32, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromI8(i64in : i64, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromR4(fltin : f32, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromUI1(bin : u8, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromUI2(uiin : u16, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromUI4(ulin : u32, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8FromUI8(ui64in : u64, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8Pow(dblleft : f64, dblright : f64, pdblresult : *mut f64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarR8Round(dblin : f64, cdecimals : i32, pdblresult : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarRound(pvarin : *const super::Variant:: VARIANT, cdecimals : i32, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarSub(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarTokenizeFormatString(pstrformat : windows_sys::core::PCWSTR, rgbtok : *mut u8, cbtok : i32, ifirstday : VARFORMAT_FIRST_DAY, ifirstweek : VARFORMAT_FIRST_WEEK, lcid : u32, pcbactual : *const i32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pbout : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromCy(cyin : super::Com:: CY, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromDate(datein : f64, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pbout : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromI1(cin : i8, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromI2(sin : i16, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromI4(lin : i32, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromI8(i64in : i64, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromR4(fltin : f32, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromR8(dblin : f64, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromUI2(uiin : u16, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromUI4(ulin : u32, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromUI8(ui64in : u64, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, puiout : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromCy(cyin : super::Com:: CY, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromDate(datein : f64, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromDec(pdecin : *const super::super::Foundation:: DECIMAL, puiout : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromI1(cin : i8, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromI2(uiin : i16, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromI4(lin : i32, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromI8(i64in : i64, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromR4(fltin : f32, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromR8(dblin : f64, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromUI1(bin : u8, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromUI4(ulin : u32, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromUI8(i64in : u64, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pulout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromCy(cyin : super::Com:: CY, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromDate(datein : f64, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pulout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromI1(cin : i8, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromI2(uiin : i16, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromI4(lin : i32, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromI8(i64in : i64, plout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromR4(fltin : f32, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromR8(dblin : f64, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromUI1(bin : u8, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromUI2(uiin : u16, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromUI8(ui64in : u64, plout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pi64out : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromCy(cyin : super::Com:: CY, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromDate(datein : f64, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pi64out : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromI1(cin : i8, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromI2(sin : i16, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromI8(ui64in : i64, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromR4(fltin : f32, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromR8(dblin : f64, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromStr(strin : windows_sys::core::PCWSTR, lcid : u32, dwflags : u32, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromUI1(bin : u8, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromUI2(uiin : u16, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromUI4(ulin : u32, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarUdateFromDate(datein : f64, dwflags : u32, pudateout : *mut UDATE) -> windows_sys::core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VarWeekdayName(iweekday : i32, fabbrev : i32, ifirstday : i32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
windows_targets::link!("oleaut32.dll" "system" fn VarXor(pvarleft : *const super::Variant:: VARIANT, pvarright : *const super::Variant:: VARIANT, pvarresult : *mut super::Variant:: VARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("oleaut32.dll" "system" fn VectorFromBstr(bstr : windows_sys::core::BSTR, ppsa : *mut *mut super::Com:: SAFEARRAY) -> windows_sys::core::HRESULT);
pub type ACTIVATEFLAGS = i32;
pub const ACTIVATE_WINDOWLESS: ACTIVATEFLAGS = 1i32;
pub type ACTIVEOBJECT_FLAGS = u32;
pub const ACTIVEOBJECT_STRONG: ACTIVEOBJECT_FLAGS = 0u32;
pub const ACTIVEOBJECT_WEAK: ACTIVEOBJECT_FLAGS = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct ARRAYDESC {
    pub tdescElem: super::Com::TYPEDESC,
    pub cDims: u16,
    pub rgbounds: [super::Com::SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl Default for ARRAYDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BINDSPEED = i32;
pub const BINDSPEED_IMMEDIATE: BINDSPEED = 3i32;
pub const BINDSPEED_INDEFINITE: BINDSPEED = 1i32;
pub const BINDSPEED_MODERATE: BINDSPEED = 2i32;
pub type BUSY_DIALOG_FLAGS = u32;
pub const BZ_DISABLECANCELBUTTON: BUSY_DIALOG_FLAGS = 1u32;
pub const BZ_DISABLERETRYBUTTON: BUSY_DIALOG_FLAGS = 4u32;
pub const BZ_DISABLESWITCHTOBUTTON: BUSY_DIALOG_FLAGS = 2u32;
pub const BZ_NOTRESPONDINGDIALOG: BUSY_DIALOG_FLAGS = 8u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CADWORD {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl Default for CADWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CALPOLESTR {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::PWSTR,
}
impl Default for CALPOLESTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAUUID {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::GUID,
}
impl Default for CAUUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CF_BITMAP: CLIPBOARD_FORMAT = 2u16;
pub const CF_CONVERTONLY: UI_CONVERT_FLAGS = 256u32;
pub const CF_DIB: CLIPBOARD_FORMAT = 8u16;
pub const CF_DIBV5: CLIPBOARD_FORMAT = 17u16;
pub const CF_DIF: CLIPBOARD_FORMAT = 5u16;
pub const CF_DISABLEACTIVATEAS: UI_CONVERT_FLAGS = 64u32;
pub const CF_DISABLEDISPLAYASICON: UI_CONVERT_FLAGS = 32u32;
pub const CF_DSPBITMAP: CLIPBOARD_FORMAT = 130u16;
pub const CF_DSPENHMETAFILE: CLIPBOARD_FORMAT = 142u16;
pub const CF_DSPMETAFILEPICT: CLIPBOARD_FORMAT = 131u16;
pub const CF_DSPTEXT: CLIPBOARD_FORMAT = 129u16;
pub const CF_ENHMETAFILE: CLIPBOARD_FORMAT = 14u16;
pub const CF_GDIOBJFIRST: CLIPBOARD_FORMAT = 768u16;
pub const CF_GDIOBJLAST: CLIPBOARD_FORMAT = 1023u16;
pub const CF_HDROP: CLIPBOARD_FORMAT = 15u16;
pub const CF_HIDECHANGEICON: UI_CONVERT_FLAGS = 128u32;
pub const CF_LOCALE: CLIPBOARD_FORMAT = 16u16;
pub const CF_MAX: CLIPBOARD_FORMAT = 18u16;
pub const CF_METAFILEPICT: CLIPBOARD_FORMAT = 3u16;
pub const CF_OEMTEXT: CLIPBOARD_FORMAT = 7u16;
pub const CF_OWNERDISPLAY: CLIPBOARD_FORMAT = 128u16;
pub const CF_PALETTE: CLIPBOARD_FORMAT = 9u16;
pub const CF_PENDATA: CLIPBOARD_FORMAT = 10u16;
pub const CF_PRIVATEFIRST: CLIPBOARD_FORMAT = 512u16;
pub const CF_PRIVATELAST: CLIPBOARD_FORMAT = 767u16;
pub const CF_RIFF: CLIPBOARD_FORMAT = 11u16;
pub const CF_SELECTACTIVATEAS: UI_CONVERT_FLAGS = 16u32;
pub const CF_SELECTCONVERTTO: UI_CONVERT_FLAGS = 8u32;
pub const CF_SETACTIVATEDEFAULT: UI_CONVERT_FLAGS = 4u32;
pub const CF_SETCONVERTDEFAULT: UI_CONVERT_FLAGS = 2u32;
pub const CF_SHOWHELPBUTTON: UI_CONVERT_FLAGS = 1u32;
pub const CF_SYLK: CLIPBOARD_FORMAT = 4u16;
pub const CF_TEXT: CLIPBOARD_FORMAT = 1u16;
pub const CF_TIFF: CLIPBOARD_FORMAT = 6u16;
pub const CF_UNICODETEXT: CLIPBOARD_FORMAT = 13u16;
pub const CF_WAVE: CLIPBOARD_FORMAT = 12u16;
pub type CHANGEKIND = i32;
pub const CHANGEKIND_ADDMEMBER: CHANGEKIND = 0i32;
pub const CHANGEKIND_CHANGEFAILED: CHANGEKIND = 6i32;
pub const CHANGEKIND_DELETEMEMBER: CHANGEKIND = 1i32;
pub const CHANGEKIND_GENERAL: CHANGEKIND = 4i32;
pub const CHANGEKIND_INVALIDATE: CHANGEKIND = 5i32;
pub const CHANGEKIND_MAX: CHANGEKIND = 7i32;
pub const CHANGEKIND_SETDOCUMENTATION: CHANGEKIND = 3i32;
pub const CHANGEKIND_SETNAMES: CHANGEKIND = 2i32;
pub type CHANGE_ICON_FLAGS = u32;
pub type CHANGE_SOURCE_FLAGS = u32;
pub const CIF_SELECTCURRENT: CHANGE_ICON_FLAGS = 2u32;
pub const CIF_SELECTDEFAULT: CHANGE_ICON_FLAGS = 4u32;
pub const CIF_SELECTFROMFILE: CHANGE_ICON_FLAGS = 8u32;
pub const CIF_SHOWHELP: CHANGE_ICON_FLAGS = 1u32;
pub const CIF_USEICONEXE: CHANGE_ICON_FLAGS = 16u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: *mut core::ffi::c_void,
    pub pStorage: *mut core::ffi::c_void,
    pub flags: u32,
}
impl Default for CLEANLOCALSTORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLIPBOARD_FORMAT = u16;
pub const CLSID_CColorPropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35201_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_CFontPropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35200_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_CPicturePropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35202_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_ConvertVBX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb8f0822_0164_101b_84ed_08002b2ec713);
pub const CLSID_PersistPropset: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb8f0821_0164_101b_84ed_08002b2ec713);
pub const CLSID_StdFont: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35203_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_StdPicture: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35204_8f91_11ce_9de3_00aa004bb851);
pub const CONNECT_E_ADVISELIMIT: windows_sys::core::HRESULT = 0x80040201_u32 as _;
pub const CONNECT_E_CANNOTCONNECT: windows_sys::core::HRESULT = 0x80040202_u32 as _;
pub const CONNECT_E_FIRST: i32 = -2147220992i32;
pub const CONNECT_E_LAST: windows_sys::core::HRESULT = 0x8004020F_u32 as _;
pub const CONNECT_E_NOCONNECTION: windows_sys::core::HRESULT = 0x80040200_u32 as _;
pub const CONNECT_E_OVERRIDDEN: windows_sys::core::HRESULT = 0x80040203_u32 as _;
pub const CONNECT_S_FIRST: windows_sys::core::HRESULT = 0x40200_u32 as _;
pub const CONNECT_S_LAST: windows_sys::core::HRESULT = 0x4020F_u32 as _;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct CONTROLINFO {
    pub cb: u32,
    pub hAccel: super::super::UI::WindowsAndMessaging::HACCEL,
    pub cAccel: u16,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for CONTROLINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CSF_EXPLORER: CHANGE_SOURCE_FLAGS = 8u32;
pub const CSF_ONLYGETSOURCE: CHANGE_SOURCE_FLAGS = 4u32;
pub const CSF_SHOWHELP: CHANGE_SOURCE_FLAGS = 1u32;
pub const CSF_VALIDSOURCE: CHANGE_SOURCE_FLAGS = 2u32;
pub const CTL_E_ILLEGALFUNCTIONCALL: i32 = -2146828283i32;
pub type CTRLINFO = i32;
pub const CTRLINFO_EATS_ESCAPE: CTRLINFO = 2i32;
pub const CTRLINFO_EATS_RETURN: CTRLINFO = 1i32;
pub const DD_DEFDRAGDELAY: u32 = 200u32;
pub const DD_DEFDRAGMINDIST: u32 = 2u32;
pub const DD_DEFSCROLLDELAY: u32 = 50u32;
pub const DD_DEFSCROLLINSET: u32 = 11u32;
pub const DD_DEFSCROLLINTERVAL: u32 = 50u32;
pub type DISCARDCACHE = i32;
pub const DISCARDCACHE_NOSAVE: DISCARDCACHE = 1i32;
pub const DISCARDCACHE_SAVEIFDIRTY: DISCARDCACHE = 0i32;
pub const DISPATCH_CONSTRUCT: u32 = 16384u32;
pub const DISPID_ABOUTBOX: i32 = -552i32;
pub const DISPID_ACCELERATOR: i32 = -543i32;
pub const DISPID_ADDITEM: i32 = -553i32;
pub const DISPID_AMBIENT_APPEARANCE: i32 = -716i32;
pub const DISPID_AMBIENT_AUTOCLIP: i32 = -715i32;
pub const DISPID_AMBIENT_BACKCOLOR: i32 = -701i32;
pub const DISPID_AMBIENT_CHARSET: i32 = -727i32;
pub const DISPID_AMBIENT_CODEPAGE: i32 = -725i32;
pub const DISPID_AMBIENT_DISPLAYASDEFAULT: i32 = -713i32;
pub const DISPID_AMBIENT_DISPLAYNAME: i32 = -702i32;
pub const DISPID_AMBIENT_FONT: i32 = -703i32;
pub const DISPID_AMBIENT_FORECOLOR: i32 = -704i32;
pub const DISPID_AMBIENT_LOCALEID: i32 = -705i32;
pub const DISPID_AMBIENT_MESSAGEREFLECT: i32 = -706i32;
pub const DISPID_AMBIENT_PALETTE: i32 = -726i32;
pub const DISPID_AMBIENT_RIGHTTOLEFT: i32 = -732i32;
pub const DISPID_AMBIENT_SCALEUNITS: i32 = -707i32;
pub const DISPID_AMBIENT_SHOWGRABHANDLES: i32 = -711i32;
pub const DISPID_AMBIENT_SHOWHATCHING: i32 = -712i32;
pub const DISPID_AMBIENT_SUPPORTSMNEMONICS: i32 = -714i32;
pub const DISPID_AMBIENT_TEXTALIGN: i32 = -708i32;
pub const DISPID_AMBIENT_TOPTOBOTTOM: i32 = -733i32;
pub const DISPID_AMBIENT_TRANSFERPRIORITY: i32 = -728i32;
pub const DISPID_AMBIENT_UIDEAD: i32 = -710i32;
pub const DISPID_AMBIENT_USERMODE: i32 = -709i32;
pub const DISPID_APPEARANCE: i32 = -520i32;
pub const DISPID_AUTOSIZE: i32 = -500i32;
pub const DISPID_BACKCOLOR: i32 = -501i32;
pub const DISPID_BACKSTYLE: i32 = -502i32;
pub const DISPID_BORDERCOLOR: i32 = -503i32;
pub const DISPID_BORDERSTYLE: i32 = -504i32;
pub const DISPID_BORDERVISIBLE: i32 = -519i32;
pub const DISPID_BORDERWIDTH: i32 = -505i32;
pub const DISPID_CAPTION: i32 = -518i32;
pub const DISPID_CLEAR: i32 = -554i32;
pub const DISPID_CLICK: i32 = -600i32;
pub const DISPID_CLICK_VALUE: i32 = -610i32;
pub const DISPID_COLLECT: i32 = -8i32;
pub const DISPID_COLUMN: i32 = -529i32;
pub const DISPID_CONSTRUCTOR: i32 = -6i32;
pub const DISPID_DBLCLICK: i32 = -601i32;
pub const DISPID_DESTRUCTOR: i32 = -7i32;
pub const DISPID_DISPLAYSTYLE: i32 = -540i32;
pub const DISPID_DOCLICK: i32 = -551i32;
pub const DISPID_DRAWMODE: i32 = -507i32;
pub const DISPID_DRAWSTYLE: i32 = -508i32;
pub const DISPID_DRAWWIDTH: i32 = -509i32;
pub const DISPID_Delete: i32 = -801i32;
pub const DISPID_ENABLED: i32 = -514i32;
pub const DISPID_ENTERKEYBEHAVIOR: i32 = -544i32;
pub const DISPID_ERROREVENT: i32 = -608i32;
pub const DISPID_EVALUATE: i32 = -5i32;
pub const DISPID_FILLCOLOR: i32 = -510i32;
pub const DISPID_FILLSTYLE: i32 = -511i32;
pub const DISPID_FONT: i32 = -512i32;
pub const DISPID_FONT_BOLD: u32 = 3u32;
pub const DISPID_FONT_CHANGED: u32 = 9u32;
pub const DISPID_FONT_CHARSET: u32 = 8u32;
pub const DISPID_FONT_ITALIC: u32 = 4u32;
pub const DISPID_FONT_NAME: u32 = 0u32;
pub const DISPID_FONT_SIZE: u32 = 2u32;
pub const DISPID_FONT_STRIKE: u32 = 6u32;
pub const DISPID_FONT_UNDER: u32 = 5u32;
pub const DISPID_FONT_WEIGHT: u32 = 7u32;
pub const DISPID_FORECOLOR: i32 = -513i32;
pub const DISPID_GROUPNAME: i32 = -541i32;
pub const DISPID_HWND: i32 = -515i32;
pub const DISPID_IMEMODE: i32 = -542i32;
pub const DISPID_KEYDOWN: i32 = -602i32;
pub const DISPID_KEYPRESS: i32 = -603i32;
pub const DISPID_KEYUP: i32 = -604i32;
pub const DISPID_LIST: i32 = -528i32;
pub const DISPID_LISTCOUNT: i32 = -531i32;
pub const DISPID_LISTINDEX: i32 = -526i32;
pub const DISPID_MAXLENGTH: i32 = -533i32;
pub const DISPID_MOUSEDOWN: i32 = -605i32;
pub const DISPID_MOUSEICON: i32 = -522i32;
pub const DISPID_MOUSEMOVE: i32 = -606i32;
pub const DISPID_MOUSEPOINTER: i32 = -521i32;
pub const DISPID_MOUSEUP: i32 = -607i32;
pub const DISPID_MULTILINE: i32 = -537i32;
pub const DISPID_MULTISELECT: i32 = -532i32;
pub const DISPID_NEWENUM: i32 = -4i32;
pub const DISPID_NUMBEROFCOLUMNS: i32 = -539i32;
pub const DISPID_NUMBEROFROWS: i32 = -538i32;
pub const DISPID_Name: i32 = -800i32;
pub const DISPID_Object: i32 = -802i32;
pub const DISPID_PASSWORDCHAR: i32 = -534i32;
pub const DISPID_PICTURE: i32 = -523i32;
pub const DISPID_PICT_HANDLE: u32 = 0u32;
pub const DISPID_PICT_HEIGHT: u32 = 5u32;
pub const DISPID_PICT_HPAL: u32 = 2u32;
pub const DISPID_PICT_RENDER: u32 = 6u32;
pub const DISPID_PICT_TYPE: u32 = 3u32;
pub const DISPID_PICT_WIDTH: u32 = 4u32;
pub const DISPID_PROPERTYPUT: i32 = -3i32;
pub const DISPID_Parent: i32 = -803i32;
pub const DISPID_READYSTATE: i32 = -525i32;
pub const DISPID_READYSTATECHANGE: i32 = -609i32;
pub const DISPID_REFRESH: i32 = -550i32;
pub const DISPID_REMOVEITEM: i32 = -555i32;
pub const DISPID_RIGHTTOLEFT: i32 = -611i32;
pub const DISPID_SCROLLBARS: i32 = -535i32;
pub const DISPID_SELECTED: i32 = -527i32;
pub const DISPID_SELLENGTH: i32 = -548i32;
pub const DISPID_SELSTART: i32 = -547i32;
pub const DISPID_SELTEXT: i32 = -546i32;
pub const DISPID_STARTENUM: i32 = -1i32;
pub const DISPID_TABKEYBEHAVIOR: i32 = -545i32;
pub const DISPID_TABSTOP: i32 = -516i32;
pub const DISPID_TEXT: i32 = -517i32;
pub const DISPID_THIS: i32 = -613i32;
pub const DISPID_TOPTOBOTTOM: i32 = -612i32;
pub const DISPID_UNKNOWN: i32 = -1i32;
pub const DISPID_VALID: i32 = -524i32;
pub const DISPID_VALUE: u32 = 0u32;
pub const DISPID_WORDWRAP: i32 = -536i32;
pub type DOCMISC = i32;
pub const DOCMISC_CANCREATEMULTIPLEVIEWS: DOCMISC = 1i32;
pub const DOCMISC_CANTOPENEDIT: DOCMISC = 4i32;
pub const DOCMISC_NOFILESUPPORT: DOCMISC = 8i32;
pub const DOCMISC_SUPPORTCOMPLEXRECTANGLES: DOCMISC = 2i32;
pub type DROPEFFECT = u32;
pub const DROPEFFECT_COPY: DROPEFFECT = 1u32;
pub const DROPEFFECT_LINK: DROPEFFECT = 4u32;
pub const DROPEFFECT_MOVE: DROPEFFECT = 2u32;
pub const DROPEFFECT_NONE: DROPEFFECT = 0u32;
pub const DROPEFFECT_SCROLL: DROPEFFECT = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVASPECTINFO {
    pub cb: u32,
    pub dwFlags: u32,
}
pub type DVASPECTINFOFLAG = i32;
pub const DVASPECTINFOFLAG_CANOPTIMIZE: DVASPECTINFOFLAG = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVEXTENTINFO {
    pub cb: u32,
    pub dwExtentMode: u32,
    pub sizelProposed: super::super::Foundation::SIZE,
}
pub type DVEXTENTMODE = i32;
pub const DVEXTENT_CONTENT: DVEXTENTMODE = 0i32;
pub const DVEXTENT_INTEGRAL: DVEXTENTMODE = 1i32;
pub type EDIT_LINKS_FLAGS = u32;
pub const ELF_DISABLECANCELLINK: EDIT_LINKS_FLAGS = 16u32;
pub const ELF_DISABLECHANGESOURCE: EDIT_LINKS_FLAGS = 8u32;
pub const ELF_DISABLEOPENSOURCE: EDIT_LINKS_FLAGS = 4u32;
pub const ELF_DISABLEUPDATENOW: EDIT_LINKS_FLAGS = 2u32;
pub const ELF_SHOWHELP: EDIT_LINKS_FLAGS = 1u32;
pub const EMBDHLP_CREATENOW: EMBDHLP_FLAGS = 0u32;
pub const EMBDHLP_DELAYCREATE: EMBDHLP_FLAGS = 65536u32;
pub type EMBDHLP_FLAGS = u32;
pub const EMBDHLP_INPROC_HANDLER: EMBDHLP_FLAGS = 0u32;
pub const EMBDHLP_INPROC_SERVER: EMBDHLP_FLAGS = 1u32;
pub type ENUM_CONTROLS_WHICH_FLAGS = u32;
pub type FDEX_PROP_FLAGS = u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct FONTDESC {
    pub cbSizeofstruct: u32,
    pub lpstrName: windows_sys::core::PWSTR,
    pub cySize: super::Com::CY,
    pub sWeight: i16,
    pub sCharset: i16,
    pub fItalic: windows_sys::core::BOOL,
    pub fUnderline: windows_sys::core::BOOL,
    pub fStrikethrough: windows_sys::core::BOOL,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for FONTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GCW_WCH_SIBLING: ENUM_CONTROLS_WHICH_FLAGS = 1u32;
pub const GC_WCH_ALL: ENUM_CONTROLS_WHICH_FLAGS = 4u32;
pub const GC_WCH_CONTAINED: ENUM_CONTROLS_WHICH_FLAGS = 3u32;
pub const GC_WCH_CONTAINER: ENUM_CONTROLS_WHICH_FLAGS = 2u32;
pub const GC_WCH_FONLYAFTER: ENUM_CONTROLS_WHICH_FLAGS = 268435456u32;
pub const GC_WCH_FONLYBEFORE: ENUM_CONTROLS_WHICH_FLAGS = 536870912u32;
pub const GC_WCH_FREVERSEDIR: ENUM_CONTROLS_WHICH_FLAGS = 134217728u32;
pub const GC_WCH_FSELECTED: ENUM_CONTROLS_WHICH_FLAGS = 1073741824u32;
pub const GC_WCH_SIBLING: i32 = 1i32;
pub type GUIDKIND = i32;
pub const GUIDKIND_DEFAULT_SOURCE_DISP_IID: GUIDKIND = 1i32;
pub const GUID_CHECKVALUEEXCLUSIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430c_be0f_101a_8bbb_00aa00300cab);
pub const GUID_COLOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504301_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTBOLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430f_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTITALIC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504310_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTNAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430d_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTSIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430e_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTSTRIKETHROUGH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504312_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTUNDERSCORE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504311_be0f_101a_8bbb_00aa00300cab);
pub const GUID_HANDLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504313_be0f_101a_8bbb_00aa00300cab);
pub const GUID_HIMETRIC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504300_be0f_101a_8bbb_00aa00300cab);
pub const GUID_OPTIONVALUEEXCLUSIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430b_be0f_101a_8bbb_00aa00300cab);
pub const GUID_TRISTATE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430a_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XPOS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504306_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XPOSPIXEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504302_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XSIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504308_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XSIZEPIXEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504304_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YPOS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504307_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YPOSPIXEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504303_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YSIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504309_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YSIZEPIXEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504305_be0f_101a_8bbb_00aa00300cab);
pub type HITRESULT = i32;
pub const HITRESULT_CLOSE: HITRESULT = 2i32;
pub const HITRESULT_HIT: HITRESULT = 3i32;
pub const HITRESULT_OUTSIDE: HITRESULT = 0i32;
pub const HITRESULT_TRANSPARENT: HITRESULT = 1i32;
pub const IDC_BZ_ICON: u32 = 601u32;
pub const IDC_BZ_MESSAGE1: u32 = 602u32;
pub const IDC_BZ_RETRY: u32 = 600u32;
pub const IDC_BZ_SWITCHTO: u32 = 604u32;
pub const IDC_CI_BROWSE: u32 = 130u32;
pub const IDC_CI_CURRENT: u32 = 121u32;
pub const IDC_CI_CURRENTICON: u32 = 122u32;
pub const IDC_CI_DEFAULT: u32 = 123u32;
pub const IDC_CI_DEFAULTICON: u32 = 124u32;
pub const IDC_CI_FROMFILE: u32 = 125u32;
pub const IDC_CI_FROMFILEEDIT: u32 = 126u32;
pub const IDC_CI_GROUP: u32 = 120u32;
pub const IDC_CI_ICONDISPLAY: u32 = 131u32;
pub const IDC_CI_ICONLIST: u32 = 127u32;
pub const IDC_CI_LABEL: u32 = 128u32;
pub const IDC_CI_LABELEDIT: u32 = 129u32;
pub const IDC_CV_ACTIVATEAS: u32 = 156u32;
pub const IDC_CV_ACTIVATELIST: u32 = 154u32;
pub const IDC_CV_CHANGEICON: u32 = 153u32;
pub const IDC_CV_CONVERTLIST: u32 = 158u32;
pub const IDC_CV_CONVERTTO: u32 = 155u32;
pub const IDC_CV_DISPLAYASICON: u32 = 152u32;
pub const IDC_CV_ICONDISPLAY: u32 = 165u32;
pub const IDC_CV_OBJECTTYPE: u32 = 150u32;
pub const IDC_CV_RESULTTEXT: u32 = 157u32;
pub const IDC_EL_AUTOMATIC: u32 = 202u32;
pub const IDC_EL_CANCELLINK: u32 = 209u32;
pub const IDC_EL_CHANGESOURCE: u32 = 201u32;
pub const IDC_EL_COL1: u32 = 220u32;
pub const IDC_EL_COL2: u32 = 221u32;
pub const IDC_EL_COL3: u32 = 222u32;
pub const IDC_EL_LINKSLISTBOX: u32 = 206u32;
pub const IDC_EL_LINKSOURCE: u32 = 216u32;
pub const IDC_EL_LINKTYPE: u32 = 217u32;
pub const IDC_EL_MANUAL: u32 = 212u32;
pub const IDC_EL_OPENSOURCE: u32 = 211u32;
pub const IDC_EL_UPDATENOW: u32 = 210u32;
pub const IDC_GP_CONVERT: u32 = 1013u32;
pub const IDC_GP_OBJECTICON: u32 = 1014u32;
pub const IDC_GP_OBJECTLOCATION: u32 = 1022u32;
pub const IDC_GP_OBJECTNAME: u32 = 1009u32;
pub const IDC_GP_OBJECTSIZE: u32 = 1011u32;
pub const IDC_GP_OBJECTTYPE: u32 = 1010u32;
pub const IDC_IO_ADDCONTROL: u32 = 2115u32;
pub const IDC_IO_CHANGEICON: u32 = 2105u32;
pub const IDC_IO_CONTROLTYPELIST: u32 = 2116u32;
pub const IDC_IO_CREATEFROMFILE: u32 = 2101u32;
pub const IDC_IO_CREATENEW: u32 = 2100u32;
pub const IDC_IO_DISPLAYASICON: u32 = 2104u32;
pub const IDC_IO_FILE: u32 = 2106u32;
pub const IDC_IO_FILEDISPLAY: u32 = 2107u32;
pub const IDC_IO_FILETEXT: u32 = 2112u32;
pub const IDC_IO_FILETYPE: u32 = 2113u32;
pub const IDC_IO_ICONDISPLAY: u32 = 2110u32;
pub const IDC_IO_INSERTCONTROL: u32 = 2114u32;
pub const IDC_IO_LINKFILE: u32 = 2102u32;
pub const IDC_IO_OBJECTTYPELIST: u32 = 2103u32;
pub const IDC_IO_OBJECTTYPETEXT: u32 = 2111u32;
pub const IDC_IO_RESULTIMAGE: u32 = 2108u32;
pub const IDC_IO_RESULTTEXT: u32 = 2109u32;
pub const IDC_LP_AUTOMATIC: u32 = 1016u32;
pub const IDC_LP_BREAKLINK: u32 = 1008u32;
pub const IDC_LP_CHANGESOURCE: u32 = 1015u32;
pub const IDC_LP_DATE: u32 = 1018u32;
pub const IDC_LP_LINKSOURCE: u32 = 1012u32;
pub const IDC_LP_MANUAL: u32 = 1017u32;
pub const IDC_LP_OPENSOURCE: u32 = 1006u32;
pub const IDC_LP_TIME: u32 = 1019u32;
pub const IDC_LP_UPDATENOW: u32 = 1007u32;
pub const IDC_OLEUIHELP: u32 = 99u32;
pub const IDC_PS_CHANGEICON: u32 = 508u32;
pub const IDC_PS_DISPLAYASICON: u32 = 506u32;
pub const IDC_PS_DISPLAYLIST: u32 = 505u32;
pub const IDC_PS_ICONDISPLAY: u32 = 507u32;
pub const IDC_PS_PASTE: u32 = 500u32;
pub const IDC_PS_PASTELINK: u32 = 501u32;
pub const IDC_PS_PASTELINKLIST: u32 = 504u32;
pub const IDC_PS_PASTELIST: u32 = 503u32;
pub const IDC_PS_RESULTIMAGE: u32 = 509u32;
pub const IDC_PS_RESULTTEXT: u32 = 510u32;
pub const IDC_PS_SOURCETEXT: u32 = 502u32;
pub const IDC_PU_CONVERT: u32 = 902u32;
pub const IDC_PU_ICON: u32 = 908u32;
pub const IDC_PU_LINKS: u32 = 900u32;
pub const IDC_PU_TEXT: u32 = 901u32;
pub const IDC_UL_METER: u32 = 1029u32;
pub const IDC_UL_PERCENT: u32 = 1031u32;
pub const IDC_UL_PROGRESS: u32 = 1032u32;
pub const IDC_UL_STOP: u32 = 1030u32;
pub const IDC_VP_ASICON: u32 = 1003u32;
pub const IDC_VP_CHANGEICON: u32 = 1001u32;
pub const IDC_VP_EDITABLE: u32 = 1002u32;
pub const IDC_VP_ICONDISPLAY: u32 = 1021u32;
pub const IDC_VP_PERCENT: u32 = 1000u32;
pub const IDC_VP_RELATIVE: u32 = 1005u32;
pub const IDC_VP_RESULTIMAGE: u32 = 1033u32;
pub const IDC_VP_SCALETXT: u32 = 1034u32;
pub const IDC_VP_SPIN: u32 = 1006u32;
pub const IDD_BUSY: u32 = 1006u32;
pub const IDD_CANNOTUPDATELINK: u32 = 1008u32;
pub const IDD_CHANGEICON: u32 = 1001u32;
pub const IDD_CHANGEICONBROWSE: u32 = 1011u32;
pub const IDD_CHANGESOURCE: u32 = 1009u32;
pub const IDD_CHANGESOURCE4: u32 = 1013u32;
pub const IDD_CONVERT: u32 = 1002u32;
pub const IDD_CONVERT4: u32 = 1103u32;
pub const IDD_CONVERTONLY: u32 = 1012u32;
pub const IDD_CONVERTONLY4: u32 = 1104u32;
pub const IDD_EDITLINKS: u32 = 1004u32;
pub const IDD_EDITLINKS4: u32 = 1105u32;
pub const IDD_GNRLPROPS: u32 = 1100u32;
pub const IDD_GNRLPROPS4: u32 = 1106u32;
pub const IDD_INSERTFILEBROWSE: u32 = 1010u32;
pub const IDD_INSERTOBJECT: u32 = 1000u32;
pub const IDD_LINKPROPS: u32 = 1102u32;
pub const IDD_LINKPROPS4: u32 = 1107u32;
pub const IDD_LINKSOURCEUNAVAILABLE: u32 = 1020u32;
pub const IDD_LINKTYPECHANGED: u32 = 1022u32;
pub const IDD_LINKTYPECHANGEDA: u32 = 1026u32;
pub const IDD_LINKTYPECHANGEDW: u32 = 1022u32;
pub const IDD_OUTOFMEMORY: u32 = 1024u32;
pub const IDD_PASTESPECIAL: u32 = 1003u32;
pub const IDD_PASTESPECIAL4: u32 = 1108u32;
pub const IDD_SERVERNOTFOUND: u32 = 1023u32;
pub const IDD_SERVERNOTREG: u32 = 1021u32;
pub const IDD_SERVERNOTREGA: u32 = 1025u32;
pub const IDD_SERVERNOTREGW: u32 = 1021u32;
pub const IDD_UPDATELINKS: u32 = 1007u32;
pub const IDD_VIEWPROPS: u32 = 1101u32;
pub const ID_BROWSE_ADDCONTROL: u32 = 3u32;
pub const ID_BROWSE_CHANGEICON: u32 = 1u32;
pub const ID_BROWSE_CHANGESOURCE: u32 = 4u32;
pub const ID_BROWSE_INSERTFILE: u32 = 2u32;
pub const ID_DEFAULTINST: i32 = -2i32;
pub type IGNOREMIME = i32;
pub const IGNOREMIME_PROMPT: IGNOREMIME = 1i32;
pub const IGNOREMIME_TEXT: IGNOREMIME = 2i32;
pub type INSERT_OBJECT_FLAGS = u32;
pub const INSTALL_SCOPE_INVALID: u32 = 0u32;
pub const INSTALL_SCOPE_MACHINE: u32 = 1u32;
pub const INSTALL_SCOPE_USER: u32 = 2u32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct INTERFACEDATA {
    pub pmethdata: *mut METHODDATA,
    pub cMembers: u32,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl Default for INTERFACEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IOF_CHECKDISPLAYASICON: INSERT_OBJECT_FLAGS = 16u32;
pub const IOF_CHECKLINK: INSERT_OBJECT_FLAGS = 8u32;
pub const IOF_CREATEFILEOBJECT: INSERT_OBJECT_FLAGS = 64u32;
pub const IOF_CREATELINKOBJECT: INSERT_OBJECT_FLAGS = 128u32;
pub const IOF_CREATENEWOBJECT: INSERT_OBJECT_FLAGS = 32u32;
pub const IOF_DISABLEDISPLAYASICON: INSERT_OBJECT_FLAGS = 1024u32;
pub const IOF_DISABLELINK: INSERT_OBJECT_FLAGS = 256u32;
pub const IOF_HIDECHANGEICON: INSERT_OBJECT_FLAGS = 2048u32;
pub const IOF_SELECTCREATECONTROL: INSERT_OBJECT_FLAGS = 8192u32;
pub const IOF_SELECTCREATEFROMFILE: INSERT_OBJECT_FLAGS = 4u32;
pub const IOF_SELECTCREATENEW: INSERT_OBJECT_FLAGS = 2u32;
pub const IOF_SHOWHELP: INSERT_OBJECT_FLAGS = 1u32;
pub const IOF_SHOWINSERTCONTROL: INSERT_OBJECT_FLAGS = 4096u32;
pub const IOF_VERIFYSERVERSEXIST: INSERT_OBJECT_FLAGS = 512u32;
pub type KEYMODIFIERS = u32;
pub const KEYMOD_ALT: KEYMODIFIERS = 4u32;
pub const KEYMOD_CONTROL: KEYMODIFIERS = 2u32;
pub const KEYMOD_SHIFT: KEYMODIFIERS = 1u32;
pub type LIBFLAGS = i32;
pub const LIBFLAG_FCONTROL: LIBFLAGS = 2i32;
pub const LIBFLAG_FHASDISKIMAGE: LIBFLAGS = 8i32;
pub const LIBFLAG_FHIDDEN: LIBFLAGS = 4i32;
pub const LIBFLAG_FRESTRICTED: LIBFLAGS = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LICINFO {
    pub cbLicInfo: i32,
    pub fRuntimeKeyAvail: windows_sys::core::BOOL,
    pub fLicVerified: windows_sys::core::BOOL,
}
pub type LOAD_PICTURE_FLAGS = u32;
pub const LOAD_TLB_AS_32BIT: u32 = 32u32;
pub const LOAD_TLB_AS_64BIT: u32 = 64u32;
pub const LOCALE_USE_NLS: u32 = 268435456u32;
pub type LPFNOLEUIHOOK = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::WPARAM, param3: super::super::Foundation::LPARAM) -> u32>;
pub const LP_COLOR: LOAD_PICTURE_FLAGS = 4u32;
pub const LP_DEFAULT: LOAD_PICTURE_FLAGS = 0u32;
pub const LP_MONOCHROME: LOAD_PICTURE_FLAGS = 1u32;
pub const LP_VGACOLOR: LOAD_PICTURE_FLAGS = 2u32;
pub const MEDIAPLAYBACK_PAUSE: MEDIAPLAYBACK_STATE = 1i32;
pub const MEDIAPLAYBACK_PAUSE_AND_SUSPEND: MEDIAPLAYBACK_STATE = 2i32;
pub const MEDIAPLAYBACK_RESUME: MEDIAPLAYBACK_STATE = 0i32;
pub const MEDIAPLAYBACK_RESUME_FROM_SUSPEND: MEDIAPLAYBACK_STATE = 3i32;
pub type MEDIAPLAYBACK_STATE = i32;
pub const MEMBERID_NIL: i32 = -1i32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct METHODDATA {
    pub szName: windows_sys::core::PWSTR,
    pub ppdata: *mut PARAMDATA,
    pub dispid: i32,
    pub iMeth: u32,
    pub cc: super::Com::CALLCONV,
    pub cArgs: u32,
    pub wFlags: u16,
    pub vtReturn: super::Variant::VARENUM,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl Default for METHODDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MK_ALT: u32 = 32u32;
pub const MSOCMDERR_E_CANCELED: i32 = -2147221245i32;
pub const MSOCMDERR_E_DISABLED: i32 = -2147221247i32;
pub const MSOCMDERR_E_FIRST: i32 = -2147221248i32;
pub const MSOCMDERR_E_NOHELP: i32 = -2147221246i32;
pub const MSOCMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
pub const MSOCMDERR_E_UNKNOWNGROUP: i32 = -2147221244i32;
pub type MULTICLASSINFO_FLAGS = u32;
pub const MULTICLASSINFO_GETIIDPRIMARY: MULTICLASSINFO_FLAGS = 4u32;
pub const MULTICLASSINFO_GETIIDSOURCE: MULTICLASSINFO_FLAGS = 8u32;
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: MULTICLASSINFO_FLAGS = 2u32;
pub const MULTICLASSINFO_GETTYPEINFO: MULTICLASSINFO_FLAGS = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NUMPARSE {
    pub cDig: i32,
    pub dwInFlags: NUMPARSE_FLAGS,
    pub dwOutFlags: NUMPARSE_FLAGS,
    pub cchUsed: i32,
    pub nBaseShift: i32,
    pub nPwr10: i32,
}
pub type NUMPARSE_FLAGS = u32;
pub const NUMPRS_CURRENCY: NUMPARSE_FLAGS = 1024u32;
pub const NUMPRS_DECIMAL: NUMPARSE_FLAGS = 256u32;
pub const NUMPRS_EXPONENT: NUMPARSE_FLAGS = 2048u32;
pub const NUMPRS_HEX_OCT: NUMPARSE_FLAGS = 64u32;
pub const NUMPRS_INEXACT: NUMPARSE_FLAGS = 131072u32;
pub const NUMPRS_LEADING_MINUS: NUMPARSE_FLAGS = 16u32;
pub const NUMPRS_LEADING_PLUS: NUMPARSE_FLAGS = 4u32;
pub const NUMPRS_LEADING_WHITE: NUMPARSE_FLAGS = 1u32;
pub const NUMPRS_NEG: NUMPARSE_FLAGS = 65536u32;
pub const NUMPRS_PARENS: NUMPARSE_FLAGS = 128u32;
pub const NUMPRS_STD: NUMPARSE_FLAGS = 8191u32;
pub const NUMPRS_THOUSANDS: NUMPARSE_FLAGS = 512u32;
pub const NUMPRS_TRAILING_MINUS: NUMPARSE_FLAGS = 32u32;
pub const NUMPRS_TRAILING_PLUS: NUMPARSE_FLAGS = 8u32;
pub const NUMPRS_TRAILING_WHITE: NUMPARSE_FLAGS = 2u32;
pub const NUMPRS_USE_ALL: NUMPARSE_FLAGS = 4096u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OBJECTDESCRIPTOR {
    pub cbSize: u32,
    pub clsid: windows_sys::core::GUID,
    pub dwDrawAspect: u32,
    pub sizel: super::super::Foundation::SIZE,
    pub pointl: super::super::Foundation::POINTL,
    pub dwStatus: u32,
    pub dwFullUserTypeName: u32,
    pub dwSrcOfCopy: u32,
}
pub type OBJECT_PROPERTIES_FLAGS = u32;
pub const OCM__BASE: u32 = 8192u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OCPFIPARAMS {
    pub cbStructSize: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub x: i32,
    pub y: i32,
    pub lpszCaption: windows_sys::core::PCWSTR,
    pub cObjects: u32,
    pub lplpUnk: *mut *mut core::ffi::c_void,
    pub cPages: u32,
    pub lpPages: *mut windows_sys::core::GUID,
    pub lcid: u32,
    pub dispidInitialProperty: i32,
}
impl Default for OCPFIPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OF_GET: u32 = 2u32;
pub const OF_HANDLER: u32 = 4u32;
pub const OF_SET: u32 = 1u32;
pub type OLECLOSE = i32;
pub const OLECLOSE_NOSAVE: OLECLOSE = 1i32;
pub const OLECLOSE_PROMPTSAVE: OLECLOSE = 2i32;
pub const OLECLOSE_SAVEIFDIRTY: OLECLOSE = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OLECMD {
    pub cmdID: u32,
    pub cmdf: u32,
}
pub const OLECMDARGINDEX_ACTIVEXINSTALL_CLSID: u32 = 2u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_DISPLAYNAME: u32 = 1u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_INSTALLSCOPE: u32 = 3u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_PUBLISHER: u32 = 0u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_SOURCEURL: u32 = 4u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_HWND: u32 = 0u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_X: u32 = 1u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_Y: u32 = 2u32;
pub const OLECMDERR_E_CANCELED: windows_sys::core::HRESULT = 0x80040103_u32 as _;
pub const OLECMDERR_E_DISABLED: windows_sys::core::HRESULT = 0x80040101_u32 as _;
pub const OLECMDERR_E_FIRST: windows_sys::core::HRESULT = 0x80040100_u32 as _;
pub const OLECMDERR_E_NOHELP: windows_sys::core::HRESULT = 0x80040102_u32 as _;
pub const OLECMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
pub const OLECMDERR_E_UNKNOWNGROUP: windows_sys::core::HRESULT = 0x80040104_u32 as _;
pub type OLECMDEXECOPT = i32;
pub const OLECMDEXECOPT_DODEFAULT: OLECMDEXECOPT = 0i32;
pub const OLECMDEXECOPT_DONTPROMPTUSER: OLECMDEXECOPT = 2i32;
pub const OLECMDEXECOPT_PROMPTUSER: OLECMDEXECOPT = 1i32;
pub const OLECMDEXECOPT_SHOWHELP: OLECMDEXECOPT = 3i32;
pub type OLECMDF = i32;
pub const OLECMDF_DEFHIDEONCTXTMENU: OLECMDF = 32i32;
pub const OLECMDF_ENABLED: OLECMDF = 2i32;
pub const OLECMDF_INVISIBLE: OLECMDF = 16i32;
pub const OLECMDF_LATCHED: OLECMDF = 4i32;
pub const OLECMDF_NINCHED: OLECMDF = 8i32;
pub const OLECMDF_SUPPORTED: OLECMDF = 1i32;
pub type OLECMDID = i32;
pub const OLECMDIDF_BROWSERSTATE_BLOCKEDVERSION: OLECMDID_BROWSERSTATEFLAG = 64i32;
pub const OLECMDIDF_BROWSERSTATE_DESKTOPHTMLDIALOG: OLECMDID_BROWSERSTATEFLAG = 32i32;
pub const OLECMDIDF_BROWSERSTATE_EXTENSIONSOFF: OLECMDID_BROWSERSTATEFLAG = 1i32;
pub const OLECMDIDF_BROWSERSTATE_IESECURITY: OLECMDID_BROWSERSTATEFLAG = 2i32;
pub const OLECMDIDF_BROWSERSTATE_PROTECTEDMODE_OFF: OLECMDID_BROWSERSTATEFLAG = 4i32;
pub const OLECMDIDF_BROWSERSTATE_REQUIRESACTIVEX: OLECMDID_BROWSERSTATEFLAG = 16i32;
pub const OLECMDIDF_BROWSERSTATE_RESET: OLECMDID_BROWSERSTATEFLAG = 8i32;
pub const OLECMDIDF_OPTICAL_ZOOM_NOLAYOUT: OLECMDID_OPTICAL_ZOOMFLAG = 16i32;
pub const OLECMDIDF_OPTICAL_ZOOM_NOPERSIST: OLECMDID_OPTICAL_ZOOMFLAG = 1i32;
pub const OLECMDIDF_OPTICAL_ZOOM_NOTRANSIENT: OLECMDID_OPTICAL_ZOOMFLAG = 32i32;
pub const OLECMDIDF_OPTICAL_ZOOM_RELOADFORNEWTAB: OLECMDID_OPTICAL_ZOOMFLAG = 64i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXDISALLOW: OLECMDID_PAGEACTIONFLAG = 16i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = 2i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXTRUSTFAIL: OLECMDID_PAGEACTIONFLAG = 4i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXUNSAFE: OLECMDID_PAGEACTIONFLAG = 32i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = 262144i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERDISABLE: OLECMDID_PAGEACTIONFLAG = 8i32;
pub const OLECMDIDF_PAGEACTION_ACTIVEX_EPM_INCOMPATIBLE: OLECMDID_PAGEACTIONFLAG = 16777216i32;
pub const OLECMDIDF_PAGEACTION_EXTENSION_COMPAT_BLOCKED: OLECMDID_PAGEACTIONFLAG = 268435456i32;
pub const OLECMDIDF_PAGEACTION_FILEDOWNLOAD: OLECMDID_PAGEACTIONFLAG = 1i32;
pub const OLECMDIDF_PAGEACTION_GENERIC_STATE: OLECMDID_PAGEACTIONFLAG = 1073741824i32;
pub const OLECMDIDF_PAGEACTION_INTRANETZONEREQUEST: OLECMDID_PAGEACTIONFLAG = 2097152i32;
pub const OLECMDIDF_PAGEACTION_INVALID_CERT: OLECMDID_PAGEACTIONFLAG = 1048576i32;
pub const OLECMDIDF_PAGEACTION_LOCALMACHINE: OLECMDID_PAGEACTIONFLAG = 128i32;
pub const OLECMDIDF_PAGEACTION_MIMETEXTPLAIN: OLECMDID_PAGEACTIONFLAG = 256i32;
pub const OLECMDIDF_PAGEACTION_MIXEDCONTENT: OLECMDID_PAGEACTIONFLAG = 524288i32;
pub const OLECMDIDF_PAGEACTION_NORESETACTIVEX: OLECMDID_PAGEACTIONFLAG = 536870912i32;
pub const OLECMDIDF_PAGEACTION_POPUPALLOWED: OLECMDID_PAGEACTIONFLAG = 65536i32;
pub const OLECMDIDF_PAGEACTION_POPUPWINDOW: OLECMDID_PAGEACTIONFLAG = 64i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNDENY: OLECMDID_PAGEACTIONFLAG = 32768i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_PAGEACTIONFLAG = 8192i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_PAGEACTIONFLAG = 4096i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_PAGEACTIONFLAG = 1024i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_PAGEACTIONFLAG = 16384i32;
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_PAGEACTIONFLAG = 2048i32;
pub const OLECMDIDF_PAGEACTION_RESET: OLECMDID_PAGEACTIONFLAG = -2147483648i32;
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE: OLECMDID_PAGEACTIONFLAG = 512i32;
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = 512i32;
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = 33554432i32;
pub const OLECMDIDF_PAGEACTION_SCRIPTPROMPT: OLECMDID_PAGEACTIONFLAG = 131072i32;
pub const OLECMDIDF_PAGEACTION_SPOOFABLEIDNHOST: OLECMDID_PAGEACTIONFLAG = 8388608i32;
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED: OLECMDID_PAGEACTIONFLAG = 67108864i32;
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED_ACTIVEX: OLECMDID_PAGEACTIONFLAG = 134217728i32;
pub const OLECMDIDF_PAGEACTION_XSSFILTERED: OLECMDID_PAGEACTIONFLAG = 4194304i32;
pub const OLECMDIDF_REFRESH_CLEARUSERINPUT: OLECMDID_REFRESHFLAG = 4096i32;
pub const OLECMDIDF_REFRESH_COMPLETELY: OLECMDID_REFRESHFLAG = 3i32;
pub const OLECMDIDF_REFRESH_CONTINUE: OLECMDID_REFRESHFLAG = 2i32;
pub const OLECMDIDF_REFRESH_IFEXPIRED: OLECMDID_REFRESHFLAG = 1i32;
pub const OLECMDIDF_REFRESH_LEVELMASK: OLECMDID_REFRESHFLAG = 255i32;
pub const OLECMDIDF_REFRESH_NORMAL: OLECMDID_REFRESHFLAG = 0i32;
pub const OLECMDIDF_REFRESH_NO_CACHE: OLECMDID_REFRESHFLAG = 4i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_ACTIVEXINSTALL: OLECMDID_REFRESHFLAG = 65536i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_ALLOW_VERSION: OLECMDID_REFRESHFLAG = 134217728i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_FILEDOWNLOAD: OLECMDID_REFRESHFLAG = 131072i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_INVALID_CERT: OLECMDID_REFRESHFLAG = 67108864i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_LOCALMACHINE: OLECMDID_REFRESHFLAG = 262144i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_MIXEDCONTENT: OLECMDID_REFRESHFLAG = 33554432i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_POPUPWINDOW: OLECMDID_REFRESHFLAG = 524288i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_REFRESHFLAG = 8388608i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_REFRESHFLAG = 4194304i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_REFRESHFLAG = 1048576i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_REFRESHFLAG = 16777216i32;
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_REFRESHFLAG = 2097152i32;
pub const OLECMDIDF_REFRESH_PROMPTIFOFFLINE: OLECMDID_REFRESHFLAG = 8192i32;
pub const OLECMDIDF_REFRESH_RELOAD: OLECMDID_REFRESHFLAG = 5i32;
pub const OLECMDIDF_REFRESH_SKIPBEFOREUNLOADEVENT: OLECMDID_REFRESHFLAG = 32768i32;
pub const OLECMDIDF_REFRESH_THROUGHSCRIPT: OLECMDID_REFRESHFLAG = 16384i32;
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM: OLECMDID_VIEWPORT_MODE_FLAG = 2i32;
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM_VALID: OLECMDID_VIEWPORT_MODE_FLAG = 131072i32;
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH: OLECMDID_VIEWPORT_MODE_FLAG = 1i32;
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH_VALID: OLECMDID_VIEWPORT_MODE_FLAG = 65536i32;
pub const OLECMDIDF_WINDOWSTATE_ENABLED: OLECMDID_WINDOWSTATE_FLAG = 2i32;
pub const OLECMDIDF_WINDOWSTATE_ENABLED_VALID: OLECMDID_WINDOWSTATE_FLAG = 131072i32;
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE: OLECMDID_WINDOWSTATE_FLAG = 1i32;
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE_VALID: OLECMDID_WINDOWSTATE_FLAG = 65536i32;
pub const OLECMDID_ACTIVEXINSTALLSCOPE: OLECMDID = 66i32;
pub const OLECMDID_ADDTRAVELENTRY: OLECMDID = 60i32;
pub const OLECMDID_ALLOWUILESSSAVEAS: OLECMDID = 46i32;
pub type OLECMDID_BROWSERSTATEFLAG = i32;
pub const OLECMDID_CLEARSELECTION: OLECMDID = 18i32;
pub const OLECMDID_CLOSE: OLECMDID = 45i32;
pub const OLECMDID_COPY: OLECMDID = 12i32;
pub const OLECMDID_CUT: OLECMDID = 11i32;
pub const OLECMDID_DELETE: OLECMDID = 33i32;
pub const OLECMDID_DONTDOWNLOADCSS: OLECMDID = 47i32;
pub const OLECMDID_ENABLE_INTERACTION: OLECMDID = 36i32;
pub const OLECMDID_ENABLE_VISIBILITY: OLECMDID = 77i32;
pub const OLECMDID_EXITFULLSCREEN: OLECMDID = 81i32;
pub const OLECMDID_FIND: OLECMDID = 32i32;
pub const OLECMDID_FOCUSVIEWCONTROLS: OLECMDID = 57i32;
pub const OLECMDID_FOCUSVIEWCONTROLSQUERY: OLECMDID = 58i32;
pub const OLECMDID_GETPRINTTEMPLATE: OLECMDID = 52i32;
pub const OLECMDID_GETUSERSCALABLE: OLECMDID = 75i32;
pub const OLECMDID_GETZOOMRANGE: OLECMDID = 20i32;
pub const OLECMDID_HIDETOOLBARS: OLECMDID = 24i32;
pub const OLECMDID_HTTPEQUIV: OLECMDID = 34i32;
pub const OLECMDID_HTTPEQUIV_DONE: OLECMDID = 35i32;
pub const OLECMDID_LAYOUT_VIEWPORT_WIDTH: OLECMDID = 71i32;
pub const OLECMDID_MEDIA_PLAYBACK: OLECMDID = 78i32;
pub const OLECMDID_NEW: OLECMDID = 2i32;
pub const OLECMDID_ONBEFOREUNLOAD: OLECMDID = 83i32;
pub const OLECMDID_ONTOOLBARACTIVATED: OLECMDID = 31i32;
pub const OLECMDID_ONUNLOAD: OLECMDID = 37i32;
pub const OLECMDID_OPEN: OLECMDID = 1i32;
pub const OLECMDID_OPTICAL_GETZOOMRANGE: OLECMDID = 64i32;
pub const OLECMDID_OPTICAL_ZOOM: OLECMDID = 63i32;
pub type OLECMDID_OPTICAL_ZOOMFLAG = i32;
pub const OLECMDID_PAGEACTIONBLOCKED: OLECMDID = 55i32;
pub type OLECMDID_PAGEACTIONFLAG = i32;
pub const OLECMDID_PAGEACTIONUIQUERY: OLECMDID = 56i32;
pub const OLECMDID_PAGEAVAILABLE: OLECMDID = 74i32;
pub const OLECMDID_PAGESETUP: OLECMDID = 8i32;
pub const OLECMDID_PASTE: OLECMDID = 13i32;
pub const OLECMDID_PASTESPECIAL: OLECMDID = 14i32;
pub const OLECMDID_POPSTATEEVENT: OLECMDID = 69i32;
pub const OLECMDID_PREREFRESH: OLECMDID = 39i32;
pub const OLECMDID_PRINT: OLECMDID = 6i32;
pub const OLECMDID_PRINT2: OLECMDID = 49i32;
pub const OLECMDID_PRINTPREVIEW: OLECMDID = 7i32;
pub const OLECMDID_PRINTPREVIEW2: OLECMDID = 50i32;
pub const OLECMDID_PROPERTIES: OLECMDID = 10i32;
pub const OLECMDID_PROPERTYBAG2: OLECMDID = 38i32;
pub const OLECMDID_REDO: OLECMDID = 16i32;
pub const OLECMDID_REFRESH: OLECMDID = 22i32;
pub type OLECMDID_REFRESHFLAG = i32;
pub const OLECMDID_SAVE: OLECMDID = 3i32;
pub const OLECMDID_SAVEAS: OLECMDID = 4i32;
pub const OLECMDID_SAVECOPYAS: OLECMDID = 5i32;
pub const OLECMDID_SCROLLCOMPLETE: OLECMDID = 82i32;
pub const OLECMDID_SELECTALL: OLECMDID = 17i32;
pub const OLECMDID_SETDOWNLOADSTATE: OLECMDID = 29i32;
pub const OLECMDID_SETFAVICON: OLECMDID = 79i32;
pub const OLECMDID_SETPRINTTEMPLATE: OLECMDID = 51i32;
pub const OLECMDID_SETPROGRESSMAX: OLECMDID = 25i32;
pub const OLECMDID_SETPROGRESSPOS: OLECMDID = 26i32;
pub const OLECMDID_SETPROGRESSTEXT: OLECMDID = 27i32;
pub const OLECMDID_SETTITLE: OLECMDID = 28i32;
pub const OLECMDID_SET_HOST_FULLSCREENMODE: OLECMDID = 80i32;
pub const OLECMDID_SHOWFIND: OLECMDID = 42i32;
pub const OLECMDID_SHOWMESSAGE: OLECMDID = 41i32;
pub const OLECMDID_SHOWMESSAGE_BLOCKABLE: OLECMDID = 84i32;
pub const OLECMDID_SHOWPAGEACTIONMENU: OLECMDID = 59i32;
pub const OLECMDID_SHOWPAGESETUP: OLECMDID = 43i32;
pub const OLECMDID_SHOWPRINT: OLECMDID = 44i32;
pub const OLECMDID_SHOWSCRIPTERROR: OLECMDID = 40i32;
pub const OLECMDID_SHOWTASKDLG: OLECMDID = 68i32;
pub const OLECMDID_SHOWTASKDLG_BLOCKABLE: OLECMDID = 85i32;
pub const OLECMDID_SPELL: OLECMDID = 9i32;
pub const OLECMDID_STOP: OLECMDID = 23i32;
pub const OLECMDID_STOPDOWNLOAD: OLECMDID = 30i32;
pub const OLECMDID_UNDO: OLECMDID = 15i32;
pub const OLECMDID_UPDATEBACKFORWARDSTATE: OLECMDID = 62i32;
pub const OLECMDID_UPDATECOMMANDS: OLECMDID = 21i32;
pub const OLECMDID_UPDATEPAGESTATUS: OLECMDID = 48i32;
pub const OLECMDID_UPDATETRAVELENTRY: OLECMDID = 61i32;
pub const OLECMDID_UPDATETRAVELENTRY_DATARECOVERY: OLECMDID = 67i32;
pub const OLECMDID_UPDATE_CARET: OLECMDID = 76i32;
pub const OLECMDID_USER_OPTICAL_ZOOM: OLECMDID = 73i32;
pub const OLECMDID_VIEWPORT_MODE: OLECMDID = 70i32;
pub type OLECMDID_VIEWPORT_MODE_FLAG = i32;
pub const OLECMDID_VISUAL_VIEWPORT_EXCLUDE_BOTTOM: OLECMDID = 72i32;
pub const OLECMDID_WINDOWSTATECHANGED: OLECMDID = 65i32;
pub type OLECMDID_WINDOWSTATE_FLAG = i32;
pub const OLECMDID_ZOOM: OLECMDID = 19i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLECMDTEXT {
    pub cmdtextf: u32,
    pub cwActual: u32,
    pub cwBuf: u32,
    pub rgwz: [u16; 1],
}
impl Default for OLECMDTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OLECMDTEXTF = i32;
pub const OLECMDTEXTF_NAME: OLECMDTEXTF = 1i32;
pub const OLECMDTEXTF_NONE: OLECMDTEXTF = 0i32;
pub const OLECMDTEXTF_STATUS: OLECMDTEXTF = 2i32;
pub const OLECMD_TASKDLGID_ONBEFOREUNLOAD: u32 = 1u32;
pub type OLECONTF = i32;
pub const OLECONTF_EMBEDDINGS: OLECONTF = 1i32;
pub const OLECONTF_LINKS: OLECONTF = 2i32;
pub const OLECONTF_ONLYIFRUNNING: OLECONTF = 16i32;
pub const OLECONTF_ONLYUSER: OLECONTF = 8i32;
pub const OLECONTF_OTHERS: OLECONTF = 4i32;
pub type OLECREATE = u32;
pub const OLECREATE_LEAVERUNNING: OLECREATE = 1u32;
pub const OLECREATE_ZERO: OLECREATE = 0u32;
pub type OLEDCFLAGS = i32;
pub const OLEDC_NODRAW: OLEDCFLAGS = 1i32;
pub const OLEDC_OFFSCREEN: OLEDCFLAGS = 4i32;
pub const OLEDC_PAINTBKGND: OLEDCFLAGS = 2i32;
pub type OLEGETMONIKER = i32;
pub const OLEGETMONIKER_FORCEASSIGN: OLEGETMONIKER = 2i32;
pub const OLEGETMONIKER_ONLYIFTHERE: OLEGETMONIKER = 1i32;
pub const OLEGETMONIKER_TEMPFORUSER: OLEGETMONIKER = 4i32;
pub const OLEGETMONIKER_UNASSIGN: OLEGETMONIKER = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct OLEINPLACEFRAMEINFO {
    pub cb: u32,
    pub fMDIApp: windows_sys::core::BOOL,
    pub hwndFrame: super::super::Foundation::HWND,
    pub haccel: super::super::UI::WindowsAndMessaging::HACCEL,
    pub cAccelEntries: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OLEINPLACEFRAMEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OLEIVERB = i32;
pub const OLEIVERB_DISCARDUNDOSTATE: OLEIVERB = -6i32;
pub const OLEIVERB_HIDE: OLEIVERB = -3i32;
pub const OLEIVERB_INPLACEACTIVATE: OLEIVERB = -5i32;
pub const OLEIVERB_OPEN: OLEIVERB = -2i32;
pub const OLEIVERB_PRIMARY: OLEIVERB = 0i32;
pub const OLEIVERB_PROPERTIES: i32 = -7i32;
pub const OLEIVERB_SHOW: OLEIVERB = -1i32;
pub const OLEIVERB_UIACTIVATE: OLEIVERB = -4i32;
pub type OLELINKBIND = i32;
pub const OLELINKBIND_EVENIFCLASSDIFF: OLELINKBIND = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLEMENUGROUPWIDTHS {
    pub width: [i32; 6],
}
impl Default for OLEMENUGROUPWIDTHS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OLEMISC = i32;
pub const OLEMISC_ACTIVATEWHENVISIBLE: OLEMISC = 256i32;
pub const OLEMISC_ACTSLIKEBUTTON: OLEMISC = 4096i32;
pub const OLEMISC_ACTSLIKELABEL: OLEMISC = 8192i32;
pub const OLEMISC_ALIGNABLE: OLEMISC = 32768i32;
pub const OLEMISC_ALWAYSRUN: OLEMISC = 2048i32;
pub const OLEMISC_CANLINKBYOLE1: OLEMISC = 32i32;
pub const OLEMISC_CANTLINKINSIDE: OLEMISC = 16i32;
pub const OLEMISC_IGNOREACTIVATEWHENVISIBLE: OLEMISC = 524288i32;
pub const OLEMISC_IMEMODE: OLEMISC = 262144i32;
pub const OLEMISC_INSERTNOTREPLACE: OLEMISC = 4i32;
pub const OLEMISC_INSIDEOUT: OLEMISC = 128i32;
pub const OLEMISC_INVISIBLEATRUNTIME: OLEMISC = 1024i32;
pub const OLEMISC_ISLINKOBJECT: OLEMISC = 64i32;
pub const OLEMISC_NOUIACTIVATE: OLEMISC = 16384i32;
pub const OLEMISC_ONLYICONIC: OLEMISC = 2i32;
pub const OLEMISC_RECOMPOSEONRESIZE: OLEMISC = 1i32;
pub const OLEMISC_RENDERINGISDEVICEINDEPENDENT: OLEMISC = 512i32;
pub const OLEMISC_SETCLIENTSITEFIRST: OLEMISC = 131072i32;
pub const OLEMISC_SIMPLEFRAME: OLEMISC = 65536i32;
pub const OLEMISC_STATIC: OLEMISC = 8i32;
pub const OLEMISC_SUPPORTSMULTILEVELUNDO: OLEMISC = 2097152i32;
pub const OLEMISC_WANTSTOMENUMERGE: OLEMISC = 1048576i32;
pub type OLERENDER = i32;
pub const OLERENDER_ASIS: OLERENDER = 3i32;
pub const OLERENDER_DRAW: OLERENDER = 1i32;
pub const OLERENDER_FORMAT: OLERENDER = 2i32;
pub const OLERENDER_NONE: OLERENDER = 0i32;
pub const OLESTDDELIM: windows_sys::core::PCWSTR = windows_sys::core::w!("\\");
pub type OLESTREAMQUERYCONVERTOLELINKCALLBACK = Option<unsafe extern "system" fn(pclsid: *const windows_sys::core::GUID, szclass: windows_sys::core::PCWSTR, sztopicname: windows_sys::core::PCWSTR, szitemname: windows_sys::core::PCWSTR, szuncname: windows_sys::core::PCWSTR, linkupdatingoption: u32, pvcontext: *const core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub const OLESTREAM_CONVERSION_DEFAULT: i32 = 0i32;
pub const OLESTREAM_CONVERSION_DISABLEOLELINK: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Media")]
#[derive(Clone, Copy)]
pub struct OLEUIBUSYA {
    pub cbStruct: u32,
    pub dwFlags: BUSY_DIALOG_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hTask: super::super::Media::HTASK,
    pub lphWndDialog: *mut super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Media")]
impl Default for OLEUIBUSYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Media")]
#[derive(Clone, Copy)]
pub struct OLEUIBUSYW {
    pub cbStruct: u32,
    pub dwFlags: BUSY_DIALOG_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hTask: super::super::Media::HTASK,
    pub lphWndDialog: *mut super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Media")]
impl Default for OLEUIBUSYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLEUICHANGEICONA {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_ICON_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub clsid: windows_sys::core::GUID,
    pub szIconExe: [i8; 260],
    pub cchIconExe: i32,
}
impl Default for OLEUICHANGEICONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLEUICHANGEICONW {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_ICON_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub clsid: windows_sys::core::GUID,
    pub szIconExe: [u16; 260],
    pub cchIconExe: i32,
}
impl Default for OLEUICHANGEICONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
#[derive(Clone, Copy)]
pub struct OLEUICHANGESOURCEA {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_SOURCE_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOFN: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: *mut core::ffi::c_void,
    pub dwLink: u32,
    pub lpszDisplayName: windows_sys::core::PSTR,
    pub nFileLength: u32,
    pub lpszFrom: windows_sys::core::PSTR,
    pub lpszTo: windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl Default for OLEUICHANGESOURCEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
#[derive(Clone, Copy)]
pub struct OLEUICHANGESOURCEW {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_SOURCE_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOFN: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: *mut core::ffi::c_void,
    pub dwLink: u32,
    pub lpszDisplayName: windows_sys::core::PWSTR,
    pub nFileLength: u32,
    pub lpszFrom: windows_sys::core::PWSTR,
    pub lpszTo: windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl Default for OLEUICHANGESOURCEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLEUICONVERTA {
    pub cbStruct: u32,
    pub dwFlags: UI_CONVERT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: windows_sys::core::GUID,
    pub clsidConvertDefault: windows_sys::core::GUID,
    pub clsidActivateDefault: windows_sys::core::GUID,
    pub clsidNew: windows_sys::core::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: windows_sys::core::BOOL,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub lpszUserType: windows_sys::core::PSTR,
    pub fObjectsIconChanged: windows_sys::core::BOOL,
    pub lpszDefLabel: windows_sys::core::PSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_sys::core::GUID,
}
impl Default for OLEUICONVERTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLEUICONVERTW {
    pub cbStruct: u32,
    pub dwFlags: UI_CONVERT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: windows_sys::core::GUID,
    pub clsidConvertDefault: windows_sys::core::GUID,
    pub clsidActivateDefault: windows_sys::core::GUID,
    pub clsidNew: windows_sys::core::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: windows_sys::core::BOOL,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub lpszUserType: windows_sys::core::PWSTR,
    pub fObjectsIconChanged: windows_sys::core::BOOL,
    pub lpszDefLabel: windows_sys::core::PWSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_sys::core::GUID,
}
impl Default for OLEUICONVERTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLEUIEDITLINKSA {
    pub cbStruct: u32,
    pub dwFlags: EDIT_LINKS_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOleUILinkContainer: *mut core::ffi::c_void,
}
impl Default for OLEUIEDITLINKSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLEUIEDITLINKSW {
    pub cbStruct: u32,
    pub dwFlags: EDIT_LINKS_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOleUILinkContainer: *mut core::ffi::c_void,
}
impl Default for OLEUIEDITLINKSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct OLEUIGNRLPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIGNRLPROPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct OLEUIGNRLPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIGNRLPROPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[derive(Clone, Copy)]
pub struct OLEUIINSERTOBJECTA {
    pub cbStruct: u32,
    pub dwFlags: INSERT_OBJECT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: windows_sys::core::GUID,
    pub lpszFile: windows_sys::core::PSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_sys::core::GUID,
    pub iid: windows_sys::core::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: *mut core::ffi::c_void,
    pub lpIStorage: *mut core::ffi::c_void,
    pub ppvObj: *mut *mut core::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Default for OLEUIINSERTOBJECTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[derive(Clone, Copy)]
pub struct OLEUIINSERTOBJECTW {
    pub cbStruct: u32,
    pub dwFlags: INSERT_OBJECT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: windows_sys::core::GUID,
    pub lpszFile: windows_sys::core::PWSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_sys::core::GUID,
    pub iid: windows_sys::core::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: *mut core::ffi::c_void,
    pub lpIStorage: *mut core::ffi::c_void,
    pub ppvObj: *mut *mut core::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Default for OLEUIINSERTOBJECTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct OLEUILINKPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUILINKPROPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct OLEUILINKPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUILINKPROPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct OLEUIOBJECTPROPSA {
    pub cbStruct: u32,
    pub dwFlags: OBJECT_PROPERTIES_FLAGS,
    pub lpPS: *mut super::super::UI::Controls::PROPSHEETHEADERA_V2,
    pub dwObject: u32,
    pub lpObjInfo: *mut core::ffi::c_void,
    pub dwLink: u32,
    pub lpLinkInfo: *mut core::ffi::c_void,
    pub lpGP: *mut OLEUIGNRLPROPSA,
    pub lpVP: *mut OLEUIVIEWPROPSA,
    pub lpLP: *mut OLEUILINKPROPSA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIOBJECTPROPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct OLEUIOBJECTPROPSW {
    pub cbStruct: u32,
    pub dwFlags: OBJECT_PROPERTIES_FLAGS,
    pub lpPS: *mut super::super::UI::Controls::PROPSHEETHEADERW_V2,
    pub dwObject: u32,
    pub lpObjInfo: *mut core::ffi::c_void,
    pub dwLink: u32,
    pub lpLinkInfo: *mut core::ffi::c_void,
    pub lpGP: *mut OLEUIGNRLPROPSW,
    pub lpVP: *mut OLEUIVIEWPROPSW,
    pub lpLP: *mut OLEUILINKPROPSW,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIOBJECTPROPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct OLEUIPASTEENTRYA {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: windows_sys::core::PCSTR,
    pub lpstrResultText: windows_sys::core::PCSTR,
    pub dwFlags: u32,
    pub dwScratchSpace: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for OLEUIPASTEENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct OLEUIPASTEENTRYW {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: windows_sys::core::PCWSTR,
    pub lpstrResultText: windows_sys::core::PCWSTR,
    pub dwFlags: u32,
    pub dwScratchSpace: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for OLEUIPASTEENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OLEUIPASTEFLAG = i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct OLEUIPASTESPECIALA {
    pub cbStruct: u32,
    pub dwFlags: PASTE_SPECIAL_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpSrcDataObj: *mut core::ffi::c_void,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYA,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_sys::core::GUID,
    pub nSelectedIndex: i32,
    pub fLink: windows_sys::core::BOOL,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub sizel: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for OLEUIPASTESPECIALA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct OLEUIPASTESPECIALW {
    pub cbStruct: u32,
    pub dwFlags: PASTE_SPECIAL_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_sys::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_sys::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpSrcDataObj: *mut core::ffi::c_void,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYW,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_sys::core::GUID,
    pub nSelectedIndex: i32,
    pub fLink: windows_sys::core::BOOL,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub sizel: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for OLEUIPASTESPECIALW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OLEUIPASTE_ENABLEICON: OLEUIPASTEFLAG = 2048i32;
pub const OLEUIPASTE_LINKANYTYPE: OLEUIPASTEFLAG = 1024i32;
pub const OLEUIPASTE_LINKTYPE1: OLEUIPASTEFLAG = 1i32;
pub const OLEUIPASTE_LINKTYPE2: OLEUIPASTEFLAG = 2i32;
pub const OLEUIPASTE_LINKTYPE3: OLEUIPASTEFLAG = 4i32;
pub const OLEUIPASTE_LINKTYPE4: OLEUIPASTEFLAG = 8i32;
pub const OLEUIPASTE_LINKTYPE5: OLEUIPASTEFLAG = 16i32;
pub const OLEUIPASTE_LINKTYPE6: OLEUIPASTEFLAG = 32i32;
pub const OLEUIPASTE_LINKTYPE7: OLEUIPASTEFLAG = 64i32;
pub const OLEUIPASTE_LINKTYPE8: OLEUIPASTEFLAG = 128i32;
pub const OLEUIPASTE_PASTE: OLEUIPASTEFLAG = 512i32;
pub const OLEUIPASTE_PASTEONLY: OLEUIPASTEFLAG = 0i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct OLEUIVIEWPROPSA {
    pub cbStruct: u32,
    pub dwFlags: VIEW_OBJECT_PROPERTIES_FLAGS,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIVIEWPROPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct OLEUIVIEWPROPSW {
    pub cbStruct: u32,
    pub dwFlags: VIEW_OBJECT_PROPERTIES_FLAGS,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIVIEWPROPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OLEUI_BZERR_HTASKINVALID: u32 = 116u32;
pub const OLEUI_BZ_CALLUNBLOCKED: u32 = 119u32;
pub const OLEUI_BZ_RETRYSELECTED: u32 = 118u32;
pub const OLEUI_BZ_SWITCHTOSELECTED: u32 = 117u32;
pub const OLEUI_CANCEL: u32 = 2u32;
pub const OLEUI_CIERR_MUSTHAVECLSID: u32 = 116u32;
pub const OLEUI_CIERR_MUSTHAVECURRENTMETAFILE: u32 = 117u32;
pub const OLEUI_CIERR_SZICONEXEINVALID: u32 = 118u32;
pub const OLEUI_CSERR_FROMNOTNULL: u32 = 118u32;
pub const OLEUI_CSERR_LINKCNTRINVALID: u32 = 117u32;
pub const OLEUI_CSERR_LINKCNTRNULL: u32 = 116u32;
pub const OLEUI_CSERR_SOURCEINVALID: u32 = 121u32;
pub const OLEUI_CSERR_SOURCENULL: u32 = 120u32;
pub const OLEUI_CSERR_SOURCEPARSEERROR: u32 = 122u32;
pub const OLEUI_CSERR_SOURCEPARSERROR: u32 = 122u32;
pub const OLEUI_CSERR_TONOTNULL: u32 = 119u32;
pub const OLEUI_CTERR_CBFORMATINVALID: u32 = 119u32;
pub const OLEUI_CTERR_CLASSIDINVALID: u32 = 117u32;
pub const OLEUI_CTERR_DVASPECTINVALID: u32 = 118u32;
pub const OLEUI_CTERR_HMETAPICTINVALID: u32 = 120u32;
pub const OLEUI_CTERR_STRINGINVALID: u32 = 121u32;
pub const OLEUI_ELERR_LINKCNTRINVALID: u32 = 117u32;
pub const OLEUI_ELERR_LINKCNTRNULL: u32 = 116u32;
pub const OLEUI_ERR_CBSTRUCTINCORRECT: u32 = 103u32;
pub const OLEUI_ERR_DIALOGFAILURE: u32 = 112u32;
pub const OLEUI_ERR_FINDTEMPLATEFAILURE: u32 = 110u32;
pub const OLEUI_ERR_GLOBALMEMALLOC: u32 = 114u32;
pub const OLEUI_ERR_HINSTANCEINVALID: u32 = 107u32;
pub const OLEUI_ERR_HRESOURCEINVALID: u32 = 109u32;
pub const OLEUI_ERR_HWNDOWNERINVALID: u32 = 104u32;
pub const OLEUI_ERR_LOADSTRING: u32 = 115u32;
pub const OLEUI_ERR_LOADTEMPLATEFAILURE: u32 = 111u32;
pub const OLEUI_ERR_LOCALMEMALLOC: u32 = 113u32;
pub const OLEUI_ERR_LPFNHOOKINVALID: u32 = 106u32;
pub const OLEUI_ERR_LPSZCAPTIONINVALID: u32 = 105u32;
pub const OLEUI_ERR_LPSZTEMPLATEINVALID: u32 = 108u32;
pub const OLEUI_ERR_OLEMEMALLOC: u32 = 100u32;
pub const OLEUI_ERR_STANDARDMAX: u32 = 116u32;
pub const OLEUI_ERR_STANDARDMIN: u32 = 100u32;
pub const OLEUI_ERR_STRUCTUREINVALID: u32 = 102u32;
pub const OLEUI_ERR_STRUCTURENULL: u32 = 101u32;
pub const OLEUI_FALSE: u32 = 0u32;
pub const OLEUI_GPERR_CBFORMATINVALID: u32 = 130u32;
pub const OLEUI_GPERR_CLASSIDINVALID: u32 = 128u32;
pub const OLEUI_GPERR_LPCLSIDEXCLUDEINVALID: u32 = 129u32;
pub const OLEUI_GPERR_STRINGINVALID: u32 = 127u32;
pub const OLEUI_IOERR_ARRLINKTYPESINVALID: u32 = 118u32;
pub const OLEUI_IOERR_ARRPASTEENTRIESINVALID: u32 = 117u32;
pub const OLEUI_IOERR_CCHFILEINVALID: u32 = 125u32;
pub const OLEUI_IOERR_HICONINVALID: u32 = 118u32;
pub const OLEUI_IOERR_LPCLSIDEXCLUDEINVALID: u32 = 124u32;
pub const OLEUI_IOERR_LPFORMATETCINVALID: u32 = 119u32;
pub const OLEUI_IOERR_LPIOLECLIENTSITEINVALID: u32 = 121u32;
pub const OLEUI_IOERR_LPISTORAGEINVALID: u32 = 122u32;
pub const OLEUI_IOERR_LPSZFILEINVALID: u32 = 116u32;
pub const OLEUI_IOERR_LPSZLABELINVALID: u32 = 117u32;
pub const OLEUI_IOERR_PPVOBJINVALID: u32 = 120u32;
pub const OLEUI_IOERR_SCODEHASERROR: u32 = 123u32;
pub const OLEUI_IOERR_SRCDATAOBJECTINVALID: u32 = 116u32;
pub const OLEUI_LPERR_LINKCNTRINVALID: u32 = 134u32;
pub const OLEUI_LPERR_LINKCNTRNULL: u32 = 133u32;
pub const OLEUI_OK: u32 = 1u32;
pub const OLEUI_OPERR_DLGPROCNOTNULL: u32 = 125u32;
pub const OLEUI_OPERR_INVALIDPAGES: u32 = 123u32;
pub const OLEUI_OPERR_LINKINFOINVALID: u32 = 137u32;
pub const OLEUI_OPERR_LPARAMNOTZERO: u32 = 126u32;
pub const OLEUI_OPERR_NOTSUPPORTED: u32 = 124u32;
pub const OLEUI_OPERR_OBJINFOINVALID: u32 = 136u32;
pub const OLEUI_OPERR_PAGESINCORRECT: u32 = 122u32;
pub const OLEUI_OPERR_PROPERTYSHEET: u32 = 135u32;
pub const OLEUI_OPERR_PROPSHEETINVALID: u32 = 119u32;
pub const OLEUI_OPERR_PROPSHEETNULL: u32 = 118u32;
pub const OLEUI_OPERR_PROPSINVALID: u32 = 121u32;
pub const OLEUI_OPERR_SUBPROPINVALID: u32 = 117u32;
pub const OLEUI_OPERR_SUBPROPNULL: u32 = 116u32;
pub const OLEUI_OPERR_SUPPROP: u32 = 120u32;
pub const OLEUI_PSERR_CLIPBOARDCHANGED: u32 = 119u32;
pub const OLEUI_PSERR_GETCLIPBOARDFAILED: u32 = 120u32;
pub const OLEUI_QUERY_GETCLASSID: u32 = 65280u32;
pub const OLEUI_QUERY_LINKBROKEN: u32 = 65281u32;
pub const OLEUI_SUCCESS: u32 = 1u32;
pub const OLEUI_VPERR_DVASPECTINVALID: u32 = 132u32;
pub const OLEUI_VPERR_METAPICTINVALID: u32 = 131u32;
pub type OLEUPDATE = i32;
pub const OLEUPDATE_ALWAYS: OLEUPDATE = 1i32;
pub const OLEUPDATE_ONCALL: OLEUPDATE = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct OLEVERB {
    pub lVerb: OLEIVERB,
    pub lpszVerbName: windows_sys::core::PWSTR,
    pub fuFlags: super::super::UI::WindowsAndMessaging::MENU_ITEM_FLAGS,
    pub grfAttribs: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OLEVERB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OLEVERBATTRIB = i32;
pub const OLEVERBATTRIB_NEVERDIRTIES: OLEVERBATTRIB = 1i32;
pub const OLEVERBATTRIB_ONCONTAINERMENU: OLEVERBATTRIB = 2i32;
pub const OLEVERB_PRIMARY: u32 = 0u32;
pub type OLEWHICHMK = i32;
pub const OLEWHICHMK_CONTAINER: OLEWHICHMK = 1i32;
pub const OLEWHICHMK_OBJFULL: OLEWHICHMK = 3i32;
pub const OLEWHICHMK_OBJREL: OLEWHICHMK = 2i32;
pub type OLE_HANDLE = u32;
pub type OLE_TRISTATE = i32;
pub const OPF_DISABLECONVERT: OBJECT_PROPERTIES_FLAGS = 8u32;
pub const OPF_NOFILLDEFAULT: OBJECT_PROPERTIES_FLAGS = 2u32;
pub const OPF_OBJECTISLINK: OBJECT_PROPERTIES_FLAGS = 1u32;
pub const OPF_SHOWHELP: OBJECT_PROPERTIES_FLAGS = 4u32;
pub const OT_EMBEDDED: i32 = 2i32;
pub const OT_LINK: i32 = 1i32;
pub const OT_STATIC: i32 = 3i32;
pub type PAGEACTION_UI = i32;
pub const PAGEACTION_UI_DEFAULT: PAGEACTION_UI = 0i32;
pub const PAGEACTION_UI_MODAL: PAGEACTION_UI = 1i32;
pub const PAGEACTION_UI_MODELESS: PAGEACTION_UI = 2i32;
pub const PAGEACTION_UI_SILENT: PAGEACTION_UI = 3i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PAGERANGE {
    pub nFromPage: i32,
    pub nToPage: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PAGESET {
    pub cbStruct: u32,
    pub fOddPages: windows_sys::core::BOOL,
    pub fEvenPages: windows_sys::core::BOOL,
    pub cPageRange: u32,
    pub rgPages: [PAGERANGE; 1],
}
impl Default for PAGESET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy)]
pub struct PARAMDATA {
    pub szName: windows_sys::core::PWSTR,
    pub vt: super::Variant::VARENUM,
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for PARAMDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct PARAMDESC {
    pub pparamdescex: *mut PARAMDESCEX,
    pub wParamFlags: PARAMFLAGS,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl Default for PARAMDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: super::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl Default for PARAMDESCEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PARAMFLAGS = u16;
pub const PARAMFLAG_FHASCUSTDATA: PARAMFLAGS = 64u16;
pub const PARAMFLAG_FHASDEFAULT: PARAMFLAGS = 32u16;
pub const PARAMFLAG_FIN: PARAMFLAGS = 1u16;
pub const PARAMFLAG_FLCID: PARAMFLAGS = 4u16;
pub const PARAMFLAG_FOPT: PARAMFLAGS = 16u16;
pub const PARAMFLAG_FOUT: PARAMFLAGS = 2u16;
pub const PARAMFLAG_FRETVAL: PARAMFLAGS = 8u16;
pub const PARAMFLAG_NONE: PARAMFLAGS = 0u16;
pub type PASTE_SPECIAL_FLAGS = u32;
pub const PERPROP_E_FIRST: i32 = -2147220992i32;
pub const PERPROP_E_LAST: windows_sys::core::HRESULT = 0x8004020F_u32 as _;
pub const PERPROP_E_NOPAGEAVAILABLE: windows_sys::core::HRESULT = 0x80040200_u32 as _;
pub const PERPROP_S_FIRST: windows_sys::core::HRESULT = 0x40200_u32 as _;
pub const PERPROP_S_LAST: windows_sys::core::HRESULT = 0x4020F_u32 as _;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PICTDESC {
    pub cbSizeofstruct: u32,
    pub picType: u32,
    pub Anonymous: PICTDESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PICTDESC_0 {
    pub bmp: PICTDESC_0_0,
    pub wmf: PICTDESC_0_1,
    pub icon: PICTDESC_0_2,
    pub emf: PICTDESC_0_3,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PICTDESC_0_0 {
    pub hbitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PICTDESC_0_3 {
    pub hemf: super::super::Graphics::Gdi::HENHMETAFILE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PICTDESC_0_2 {
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PICTDESC_0_1 {
    pub hmeta: super::super::Graphics::Gdi::HMETAFILE,
    pub xExt: i32,
    pub yExt: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PICTUREATTRIBUTES = i32;
pub const PICTURE_SCALABLE: PICTUREATTRIBUTES = 1i32;
pub const PICTURE_TRANSPARENT: PICTUREATTRIBUTES = 2i32;
pub type PICTYPE = i16;
pub const PICTYPE_BITMAP: PICTYPE = 1i16;
pub const PICTYPE_ENHMETAFILE: PICTYPE = 4i16;
pub const PICTYPE_ICON: PICTYPE = 3i16;
pub const PICTYPE_METAFILE: PICTYPE = 2i16;
pub const PICTYPE_NONE: PICTYPE = 0i16;
pub const PICTYPE_UNINITIALIZED: PICTYPE = -1i16;
pub type POINTERINACTIVE = i32;
pub const POINTERINACTIVE_ACTIVATEONDRAG: POINTERINACTIVE = 4i32;
pub const POINTERINACTIVE_ACTIVATEONENTRY: POINTERINACTIVE = 1i32;
pub const POINTERINACTIVE_DEACTIVATEONLEAVE: POINTERINACTIVE = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINTF {
    pub x: f32,
    pub y: f32,
}
pub type PRINTFLAG = i32;
pub const PRINTFLAG_DONTACTUALLYPRINT: PRINTFLAG = 16i32;
pub const PRINTFLAG_FORCEPROPERTIES: PRINTFLAG = 32i32;
pub const PRINTFLAG_MAYBOTHERUSER: PRINTFLAG = 1i32;
pub const PRINTFLAG_PRINTTOFILE: PRINTFLAG = 64i32;
pub const PRINTFLAG_PROMPTUSER: PRINTFLAG = 2i32;
pub const PRINTFLAG_RECOMPOSETODEVICE: PRINTFLAG = 8i32;
pub const PRINTFLAG_USERMAYCHANGEPRINTER: PRINTFLAG = 4i32;
pub type PROPBAG2_TYPE = i32;
pub const PROPBAG2_TYPE_DATA: PROPBAG2_TYPE = 1i32;
pub const PROPBAG2_TYPE_MONIKER: PROPBAG2_TYPE = 6i32;
pub const PROPBAG2_TYPE_OBJECT: PROPBAG2_TYPE = 3i32;
pub const PROPBAG2_TYPE_STORAGE: PROPBAG2_TYPE = 5i32;
pub const PROPBAG2_TYPE_STREAM: PROPBAG2_TYPE = 4i32;
pub const PROPBAG2_TYPE_UNDEFINED: PROPBAG2_TYPE = 0i32;
pub const PROPBAG2_TYPE_URL: PROPBAG2_TYPE = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROPPAGEINFO {
    pub cb: u32,
    pub pszTitle: windows_sys::core::PWSTR,
    pub size: super::super::Foundation::SIZE,
    pub pszDocString: windows_sys::core::PWSTR,
    pub pszHelpFile: windows_sys::core::PWSTR,
    pub dwHelpContext: u32,
}
impl Default for PROPPAGEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PROPPAGESTATUS = i32;
pub const PROPPAGESTATUS_CLEAN: PROPPAGESTATUS = 4i32;
pub const PROPPAGESTATUS_DIRTY: PROPPAGESTATUS = 1i32;
pub const PROPPAGESTATUS_VALIDATE: PROPPAGESTATUS = 2i32;
pub const PROP_HWND_CHGICONDLG: windows_sys::core::PCWSTR = windows_sys::core::w!("HWND_CIDLG");
pub const PSF_CHECKDISPLAYASICON: PASTE_SPECIAL_FLAGS = 8u32;
pub const PSF_DISABLEDISPLAYASICON: PASTE_SPECIAL_FLAGS = 16u32;
pub const PSF_HIDECHANGEICON: PASTE_SPECIAL_FLAGS = 32u32;
pub const PSF_NOREFRESHDATAOBJECT: PASTE_SPECIAL_FLAGS = 128u32;
pub const PSF_SELECTPASTE: PASTE_SPECIAL_FLAGS = 2u32;
pub const PSF_SELECTPASTELINK: PASTE_SPECIAL_FLAGS = 4u32;
pub const PSF_SHOWHELP: PASTE_SPECIAL_FLAGS = 1u32;
pub const PSF_STAYONCLIPBOARDCHANGE: PASTE_SPECIAL_FLAGS = 64u32;
pub const PS_MAXLINKTYPES: u32 = 8u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[derive(Clone, Copy)]
pub struct QACONTAINER {
    pub cbSize: u32,
    pub pClientSite: *mut core::ffi::c_void,
    pub pAdviseSink: *mut core::ffi::c_void,
    pub pPropertyNotifySink: *mut core::ffi::c_void,
    pub pUnkEventSink: *mut core::ffi::c_void,
    pub dwAmbientFlags: u32,
    pub colorFore: u32,
    pub colorBack: u32,
    pub pFont: *mut core::ffi::c_void,
    pub pUndoMgr: *mut core::ffi::c_void,
    pub dwAppearance: u32,
    pub lcid: i32,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
    pub pBindHost: *mut core::ffi::c_void,
    pub pOleControlSite: *mut core::ffi::c_void,
    pub pServiceProvider: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl Default for QACONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type QACONTAINERFLAGS = i32;
pub const QACONTAINER_AUTOCLIP: QACONTAINERFLAGS = 32i32;
pub const QACONTAINER_DISPLAYASDEFAULT: QACONTAINERFLAGS = 8i32;
pub const QACONTAINER_MESSAGEREFLECT: QACONTAINERFLAGS = 64i32;
pub const QACONTAINER_SHOWGRABHANDLES: QACONTAINERFLAGS = 2i32;
pub const QACONTAINER_SHOWHATCHING: QACONTAINERFLAGS = 1i32;
pub const QACONTAINER_SUPPORTSMNEMONICS: QACONTAINERFLAGS = 128i32;
pub const QACONTAINER_UIDEAD: QACONTAINERFLAGS = 16i32;
pub const QACONTAINER_USERMODE: QACONTAINERFLAGS = 4i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QACONTROL {
    pub cbSize: u32,
    pub dwMiscStatus: u32,
    pub dwViewStatus: u32,
    pub dwEventCookie: u32,
    pub dwPropNotifyCookie: u32,
    pub dwPointerActivationPolicy: u32,
}
pub type READYSTATE = i32;
pub const READYSTATE_COMPLETE: READYSTATE = 4i32;
pub const READYSTATE_INTERACTIVE: READYSTATE = 3i32;
pub const READYSTATE_LOADED: READYSTATE = 2i32;
pub const READYSTATE_LOADING: READYSTATE = 1i32;
pub const READYSTATE_UNINITIALIZED: READYSTATE = 0i32;
pub type REGKIND = i32;
pub const REGKIND_DEFAULT: REGKIND = 0i32;
pub const REGKIND_NONE: REGKIND = 2i32;
pub const REGKIND_REGISTER: REGKIND = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SAFEARRAYUNION {
    pub sfType: u32,
    pub u: SAFEARRAYUNION_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAFEARRAYUNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union SAFEARRAYUNION_0 {
    pub BstrStr: SAFEARR_BSTR,
    pub UnknownStr: SAFEARR_UNKNOWN,
    pub DispatchStr: SAFEARR_DISPATCH,
    pub VariantStr: SAFEARR_VARIANT,
    pub RecordStr: SAFEARR_BRECORD,
    pub HaveIidStr: SAFEARR_HAVEIID,
    pub ByteStr: super::Com::BYTE_SIZEDARR,
    pub WordStr: super::Com::WORD_SIZEDARR,
    pub LongStr: super::Com::DWORD_SIZEDARR,
    pub HyperStr: super::Com::HYPER_SIZEDARR,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAFEARRAYUNION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARR_BRECORD {
    pub Size: u32,
    pub aRecord: *mut *mut _wireBRECORD,
}
impl Default for SAFEARR_BRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SAFEARR_BSTR {
    pub Size: u32,
    pub aBstr: *mut *mut super::Com::FLAGGED_WORD_BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAFEARR_BSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SAFEARR_DISPATCH {
    pub Size: u32,
    pub apDispatch: *mut *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAFEARR_DISPATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARR_HAVEIID {
    pub Size: u32,
    pub apUnknown: *mut *mut core::ffi::c_void,
    pub iid: windows_sys::core::GUID,
}
impl Default for SAFEARR_HAVEIID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARR_UNKNOWN {
    pub Size: u32,
    pub apUnknown: *mut *mut core::ffi::c_void,
}
impl Default for SAFEARR_UNKNOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SAFEARR_VARIANT {
    pub Size: u32,
    pub aVariant: *mut *mut _wireVARIANT,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAFEARR_VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SELFREG_E_CLASS: windows_sys::core::HRESULT = 0x80040201_u32 as _;
pub const SELFREG_E_FIRST: i32 = -2147220992i32;
pub const SELFREG_E_LAST: windows_sys::core::HRESULT = 0x8004020F_u32 as _;
pub const SELFREG_E_TYPELIB: windows_sys::core::HRESULT = 0x80040200_u32 as _;
pub const SELFREG_S_FIRST: windows_sys::core::HRESULT = 0x40200_u32 as _;
pub const SELFREG_S_LAST: windows_sys::core::HRESULT = 0x4020F_u32 as _;
pub const SF_BSTR: SF_TYPE = 8i32;
pub const SF_DISPATCH: SF_TYPE = 9i32;
pub const SF_ERROR: SF_TYPE = 10i32;
pub const SF_HAVEIID: SF_TYPE = 32781i32;
pub const SF_I1: SF_TYPE = 16i32;
pub const SF_I2: SF_TYPE = 2i32;
pub const SF_I4: SF_TYPE = 3i32;
pub const SF_I8: SF_TYPE = 20i32;
pub const SF_RECORD: SF_TYPE = 36i32;
pub type SF_TYPE = i32;
pub const SF_UNKNOWN: SF_TYPE = 13i32;
pub const SF_VARIANT: SF_TYPE = 12i32;
pub const SID_GetCaller: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4717cc40_bcb9_11d0_9336_00a0c90dcaa9);
pub const SID_ProvideRuntimeContext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x74a5040c_dd0c_48f0_ac85_194c3259180a);
pub const SID_VariantConversion: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1f101481_bccd_11d0_9336_00a0c90dcaa9);
pub const STDOLE2_LCID: u32 = 0u32;
pub const STDOLE2_MAJORVERNUM: u32 = 2u32;
pub const STDOLE2_MINORVERNUM: u32 = 0u32;
pub const STDOLE_LCID: u32 = 0u32;
pub const STDOLE_MAJORVERNUM: u32 = 1u32;
pub const STDOLE_MINORVERNUM: u32 = 0u32;
pub const STDOLE_TLB: windows_sys::core::PCSTR = windows_sys::core::s!("stdole2.tlb");
pub const STDTYPE_TLB: windows_sys::core::PCSTR = windows_sys::core::s!("stdole2.tlb");
pub const SZOLEUI_MSG_ADDCONTROL: windows_sys::core::PCWSTR = windows_sys::core::w!("OLEUI_MSG_ADDCONTROL");
pub const SZOLEUI_MSG_BROWSE: windows_sys::core::PCWSTR = windows_sys::core::w!("OLEUI_MSG_BROWSE");
pub const SZOLEUI_MSG_BROWSE_OFN: windows_sys::core::PCWSTR = windows_sys::core::w!("OLEUI_MSG_BROWSE_OFN");
pub const SZOLEUI_MSG_CHANGEICON: windows_sys::core::PCWSTR = windows_sys::core::w!("OLEUI_MSG_CHANGEICON");
pub const SZOLEUI_MSG_CHANGESOURCE: windows_sys::core::PCWSTR = windows_sys::core::w!("OLEUI_MSG_CHANGESOURCE");
pub const SZOLEUI_MSG_CLOSEBUSYDIALOG: windows_sys::core::PCWSTR = windows_sys::core::w!("OLEUI_MSG_CLOSEBUSYDIALOG");
pub const SZOLEUI_MSG_CONVERT: windows_sys::core::PCWSTR = windows_sys::core::w!("OLEUI_MSG_CONVERT");
pub const SZOLEUI_MSG_ENDDIALOG: windows_sys::core::PCWSTR = windows_sys::core::w!("OLEUI_MSG_ENDDIALOG");
pub const SZOLEUI_MSG_HELP: windows_sys::core::PCWSTR = windows_sys::core::w!("OLEUI_MSG_HELP");
pub const TIFLAGS_EXTENDDISPATCHONLY: u32 = 1u32;
pub type TYPEFLAGS = i32;
pub const TYPEFLAG_FAGGREGATABLE: TYPEFLAGS = 1024i32;
pub const TYPEFLAG_FAPPOBJECT: TYPEFLAGS = 1i32;
pub const TYPEFLAG_FCANCREATE: TYPEFLAGS = 2i32;
pub const TYPEFLAG_FCONTROL: TYPEFLAGS = 32i32;
pub const TYPEFLAG_FDISPATCHABLE: TYPEFLAGS = 4096i32;
pub const TYPEFLAG_FDUAL: TYPEFLAGS = 64i32;
pub const TYPEFLAG_FHIDDEN: TYPEFLAGS = 16i32;
pub const TYPEFLAG_FLICENSED: TYPEFLAGS = 4i32;
pub const TYPEFLAG_FNONEXTENSIBLE: TYPEFLAGS = 128i32;
pub const TYPEFLAG_FOLEAUTOMATION: TYPEFLAGS = 256i32;
pub const TYPEFLAG_FPREDECLID: TYPEFLAGS = 8i32;
pub const TYPEFLAG_FPROXY: TYPEFLAGS = 16384i32;
pub const TYPEFLAG_FREPLACEABLE: TYPEFLAGS = 2048i32;
pub const TYPEFLAG_FRESTRICTED: TYPEFLAGS = 512i32;
pub const TYPEFLAG_FREVERSEBIND: TYPEFLAGS = 8192i32;
pub type UASFLAGS = i32;
pub const UAS_BLOCKED: UASFLAGS = 1i32;
pub const UAS_MASK: UASFLAGS = 3i32;
pub const UAS_NOPARENTENABLE: UASFLAGS = 2i32;
pub const UAS_NORMAL: UASFLAGS = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UDATE {
    pub st: super::super::Foundation::SYSTEMTIME,
    pub wDayOfYear: u16,
}
pub type UI_CONVERT_FLAGS = u32;
pub const UPDFCACHE_ALL: UPDFCACHE_FLAGS = 2147483647u32;
pub const UPDFCACHE_ALLBUTNODATACACHE: UPDFCACHE_FLAGS = 2147483646u32;
pub type UPDFCACHE_FLAGS = u32;
pub const UPDFCACHE_IFBLANK: UPDFCACHE_FLAGS = 16u32;
pub const UPDFCACHE_IFBLANKORONSAVECACHE: UPDFCACHE_FLAGS = 18u32;
pub const UPDFCACHE_NODATACACHE: UPDFCACHE_FLAGS = 1u32;
pub const UPDFCACHE_NORMALCACHE: UPDFCACHE_FLAGS = 8u32;
pub const UPDFCACHE_ONLYIFBLANK: UPDFCACHE_FLAGS = 2147483648u32;
pub const UPDFCACHE_ONSAVECACHE: UPDFCACHE_FLAGS = 2u32;
pub const UPDFCACHE_ONSTOPCACHE: UPDFCACHE_FLAGS = 4u32;
pub type USERCLASSTYPE = i32;
pub const USERCLASSTYPE_APPNAME: USERCLASSTYPE = 3i32;
pub const USERCLASSTYPE_FULL: USERCLASSTYPE = 1i32;
pub const USERCLASSTYPE_SHORT: USERCLASSTYPE = 2i32;
pub type VARCMP = u32;
pub const VARCMP_EQ: VARCMP = 1u32;
pub const VARCMP_GT: VARCMP = 2u32;
pub const VARCMP_LT: VARCMP = 0u32;
pub const VARCMP_NULL: VARCMP = 3u32;
pub type VARFORMAT_FIRST_DAY = i32;
pub const VARFORMAT_FIRST_DAY_FRIDAY: VARFORMAT_FIRST_DAY = 5i32;
pub const VARFORMAT_FIRST_DAY_MONDAY: VARFORMAT_FIRST_DAY = 1i32;
pub const VARFORMAT_FIRST_DAY_SATURDAY: VARFORMAT_FIRST_DAY = 6i32;
pub const VARFORMAT_FIRST_DAY_SUNDAY: VARFORMAT_FIRST_DAY = 7i32;
pub const VARFORMAT_FIRST_DAY_SYSTEMDEFAULT: VARFORMAT_FIRST_DAY = 0i32;
pub const VARFORMAT_FIRST_DAY_THURSDAY: VARFORMAT_FIRST_DAY = 4i32;
pub const VARFORMAT_FIRST_DAY_TUESDAY: VARFORMAT_FIRST_DAY = 2i32;
pub const VARFORMAT_FIRST_DAY_WEDNESDAY: VARFORMAT_FIRST_DAY = 3i32;
pub type VARFORMAT_FIRST_WEEK = i32;
pub const VARFORMAT_FIRST_WEEK_CONTAINS_JANUARY_FIRST: VARFORMAT_FIRST_WEEK = 1i32;
pub const VARFORMAT_FIRST_WEEK_HAS_SEVEN_DAYS: VARFORMAT_FIRST_WEEK = 3i32;
pub const VARFORMAT_FIRST_WEEK_LARGER_HALF_IN_CURRENT_YEAR: VARFORMAT_FIRST_WEEK = 2i32;
pub const VARFORMAT_FIRST_WEEK_SYSTEMDEFAULT: VARFORMAT_FIRST_WEEK = 0i32;
pub type VARFORMAT_GROUP = i32;
pub const VARFORMAT_GROUP_NOTTHOUSANDS: VARFORMAT_GROUP = 0i32;
pub const VARFORMAT_GROUP_SYSTEMDEFAULT: VARFORMAT_GROUP = -2i32;
pub const VARFORMAT_GROUP_THOUSANDS: VARFORMAT_GROUP = -1i32;
pub type VARFORMAT_LEADING_DIGIT = i32;
pub const VARFORMAT_LEADING_DIGIT_INCLUDED: VARFORMAT_LEADING_DIGIT = -1i32;
pub const VARFORMAT_LEADING_DIGIT_NOTINCLUDED: VARFORMAT_LEADING_DIGIT = 0i32;
pub const VARFORMAT_LEADING_DIGIT_SYSTEMDEFAULT: VARFORMAT_LEADING_DIGIT = -2i32;
pub type VARFORMAT_NAMED_FORMAT = i32;
pub const VARFORMAT_NAMED_FORMAT_GENERALDATE: VARFORMAT_NAMED_FORMAT = 0i32;
pub const VARFORMAT_NAMED_FORMAT_LONGDATE: VARFORMAT_NAMED_FORMAT = 1i32;
pub const VARFORMAT_NAMED_FORMAT_LONGTIME: VARFORMAT_NAMED_FORMAT = 3i32;
pub const VARFORMAT_NAMED_FORMAT_SHORTDATE: VARFORMAT_NAMED_FORMAT = 2i32;
pub const VARFORMAT_NAMED_FORMAT_SHORTTIME: VARFORMAT_NAMED_FORMAT = 4i32;
pub type VARFORMAT_PARENTHESES = i32;
pub const VARFORMAT_PARENTHESES_NOTUSED: VARFORMAT_PARENTHESES = 0i32;
pub const VARFORMAT_PARENTHESES_SYSTEMDEFAULT: VARFORMAT_PARENTHESES = -2i32;
pub const VARFORMAT_PARENTHESES_USED: VARFORMAT_PARENTHESES = -1i32;
pub const VAR_CALENDAR_GREGORIAN: u32 = 256u32;
pub const VAR_CALENDAR_HIJRI: u32 = 8u32;
pub const VAR_CALENDAR_THAI: u32 = 128u32;
pub const VAR_DATEVALUEONLY: u32 = 2u32;
pub const VAR_FORMAT_NOSUBSTITUTE: u32 = 32u32;
pub const VAR_FOURDIGITYEARS: u32 = 64u32;
pub const VAR_LOCALBOOL: u32 = 16u32;
pub const VAR_TIMEVALUEONLY: u32 = 1u32;
pub const VAR_VALIDDATE: u32 = 4u32;
pub type VIEWSTATUS = i32;
pub const VIEWSTATUS_3DSURFACE: VIEWSTATUS = 32i32;
pub const VIEWSTATUS_DVASPECTOPAQUE: VIEWSTATUS = 4i32;
pub const VIEWSTATUS_DVASPECTTRANSPARENT: VIEWSTATUS = 8i32;
pub const VIEWSTATUS_OPAQUE: VIEWSTATUS = 1i32;
pub const VIEWSTATUS_SOLIDBKGND: VIEWSTATUS = 2i32;
pub const VIEWSTATUS_SURFACE: VIEWSTATUS = 16i32;
pub type VIEW_OBJECT_PROPERTIES_FLAGS = u32;
pub const VPF_DISABLERELATIVE: VIEW_OBJECT_PROPERTIES_FLAGS = 2u32;
pub const VPF_DISABLESCALE: VIEW_OBJECT_PROPERTIES_FLAGS = 4u32;
pub const VPF_SELECTRELATIVE: VIEW_OBJECT_PROPERTIES_FLAGS = 1u32;
pub const VTDATEGRE_MAX: u32 = 2958465u32;
pub const VTDATEGRE_MIN: i32 = -657434i32;
pub const VT_BLOB_PROPSET: u32 = 75u32;
pub const VT_STORED_PROPSET: u32 = 74u32;
pub const VT_STREAMED_PROPSET: u32 = 73u32;
pub const VT_VERBOSE_ENUM: u32 = 76u32;
pub const WIN32: u32 = 100u32;
pub type WPCSETTING = i32;
pub const WPCSETTING_FILEDOWNLOAD_BLOCKED: WPCSETTING = 2i32;
pub const WPCSETTING_LOGGING_ENABLED: WPCSETTING = 1i32;
pub type XFORMCOORDS = i32;
pub const XFORMCOORDS_CONTAINERTOHIMETRIC: XFORMCOORDS = 8i32;
pub const XFORMCOORDS_EVENTCOMPAT: XFORMCOORDS = 16i32;
pub const XFORMCOORDS_HIMETRICTOCONTAINER: XFORMCOORDS = 4i32;
pub const XFORMCOORDS_POSITION: XFORMCOORDS = 1i32;
pub const XFORMCOORDS_SIZE: XFORMCOORDS = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _wireBRECORD {
    pub fFlags: u32,
    pub clSize: u32,
    pub pRecInfo: *mut core::ffi::c_void,
    pub pRecord: *mut u8,
}
impl Default for _wireBRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct _wireSAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub uArrayStructs: SAFEARRAYUNION,
    pub rgsabound: [super::Com::SAFEARRAYBOUND; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl Default for _wireSAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct _wireVARIANT {
    pub clSize: u32,
    pub rpcReserved: u32,
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: _wireVARIANT_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for _wireVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union _wireVARIANT_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::super::Foundation::VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: super::Com::CY,
    pub date: f64,
    pub bstrVal: *mut super::Com::FLAGGED_WORD_BLOB,
    pub punkVal: *mut core::ffi::c_void,
    pub pdispVal: *mut core::ffi::c_void,
    pub parray: *mut *mut _wireSAFEARRAY,
    pub brecVal: *mut _wireBRECORD,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::super::Foundation::VARIANT_BOOL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::Com::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut *mut super::Com::FLAGGED_WORD_BLOB,
    pub ppunkVal: *mut *mut core::ffi::c_void,
    pub ppdispVal: *mut *mut core::ffi::c_void,
    pub pparray: *mut *mut *mut _wireSAFEARRAY,
    pub pvarVal: *mut *mut _wireVARIANT,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub decVal: super::super::Foundation::DECIMAL,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: windows_sys::core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for _wireVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const fdexEnumAll: i32 = 2i32;
pub const fdexEnumDefault: i32 = 1i32;
pub const fdexNameCaseInsensitive: i32 = 8i32;
pub const fdexNameCaseSensitive: i32 = 1i32;
pub const fdexNameEnsure: i32 = 2i32;
pub const fdexNameImplicit: i32 = 4i32;
pub const fdexNameInternal: i32 = 16i32;
pub const fdexNameNoDynamicProperties: i32 = 32i32;
pub const fdexPropCanCall: FDEX_PROP_FLAGS = 256u32;
pub const fdexPropCanConstruct: FDEX_PROP_FLAGS = 1024u32;
pub const fdexPropCanGet: FDEX_PROP_FLAGS = 1u32;
pub const fdexPropCanPut: FDEX_PROP_FLAGS = 4u32;
pub const fdexPropCanPutRef: FDEX_PROP_FLAGS = 16u32;
pub const fdexPropCanSourceEvents: FDEX_PROP_FLAGS = 4096u32;
pub const fdexPropCannotCall: FDEX_PROP_FLAGS = 512u32;
pub const fdexPropCannotConstruct: FDEX_PROP_FLAGS = 2048u32;
pub const fdexPropCannotGet: FDEX_PROP_FLAGS = 2u32;
pub const fdexPropCannotPut: FDEX_PROP_FLAGS = 8u32;
pub const fdexPropCannotPutRef: FDEX_PROP_FLAGS = 32u32;
pub const fdexPropCannotSourceEvents: FDEX_PROP_FLAGS = 8192u32;
pub const fdexPropDynamicType: FDEX_PROP_FLAGS = 128u32;
pub const fdexPropNoSideEffects: FDEX_PROP_FLAGS = 64u32;
pub const triChecked: OLE_TRISTATE = 1i32;
pub const triGray: OLE_TRISTATE = 2i32;
pub const triUnchecked: OLE_TRISTATE = 0i32;
