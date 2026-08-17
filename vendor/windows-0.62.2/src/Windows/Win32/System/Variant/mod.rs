#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ClearVariantArray(pvars: &mut [VARIANT]) {
    windows_core::link!("propsys.dll" "system" fn ClearVariantArray(pvars : *mut VARIANT, cvars : u32));
    unsafe { ClearVariantArray(core::mem::transmute(pvars.as_ptr()), pvars.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn DosDateTimeToVariantTime(wdosdate: u16, wdostime: u16, pvtime: *mut f64) -> i32 {
    windows_core::link!("oleaut32.dll" "system" fn DosDateTimeToVariantTime(wdosdate : u16, wdostime : u16, pvtime : *mut f64) -> i32);
    unsafe { DosDateTimeToVariantTime(wdosdate, wdostime, pvtime as _) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromBooleanArray(prgf: &[windows_core::BOOL]) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromBooleanArray(prgf : *const windows_core::BOOL, celems : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromBooleanArray(core::mem::transmute(prgf.as_ptr()), prgf.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromBuffer(pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromBuffer(pv : *const core::ffi::c_void, cb : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromBuffer(pv, cb, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromDoubleArray(prgn: &[f64]) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromDoubleArray(prgn : *const f64, celems : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromDoubleArray(core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromFileTime(pft: *const super::super::Foundation::FILETIME) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromFileTime(pft : *const super::super::Foundation:: FILETIME, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromFileTime(pft, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromFileTimeArray(prgft: Option<&[super::super::Foundation::FILETIME]>) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromFileTimeArray(prgft : *const super::super::Foundation:: FILETIME, celems : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromFileTimeArray(core::mem::transmute(prgft.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prgft.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromGUIDAsString(guid: *const windows_core::GUID) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromGUIDAsString(guid : *const windows_core::GUID, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromGUIDAsString(guid, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt16Array(prgn: &[i16]) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromInt16Array(prgn : *const i16, celems : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromInt16Array(core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt32Array(prgn: &[i32]) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromInt32Array(prgn : *const i32, celems : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromInt32Array(core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt64Array(prgn: &[i64]) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromInt64Array(prgn : *const i64, celems : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromInt64Array(core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromResource(hinst: super::super::Foundation::HINSTANCE, id: u32) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromResource(hinst : super::super::Foundation:: HINSTANCE, id : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromResource(hinst, id, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromStringArray(prgsz: &[windows_core::PCWSTR]) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromStringArray(prgsz : *const windows_core::PCWSTR, celems : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromStringArray(core::mem::transmute(prgsz.as_ptr()), prgsz.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt16Array(prgn: &[u16]) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromUInt16Array(prgn : *const u16, celems : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromUInt16Array(core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt32Array(prgn: &[u32]) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromUInt32Array(prgn : *const u32, celems : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromUInt32Array(core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt64Array(prgn: &[u64]) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromUInt64Array(prgn : *const u64, celems : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromUInt64Array(core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromVariantArrayElem(varin: *const VARIANT, ielem: u32) -> windows_core::Result<VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromVariantArrayElem(varin : *const VARIANT, ielem : u32, pvar : *mut VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromVariantArrayElem(core::mem::transmute(varin), ielem, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn SystemTimeToVariantTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, pvtime: *mut f64) -> i32 {
    windows_core::link!("oleaut32.dll" "system" fn SystemTimeToVariantTime(lpsystemtime : *const super::super::Foundation:: SYSTEMTIME, pvtime : *mut f64) -> i32);
    unsafe { SystemTimeToVariantTime(lpsystemtime, pvtime as _) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserFree(param0: *const u32, param1: *const VARIANT) {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserFree(param0 : *const u32, param1 : *const VARIANT));
    unsafe { VARIANT_UserFree(param0, core::mem::transmute(param1)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserFree64(param0: *const u32, param1: *const VARIANT) {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserFree64(param0 : *const u32, param1 : *const VARIANT));
    unsafe { VARIANT_UserFree64(param0, core::mem::transmute(param1)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const VARIANT) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const VARIANT) -> *mut u8);
    unsafe { VARIANT_UserMarshal(param0, param1 as _, core::mem::transmute(param2)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const VARIANT) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const VARIANT) -> *mut u8);
    unsafe { VARIANT_UserMarshal64(param0, param1 as _, core::mem::transmute(param2)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const VARIANT) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserSize(param0 : *const u32, param1 : u32, param2 : *const VARIANT) -> u32);
    unsafe { VARIANT_UserSize(param0, param1, core::mem::transmute(param2)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const VARIANT) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserSize64(param0 : *const u32, param1 : u32, param2 : *const VARIANT) -> u32);
    unsafe { VARIANT_UserSize64(param0, param1, core::mem::transmute(param2)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut VARIANT) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut VARIANT) -> *mut u8);
    unsafe { VARIANT_UserUnmarshal(param0, param1, core::mem::transmute(param2)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut VARIANT) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut VARIANT) -> *mut u8);
    unsafe { VARIANT_UserUnmarshal64(param0, param1, core::mem::transmute(param2)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantChangeType(pvargdest: *mut VARIANT, pvarsrc: *const VARIANT, wflags: VAR_CHANGE_FLAGS, vt: VARENUM) -> windows_core::Result<()> {
    windows_core::link!("oleaut32.dll" "system" fn VariantChangeType(pvargdest : *mut VARIANT, pvarsrc : *const VARIANT, wflags : VAR_CHANGE_FLAGS, vt : VARENUM) -> windows_core::HRESULT);
    unsafe { VariantChangeType(core::mem::transmute(pvargdest), core::mem::transmute(pvarsrc), wflags, vt).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantChangeTypeEx(pvargdest: *mut VARIANT, pvarsrc: *const VARIANT, lcid: u32, wflags: VAR_CHANGE_FLAGS, vt: VARENUM) -> windows_core::Result<()> {
    windows_core::link!("oleaut32.dll" "system" fn VariantChangeTypeEx(pvargdest : *mut VARIANT, pvarsrc : *const VARIANT, lcid : u32, wflags : VAR_CHANGE_FLAGS, vt : VARENUM) -> windows_core::HRESULT);
    unsafe { VariantChangeTypeEx(core::mem::transmute(pvargdest), core::mem::transmute(pvarsrc), lcid, wflags, vt).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantClear(pvarg: *mut VARIANT) -> windows_core::Result<()> {
    windows_core::link!("oleaut32.dll" "system" fn VariantClear(pvarg : *mut VARIANT) -> windows_core::HRESULT);
    unsafe { VariantClear(core::mem::transmute(pvarg)).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantCompare(var1: *const VARIANT, var2: *const VARIANT) -> i32 {
    windows_core::link!("propsys.dll" "system" fn VariantCompare(var1 : *const VARIANT, var2 : *const VARIANT) -> i32);
    unsafe { VariantCompare(core::mem::transmute(var1), core::mem::transmute(var2)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantCopy(pvargdest: *mut VARIANT, pvargsrc: *const VARIANT) -> windows_core::Result<()> {
    windows_core::link!("oleaut32.dll" "system" fn VariantCopy(pvargdest : *mut VARIANT, pvargsrc : *const VARIANT) -> windows_core::HRESULT);
    unsafe { VariantCopy(core::mem::transmute(pvargdest), core::mem::transmute(pvargsrc)).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantCopyInd(pvardest: *mut VARIANT, pvargsrc: *const VARIANT) -> windows_core::Result<()> {
    windows_core::link!("oleaut32.dll" "system" fn VariantCopyInd(pvardest : *mut VARIANT, pvargsrc : *const VARIANT) -> windows_core::HRESULT);
    unsafe { VariantCopyInd(core::mem::transmute(pvardest), core::mem::transmute(pvargsrc)).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetBooleanElem(var: *const VARIANT, ielem: u32) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("propsys.dll" "system" fn VariantGetBooleanElem(var : *const VARIANT, ielem : u32, pfval : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetBooleanElem(core::mem::transmute(var), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetDoubleElem(var: *const VARIANT, ielem: u32) -> windows_core::Result<f64> {
    windows_core::link!("propsys.dll" "system" fn VariantGetDoubleElem(var : *const VARIANT, ielem : u32, pnval : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetDoubleElem(core::mem::transmute(var), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetElementCount(varin: *const VARIANT) -> u32 {
    windows_core::link!("propsys.dll" "system" fn VariantGetElementCount(varin : *const VARIANT) -> u32);
    unsafe { VariantGetElementCount(core::mem::transmute(varin)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt16Elem(var: *const VARIANT, ielem: u32) -> windows_core::Result<i16> {
    windows_core::link!("propsys.dll" "system" fn VariantGetInt16Elem(var : *const VARIANT, ielem : u32, pnval : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetInt16Elem(core::mem::transmute(var), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt32Elem(var: *const VARIANT, ielem: u32) -> windows_core::Result<i32> {
    windows_core::link!("propsys.dll" "system" fn VariantGetInt32Elem(var : *const VARIANT, ielem : u32, pnval : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetInt32Elem(core::mem::transmute(var), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt64Elem(var: *const VARIANT, ielem: u32) -> windows_core::Result<i64> {
    windows_core::link!("propsys.dll" "system" fn VariantGetInt64Elem(var : *const VARIANT, ielem : u32, pnval : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetInt64Elem(core::mem::transmute(var), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetStringElem(var: *const VARIANT, ielem: u32) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn VariantGetStringElem(var : *const VARIANT, ielem : u32, ppszval : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetStringElem(core::mem::transmute(var), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt16Elem(var: *const VARIANT, ielem: u32) -> windows_core::Result<u16> {
    windows_core::link!("propsys.dll" "system" fn VariantGetUInt16Elem(var : *const VARIANT, ielem : u32, pnval : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetUInt16Elem(core::mem::transmute(var), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt32Elem(var: *const VARIANT, ielem: u32) -> windows_core::Result<u32> {
    windows_core::link!("propsys.dll" "system" fn VariantGetUInt32Elem(var : *const VARIANT, ielem : u32, pnval : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetUInt32Elem(core::mem::transmute(var), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt64Elem(var: *const VARIANT, ielem: u32) -> windows_core::Result<u64> {
    windows_core::link!("propsys.dll" "system" fn VariantGetUInt64Elem(var : *const VARIANT, ielem : u32, pnval : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetUInt64Elem(core::mem::transmute(var), ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantInit() -> VARIANT {
    windows_core::link!("oleaut32.dll" "system" fn VariantInit(pvarg : *mut VARIANT));
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantInit(&mut result__);
        core::mem::transmute(result__)
    }
}
#[inline]
pub unsafe fn VariantTimeToDosDateTime(vtime: f64, pwdosdate: *mut u16, pwdostime: *mut u16) -> i32 {
    windows_core::link!("oleaut32.dll" "system" fn VariantTimeToDosDateTime(vtime : f64, pwdosdate : *mut u16, pwdostime : *mut u16) -> i32);
    unsafe { VariantTimeToDosDateTime(vtime, pwdosdate as _, pwdostime as _) }
}
#[inline]
pub unsafe fn VariantTimeToSystemTime(vtime: f64, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> i32 {
    windows_core::link!("oleaut32.dll" "system" fn VariantTimeToSystemTime(vtime : f64, lpsystemtime : *mut super::super::Foundation:: SYSTEMTIME) -> i32);
    unsafe { VariantTimeToSystemTime(vtime, lpsystemtime as _) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBoolean(varin: *const VARIANT) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("propsys.dll" "system" fn VariantToBoolean(varin : *const VARIANT, pfret : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToBoolean(core::mem::transmute(varin), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanArray(var: *const VARIANT, prgf: &mut [windows_core::BOOL], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToBooleanArray(var : *const VARIANT, prgf : *mut windows_core::BOOL, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToBooleanArray(core::mem::transmute(var), core::mem::transmute(prgf.as_ptr()), prgf.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanArrayAlloc(var: *const VARIANT, pprgf: *mut *mut windows_core::BOOL, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToBooleanArrayAlloc(var : *const VARIANT, pprgf : *mut *mut windows_core::BOOL, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToBooleanArrayAlloc(core::mem::transmute(var), pprgf as _, pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanWithDefault(varin: *const VARIANT, fdefault: bool) -> windows_core::BOOL {
    windows_core::link!("propsys.dll" "system" fn VariantToBooleanWithDefault(varin : *const VARIANT, fdefault : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { VariantToBooleanWithDefault(core::mem::transmute(varin), fdefault.into()) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBuffer(varin: *const VARIANT, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToBuffer(varin : *const VARIANT, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::HRESULT);
    unsafe { VariantToBuffer(core::mem::transmute(varin), pv as _, cb).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDosDateTime(varin: *const VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToDosDateTime(varin : *const VARIANT, pwdate : *mut u16, pwtime : *mut u16) -> windows_core::HRESULT);
    unsafe { VariantToDosDateTime(core::mem::transmute(varin), pwdate as _, pwtime as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDouble(varin: *const VARIANT) -> windows_core::Result<f64> {
    windows_core::link!("propsys.dll" "system" fn VariantToDouble(varin : *const VARIANT, pdblret : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToDouble(core::mem::transmute(varin), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleArray(var: *const VARIANT, prgn: &mut [f64], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToDoubleArray(var : *const VARIANT, prgn : *mut f64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToDoubleArray(core::mem::transmute(var), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleArrayAlloc(var: *const VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToDoubleArrayAlloc(var : *const VARIANT, pprgn : *mut *mut f64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToDoubleArrayAlloc(core::mem::transmute(var), pprgn as _, pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleWithDefault(varin: *const VARIANT, dbldefault: f64) -> f64 {
    windows_core::link!("propsys.dll" "system" fn VariantToDoubleWithDefault(varin : *const VARIANT, dbldefault : f64) -> f64);
    unsafe { VariantToDoubleWithDefault(core::mem::transmute(varin), dbldefault) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToFileTime(varin: *const VARIANT, stfout: PSTIME_FLAGS) -> windows_core::Result<super::super::Foundation::FILETIME> {
    windows_core::link!("propsys.dll" "system" fn VariantToFileTime(varin : *const VARIANT, stfout : PSTIME_FLAGS, pftout : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToFileTime(core::mem::transmute(varin), stfout, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToGUID(varin: *const VARIANT) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("propsys.dll" "system" fn VariantToGUID(varin : *const VARIANT, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToGUID(core::mem::transmute(varin), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16(varin: *const VARIANT) -> windows_core::Result<i16> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16(varin : *const VARIANT, piret : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToInt16(core::mem::transmute(varin), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16Array(var: *const VARIANT, prgn: &mut [i16], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16Array(var : *const VARIANT, prgn : *mut i16, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt16Array(core::mem::transmute(var), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut i16, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt16ArrayAlloc(core::mem::transmute(var), pprgn as _, pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16WithDefault(varin: *const VARIANT, idefault: i16) -> i16 {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16WithDefault(varin : *const VARIANT, idefault : i16) -> i16);
    unsafe { VariantToInt16WithDefault(core::mem::transmute(varin), idefault) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32(varin: *const VARIANT) -> windows_core::Result<i32> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32(varin : *const VARIANT, plret : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToInt32(core::mem::transmute(varin), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32Array(var: *const VARIANT, prgn: &mut [i32], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32Array(var : *const VARIANT, prgn : *mut i32, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt32Array(core::mem::transmute(var), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut i32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt32ArrayAlloc(core::mem::transmute(var), pprgn as _, pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32WithDefault(varin: *const VARIANT, ldefault: i32) -> i32 {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32WithDefault(varin : *const VARIANT, ldefault : i32) -> i32);
    unsafe { VariantToInt32WithDefault(core::mem::transmute(varin), ldefault) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64(varin: *const VARIANT) -> windows_core::Result<i64> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64(varin : *const VARIANT, pllret : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToInt64(core::mem::transmute(varin), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64Array(var: *const VARIANT, prgn: &mut [i64], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64Array(var : *const VARIANT, prgn : *mut i64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt64Array(core::mem::transmute(var), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut i64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt64ArrayAlloc(core::mem::transmute(var), pprgn as _, pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64WithDefault(varin: *const VARIANT, lldefault: i64) -> i64 {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64WithDefault(varin : *const VARIANT, lldefault : i64) -> i64);
    unsafe { VariantToInt64WithDefault(core::mem::transmute(varin), lldefault) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToString(varin: *const VARIANT, pszbuf: &mut [u16]) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToString(varin : *const VARIANT, pszbuf : windows_core::PWSTR, cchbuf : u32) -> windows_core::HRESULT);
    unsafe { VariantToString(core::mem::transmute(varin), core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringAlloc(varin: *const VARIANT) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn VariantToStringAlloc(varin : *const VARIANT, ppszbuf : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToStringAlloc(core::mem::transmute(varin), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringArray(var: *const VARIANT, prgsz: &mut [windows_core::PWSTR], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToStringArray(var : *const VARIANT, prgsz : *mut windows_core::PWSTR, crgsz : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToStringArray(core::mem::transmute(var), core::mem::transmute(prgsz.as_ptr()), prgsz.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringArrayAlloc(var: *const VARIANT, pprgsz: *mut *mut windows_core::PWSTR, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToStringArrayAlloc(var : *const VARIANT, pprgsz : *mut *mut windows_core::PWSTR, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToStringArrayAlloc(core::mem::transmute(var), pprgsz as _, pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringWithDefault<P1>(varin: *const VARIANT, pszdefault: P1) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn VariantToStringWithDefault(varin : *const VARIANT, pszdefault : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { VariantToStringWithDefault(core::mem::transmute(varin), pszdefault.param().abi()) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16(varin: *const VARIANT) -> windows_core::Result<u16> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16(varin : *const VARIANT, puiret : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToUInt16(core::mem::transmute(varin), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16Array(var: *const VARIANT, prgn: &mut [u16], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16Array(var : *const VARIANT, prgn : *mut u16, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt16Array(core::mem::transmute(var), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut u16, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt16ArrayAlloc(core::mem::transmute(var), pprgn as _, pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16WithDefault(varin: *const VARIANT, uidefault: u16) -> u16 {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16WithDefault(varin : *const VARIANT, uidefault : u16) -> u16);
    unsafe { VariantToUInt16WithDefault(core::mem::transmute(varin), uidefault) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32(varin: *const VARIANT) -> windows_core::Result<u32> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32(varin : *const VARIANT, pulret : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToUInt32(core::mem::transmute(varin), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32Array(var: *const VARIANT, prgn: &mut [u32], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32Array(var : *const VARIANT, prgn : *mut u32, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt32Array(core::mem::transmute(var), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt32ArrayAlloc(core::mem::transmute(var), pprgn as _, pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32WithDefault(varin: *const VARIANT, uldefault: u32) -> u32 {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32WithDefault(varin : *const VARIANT, uldefault : u32) -> u32);
    unsafe { VariantToUInt32WithDefault(core::mem::transmute(varin), uldefault) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64(varin: *const VARIANT) -> windows_core::Result<u64> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64(varin : *const VARIANT, pullret : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToUInt64(core::mem::transmute(varin), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64Array(var: *const VARIANT, prgn: &mut [u64], pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64Array(var : *const VARIANT, prgn : *mut u64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt64Array(core::mem::transmute(var), core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut u64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt64ArrayAlloc(core::mem::transmute(var), pprgn as _, pcelem as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64WithDefault(varin: *const VARIANT, ulldefault: u64) -> u64 {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64WithDefault(varin : *const VARIANT, ulldefault : u64) -> u64);
    unsafe { VariantToUInt64WithDefault(core::mem::transmute(varin), ulldefault) }
}
pub const DPF_ERROR: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(4i32);
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(1i32);
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(2i32);
pub const DPF_NONE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(0i32);
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(16i32);
pub const DPF_WARNING: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAWPROGRESSFLAGS(pub i32);
impl DRAWPROGRESSFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DRAWPROGRESSFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DRAWPROGRESSFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PSTF_LOCAL: PSTIME_FLAGS = PSTIME_FLAGS(1i32);
pub const PSTF_UTC: PSTIME_FLAGS = PSTIME_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSTIME_FLAGS(pub i32);
impl PSTIME_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PSTIME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PSTIME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PSTIME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PSTIME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PSTIME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VARENUM(pub u16);
impl VARENUM {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VARENUM {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VARENUM {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VARENUM {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VARENUM {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VARENUM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Default for VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub union VARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::super::Foundation::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::super::Foundation::VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: super::Com::CY,
    pub date: f64,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<super::Com::IDispatch>>,
    pub parray: *mut super::Com::SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::super::Foundation::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut super::super::Foundation::VARIANT_BOOL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::Com::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<super::Com::IDispatch>,
    pub pparray: *mut *mut super::Com::SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut core::ffi::c_void,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: windows_core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: core::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[derive(Clone, Debug, PartialEq)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut core::ffi::c_void,
    pub pRecInfo: core::mem::ManuallyDrop<Option<super::Ole::IRecordInfo>>,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VARIANT_ALPHABOOL: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(2u16);
pub const VARIANT_CALENDAR_GREGORIAN: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(64u16);
pub const VARIANT_CALENDAR_HIJRI: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(8u16);
pub const VARIANT_CALENDAR_THAI: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(32u16);
pub const VARIANT_LOCALBOOL: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(16u16);
pub const VARIANT_NOUSEROVERRIDE: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(4u16);
pub const VARIANT_NOVALUEPROP: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(1u16);
pub const VARIANT_USE_NLS: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(128u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VAR_CHANGE_FLAGS(pub u16);
impl VAR_CHANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VAR_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VAR_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VAR_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const VT_ARRAY: VARENUM = VARENUM(8192u16);
pub const VT_BLOB: VARENUM = VARENUM(65u16);
pub const VT_BLOB_OBJECT: VARENUM = VARENUM(70u16);
pub const VT_BOOL: VARENUM = VARENUM(11u16);
pub const VT_BSTR: VARENUM = VARENUM(8u16);
pub const VT_BSTR_BLOB: VARENUM = VARENUM(4095u16);
pub const VT_BYREF: VARENUM = VARENUM(16384u16);
pub const VT_CARRAY: VARENUM = VARENUM(28u16);
pub const VT_CF: VARENUM = VARENUM(71u16);
pub const VT_CLSID: VARENUM = VARENUM(72u16);
pub const VT_CY: VARENUM = VARENUM(6u16);
pub const VT_DATE: VARENUM = VARENUM(7u16);
pub const VT_DECIMAL: VARENUM = VARENUM(14u16);
pub const VT_DISPATCH: VARENUM = VARENUM(9u16);
pub const VT_EMPTY: VARENUM = VARENUM(0u16);
pub const VT_ERROR: VARENUM = VARENUM(10u16);
pub const VT_FILETIME: VARENUM = VARENUM(64u16);
pub const VT_HRESULT: VARENUM = VARENUM(25u16);
pub const VT_I1: VARENUM = VARENUM(16u16);
pub const VT_I2: VARENUM = VARENUM(2u16);
pub const VT_I4: VARENUM = VARENUM(3u16);
pub const VT_I8: VARENUM = VARENUM(20u16);
pub const VT_ILLEGAL: VARENUM = VARENUM(65535u16);
pub const VT_ILLEGALMASKED: VARENUM = VARENUM(4095u16);
pub const VT_INT: VARENUM = VARENUM(22u16);
pub const VT_INT_PTR: VARENUM = VARENUM(37u16);
pub const VT_LPSTR: VARENUM = VARENUM(30u16);
pub const VT_LPWSTR: VARENUM = VARENUM(31u16);
pub const VT_NULL: VARENUM = VARENUM(1u16);
pub const VT_PTR: VARENUM = VARENUM(26u16);
pub const VT_R4: VARENUM = VARENUM(4u16);
pub const VT_R8: VARENUM = VARENUM(5u16);
pub const VT_RECORD: VARENUM = VARENUM(36u16);
pub const VT_RESERVED: VARENUM = VARENUM(32768u16);
pub const VT_SAFEARRAY: VARENUM = VARENUM(27u16);
pub const VT_STORAGE: VARENUM = VARENUM(67u16);
pub const VT_STORED_OBJECT: VARENUM = VARENUM(69u16);
pub const VT_STREAM: VARENUM = VARENUM(66u16);
pub const VT_STREAMED_OBJECT: VARENUM = VARENUM(68u16);
pub const VT_TYPEMASK: VARENUM = VARENUM(4095u16);
pub const VT_UI1: VARENUM = VARENUM(17u16);
pub const VT_UI2: VARENUM = VARENUM(18u16);
pub const VT_UI4: VARENUM = VARENUM(19u16);
pub const VT_UI8: VARENUM = VARENUM(21u16);
pub const VT_UINT: VARENUM = VARENUM(23u16);
pub const VT_UINT_PTR: VARENUM = VARENUM(38u16);
pub const VT_UNKNOWN: VARENUM = VARENUM(13u16);
pub const VT_USERDEFINED: VARENUM = VARENUM(29u16);
pub const VT_VARIANT: VARENUM = VARENUM(12u16);
pub const VT_VECTOR: VARENUM = VARENUM(4096u16);
pub const VT_VERSIONED_STREAM: VARENUM = VARENUM(73u16);
pub const VT_VOID: VARENUM = VARENUM(24u16);
