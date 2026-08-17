#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn ClearPropVariantArray(rgpropvar: &mut [PROPVARIANT]) {
    windows_core::link!("propsys.dll" "system" fn ClearPropVariantArray(rgpropvar : *mut PROPVARIANT, cvars : u32));
    unsafe { ClearPropVariantArray(core::mem::transmute(rgpropvar.as_ptr()), rgpropvar.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CoGetInstanceFromFile<P2, P5>(pserverinfo: Option<*const super::COSERVERINFO>, pclsid: Option<*const windows_core::GUID>, punkouter: P2, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: P5, presults: &mut [super::MULTI_QI]) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::IUnknown>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetInstanceFromFile(pserverinfo : *const super:: COSERVERINFO, pclsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, dwclsctx : super:: CLSCTX, grfmode : u32, pwszname : windows_core::PCWSTR, dwcount : u32, presults : *mut super:: MULTI_QI) -> windows_core::HRESULT);
    unsafe { CoGetInstanceFromFile(pserverinfo.unwrap_or(core::mem::zeroed()) as _, pclsid.unwrap_or(core::mem::zeroed()) as _, punkouter.param().abi(), dwclsctx, grfmode, pwszname.param().abi(), presults.len().try_into().unwrap(), core::mem::transmute(presults.as_ptr())).ok() }
}
#[inline]
pub unsafe fn CoGetInstanceFromIStorage<P2, P4>(pserverinfo: Option<*const super::COSERVERINFO>, pclsid: Option<*const windows_core::GUID>, punkouter: P2, dwclsctx: super::CLSCTX, pstg: P4, presults: &mut [super::MULTI_QI]) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::IUnknown>,
    P4: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetInstanceFromIStorage(pserverinfo : *const super:: COSERVERINFO, pclsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, dwclsctx : super:: CLSCTX, pstg : * mut core::ffi::c_void, dwcount : u32, presults : *mut super:: MULTI_QI) -> windows_core::HRESULT);
    unsafe { CoGetInstanceFromIStorage(pserverinfo.unwrap_or(core::mem::zeroed()) as _, pclsid.unwrap_or(core::mem::zeroed()) as _, punkouter.param().abi(), dwclsctx, pstg.param().abi(), presults.len().try_into().unwrap(), core::mem::transmute(presults.as_ptr())).ok() }
}
#[inline]
pub unsafe fn CoGetInterfaceAndReleaseStream<P0, T>(pstm: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::IStream>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoGetInterfaceAndReleaseStream(pstm : * mut core::ffi::c_void, iid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoGetInterfaceAndReleaseStream(pstm.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CreateILockBytesOnHGlobal(hglobal: Option<super::super::super::Foundation::HGLOBAL>, fdeleteonrelease: bool) -> windows_core::Result<ILockBytes> {
    windows_core::link!("ole32.dll" "system" fn CreateILockBytesOnHGlobal(hglobal : super::super::super::Foundation:: HGLOBAL, fdeleteonrelease : windows_core::BOOL, pplkbyt : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateILockBytesOnHGlobal(hglobal.unwrap_or(core::mem::zeroed()) as _, fdeleteonrelease.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateStreamOnHGlobal(hglobal: super::super::super::Foundation::HGLOBAL, fdeleteonrelease: bool) -> windows_core::Result<super::IStream> {
    windows_core::link!("ole32.dll" "system" fn CreateStreamOnHGlobal(hglobal : super::super::super::Foundation:: HGLOBAL, fdeleteonrelease : windows_core::BOOL, ppstm : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateStreamOnHGlobal(hglobal, fdeleteonrelease.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn FmtIdToPropStgName(pfmtid: *const windows_core::GUID, oszname: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn FmtIdToPropStgName(pfmtid : *const windows_core::GUID, oszname : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { FmtIdToPropStgName(pfmtid, core::mem::transmute(oszname)).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn FreePropVariantArray(rgvars: &mut [PROPVARIANT]) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn FreePropVariantArray(cvariants : u32, rgvars : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe { FreePropVariantArray(rgvars.len().try_into().unwrap(), core::mem::transmute(rgvars.as_ptr())).ok() }
}
#[inline]
pub unsafe fn GetConvertStg<P0>(pstg: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn GetConvertStg(pstg : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetConvertStg(pstg.param().abi()).ok() }
}
#[inline]
pub unsafe fn GetHGlobalFromILockBytes<P0>(plkbyt: P0) -> windows_core::Result<super::super::super::Foundation::HGLOBAL>
where
    P0: windows_core::Param<ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn GetHGlobalFromILockBytes(plkbyt : * mut core::ffi::c_void, phglobal : *mut super::super::super::Foundation:: HGLOBAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetHGlobalFromILockBytes(plkbyt.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetHGlobalFromStream<P0>(pstm: P0) -> windows_core::Result<super::super::super::Foundation::HGLOBAL>
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("ole32.dll" "system" fn GetHGlobalFromStream(pstm : * mut core::ffi::c_void, phglobal : *mut super::super::super::Foundation:: HGLOBAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetHGlobalFromStream(pstm.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromBooleanVector(prgf: Option<&[windows_core::BOOL]>) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromBooleanVector(prgf : *const windows_core::BOOL, celems : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromBooleanVector(core::mem::transmute(prgf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromBuffer(pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromBuffer(pv : *const core::ffi::c_void, cb : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromBuffer(pv, cb, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromCLSID(clsid: *const windows_core::GUID) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromCLSID(clsid : *const windows_core::GUID, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromCLSID(clsid, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromDoubleVector(prgn: Option<&[f64]>) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromDoubleVector(prgn : *const f64, celems : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromDoubleVector(core::mem::transmute(prgn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromFileTime(pftin: *const super::super::super::Foundation::FILETIME) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromFileTime(pftin : *const super::super::super::Foundation:: FILETIME, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromFileTime(pftin, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromFileTimeVector(prgft: Option<&[super::super::super::Foundation::FILETIME]>) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromFileTimeVector(prgft : *const super::super::super::Foundation:: FILETIME, celems : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromFileTimeVector(core::mem::transmute(prgft.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgft.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromGUIDAsString(guid: *const windows_core::GUID) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromGUIDAsString(guid : *const windows_core::GUID, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromGUIDAsString(guid, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromInt16Vector(prgn: Option<&[i16]>) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromInt16Vector(prgn : *const i16, celems : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromInt16Vector(core::mem::transmute(prgn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromInt32Vector(prgn: Option<&[i32]>) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromInt32Vector(prgn : *const i32, celems : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromInt32Vector(core::mem::transmute(prgn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromInt64Vector(prgn: Option<&[i64]>) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromInt64Vector(prgn : *const i64, celems : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromInt64Vector(core::mem::transmute(prgn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromPropVariantVectorElem(propvarin: *const PROPVARIANT, ielem: u32) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromPropVariantVectorElem(propvarin : *const PROPVARIANT, ielem : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromPropVariantVectorElem(core::mem::transmute(propvarin), ielem, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromResource(hinst: super::super::super::Foundation::HINSTANCE, id: u32) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromResource(hinst : super::super::super::Foundation:: HINSTANCE, id : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromResource(hinst, id, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromStringAsVector<P0>(psz: P0) -> windows_core::Result<PROPVARIANT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromStringAsVector(psz : windows_core::PCWSTR, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromStringAsVector(psz.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromStringVector(prgsz: Option<&[windows_core::PCWSTR]>) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromStringVector(prgsz : *const windows_core::PCWSTR, celems : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromStringVector(core::mem::transmute(prgsz.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgsz.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromUInt16Vector(prgn: Option<&[u16]>) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromUInt16Vector(prgn : *const u16, celems : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromUInt16Vector(core::mem::transmute(prgn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromUInt32Vector(prgn: Option<&[u32]>) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromUInt32Vector(prgn : *const u32, celems : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromUInt32Vector(core::mem::transmute(prgn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantFromUInt64Vector(prgn: Option<&[u64]>) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromUInt64Vector(prgn : *const u64, celems : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromUInt64Vector(core::mem::transmute(prgn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn InitPropVariantVectorFromPropVariant(propvarsingle: *const PROPVARIANT) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantVectorFromPropVariant(propvarsingle : *const PROPVARIANT, ppropvarvector : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantVectorFromPropVariant(core::mem::transmute(propvarsingle), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAM<P0>(pstg: P0) -> windows_core::Result<OLESTREAM>
where
    P0: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAM(pstg : * mut core::ffi::c_void, lpolestream : *mut OLESTREAM) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleConvertIStorageToOLESTREAM(pstg.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAMEx<P0>(pstg: P0, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *const super::STGMEDIUM) -> windows_core::Result<OLESTREAM>
where
    P0: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAMEx(pstg : * mut core::ffi::c_void, cfformat : u16, lwidth : i32, lheight : i32, dwsize : u32, pmedium : *const super:: STGMEDIUM, polestm : *mut OLESTREAM) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleConvertIStorageToOLESTREAMEx(pstg.param().abi(), cfformat, lwidth, lheight, dwsize, core::mem::transmute(pmedium), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage<P1>(lpolestream: *const OLESTREAM, pstg: P1, ptd: *const super::DVTARGETDEVICE) -> windows_core::Result<()>
where
    P1: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage(lpolestream : *const OLESTREAM, pstg : * mut core::ffi::c_void, ptd : *const super:: DVTARGETDEVICE) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorage(lpolestream, pstg.param().abi(), ptd).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx<P1>(polestm: *const OLESTREAM, pstg: P1, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> windows_core::Result<()>
where
    P1: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx(polestm : *const OLESTREAM, pstg : * mut core::ffi::c_void, pcfformat : *mut u16, plwwidth : *mut i32, plheight : *mut i32, pdwsize : *mut u32, pmedium : *mut super:: STGMEDIUM) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorageEx(polestm, pstg.param().abi(), pcfformat as _, plwwidth as _, plheight as _, pdwsize as _, core::mem::transmute(pmedium)).ok() }
}
#[inline]
pub unsafe fn PropStgNameToFmtId<P0>(oszname: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn PropStgNameToFmtId(oszname : windows_core::PCWSTR, pfmtid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropStgNameToFmtId(oszname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantChangeType(ppropvardest: *mut PROPVARIANT, propvarsrc: *const PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: super::super::Variant::VARENUM) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantChangeType(ppropvardest : *mut PROPVARIANT, propvarsrc : *const PROPVARIANT, flags : PROPVAR_CHANGE_FLAGS, vt : super::super::Variant:: VARENUM) -> windows_core::HRESULT);
    unsafe { PropVariantChangeType(core::mem::transmute(ppropvardest), core::mem::transmute(propvarsrc), flags, vt).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantClear(pvar: *mut PROPVARIANT) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn PropVariantClear(pvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe { PropVariantClear(core::mem::transmute(pvar)).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantCompareEx(propvar1: *const PROPVARIANT, propvar2: *const PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32 {
    windows_core::link!("propsys.dll" "system" fn PropVariantCompareEx(propvar1 : *const PROPVARIANT, propvar2 : *const PROPVARIANT, unit : PROPVAR_COMPARE_UNIT, flags : PROPVAR_COMPARE_FLAGS) -> i32);
    unsafe { PropVariantCompareEx(core::mem::transmute(propvar1), core::mem::transmute(propvar2), unit, flags) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantCopy(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn PropVariantCopy(pvardest : *mut PROPVARIANT, pvarsrc : *const PROPVARIANT) -> windows_core::HRESULT);
    unsafe { PropVariantCopy(core::mem::transmute(pvardest), core::mem::transmute(pvarsrc)).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetBooleanElem(propvar: *const PROPVARIANT, ielem: u32) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetBooleanElem(propvar : *const PROPVARIANT, ielem : u32, pfval : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetBooleanElem(core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetDoubleElem(propvar: *const PROPVARIANT, ielem: u32) -> windows_core::Result<f64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetDoubleElem(propvar : *const PROPVARIANT, ielem : u32, pnval : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetDoubleElem(core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetElementCount(propvar: *const PROPVARIANT) -> u32 {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetElementCount(propvar : *const PROPVARIANT) -> u32);
    unsafe { PropVariantGetElementCount(core::mem::transmute(propvar)) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetFileTimeElem(propvar: *const PROPVARIANT, ielem: u32) -> windows_core::Result<super::super::super::Foundation::FILETIME> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetFileTimeElem(propvar : *const PROPVARIANT, ielem : u32, pftval : *mut super::super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetFileTimeElem(core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetInt16Elem(propvar: *const PROPVARIANT, ielem: u32) -> windows_core::Result<i16> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetInt16Elem(propvar : *const PROPVARIANT, ielem : u32, pnval : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetInt16Elem(core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetInt32Elem(propvar: *const PROPVARIANT, ielem: u32) -> windows_core::Result<i32> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetInt32Elem(propvar : *const PROPVARIANT, ielem : u32, pnval : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetInt32Elem(core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetInt64Elem(propvar: *const PROPVARIANT, ielem: u32) -> windows_core::Result<i64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetInt64Elem(propvar : *const PROPVARIANT, ielem : u32, pnval : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetInt64Elem(core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetStringElem(propvar: *const PROPVARIANT, ielem: u32) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetStringElem(propvar : *const PROPVARIANT, ielem : u32, ppszval : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetStringElem(core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetUInt16Elem(propvar: *const PROPVARIANT, ielem: u32) -> windows_core::Result<u16> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetUInt16Elem(propvar : *const PROPVARIANT, ielem : u32, pnval : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetUInt16Elem(core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetUInt32Elem(propvar: *const PROPVARIANT, ielem: u32) -> windows_core::Result<u32> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetUInt32Elem(propvar : *const PROPVARIANT, ielem : u32, pnval : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetUInt32Elem(core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantGetUInt64Elem(propvar: *const PROPVARIANT, ielem: u32) -> windows_core::Result<u64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetUInt64Elem(propvar : *const PROPVARIANT, ielem : u32, pnval : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetUInt64Elem(core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToBSTR(propvar: *const PROPVARIANT) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBSTR(propvar : *const PROPVARIANT, pbstrout : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToBSTR(core::mem::transmute(propvar), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToBoolean(propvarin: *const PROPVARIANT) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBoolean(propvarin : *const PROPVARIANT, pfret : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToBoolean(core::mem::transmute(propvarin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToBooleanVector(propvar: *const PROPVARIANT, prgf: &mut [windows_core::BOOL], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBooleanVector(propvar : *const PROPVARIANT, prgf : *mut windows_core::BOOL, crgf : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToBooleanVector(core::mem::transmute(propvar), core::mem::transmute(prgf.as_ptr()), prgf.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToBooleanVectorAlloc(propvar: *const PROPVARIANT, pprgf: *mut *mut windows_core::BOOL, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBooleanVectorAlloc(propvar : *const PROPVARIANT, pprgf : *mut *mut windows_core::BOOL, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToBooleanVectorAlloc(core::mem::transmute(propvar), pprgf as _, pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToBooleanWithDefault(propvarin: *const PROPVARIANT, fdefault: bool) -> windows_core::BOOL {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBooleanWithDefault(propvarin : *const PROPVARIANT, fdefault : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { PropVariantToBooleanWithDefault(core::mem::transmute(propvarin), fdefault.into()) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToBuffer(propvar: *const PROPVARIANT, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBuffer(propvar : *const PROPVARIANT, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::HRESULT);
    unsafe { PropVariantToBuffer(core::mem::transmute(propvar), pv as _, cb).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToDouble(propvarin: *const PROPVARIANT) -> windows_core::Result<f64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDouble(propvarin : *const PROPVARIANT, pdblret : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToDouble(core::mem::transmute(propvarin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToDoubleVector(propvar: *const PROPVARIANT, prgn: &mut [f64], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDoubleVector(propvar : *const PROPVARIANT, prgn : *mut f64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToDoubleVector(core::mem::transmute(propvar), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToDoubleVectorAlloc(propvar: *const PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDoubleVectorAlloc(propvar : *const PROPVARIANT, pprgn : *mut *mut f64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToDoubleVectorAlloc(core::mem::transmute(propvar), pprgn as _, pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToDoubleWithDefault(propvarin: *const PROPVARIANT, dbldefault: f64) -> f64 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDoubleWithDefault(propvarin : *const PROPVARIANT, dbldefault : f64) -> f64);
    unsafe { PropVariantToDoubleWithDefault(core::mem::transmute(propvarin), dbldefault) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToFileTime(propvar: *const PROPVARIANT, pstfout: super::super::Variant::PSTIME_FLAGS) -> windows_core::Result<super::super::super::Foundation::FILETIME> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToFileTime(propvar : *const PROPVARIANT, pstfout : super::super::Variant:: PSTIME_FLAGS, pftout : *mut super::super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToFileTime(core::mem::transmute(propvar), pstfout, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToFileTimeVector(propvar: *const PROPVARIANT, prgft: &mut [super::super::super::Foundation::FILETIME], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToFileTimeVector(propvar : *const PROPVARIANT, prgft : *mut super::super::super::Foundation:: FILETIME, crgft : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToFileTimeVector(core::mem::transmute(propvar), core::mem::transmute(prgft.as_ptr()), prgft.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToFileTimeVectorAlloc(propvar: *const PROPVARIANT, pprgft: *mut *mut super::super::super::Foundation::FILETIME, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToFileTimeVectorAlloc(propvar : *const PROPVARIANT, pprgft : *mut *mut super::super::super::Foundation:: FILETIME, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToFileTimeVectorAlloc(core::mem::transmute(propvar), pprgft as _, pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToGUID(propvar: *const PROPVARIANT) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToGUID(propvar : *const PROPVARIANT, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToGUID(core::mem::transmute(propvar), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt16(propvarin: *const PROPVARIANT) -> windows_core::Result<i16> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16(propvarin : *const PROPVARIANT, piret : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToInt16(core::mem::transmute(propvarin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt16Vector(propvar: *const PROPVARIANT, prgn: &mut [i16], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16Vector(propvar : *const PROPVARIANT, prgn : *mut i16, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt16Vector(core::mem::transmute(propvar), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt16VectorAlloc(propvar: *const PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16VectorAlloc(propvar : *const PROPVARIANT, pprgn : *mut *mut i16, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt16VectorAlloc(core::mem::transmute(propvar), pprgn as _, pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt16WithDefault(propvarin: *const PROPVARIANT, idefault: i16) -> i16 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16WithDefault(propvarin : *const PROPVARIANT, idefault : i16) -> i16);
    unsafe { PropVariantToInt16WithDefault(core::mem::transmute(propvarin), idefault) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt32(propvarin: *const PROPVARIANT) -> windows_core::Result<i32> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32(propvarin : *const PROPVARIANT, plret : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToInt32(core::mem::transmute(propvarin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt32Vector(propvar: *const PROPVARIANT, prgn: &mut [i32], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32Vector(propvar : *const PROPVARIANT, prgn : *mut i32, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt32Vector(core::mem::transmute(propvar), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt32VectorAlloc(propvar: *const PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32VectorAlloc(propvar : *const PROPVARIANT, pprgn : *mut *mut i32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt32VectorAlloc(core::mem::transmute(propvar), pprgn as _, pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt32WithDefault(propvarin: *const PROPVARIANT, ldefault: i32) -> i32 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32WithDefault(propvarin : *const PROPVARIANT, ldefault : i32) -> i32);
    unsafe { PropVariantToInt32WithDefault(core::mem::transmute(propvarin), ldefault) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt64(propvarin: *const PROPVARIANT) -> windows_core::Result<i64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64(propvarin : *const PROPVARIANT, pllret : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToInt64(core::mem::transmute(propvarin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt64Vector(propvar: *const PROPVARIANT, prgn: &mut [i64], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64Vector(propvar : *const PROPVARIANT, prgn : *mut i64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt64Vector(core::mem::transmute(propvar), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt64VectorAlloc(propvar: *const PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64VectorAlloc(propvar : *const PROPVARIANT, pprgn : *mut *mut i64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt64VectorAlloc(core::mem::transmute(propvar), pprgn as _, pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToInt64WithDefault(propvarin: *const PROPVARIANT, lldefault: i64) -> i64 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64WithDefault(propvarin : *const PROPVARIANT, lldefault : i64) -> i64);
    unsafe { PropVariantToInt64WithDefault(core::mem::transmute(propvarin), lldefault) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToString(propvar: *const PROPVARIANT, psz: &mut [u16]) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToString(propvar : *const PROPVARIANT, psz : windows_core::PWSTR, cch : u32) -> windows_core::HRESULT);
    unsafe { PropVariantToString(core::mem::transmute(propvar), core::mem::transmute(psz.as_ptr()), psz.len().try_into().unwrap()).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToStringAlloc(propvar: *const PROPVARIANT) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringAlloc(propvar : *const PROPVARIANT, ppszout : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToStringAlloc(core::mem::transmute(propvar), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToStringVector(propvar: *const PROPVARIANT, prgsz: &mut [windows_core::PWSTR], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringVector(propvar : *const PROPVARIANT, prgsz : *mut windows_core::PWSTR, crgsz : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToStringVector(core::mem::transmute(propvar), core::mem::transmute(prgsz.as_ptr()), prgsz.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToStringVectorAlloc(propvar: *const PROPVARIANT, pprgsz: *mut *mut windows_core::PWSTR, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringVectorAlloc(propvar : *const PROPVARIANT, pprgsz : *mut *mut windows_core::PWSTR, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToStringVectorAlloc(core::mem::transmute(propvar), pprgsz as _, pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToStringWithDefault<P1>(propvarin: *const PROPVARIANT, pszdefault: P1) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringWithDefault(propvarin : *const PROPVARIANT, pszdefault : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { PropVariantToStringWithDefault(core::mem::transmute(propvarin), pszdefault.param().abi()) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt16(propvarin: *const PROPVARIANT) -> windows_core::Result<u16> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16(propvarin : *const PROPVARIANT, puiret : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToUInt16(core::mem::transmute(propvarin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt16Vector(propvar: *const PROPVARIANT, prgn: &mut [u16], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16Vector(propvar : *const PROPVARIANT, prgn : *mut u16, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt16Vector(core::mem::transmute(propvar), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt16VectorAlloc(propvar: *const PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16VectorAlloc(propvar : *const PROPVARIANT, pprgn : *mut *mut u16, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt16VectorAlloc(core::mem::transmute(propvar), pprgn as _, pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt16WithDefault(propvarin: *const PROPVARIANT, uidefault: u16) -> u16 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16WithDefault(propvarin : *const PROPVARIANT, uidefault : u16) -> u16);
    unsafe { PropVariantToUInt16WithDefault(core::mem::transmute(propvarin), uidefault) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt32(propvarin: *const PROPVARIANT) -> windows_core::Result<u32> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32(propvarin : *const PROPVARIANT, pulret : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToUInt32(core::mem::transmute(propvarin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt32Vector(propvar: *const PROPVARIANT, prgn: &mut [u32], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32Vector(propvar : *const PROPVARIANT, prgn : *mut u32, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt32Vector(core::mem::transmute(propvar), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt32VectorAlloc(propvar: *const PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32VectorAlloc(propvar : *const PROPVARIANT, pprgn : *mut *mut u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt32VectorAlloc(core::mem::transmute(propvar), pprgn as _, pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt32WithDefault(propvarin: *const PROPVARIANT, uldefault: u32) -> u32 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32WithDefault(propvarin : *const PROPVARIANT, uldefault : u32) -> u32);
    unsafe { PropVariantToUInt32WithDefault(core::mem::transmute(propvarin), uldefault) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt64(propvarin: *const PROPVARIANT) -> windows_core::Result<u64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64(propvarin : *const PROPVARIANT, pullret : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToUInt64(core::mem::transmute(propvarin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt64Vector(propvar: *const PROPVARIANT, prgn: &mut [u64], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64Vector(propvar : *const PROPVARIANT, prgn : *mut u64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt64Vector(core::mem::transmute(propvar), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt64VectorAlloc(propvar: *const PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64VectorAlloc(propvar : *const PROPVARIANT, pprgn : *mut *mut u64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt64VectorAlloc(core::mem::transmute(propvar), pprgn as _, pcelem as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToUInt64WithDefault(propvarin: *const PROPVARIANT, ulldefault: u64) -> u64 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64WithDefault(propvarin : *const PROPVARIANT, ulldefault : u64) -> u64);
    unsafe { PropVariantToUInt64WithDefault(core::mem::transmute(propvarin), ulldefault) }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropVariantToVariant(ppropvar: *const PROPVARIANT) -> windows_core::Result<super::super::Variant::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToVariant(ppropvar : *const PROPVARIANT, pvar : *mut super::super::Variant:: VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToVariant(core::mem::transmute(ppropvar), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToWinRTPropertyValue<T>(propvar: *const PROPVARIANT) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PropVariantToWinRTPropertyValue(propvar : *const PROPVARIANT, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PropVariantToWinRTPropertyValue(core::mem::transmute(propvar), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn ReadClassStg<P0>(pstg: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn ReadClassStg(pstg : * mut core::ffi::c_void, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ReadClassStg(pstg.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ReadClassStm<P0>(pstm: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("ole32.dll" "system" fn ReadClassStm(pstm : * mut core::ffi::c_void, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ReadClassStm(pstm.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ReadFmtUserTypeStg<P0>(pstg: P0, pcf: *mut u16, lplpszusertype: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn ReadFmtUserTypeStg(pstg : * mut core::ffi::c_void, pcf : *mut u16, lplpszusertype : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { ReadFmtUserTypeStg(pstg.param().abi(), pcf as _, lplpszusertype.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetConvertStg<P0>(pstg: P0, fconvert: bool) -> windows_core::Result<()>
where
    P0: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn SetConvertStg(pstg : * mut core::ffi::c_void, fconvert : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { SetConvertStg(pstg.param().abi(), fconvert.into()).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn StgConvertPropertyToVariant<P3>(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: P3) -> bool
where
    P3: windows_core::Param<IMemoryAllocator>,
{
    windows_core::link!("ole32.dll" "system" fn StgConvertPropertyToVariant(pprop : *const SERIALIZEDPROPERTYVALUE, codepage : u16, pvar : *mut PROPVARIANT, pma : * mut core::ffi::c_void) -> bool);
    unsafe { StgConvertPropertyToVariant(pprop, codepage, core::mem::transmute(pvar), pma.param().abi()) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn StgConvertVariantToProperty(pvar: *const PROPVARIANT, codepage: u16, pprop: Option<*mut SERIALIZEDPROPERTYVALUE>, pcb: *mut u32, pid: u32, freserved: Option<bool>, pcindirect: Option<*mut u32>) -> *mut SERIALIZEDPROPERTYVALUE {
    windows_core::link!("ole32.dll" "system" fn StgConvertVariantToProperty(pvar : *const PROPVARIANT, codepage : u16, pprop : *mut SERIALIZEDPROPERTYVALUE, pcb : *mut u32, pid : u32, freserved : bool, pcindirect : *mut u32) -> *mut SERIALIZEDPROPERTYVALUE);
    unsafe { StgConvertVariantToProperty(core::mem::transmute(pvar), codepage, pprop.unwrap_or(core::mem::zeroed()) as _, pcb as _, pid, freserved.unwrap_or(core::mem::zeroed()) as _, pcindirect.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn StgCreateDocfile<P0>(pwcsname: P0, grfmode: super::STGM, reserved: Option<u32>) -> windows_core::Result<IStorage>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreateDocfile(pwcsname : windows_core::PCWSTR, grfmode : super:: STGM, reserved : u32, ppstgopen : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgCreateDocfile(pwcsname.param().abi(), grfmode, reserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn StgCreateDocfileOnILockBytes<P0>(plkbyt: P0, grfmode: super::STGM, reserved: u32) -> windows_core::Result<IStorage>
where
    P0: windows_core::Param<ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreateDocfileOnILockBytes(plkbyt : * mut core::ffi::c_void, grfmode : super:: STGM, reserved : u32, ppstgopen : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgCreateDocfileOnILockBytes(plkbyt.param().abi(), grfmode, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn StgCreatePropSetStg<P0>(pstorage: P0, dwreserved: Option<u32>) -> windows_core::Result<IPropertySetStorage>
where
    P0: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreatePropSetStg(pstorage : * mut core::ffi::c_void, dwreserved : u32, pppropsetstg : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgCreatePropSetStg(pstorage.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn StgCreatePropStg<P0>(punk: P0, fmtid: *const windows_core::GUID, pclsid: *const windows_core::GUID, grfflags: u32, dwreserved: Option<u32>) -> windows_core::Result<IPropertyStorage>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreatePropStg(punk : * mut core::ffi::c_void, fmtid : *const windows_core::GUID, pclsid : *const windows_core::GUID, grfflags : u32, dwreserved : u32, pppropstg : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgCreatePropStg(punk.param().abi(), fmtid, pclsid, grfflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn StgCreateStorageEx<P0>(pwcsname: P0, grfmode: super::STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: Option<*mut STGOPTIONS>, psecuritydescriptor: Option<super::super::super::Security::PSECURITY_DESCRIPTOR>, riid: *const windows_core::GUID, ppobjectopen: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreateStorageEx(pwcsname : windows_core::PCWSTR, grfmode : super:: STGM, stgfmt : STGFMT, grfattrs : u32, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::super::super::Security:: PSECURITY_DESCRIPTOR, riid : *const windows_core::GUID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgCreateStorageEx(pwcsname.param().abi(), grfmode, stgfmt, grfattrs, pstgoptions.unwrap_or(core::mem::zeroed()) as _, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, riid, ppobjectopen as _).ok() }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn StgDeserializePropVariant(pprop : *const SERIALIZEDPROPERTYVALUE, cbmax : u32, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgDeserializePropVariant(pprop, cbmax, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn StgGetIFillLockBytesOnFile<P0>(pwcsname: P0) -> windows_core::Result<IFillLockBytes>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnFile(pwcsname : windows_core::PCWSTR, ppflb : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgGetIFillLockBytesOnFile(pwcsname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn StgGetIFillLockBytesOnILockBytes<P0>(pilb: P0) -> windows_core::Result<IFillLockBytes>
where
    P0: windows_core::Param<ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnILockBytes(pilb : * mut core::ffi::c_void, ppflb : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgGetIFillLockBytesOnILockBytes(pilb.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn StgIsStorageFile<P0>(pwcsname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn StgIsStorageFile(pwcsname : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { StgIsStorageFile(pwcsname.param().abi()).ok() }
}
#[inline]
pub unsafe fn StgIsStorageILockBytes<P0>(plkbyt: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgIsStorageILockBytes(plkbyt : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgIsStorageILockBytes(plkbyt.param().abi()).ok() }
}
#[inline]
pub unsafe fn StgOpenAsyncDocfileOnIFillLockBytes<P0>(pflb: P0, grfmode: u32, asyncflags: u32) -> windows_core::Result<IStorage>
where
    P0: windows_core::Param<IFillLockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenAsyncDocfileOnIFillLockBytes(pflb : * mut core::ffi::c_void, grfmode : u32, asyncflags : u32, ppstgopen : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenAsyncDocfileOnIFillLockBytes(pflb.param().abi(), grfmode, asyncflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn StgOpenLayoutDocfile<P0>(pwcsdfname: P0, grfmode: u32, reserved: u32) -> windows_core::Result<IStorage>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dflayout.dll" "system" fn StgOpenLayoutDocfile(pwcsdfname : windows_core::PCWSTR, grfmode : u32, reserved : u32, ppstgopen : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenLayoutDocfile(pwcsdfname.param().abi(), grfmode, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn StgOpenPropStg<P0>(punk: P0, fmtid: *const windows_core::GUID, grfflags: u32, dwreserved: Option<u32>) -> windows_core::Result<IPropertyStorage>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenPropStg(punk : * mut core::ffi::c_void, fmtid : *const windows_core::GUID, grfflags : u32, dwreserved : u32, pppropstg : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenPropStg(punk.param().abi(), fmtid, grfflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn StgOpenStorage<P0, P1>(pwcsname: P0, pstgpriority: P1, grfmode: super::STGM, snbexclude: Option<*const *const u16>, reserved: u32) -> windows_core::Result<IStorage>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenStorage(pwcsname : windows_core::PCWSTR, pstgpriority : * mut core::ffi::c_void, grfmode : super:: STGM, snbexclude : *const *const u16, reserved : u32, ppstgopen : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenStorage(pwcsname.param().abi(), pstgpriority.param().abi(), grfmode, snbexclude.unwrap_or(core::mem::zeroed()) as _, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn StgOpenStorageEx<P0>(pwcsname: P0, grfmode: super::STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: Option<*mut STGOPTIONS>, psecuritydescriptor: Option<super::super::super::Security::PSECURITY_DESCRIPTOR>, riid: *const windows_core::GUID, ppobjectopen: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenStorageEx(pwcsname : windows_core::PCWSTR, grfmode : super:: STGM, stgfmt : STGFMT, grfattrs : u32, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::super::super::Security:: PSECURITY_DESCRIPTOR, riid : *const windows_core::GUID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgOpenStorageEx(pwcsname.param().abi(), grfmode, stgfmt, grfattrs, pstgoptions.unwrap_or(core::mem::zeroed()) as _, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, riid, ppobjectopen as _).ok() }
}
#[inline]
pub unsafe fn StgOpenStorageOnILockBytes<P0, P1>(plkbyt: P0, pstgpriority: P1, grfmode: super::STGM, snbexclude: Option<*const *const u16>, reserved: Option<u32>) -> windows_core::Result<IStorage>
where
    P0: windows_core::Param<ILockBytes>,
    P1: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenStorageOnILockBytes(plkbyt : * mut core::ffi::c_void, pstgpriority : * mut core::ffi::c_void, grfmode : super:: STGM, snbexclude : *const *const u16, reserved : u32, ppstgopen : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenStorageOnILockBytes(plkbyt.param().abi(), pstgpriority.param().abi(), grfmode, snbexclude.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: Option<u8>) -> u32 {
    windows_core::link!("ole32.dll" "system" fn StgPropertyLengthAsVariant(pprop : *const SERIALIZEDPROPERTYVALUE, cbprop : u32, codepage : u16, breserved : u8) -> u32);
    unsafe { StgPropertyLengthAsVariant(pprop, cbprop, codepage, breserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn StgSerializePropVariant(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn StgSerializePropVariant(ppropvar : *const PROPVARIANT, ppprop : *mut *mut SERIALIZEDPROPERTYVALUE, pcb : *mut u32) -> windows_core::HRESULT);
    unsafe { StgSerializePropVariant(core::mem::transmute(ppropvar), ppprop as _, pcb as _).ok() }
}
#[inline]
pub unsafe fn StgSetTimes<P0>(lpszname: P0, pctime: Option<*const super::super::super::Foundation::FILETIME>, patime: Option<*const super::super::super::Foundation::FILETIME>, pmtime: Option<*const super::super::super::Foundation::FILETIME>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn StgSetTimes(lpszname : windows_core::PCWSTR, pctime : *const super::super::super::Foundation:: FILETIME, patime : *const super::super::super::Foundation:: FILETIME, pmtime : *const super::super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    unsafe { StgSetTimes(lpszname.param().abi(), pctime.unwrap_or(core::mem::zeroed()) as _, patime.unwrap_or(core::mem::zeroed()) as _, pmtime.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn VariantToPropVariant(pvar: *const super::super::Variant::VARIANT) -> windows_core::Result<PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn VariantToPropVariant(pvar : *const super::super::Variant:: VARIANT, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToPropVariant(core::mem::transmute(pvar), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn WinRTPropertyValueToPropVariant<P0>(punkpropertyvalue: P0) -> windows_core::Result<PROPVARIANT>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("propsys.dll" "system" fn WinRTPropertyValueToPropVariant(punkpropertyvalue : * mut core::ffi::c_void, ppropvar : *mut PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WinRTPropertyValueToPropVariant(punkpropertyvalue.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WriteClassStg<P0>(pstg: P0, rclsid: *const windows_core::GUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn WriteClassStg(pstg : * mut core::ffi::c_void, rclsid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { WriteClassStg(pstg.param().abi(), rclsid).ok() }
}
#[inline]
pub unsafe fn WriteClassStm<P0>(pstm: P0, rclsid: *const windows_core::GUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("ole32.dll" "system" fn WriteClassStm(pstm : * mut core::ffi::c_void, rclsid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { WriteClassStm(pstm.param().abi(), rclsid).ok() }
}
#[inline]
pub unsafe fn WriteFmtUserTypeStg<P0, P2>(pstg: P0, cf: u16, lpszusertype: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<IStorage>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn WriteFmtUserTypeStg(pstg : * mut core::ffi::c_void, cf : u16, lpszusertype : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { WriteFmtUserTypeStg(pstg.param().abi(), cf, lpszusertype.param().abi()).ok() }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
impl Default for BSTRBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::VARIANT_BOOL,
}
impl Default for CABOOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::BSTR,
}
impl Default for CABSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
impl Default for CABSTRBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CAC {
    pub cElems: u32,
    pub pElems: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
impl Default for CACLIPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut windows_core::GUID,
}
impl Default for CACLSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut super::CY,
}
impl Default for CACY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl Default for CADATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl Default for CADBL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::FILETIME,
}
impl Default for CAFILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
impl Default for CAFLT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
impl Default for CAH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl Default for CAI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl Default for CAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PSTR,
}
impl Default for CALPSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PWSTR,
}
impl Default for CALPWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for CAPROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl Default for CASCODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
impl Default for CAUB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
impl Default for CAUH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
impl Default for CAUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl Default for CAUL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
impl Default for CLIPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CWCSTORAGENAME: u32 = 32u32;
windows_core::imp::define_interface!(IDirectWriterLock, IDirectWriterLock_Vtbl, 0x0e6d4d92_6738_11cf_9608_00aa00680db4);
windows_core::imp::interface_hierarchy!(IDirectWriterLock, windows_core::IUnknown);
impl IDirectWriterLock {
    pub unsafe fn WaitForWriteAccess(&self, dwtimeout: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WaitForWriteAccess)(windows_core::Interface::as_raw(self), dwtimeout).ok() }
    }
    pub unsafe fn ReleaseWriteAccess(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseWriteAccess)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn HaveWriteAccess(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HaveWriteAccess)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectWriterLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WaitForWriteAccess: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReleaseWriteAccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HaveWriteAccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectWriterLock_Impl: windows_core::IUnknownImpl {
    fn WaitForWriteAccess(&self, dwtimeout: u32) -> windows_core::Result<()>;
    fn ReleaseWriteAccess(&self) -> windows_core::Result<()>;
    fn HaveWriteAccess(&self) -> windows_core::Result<()>;
}
impl IDirectWriterLock_Vtbl {
    pub const fn new<Identity: IDirectWriterLock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WaitForWriteAccess<Identity: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectWriterLock_Impl::WaitForWriteAccess(this, core::mem::transmute_copy(&dwtimeout)).into()
            }
        }
        unsafe extern "system" fn ReleaseWriteAccess<Identity: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectWriterLock_Impl::ReleaseWriteAccess(this).into()
            }
        }
        unsafe extern "system" fn HaveWriteAccess<Identity: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectWriterLock_Impl::HaveWriteAccess(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WaitForWriteAccess: WaitForWriteAccess::<Identity, OFFSET>,
            ReleaseWriteAccess: ReleaseWriteAccess::<Identity, OFFSET>,
            HaveWriteAccess: HaveWriteAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectWriterLock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectWriterLock {}
windows_core::imp::define_interface!(IEnumSTATPROPSETSTG, IEnumSTATPROPSETSTG_Vtbl, 0x0000013b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumSTATPROPSETSTG, windows_core::IUnknown);
impl IEnumSTATPROPSETSTG {
    pub unsafe fn Next(&self, rgelt: &mut [STATPROPSETSTG], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSTATPROPSETSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSETSTG_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STATPROPSETSTG, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumSTATPROPSETSTG_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATPROPSETSTG>;
}
impl IEnumSTATPROPSETSTG_Vtbl {
    pub const fn new<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSETSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSETSTG_Impl::Skip(this, core::mem::transmute_copy(&celt))
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSETSTG_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSTATPROPSETSTG_Impl::Clone(this) {
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
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATPROPSETSTG as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumSTATPROPSETSTG {}
windows_core::imp::define_interface!(IEnumSTATPROPSTG, IEnumSTATPROPSTG_Vtbl, 0x00000139_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumSTATPROPSTG, windows_core::IUnknown);
impl IEnumSTATPROPSTG {
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn Next(&self, rgelt: &mut [STATPROPSTG], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSTATPROPSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSTG_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Variant")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STATPROPSTG, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Variant")]
pub trait IEnumSTATPROPSTG_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATPROPSTG>;
}
#[cfg(feature = "Win32_System_Variant")]
impl IEnumSTATPROPSTG_Vtbl {
    pub const fn new<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSTG_Impl::Skip(this, core::mem::transmute_copy(&celt))
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSTG_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSTATPROPSTG_Impl::Clone(this) {
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
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATPROPSTG as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::RuntimeName for IEnumSTATPROPSTG {}
windows_core::imp::define_interface!(IEnumSTATSTG, IEnumSTATSTG_Vtbl, 0x0000000d_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumSTATSTG, windows_core::IUnknown);
impl IEnumSTATSTG {
    pub unsafe fn Next(&self, rgelt: &mut [super::STATSTG], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSTATSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATSTG_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::STATSTG, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumSTATSTG_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATSTG>;
}
impl IEnumSTATSTG_Vtbl {
    pub const fn new<Identity: IEnumSTATSTG_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATSTG_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATSTG_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSTATSTG_Impl::Clone(this) {
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
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATSTG as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumSTATSTG {}
windows_core::imp::define_interface!(IFillLockBytes, IFillLockBytes_Vtbl, 0x99caf010_415e_11cf_8814_00aa00b569f5);
windows_core::imp::interface_hierarchy!(IFillLockBytes, windows_core::IUnknown);
impl IFillLockBytes {
    pub unsafe fn FillAppend(&self, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FillAppend)(windows_core::Interface::as_raw(self), pv, cb, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FillAt(&self, uloffset: u64, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FillAt)(windows_core::Interface::as_raw(self), uloffset, pv, cb, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFillSize(&self, ulsize: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFillSize)(windows_core::Interface::as_raw(self), ulsize).ok() }
    }
    pub unsafe fn Terminate(&self, bcanceled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), bcanceled.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFillLockBytes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FillAppend: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub FillAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetFillSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IFillLockBytes_Impl: windows_core::IUnknownImpl {
    fn FillAppend(&self, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32>;
    fn FillAt(&self, uloffset: u64, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32>;
    fn SetFillSize(&self, ulsize: u64) -> windows_core::Result<()>;
    fn Terminate(&self, bcanceled: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IFillLockBytes_Vtbl {
    pub const fn new<Identity: IFillLockBytes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FillAppend<Identity: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFillLockBytes_Impl::FillAppend(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb)) {
                    Ok(ok__) => {
                        pcbwritten.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FillAt<Identity: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFillLockBytes_Impl::FillAt(this, core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb)) {
                    Ok(ok__) => {
                        pcbwritten.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFillSize<Identity: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFillLockBytes_Impl::SetFillSize(this, core::mem::transmute_copy(&ulsize)).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcanceled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFillLockBytes_Impl::Terminate(this, core::mem::transmute_copy(&bcanceled)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FillAppend: FillAppend::<Identity, OFFSET>,
            FillAt: FillAt::<Identity, OFFSET>,
            SetFillSize: SetFillSize::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFillLockBytes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFillLockBytes {}
windows_core::imp::define_interface!(ILayoutStorage, ILayoutStorage_Vtbl, 0x0e6d4d90_6738_11cf_9608_00aa00680db4);
windows_core::imp::interface_hierarchy!(ILayoutStorage, windows_core::IUnknown);
impl ILayoutStorage {
    pub unsafe fn LayoutScript(&self, pstoragelayout: &[super::StorageLayout], glfinterleavedflag: Option<u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LayoutScript)(windows_core::Interface::as_raw(self), core::mem::transmute(pstoragelayout.as_ptr()), pstoragelayout.len().try_into().unwrap(), glfinterleavedflag.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn BeginMonitor(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginMonitor)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EndMonitor(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndMonitor)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ReLayoutDocfile<P0>(&self, pwcsnewdfname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReLayoutDocfile)(windows_core::Interface::as_raw(self), pwcsnewdfname.param().abi()).ok() }
    }
    pub unsafe fn ReLayoutDocfileOnILockBytes<P0>(&self, pilockbytes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILockBytes>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReLayoutDocfileOnILockBytes)(windows_core::Interface::as_raw(self), pilockbytes.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LayoutScript: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::StorageLayout, u32, u32) -> windows_core::HRESULT,
    pub BeginMonitor: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndMonitor: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReLayoutDocfile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ReLayoutDocfileOnILockBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILayoutStorage_Impl: windows_core::IUnknownImpl {
    fn LayoutScript(&self, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> windows_core::Result<()>;
    fn BeginMonitor(&self) -> windows_core::Result<()>;
    fn EndMonitor(&self) -> windows_core::Result<()>;
    fn ReLayoutDocfile(&self, pwcsnewdfname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReLayoutDocfileOnILockBytes(&self, pilockbytes: windows_core::Ref<ILockBytes>) -> windows_core::Result<()>;
}
impl ILayoutStorage_Vtbl {
    pub const fn new<Identity: ILayoutStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LayoutScript<Identity: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutStorage_Impl::LayoutScript(this, core::mem::transmute_copy(&pstoragelayout), core::mem::transmute_copy(&nentries), core::mem::transmute_copy(&glfinterleavedflag)).into()
            }
        }
        unsafe extern "system" fn BeginMonitor<Identity: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutStorage_Impl::BeginMonitor(this).into()
            }
        }
        unsafe extern "system" fn EndMonitor<Identity: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutStorage_Impl::EndMonitor(this).into()
            }
        }
        unsafe extern "system" fn ReLayoutDocfile<Identity: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsnewdfname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutStorage_Impl::ReLayoutDocfile(this, core::mem::transmute(&pwcsnewdfname)).into()
            }
        }
        unsafe extern "system" fn ReLayoutDocfileOnILockBytes<Identity: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pilockbytes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutStorage_Impl::ReLayoutDocfileOnILockBytes(this, core::mem::transmute_copy(&pilockbytes)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LayoutScript: LayoutScript::<Identity, OFFSET>,
            BeginMonitor: BeginMonitor::<Identity, OFFSET>,
            EndMonitor: EndMonitor::<Identity, OFFSET>,
            ReLayoutDocfile: ReLayoutDocfile::<Identity, OFFSET>,
            ReLayoutDocfileOnILockBytes: ReLayoutDocfileOnILockBytes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILayoutStorage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILayoutStorage {}
windows_core::imp::define_interface!(ILockBytes, ILockBytes_Vtbl, 0x0000000a_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ILockBytes, windows_core::IUnknown);
impl ILockBytes {
    pub unsafe fn ReadAt(&self, uloffset: u64, pv: *mut core::ffi::c_void, cb: u32, pcbread: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadAt)(windows_core::Interface::as_raw(self), uloffset, pv as _, cb, pcbread.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn WriteAt(&self, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteAt)(windows_core::Interface::as_raw(self), uloffset, pv, cb, pcbwritten.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetSize(&self, cb: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), cb).ok() }
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok() }
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok() }
    }
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatstg as _, grfstatflag).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockBytes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReadAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub WriteAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::STATSTG, u32) -> windows_core::HRESULT,
}
pub trait ILockBytes_Impl: windows_core::IUnknownImpl {
    fn ReadAt(&self, uloffset: u64, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::Result<()>;
    fn WriteAt(&self, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
    fn SetSize(&self, cb: u64) -> windows_core::Result<()>;
    fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> windows_core::Result<()>;
}
impl ILockBytes_Vtbl {
    pub const fn new<Identity: ILockBytes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadAt<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloffset: u64, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::ReadAt(this, core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn WriteAt<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::WriteAt(this, core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::Flush(this).into()
            }
        }
        unsafe extern "system" fn SetSize<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::SetSize(this, core::mem::transmute_copy(&cb)).into()
            }
        }
        unsafe extern "system" fn LockRegion<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::LockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&dwlocktype)).into()
            }
        }
        unsafe extern "system" fn UnlockRegion<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::UnlockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&dwlocktype)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::Stat(this, core::mem::transmute_copy(&pstatstg), core::mem::transmute_copy(&grfstatflag)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadAt: ReadAt::<Identity, OFFSET>,
            WriteAt: WriteAt::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            LockRegion: LockRegion::<Identity, OFFSET>,
            UnlockRegion: UnlockRegion::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILockBytes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILockBytes {}
windows_core::imp::define_interface!(IMemoryAllocator, IMemoryAllocator_Vtbl);
impl IMemoryAllocator {
    pub unsafe fn Allocate(&self, cbsize: u32) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).Allocate)(windows_core::Interface::as_raw(self), cbsize) }
    }
    pub unsafe fn Free(&self, pv: *mut core::ffi::c_void) {
        unsafe { (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pv as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryAllocator_Vtbl {
    pub Allocate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> *mut core::ffi::c_void,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
pub trait IMemoryAllocator_Impl {
    fn Allocate(&self, cbsize: u32) -> *mut core::ffi::c_void;
    fn Free(&self, pv: *mut core::ffi::c_void);
}
impl IMemoryAllocator_Vtbl {
    pub const fn new<Identity: IMemoryAllocator_Impl>() -> Self {
        unsafe extern "system" fn Allocate<Identity: IMemoryAllocator_Impl>(this: *mut core::ffi::c_void, cbsize: u32) -> *mut core::ffi::c_void {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IMemoryAllocator_Impl::Allocate(this, core::mem::transmute_copy(&cbsize))
            }
        }
        unsafe extern "system" fn Free<Identity: IMemoryAllocator_Impl>(this: *mut core::ffi::c_void, pv: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IMemoryAllocator_Impl::Free(this, core::mem::transmute_copy(&pv))
            }
        }
        Self { Allocate: Allocate::<Identity>, Free: Free::<Identity> }
    }
}
struct IMemoryAllocator_ImplVtbl<T: IMemoryAllocator_Impl>(core::marker::PhantomData<T>);
impl<T: IMemoryAllocator_Impl> IMemoryAllocator_ImplVtbl<T> {
    const VTABLE: IMemoryAllocator_Vtbl = IMemoryAllocator_Vtbl::new::<T>();
}
impl IMemoryAllocator {
    pub fn new<'a, T: IMemoryAllocator_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IMemoryAllocator_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IPersistStorage, IPersistStorage_Vtbl, 0x0000010a_0000_0000_c000_000000000046);
impl core::ops::Deref for IPersistStorage {
    type Target = super::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistStorage, windows_core::IUnknown, super::IPersist);
impl IPersistStorage {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn InitNew<P0>(&self, pstg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self), pstg.param().abi()).ok() }
    }
    pub unsafe fn Load<P0>(&self, pstg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pstg.param().abi()).ok() }
    }
    pub unsafe fn Save<P0>(&self, pstgsave: P0, fsameasload: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pstgsave.param().abi(), fsameasload.into()).ok() }
    }
    pub unsafe fn SaveCompleted<P0>(&self, pstgnew: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveCompleted)(windows_core::Interface::as_raw(self), pstgnew.param().abi()).ok() }
    }
    pub unsafe fn HandsOffStorage(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandsOffStorage)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStorage_Vtbl {
    pub base__: super::IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HandsOffStorage: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPersistStorage_Impl: super::IPersist_Impl {
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn InitNew(&self, pstg: windows_core::Ref<IStorage>) -> windows_core::Result<()>;
    fn Load(&self, pstg: windows_core::Ref<IStorage>) -> windows_core::Result<()>;
    fn Save(&self, pstgsave: windows_core::Ref<IStorage>, fsameasload: windows_core::BOOL) -> windows_core::Result<()>;
    fn SaveCompleted(&self, pstgnew: windows_core::Ref<IStorage>) -> windows_core::Result<()>;
    fn HandsOffStorage(&self) -> windows_core::Result<()>;
}
impl IPersistStorage_Vtbl {
    pub const fn new<Identity: IPersistStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::IsDirty(this)
            }
        }
        unsafe extern "system" fn InitNew<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::InitNew(this, core::mem::transmute_copy(&pstg)).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::Load(this, core::mem::transmute_copy(&pstg)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstgsave: *mut core::ffi::c_void, fsameasload: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::Save(this, core::mem::transmute_copy(&pstgsave), core::mem::transmute_copy(&fsameasload)).into()
            }
        }
        unsafe extern "system" fn SaveCompleted<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstgnew: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::SaveCompleted(this, core::mem::transmute_copy(&pstgnew)).into()
            }
        }
        unsafe extern "system" fn HandsOffStorage<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::HandsOffStorage(this).into()
            }
        }
        Self {
            base__: super::IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            InitNew: InitNew::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, OFFSET>,
            HandsOffStorage: HandsOffStorage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistStorage as windows_core::Interface>::IID || iid == &<super::IPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersistStorage {}
windows_core::imp::define_interface!(IPropertyBag, IPropertyBag_Vtbl, 0x55272a00_42cb_11ce_8135_00aa004bb851);
windows_core::imp::interface_hierarchy!(IPropertyBag, windows_core::IUnknown);
impl IPropertyBag {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Read<P0, P2>(&self, pszpropname: P0, pvar: *mut super::super::Variant::VARIANT, perrorlog: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pszpropname.param().abi(), core::mem::transmute(pvar), perrorlog.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pszpropname.param().abi(), core::mem::transmute(pvar)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Read: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Write: usize,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPropertyBag_Impl: windows_core::IUnknownImpl {
    fn Read(&self, pszpropname: &windows_core::PCWSTR, pvar: *mut super::super::Variant::VARIANT, perrorlog: windows_core::Ref<super::IErrorLog>) -> windows_core::Result<()>;
    fn Write(&self, pszpropname: &windows_core::PCWSTR, pvar: *const super::super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPropertyBag_Vtbl {
    pub const fn new<Identity: IPropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: windows_core::PCWSTR, pvar: *mut super::super::Variant::VARIANT, perrorlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag_Impl::Read(this, core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&perrorlog)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: windows_core::PCWSTR, pvar: *const super::super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag_Impl::Write(this, core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pvar)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Write: Write::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyBag {}
windows_core::imp::define_interface!(IPropertyBag2, IPropertyBag2_Vtbl, 0x22f55882_280b_11d0_a8a9_00a0c90c2004);
windows_core::imp::interface_hierarchy!(IPropertyBag2, windows_core::IUnknown);
impl IPropertyBag2 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Read<P2>(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: P2, pvarvalue: *mut super::super::Variant::VARIANT, phrerror: *mut windows_core::HRESULT) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), cproperties, ppropbag, perrlog.param().abi(), core::mem::transmute(pvarvalue), phrerror as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), cproperties, ppropbag, core::mem::transmute(pvarvalue)).ok() }
    }
    pub unsafe fn CountProperties(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CountProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn GetPropertyInfo(&self, iproperty: u32, ppropbag: &mut [PROPBAG2], pcproperties: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyInfo)(windows_core::Interface::as_raw(self), iproperty, ppropbag.len().try_into().unwrap(), core::mem::transmute(ppropbag.as_ptr()), pcproperties as _).ok() }
    }
    pub unsafe fn LoadObject<P0, P2, P3>(&self, pstrname: P0, dwhint: u32, punkobject: P2, perrlog: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<super::IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadObject)(windows_core::Interface::as_raw(self), pstrname.param().abi(), dwhint, punkobject.param().abi(), perrlog.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PROPBAG2, *mut core::ffi::c_void, *mut super::super::Variant::VARIANT, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Read: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PROPBAG2, *const super::super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Write: usize,
    pub CountProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Variant")]
    pub GetPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut PROPBAG2, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    GetPropertyInfo: usize,
    pub LoadObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPropertyBag2_Impl: windows_core::IUnknownImpl {
    fn Read(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: windows_core::Ref<super::IErrorLog>, pvarvalue: *mut super::super::Variant::VARIANT, phrerror: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::super::Variant::VARIANT) -> windows_core::Result<()>;
    fn CountProperties(&self) -> windows_core::Result<u32>;
    fn GetPropertyInfo(&self, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> windows_core::Result<()>;
    fn LoadObject(&self, pstrname: &windows_core::PCWSTR, dwhint: u32, punkobject: windows_core::Ref<windows_core::IUnknown>, perrlog: windows_core::Ref<super::IErrorLog>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPropertyBag2_Vtbl {
    pub const fn new<Identity: IPropertyBag2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: *mut core::ffi::c_void, pvarvalue: *mut super::super::Variant::VARIANT, phrerror: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::Read(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&perrlog), core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&phrerror)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::Write(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn CountProperties<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcproperties: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyBag2_Impl::CountProperties(this) {
                    Ok(ok__) => {
                        pcproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&iproperty), core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&pcproperties)).into()
            }
        }
        unsafe extern "system" fn LoadObject<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR, dwhint: u32, punkobject: *mut core::ffi::c_void, perrlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::LoadObject(this, core::mem::transmute(&pstrname), core::mem::transmute_copy(&dwhint), core::mem::transmute_copy(&punkobject), core::mem::transmute_copy(&perrlog)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            CountProperties: CountProperties::<Identity, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            LoadObject: LoadObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyBag2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyBag2 {}
windows_core::imp::define_interface!(IPropertySetStorage, IPropertySetStorage_Vtbl, 0x0000013a_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IPropertySetStorage, windows_core::IUnknown);
impl IPropertySetStorage {
    pub unsafe fn Create(&self, rfmtid: *const windows_core::GUID, pclsid: *const windows_core::GUID, grfflags: u32, grfmode: u32) -> windows_core::Result<IPropertyStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), rfmtid, pclsid, grfflags, grfmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Open(&self, rfmtid: *const windows_core::GUID, grfmode: u32) -> windows_core::Result<IPropertyStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), rfmtid, grfmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self, rfmtid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), rfmtid).ok() }
    }
    pub unsafe fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSETSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPropertySetStorage_Impl: windows_core::IUnknownImpl {
    fn Create(&self, rfmtid: *const windows_core::GUID, pclsid: *const windows_core::GUID, grfflags: u32, grfmode: u32) -> windows_core::Result<IPropertyStorage>;
    fn Open(&self, rfmtid: *const windows_core::GUID, grfmode: u32) -> windows_core::Result<IPropertyStorage>;
    fn Delete(&self, rfmtid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSETSTG>;
}
impl IPropertySetStorage_Vtbl {
    pub const fn new<Identity: IPropertySetStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const windows_core::GUID, pclsid: *const windows_core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertySetStorage_Impl::Create(this, core::mem::transmute_copy(&rfmtid), core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&grfmode)) {
                    Ok(ok__) => {
                        ppprstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Open<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const windows_core::GUID, grfmode: u32, ppprstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertySetStorage_Impl::Open(this, core::mem::transmute_copy(&rfmtid), core::mem::transmute_copy(&grfmode)) {
                    Ok(ok__) => {
                        ppprstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySetStorage_Impl::Delete(this, core::mem::transmute_copy(&rfmtid)).into()
            }
        }
        unsafe extern "system" fn Enum<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertySetStorage_Impl::Enum(this) {
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
            Create: Create::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySetStorage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPropertySetStorage {}
windows_core::imp::define_interface!(IPropertyStorage, IPropertyStorage_Vtbl, 0x00000138_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IPropertyStorage, windows_core::IUnknown);
impl IPropertyStorage {
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, core::mem::transmute(rgpropvar)).ok() }
    }
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, core::mem::transmute(rgpropvar), propidnamefirst).ok() }
    }
    pub unsafe fn DeleteMultiple(&self, rgpspec: &[PROPSPEC]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteMultiple)(windows_core::Interface::as_raw(self), rgpspec.len().try_into().unwrap(), core::mem::transmute(rgpspec.as_ptr())).ok() }
    }
    pub unsafe fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadPropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname as _).ok() }
    }
    pub unsafe fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WritePropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname).ok() }
    }
    pub unsafe fn DeletePropertyNames(&self, rgpropid: &[u32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyNames)(windows_core::Interface::as_raw(self), rgpropid.len().try_into().unwrap(), core::mem::transmute(rgpropid.as_ptr())).ok() }
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags).ok() }
    }
    pub unsafe fn Revert(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTimes(&self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTimes)(windows_core::Interface::as_raw(self), pctime, patime, pmtime).ok() }
    }
    pub unsafe fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), clsid).ok() }
    }
    pub unsafe fn Stat(&self, pstatpsstg: *mut STATPROPSETSTG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatpsstg as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Variant")]
    pub ReadMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PROPSPEC, *mut PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    ReadMultiple: usize,
    #[cfg(feature = "Win32_System_Variant")]
    pub WriteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PROPSPEC, *const PROPVARIANT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    WriteMultiple: usize,
    pub DeleteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PROPSPEC) -> windows_core::HRESULT,
    pub ReadPropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub WritePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub DeletePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::FILETIME, *const super::super::super::Foundation::FILETIME, *const super::super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STATPROPSETSTG) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Variant")]
pub trait IPropertyStorage_Impl: windows_core::IUnknownImpl {
    fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> windows_core::Result<()>;
    fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> windows_core::Result<()>;
    fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC) -> windows_core::Result<()>;
    fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const u32) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSTG>;
    fn SetTimes(&self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Stat(&self, pstatpsstg: *mut STATPROPSETSTG) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Variant")]
impl IPropertyStorage_Vtbl {
    pub const fn new<Identity: IPropertyStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadMultiple<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::ReadMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar)).into()
            }
        }
        unsafe extern "system" fn WriteMultiple<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::WriteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar), core::mem::transmute_copy(&propidnamefirst)).into()
            }
        }
        unsafe extern "system" fn DeleteMultiple<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::DeleteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec)).into()
            }
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::ReadPropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
            }
        }
        unsafe extern "system" fn WritePropertyNames<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::WritePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::DeletePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
            }
        }
        unsafe extern "system" fn Revert<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::Revert(this).into()
            }
        }
        unsafe extern "system" fn Enum<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStorage_Impl::Enum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTimes<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::SetTimes(this, core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
            }
        }
        unsafe extern "system" fn SetClass<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::Stat(this, core::mem::transmute_copy(&pstatpsstg)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadMultiple: ReadMultiple::<Identity, OFFSET>,
            WriteMultiple: WriteMultiple::<Identity, OFFSET>,
            DeleteMultiple: DeleteMultiple::<Identity, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, OFFSET>,
            WritePropertyNames: WritePropertyNames::<Identity, OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
            SetTimes: SetTimes::<Identity, OFFSET>,
            SetClass: SetClass::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStorage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::RuntimeName for IPropertyStorage {}
windows_core::imp::define_interface!(IRootStorage, IRootStorage_Vtbl, 0x00000012_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRootStorage, windows_core::IUnknown);
impl IRootStorage {
    pub unsafe fn SwitchToFile<P0>(&self, pszfile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SwitchToFile)(windows_core::Interface::as_raw(self), pszfile.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SwitchToFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IRootStorage_Impl: windows_core::IUnknownImpl {
    fn SwitchToFile(&self, pszfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IRootStorage_Vtbl {
    pub const fn new<Identity: IRootStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SwitchToFile<Identity: IRootStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRootStorage_Impl::SwitchToFile(this, core::mem::transmute(&pszfile)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SwitchToFile: SwitchToFile::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRootStorage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRootStorage {}
windows_core::imp::define_interface!(IStorage, IStorage_Vtbl, 0x0000000b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IStorage, windows_core::IUnknown);
impl IStorage {
    pub unsafe fn CreateStream<P0>(&self, pwcsname: P0, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> windows_core::Result<super::IStream>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStream)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), grfmode, reserved1, reserved2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenStream<P0>(&self, pwcsname: P0, reserved1: Option<*const core::ffi::c_void>, grfmode: super::STGM, reserved2: u32) -> windows_core::Result<super::IStream>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenStream)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), reserved1.unwrap_or(core::mem::zeroed()) as _, grfmode, reserved2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateStorage<P0>(&self, pwcsname: P0, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> windows_core::Result<IStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStorage)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), grfmode, reserved1, reserved2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenStorage<P0, P1>(&self, pwcsname: P0, pstgpriority: P1, grfmode: super::STGM, snbexclude: *const *const u16, reserved: u32) -> windows_core::Result<IStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IStorage>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenStorage)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), pstgpriority.param().abi(), grfmode, snbexclude, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CopyTo<P3>(&self, rgiidexclude: Option<&[windows_core::GUID]>, snbexclude: Option<*const *const u16>, pstgdest: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), rgiidexclude.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgiidexclude.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), snbexclude.unwrap_or(core::mem::zeroed()) as _, pstgdest.param().abi()).ok() }
    }
    pub unsafe fn MoveElementTo<P0, P1, P2>(&self, pwcsname: P0, pstgdest: P1, pwcsnewname: P2, grfflags: STGMOVE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IStorage>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MoveElementTo)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), pstgdest.param().abi(), pwcsnewname.param().abi(), grfflags.0 as _).ok() }
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags).ok() }
    }
    pub unsafe fn Revert(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnumElements(&self, reserved1: Option<u32>, reserved2: Option<*const core::ffi::c_void>, reserved3: Option<u32>) -> windows_core::Result<IEnumSTATSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumElements)(windows_core::Interface::as_raw(self), reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _, reserved3.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DestroyElement<P0>(&self, pwcsname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DestroyElement)(windows_core::Interface::as_raw(self), pwcsname.param().abi()).ok() }
    }
    pub unsafe fn RenameElement<P0, P1>(&self, pwcsoldname: P0, pwcsnewname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RenameElement)(windows_core::Interface::as_raw(self), pwcsoldname.param().abi(), pwcsnewname.param().abi()).ok() }
    }
    pub unsafe fn SetElementTimes<P0>(&self, pwcsname: P0, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetElementTimes)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), pctime, patime, pmtime).ok() }
    }
    pub unsafe fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), clsid).ok() }
    }
    pub unsafe fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStateBits)(windows_core::Interface::as_raw(self), grfstatebits, grfmask).ok() }
    }
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatstg as _, grfstatflag).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateStream: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::STGM, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenStream: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::ffi::c_void, super::STGM, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::STGM, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, super::STGM, *const *const u16, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const *const u16, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveElementTo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumElements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DestroyElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RenameElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetElementTimes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::super::Foundation::FILETIME, *const super::super::super::Foundation::FILETIME, *const super::super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetStateBits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::STATSTG, u32) -> windows_core::HRESULT,
}
pub trait IStorage_Impl: windows_core::IUnknownImpl {
    fn CreateStream(&self, pwcsname: &windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> windows_core::Result<super::IStream>;
    fn OpenStream(&self, pwcsname: &windows_core::PCWSTR, reserved1: *const core::ffi::c_void, grfmode: super::STGM, reserved2: u32) -> windows_core::Result<super::IStream>;
    fn CreateStorage(&self, pwcsname: &windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> windows_core::Result<IStorage>;
    fn OpenStorage(&self, pwcsname: &windows_core::PCWSTR, pstgpriority: windows_core::Ref<IStorage>, grfmode: super::STGM, snbexclude: *const *const u16, reserved: u32) -> windows_core::Result<IStorage>;
    fn CopyTo(&self, ciidexclude: u32, rgiidexclude: *const windows_core::GUID, snbexclude: *const *const u16, pstgdest: windows_core::Ref<IStorage>) -> windows_core::Result<()>;
    fn MoveElementTo(&self, pwcsname: &windows_core::PCWSTR, pstgdest: windows_core::Ref<IStorage>, pwcsnewname: &windows_core::PCWSTR, grfflags: &STGMOVE) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn EnumElements(&self, reserved1: u32, reserved2: *const core::ffi::c_void, reserved3: u32) -> windows_core::Result<IEnumSTATSTG>;
    fn DestroyElement(&self, pwcsname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RenameElement(&self, pwcsoldname: &windows_core::PCWSTR, pwcsnewname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetElementTimes(&self, pwcsname: &windows_core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> windows_core::Result<()>;
    fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> windows_core::Result<()>;
}
impl IStorage_Vtbl {
    pub const fn new<Identity: IStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateStream<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32, ppstm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::CreateStream(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&reserved2)) {
                    Ok(ok__) => {
                        ppstm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenStream<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, reserved1: *const core::ffi::c_void, grfmode: super::STGM, reserved2: u32, ppstm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::OpenStream(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&reserved2)) {
                    Ok(ok__) => {
                        ppstm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStorage<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32, ppstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::CreateStorage(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&reserved2)) {
                    Ok(ok__) => {
                        ppstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenStorage<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, pstgpriority: *mut core::ffi::c_void, grfmode: super::STGM, snbexclude: *const *const u16, reserved: u32, ppstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::OpenStorage(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&pstgpriority), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&snbexclude), core::mem::transmute_copy(&reserved)) {
                    Ok(ok__) => {
                        ppstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyTo<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const windows_core::GUID, snbexclude: *const *const u16, pstgdest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::CopyTo(this, core::mem::transmute_copy(&ciidexclude), core::mem::transmute_copy(&rgiidexclude), core::mem::transmute_copy(&snbexclude), core::mem::transmute_copy(&pstgdest)).into()
            }
        }
        unsafe extern "system" fn MoveElementTo<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, pstgdest: *mut core::ffi::c_void, pwcsnewname: windows_core::PCWSTR, grfflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::MoveElementTo(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&pstgdest), core::mem::transmute(&pwcsnewname), core::mem::transmute(&grfflags)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
            }
        }
        unsafe extern "system" fn Revert<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::Revert(this).into()
            }
        }
        unsafe extern "system" fn EnumElements<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved1: u32, reserved2: *const core::ffi::c_void, reserved3: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::EnumElements(this, core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&reserved2), core::mem::transmute_copy(&reserved3)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestroyElement<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::DestroyElement(this, core::mem::transmute(&pwcsname)).into()
            }
        }
        unsafe extern "system" fn RenameElement<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsoldname: windows_core::PCWSTR, pwcsnewname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::RenameElement(this, core::mem::transmute(&pwcsoldname), core::mem::transmute(&pwcsnewname)).into()
            }
        }
        unsafe extern "system" fn SetElementTimes<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::SetElementTimes(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
            }
        }
        unsafe extern "system" fn SetClass<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn SetStateBits<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::SetStateBits(this, core::mem::transmute_copy(&grfstatebits), core::mem::transmute_copy(&grfmask)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::Stat(this, core::mem::transmute_copy(&pstatstg), core::mem::transmute_copy(&grfstatflag)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStream: CreateStream::<Identity, OFFSET>,
            OpenStream: OpenStream::<Identity, OFFSET>,
            CreateStorage: CreateStorage::<Identity, OFFSET>,
            OpenStorage: OpenStorage::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            MoveElementTo: MoveElementTo::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            EnumElements: EnumElements::<Identity, OFFSET>,
            DestroyElement: DestroyElement::<Identity, OFFSET>,
            RenameElement: RenameElement::<Identity, OFFSET>,
            SetElementTimes: SetElementTimes::<Identity, OFFSET>,
            SetClass: SetClass::<Identity, OFFSET>,
            SetStateBits: SetStateBits::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStorage {}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OLESTREAM {
    pub lpstbl: *mut OLESTREAMVTBL,
}
impl Default for OLESTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OLESTREAMVTBL {
    pub Get: isize,
    pub Put: isize,
}
pub const PIDDI_THUMBNAIL: i32 = 2i32;
pub const PIDDSI_BYTECOUNT: u32 = 4u32;
pub const PIDDSI_CATEGORY: u32 = 2u32;
pub const PIDDSI_COMPANY: u32 = 15u32;
pub const PIDDSI_DOCPARTS: u32 = 13u32;
pub const PIDDSI_HEADINGPAIR: u32 = 12u32;
pub const PIDDSI_HIDDENCOUNT: u32 = 9u32;
pub const PIDDSI_LINECOUNT: u32 = 5u32;
pub const PIDDSI_LINKSDIRTY: u32 = 16u32;
pub const PIDDSI_MANAGER: u32 = 14u32;
pub const PIDDSI_MMCLIPCOUNT: u32 = 10u32;
pub const PIDDSI_NOTECOUNT: u32 = 8u32;
pub const PIDDSI_PARCOUNT: u32 = 6u32;
pub const PIDDSI_PRESFORMAT: u32 = 3u32;
pub const PIDDSI_SCALE: u32 = 11u32;
pub const PIDDSI_SLIDECOUNT: u32 = 7u32;
pub const PIDMSI_COPYRIGHT: i32 = 11i32;
pub const PIDMSI_EDITOR: i32 = 2i32;
pub const PIDMSI_OWNER: i32 = 8i32;
pub const PIDMSI_PRODUCTION: i32 = 10i32;
pub const PIDMSI_PROJECT: i32 = 6i32;
pub const PIDMSI_RATING: i32 = 9i32;
pub const PIDMSI_SEQUENCE_NO: i32 = 5i32;
pub const PIDMSI_SOURCE: i32 = 4i32;
pub const PIDMSI_STATUS: i32 = 7i32;
pub const PIDMSI_STATUS_DRAFT: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(3i32);
pub const PIDMSI_STATUS_EDIT: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(5i32);
pub const PIDMSI_STATUS_FINAL: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(8i32);
pub const PIDMSI_STATUS_INPROGRESS: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(4i32);
pub const PIDMSI_STATUS_NEW: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(1i32);
pub const PIDMSI_STATUS_NORMAL: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(0i32);
pub const PIDMSI_STATUS_OTHER: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(32767i32);
pub const PIDMSI_STATUS_PRELIM: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(2i32);
pub const PIDMSI_STATUS_PROOF: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(7i32);
pub const PIDMSI_STATUS_REVIEW: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PIDMSI_STATUS_VALUE(pub i32);
pub const PIDMSI_SUPPLIER: i32 = 3i32;
pub const PIDSI_APPNAME: i32 = 18i32;
pub const PIDSI_AUTHOR: i32 = 4i32;
pub const PIDSI_CHARCOUNT: i32 = 16i32;
pub const PIDSI_COMMENTS: i32 = 6i32;
pub const PIDSI_CREATE_DTM: i32 = 12i32;
pub const PIDSI_DOC_SECURITY: i32 = 19i32;
pub const PIDSI_EDITTIME: i32 = 10i32;
pub const PIDSI_KEYWORDS: i32 = 5i32;
pub const PIDSI_LASTAUTHOR: i32 = 8i32;
pub const PIDSI_LASTPRINTED: i32 = 11i32;
pub const PIDSI_LASTSAVE_DTM: i32 = 13i32;
pub const PIDSI_PAGECOUNT: i32 = 14i32;
pub const PIDSI_REVNUMBER: i32 = 9i32;
pub const PIDSI_SUBJECT: i32 = 3i32;
pub const PIDSI_TEMPLATE: i32 = 7i32;
pub const PIDSI_THUMBNAIL: i32 = 17i32;
pub const PIDSI_TITLE: i32 = 2i32;
pub const PIDSI_WORDCOUNT: i32 = 15i32;
pub const PID_BEHAVIOR: u32 = 2147483651u32;
pub const PID_CODEPAGE: u32 = 1u32;
pub const PID_DICTIONARY: u32 = 0u32;
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095u32;
pub const PID_FIRST_USABLE: u32 = 2u32;
pub const PID_ILLEGAL: u32 = 4294967295u32;
pub const PID_LOCALE: u32 = 2147483648u32;
pub const PID_MAX_READONLY: u32 = 3221225471u32;
pub const PID_MIN_READONLY: u32 = 2147483648u32;
pub const PID_MODIFY_TIME: u32 = 2147483649u32;
pub const PID_SECURITY: u32 = 2147483650u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: super::super::Variant::VARENUM,
    pub cfType: u16,
    pub dwHint: u32,
    pub pstrName: windows_core::PWSTR,
    pub clsid: windows_core::GUID,
}
pub const PROPSETFLAG_ANSI: u32 = 2u32;
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8u32;
pub const PROPSETFLAG_DEFAULT: u32 = 0u32;
pub const PROPSETFLAG_NONSIMPLE: u32 = 1u32;
pub const PROPSETFLAG_UNBUFFERED: u32 = 4u32;
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295u32;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROPSPEC {
    pub ulKind: PROPSPEC_KIND,
    pub Anonymous: PROPSPEC_0,
}
impl Default for PROPSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROPSPEC_0 {
    pub propid: u32,
    pub lpwstr: windows_core::PWSTR,
}
impl Default for PROPSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPSPEC_KIND(pub u32);
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for PROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
pub union PROPVARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<PROPVARIANT_0_0>,
    pub decVal: super::super::super::Foundation::DECIMAL,
}
#[cfg(feature = "Win32_System_Variant")]
impl Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for PROPVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
pub struct PROPVARIANT_0_0 {
    pub vt: super::super::Variant::VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[cfg(feature = "Win32_System_Variant")]
impl Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for PROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
pub union PROPVARIANT_0_0_0 {
    pub cVal: i8,
    pub bVal: u8,
    pub iVal: i16,
    pub uiVal: u16,
    pub lVal: i32,
    pub ulVal: u32,
    pub intVal: i32,
    pub uintVal: u32,
    pub hVal: i64,
    pub uhVal: u64,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::super::super::Foundation::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::super::super::Foundation::VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: super::CY,
    pub date: f64,
    pub filetime: super::super::super::Foundation::FILETIME,
    pub puuid: *mut windows_core::GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrblobVal: BSTRBLOB,
    pub blob: super::BLOB,
    pub pszVal: windows_core::PSTR,
    pub pwszVal: windows_core::PWSTR,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<super::IDispatch>>,
    pub pStream: core::mem::ManuallyDrop<Option<super::IStream>>,
    pub pStorage: core::mem::ManuallyDrop<Option<IStorage>>,
    pub pVersionedStream: *mut VERSIONEDSTREAM,
    pub parray: *mut super::SAFEARRAY,
    pub cac: CAC,
    pub caub: CAUB,
    pub cai: CAI,
    pub caui: CAUI,
    pub cal: CAL,
    pub caul: CAUL,
    pub cah: CAH,
    pub cauh: CAUH,
    pub caflt: CAFLT,
    pub cadbl: CADBL,
    pub cabool: CABOOL,
    pub cascode: CASCODE,
    pub cacy: CACY,
    pub cadate: CADATE,
    pub cafiletime: CAFILETIME,
    pub cauuid: CACLSID,
    pub caclipdata: CACLIPDATA,
    pub cabstr: CABSTR,
    pub cabstrblob: CABSTRBLOB,
    pub calpstr: CALPSTR,
    pub calpwstr: CALPWSTR,
    pub capropvar: CAPROPVARIANT,
    pub pcVal: windows_core::PSTR,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::super::super::Foundation::VARIANT_BOOL,
    pub pdecVal: *mut super::super::super::Foundation::DECIMAL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<super::IDispatch>,
    pub pparray: *mut *mut super::SAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_System_Variant")]
impl Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for PROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPVAR_CHANGE_FLAGS(pub i32);
impl PROPVAR_CHANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROPVAR_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROPVAR_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPVAR_COMPARE_FLAGS(pub i32);
impl PROPVAR_COMPARE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROPVAR_COMPARE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROPVAR_COMPARE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPVAR_COMPARE_UNIT(pub i32);
pub const PRSPEC_INVALID: u32 = 4294967295u32;
pub const PRSPEC_LPWSTR: PROPSPEC_KIND = PROPSPEC_KIND(0u32);
pub const PRSPEC_PROPID: PROPSPEC_KIND = PROPSPEC_KIND(1u32);
pub const PVCF_DEFAULT: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(0i32);
pub const PVCF_DIGITSASNUMBERS_CASESENSITIVE: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(32i32);
pub const PVCF_TREATEMPTYASGREATERTHAN: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(1i32);
pub const PVCF_USESTRCMP: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(2i32);
pub const PVCF_USESTRCMPC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(4i32);
pub const PVCF_USESTRCMPI: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(8i32);
pub const PVCF_USESTRCMPIC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(16i32);
pub const PVCHF_ALPHABOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(2i32);
pub const PVCHF_DEFAULT: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(0i32);
pub const PVCHF_LOCALBOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(8i32);
pub const PVCHF_NOHEXSTRING: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(16i32);
pub const PVCHF_NOUSEROVERRIDE: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(4i32);
pub const PVCHF_NOVALUEPROP: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(1i32);
pub const PVCU_DAY: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(4i32);
pub const PVCU_DEFAULT: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(0i32);
pub const PVCU_HOUR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(3i32);
pub const PVCU_MINUTE: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(2i32);
pub const PVCU_MONTH: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(5i32);
pub const PVCU_SECOND: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(1i32);
pub const PVCU_YEAR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(6i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemSNB {
    pub ulCntStr: u32,
    pub ulCntChar: u32,
    pub rgString: [u16; 1],
}
impl Default for RemSNB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERIALIZEDPROPERTYVALUE {
    pub dwType: u32,
    pub rgb: [u8; 1],
}
impl Default for SERIALIZEDPROPERTYVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STATPROPSETSTG {
    pub fmtid: windows_core::GUID,
    pub clsid: windows_core::GUID,
    pub grfFlags: u32,
    pub mtime: super::super::super::Foundation::FILETIME,
    pub ctime: super::super::super::Foundation::FILETIME,
    pub atime: super::super::super::Foundation::FILETIME,
    pub dwOSVersion: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STATPROPSTG {
    pub lpwstrName: windows_core::PWSTR,
    pub propid: u32,
    pub vt: super::super::Variant::VARENUM,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STGFMT(pub u32);
pub const STGFMT_ANY: STGFMT = STGFMT(4u32);
pub const STGFMT_DOCFILE: STGFMT = STGFMT(5u32);
pub const STGFMT_DOCUMENT: STGFMT = STGFMT(0u32);
pub const STGFMT_FILE: STGFMT = STGFMT(3u32);
pub const STGFMT_NATIVE: STGFMT = STGFMT(1u32);
pub const STGFMT_STORAGE: STGFMT = STGFMT(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STGMOVE(pub i32);
pub const STGMOVE_COPY: STGMOVE = STGMOVE(1i32);
pub const STGMOVE_MOVE: STGMOVE = STGMOVE(0i32);
pub const STGMOVE_SHALLOWCOPY: STGMOVE = STGMOVE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: windows_core::PCWSTR,
}
pub const STGOPTIONS_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: windows_core::GUID,
    pub pStream: core::mem::ManuallyDrop<Option<super::IStream>>,
}
