#[cfg(feature = "Win32_System_Search_Common")]
pub mod Common;
#[inline]
pub unsafe fn ODBCGetTryWaitValue() -> u32 {
    windows_targets::link!("odbc32.dll" "system" fn ODBCGetTryWaitValue() -> u32);
    ODBCGetTryWaitValue()
}
#[inline]
pub unsafe fn ODBCSetTryWaitValue(dwvalue: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("odbc32.dll" "system" fn ODBCSetTryWaitValue(dwvalue : u32) -> super::super::Foundation:: BOOL);
    ODBCSetTryWaitValue(dwvalue)
}
#[inline]
pub unsafe fn SQLAllocConnect(environmenthandle: *mut core::ffi::c_void, connectionhandle: *mut *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLAllocConnect(environmenthandle : *mut core::ffi::c_void, connectionhandle : *mut *mut core::ffi::c_void) -> i16);
    SQLAllocConnect(environmenthandle, connectionhandle)
}
#[inline]
pub unsafe fn SQLAllocEnv(environmenthandle: *mut *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLAllocEnv(environmenthandle : *mut *mut core::ffi::c_void) -> i16);
    SQLAllocEnv(environmenthandle)
}
#[inline]
pub unsafe fn SQLAllocHandle(handletype: i16, inputhandle: *mut core::ffi::c_void, outputhandle: *mut *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLAllocHandle(handletype : i16, inputhandle : *mut core::ffi::c_void, outputhandle : *mut *mut core::ffi::c_void) -> i16);
    SQLAllocHandle(handletype, inputhandle, outputhandle)
}
#[inline]
pub unsafe fn SQLAllocHandleStd(fhandletype: i16, hinput: *mut core::ffi::c_void, phoutput: *mut *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLAllocHandleStd(fhandletype : i16, hinput : *mut core::ffi::c_void, phoutput : *mut *mut core::ffi::c_void) -> i16);
    SQLAllocHandleStd(fhandletype, hinput, phoutput)
}
#[inline]
pub unsafe fn SQLAllocStmt(connectionhandle: *mut core::ffi::c_void, statementhandle: *mut *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLAllocStmt(connectionhandle : *mut core::ffi::c_void, statementhandle : *mut *mut core::ffi::c_void) -> i16);
    SQLAllocStmt(connectionhandle, statementhandle)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLBindCol(statementhandle: *mut core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: Option<*mut core::ffi::c_void>, bufferlength: i64, strlen_or_ind: Option<*mut i64>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLBindCol(statementhandle : *mut core::ffi::c_void, columnnumber : u16, targettype : i16, targetvalue : *mut core::ffi::c_void, bufferlength : i64, strlen_or_ind : *mut i64) -> i16);
    SQLBindCol(statementhandle, columnnumber, targettype, core::mem::transmute(targetvalue.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(strlen_or_ind.unwrap_or(std::ptr::null_mut())))
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLBindCol(statementhandle: *mut core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: Option<*mut core::ffi::c_void>, bufferlength: i32, strlen_or_ind: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLBindCol(statementhandle : *mut core::ffi::c_void, columnnumber : u16, targettype : i16, targetvalue : *mut core::ffi::c_void, bufferlength : i32, strlen_or_ind : *mut i32) -> i16);
    SQLBindCol(statementhandle, columnnumber, targettype, core::mem::transmute(targetvalue.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(strlen_or_ind.unwrap_or(std::ptr::null_mut())))
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLBindParam(statementhandle: *mut core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u64, parameterscale: i16, parametervalue: *mut core::ffi::c_void, strlen_or_ind: *mut i64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLBindParam(statementhandle : *mut core::ffi::c_void, parameternumber : u16, valuetype : i16, parametertype : i16, lengthprecision : u64, parameterscale : i16, parametervalue : *mut core::ffi::c_void, strlen_or_ind : *mut i64) -> i16);
    SQLBindParam(statementhandle, parameternumber, valuetype, parametertype, lengthprecision, parameterscale, parametervalue, strlen_or_ind)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLBindParam(statementhandle: *mut core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u32, parameterscale: i16, parametervalue: *mut core::ffi::c_void, strlen_or_ind: *mut i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLBindParam(statementhandle : *mut core::ffi::c_void, parameternumber : u16, valuetype : i16, parametertype : i16, lengthprecision : u32, parameterscale : i16, parametervalue : *mut core::ffi::c_void, strlen_or_ind : *mut i32) -> i16);
    SQLBindParam(statementhandle, parameternumber, valuetype, parametertype, lengthprecision, parameterscale, parametervalue, strlen_or_ind)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLBindParameter(hstmt: *mut core::ffi::c_void, ipar: u16, fparamtype: i16, fctype: i16, fsqltype: i16, cbcoldef: u64, ibscale: i16, rgbvalue: *mut core::ffi::c_void, cbvaluemax: i64, pcbvalue: *mut i64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLBindParameter(hstmt : *mut core::ffi::c_void, ipar : u16, fparamtype : i16, fctype : i16, fsqltype : i16, cbcoldef : u64, ibscale : i16, rgbvalue : *mut core::ffi::c_void, cbvaluemax : i64, pcbvalue : *mut i64) -> i16);
    SQLBindParameter(hstmt, ipar, fparamtype, fctype, fsqltype, cbcoldef, ibscale, rgbvalue, cbvaluemax, pcbvalue)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLBindParameter(hstmt: *mut core::ffi::c_void, ipar: u16, fparamtype: i16, fctype: i16, fsqltype: i16, cbcoldef: u32, ibscale: i16, rgbvalue: *mut core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLBindParameter(hstmt : *mut core::ffi::c_void, ipar : u16, fparamtype : i16, fctype : i16, fsqltype : i16, cbcoldef : u32, ibscale : i16, rgbvalue : *mut core::ffi::c_void, cbvaluemax : i32, pcbvalue : *mut i32) -> i16);
    SQLBindParameter(hstmt, ipar, fparamtype, fctype, fsqltype, cbcoldef, ibscale, rgbvalue, cbvaluemax, pcbvalue)
}
#[inline]
pub unsafe fn SQLBrowseConnect(hdbc: *mut core::ffi::c_void, szconnstrin: &[u8], szconnstrout: Option<&mut [u8]>, pcchconnstrout: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLBrowseConnect(hdbc : *mut core::ffi::c_void, szconnstrin : *const u8, cchconnstrin : i16, szconnstrout : *mut u8, cchconnstroutmax : i16, pcchconnstrout : *mut i16) -> i16);
    SQLBrowseConnect(hdbc, core::mem::transmute(szconnstrin.as_ptr()), szconnstrin.len().try_into().unwrap(), core::mem::transmute(szconnstrout.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szconnstrout.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcchconnstrout.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLBrowseConnectA(hdbc: *mut core::ffi::c_void, szconnstrin: &[u8], szconnstrout: Option<&mut [u8]>, pcbconnstrout: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLBrowseConnectA(hdbc : *mut core::ffi::c_void, szconnstrin : *const u8, cbconnstrin : i16, szconnstrout : *mut u8, cbconnstroutmax : i16, pcbconnstrout : *mut i16) -> i16);
    SQLBrowseConnectA(hdbc, core::mem::transmute(szconnstrin.as_ptr()), szconnstrin.len().try_into().unwrap(), core::mem::transmute(szconnstrout.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szconnstrout.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcbconnstrout.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLBrowseConnectW(hdbc: *mut core::ffi::c_void, szconnstrin: &[u16], szconnstrout: Option<&mut [u16]>, pcchconnstrout: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLBrowseConnectW(hdbc : *mut core::ffi::c_void, szconnstrin : *const u16, cchconnstrin : i16, szconnstrout : *mut u16, cchconnstroutmax : i16, pcchconnstrout : *mut i16) -> i16);
    SQLBrowseConnectW(hdbc, core::mem::transmute(szconnstrin.as_ptr()), szconnstrin.len().try_into().unwrap(), core::mem::transmute(szconnstrout.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szconnstrout.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcchconnstrout.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLBulkOperations(statementhandle: *mut core::ffi::c_void, operation: i16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLBulkOperations(statementhandle : *mut core::ffi::c_void, operation : i16) -> i16);
    SQLBulkOperations(statementhandle, operation)
}
#[inline]
pub unsafe fn SQLCancel(statementhandle: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLCancel(statementhandle : *mut core::ffi::c_void) -> i16);
    SQLCancel(statementhandle)
}
#[inline]
pub unsafe fn SQLCancelHandle(handletype: i16, inputhandle: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLCancelHandle(handletype : i16, inputhandle : *mut core::ffi::c_void) -> i16);
    SQLCancelHandle(handletype, inputhandle)
}
#[inline]
pub unsafe fn SQLCloseCursor(statementhandle: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLCloseCursor(statementhandle : *mut core::ffi::c_void) -> i16);
    SQLCloseCursor(statementhandle)
}
#[inline]
pub unsafe fn SQLCloseEnumServers<P0>(henumhandle: P0) -> i16
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn SQLCloseEnumServers(henumhandle : super::super::Foundation:: HANDLE) -> i16);
    SQLCloseEnumServers(henumhandle.param().abi())
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLColAttribute(statementhandle: *mut core::ffi::c_void, columnnumber: u16, fieldidentifier: u16, characterattribute: Option<*mut core::ffi::c_void>, bufferlength: i16, stringlength: Option<*mut i16>, numericattribute: Option<*mut i64>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttribute(statementhandle : *mut core::ffi::c_void, columnnumber : u16, fieldidentifier : u16, characterattribute : *mut core::ffi::c_void, bufferlength : i16, stringlength : *mut i16, numericattribute : *mut i64) -> i16);
    SQLColAttribute(statementhandle, columnnumber, fieldidentifier, core::mem::transmute(characterattribute.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(stringlength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(numericattribute.unwrap_or(std::ptr::null_mut())))
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLColAttribute(statementhandle: *mut core::ffi::c_void, columnnumber: u16, fieldidentifier: u16, characterattribute: Option<*mut core::ffi::c_void>, bufferlength: i16, stringlength: Option<*mut i16>, numericattribute: Option<*mut core::ffi::c_void>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttribute(statementhandle : *mut core::ffi::c_void, columnnumber : u16, fieldidentifier : u16, characterattribute : *mut core::ffi::c_void, bufferlength : i16, stringlength : *mut i16, numericattribute : *mut core::ffi::c_void) -> i16);
    SQLColAttribute(statementhandle, columnnumber, fieldidentifier, core::mem::transmute(characterattribute.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(stringlength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(numericattribute.unwrap_or(std::ptr::null_mut())))
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLColAttributeA(hstmt: *mut core::ffi::c_void, icol: i16, ifield: i16, pcharattr: Option<*mut core::ffi::c_void>, cbcharattrmax: i16, pcbcharattr: Option<*mut i16>, pnumattr: Option<*mut i64>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttributeA(hstmt : *mut core::ffi::c_void, icol : i16, ifield : i16, pcharattr : *mut core::ffi::c_void, cbcharattrmax : i16, pcbcharattr : *mut i16, pnumattr : *mut i64) -> i16);
    SQLColAttributeA(hstmt, icol, ifield, core::mem::transmute(pcharattr.unwrap_or(std::ptr::null_mut())), cbcharattrmax, core::mem::transmute(pcbcharattr.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumattr.unwrap_or(std::ptr::null_mut())))
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLColAttributeA(hstmt: *mut core::ffi::c_void, icol: i16, ifield: i16, pcharattr: Option<*mut core::ffi::c_void>, cbcharattrmax: i16, pcbcharattr: Option<*mut i16>, pnumattr: Option<*mut core::ffi::c_void>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttributeA(hstmt : *mut core::ffi::c_void, icol : i16, ifield : i16, pcharattr : *mut core::ffi::c_void, cbcharattrmax : i16, pcbcharattr : *mut i16, pnumattr : *mut core::ffi::c_void) -> i16);
    SQLColAttributeA(hstmt, icol, ifield, core::mem::transmute(pcharattr.unwrap_or(std::ptr::null_mut())), cbcharattrmax, core::mem::transmute(pcbcharattr.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumattr.unwrap_or(std::ptr::null_mut())))
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLColAttributeW(hstmt: *mut core::ffi::c_void, icol: u16, ifield: u16, pcharattr: Option<*mut core::ffi::c_void>, cbdescmax: i16, pcbcharattr: Option<*mut i16>, pnumattr: Option<*mut i64>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttributeW(hstmt : *mut core::ffi::c_void, icol : u16, ifield : u16, pcharattr : *mut core::ffi::c_void, cbdescmax : i16, pcbcharattr : *mut i16, pnumattr : *mut i64) -> i16);
    SQLColAttributeW(hstmt, icol, ifield, core::mem::transmute(pcharattr.unwrap_or(std::ptr::null_mut())), cbdescmax, core::mem::transmute(pcbcharattr.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumattr.unwrap_or(std::ptr::null_mut())))
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLColAttributeW(hstmt: *mut core::ffi::c_void, icol: u16, ifield: u16, pcharattr: Option<*mut core::ffi::c_void>, cbdescmax: i16, pcbcharattr: Option<*mut i16>, pnumattr: Option<*mut core::ffi::c_void>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttributeW(hstmt : *mut core::ffi::c_void, icol : u16, ifield : u16, pcharattr : *mut core::ffi::c_void, cbdescmax : i16, pcbcharattr : *mut i16, pnumattr : *mut core::ffi::c_void) -> i16);
    SQLColAttributeW(hstmt, icol, ifield, core::mem::transmute(pcharattr.unwrap_or(std::ptr::null_mut())), cbdescmax, core::mem::transmute(pcbcharattr.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumattr.unwrap_or(std::ptr::null_mut())))
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLColAttributes(hstmt: *mut core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttributes(hstmt : *mut core::ffi::c_void, icol : u16, fdesctype : u16, rgbdesc : *mut core::ffi::c_void, cbdescmax : i16, pcbdesc : *mut i16, pfdesc : *mut i64) -> i16);
    SQLColAttributes(hstmt, icol, fdesctype, rgbdesc, cbdescmax, pcbdesc, pfdesc)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLColAttributes(hstmt: *mut core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttributes(hstmt : *mut core::ffi::c_void, icol : u16, fdesctype : u16, rgbdesc : *mut core::ffi::c_void, cbdescmax : i16, pcbdesc : *mut i16, pfdesc : *mut i32) -> i16);
    SQLColAttributes(hstmt, icol, fdesctype, rgbdesc, cbdescmax, pcbdesc, pfdesc)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLColAttributesA(hstmt: *mut core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: Option<*mut core::ffi::c_void>, cbdescmax: i16, pcbdesc: Option<*mut i16>, pfdesc: Option<*mut i64>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttributesA(hstmt : *mut core::ffi::c_void, icol : u16, fdesctype : u16, rgbdesc : *mut core::ffi::c_void, cbdescmax : i16, pcbdesc : *mut i16, pfdesc : *mut i64) -> i16);
    SQLColAttributesA(hstmt, icol, fdesctype, core::mem::transmute(rgbdesc.unwrap_or(std::ptr::null_mut())), cbdescmax, core::mem::transmute(pcbdesc.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfdesc.unwrap_or(std::ptr::null_mut())))
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLColAttributesA(hstmt: *mut core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: Option<*mut core::ffi::c_void>, cbdescmax: i16, pcbdesc: Option<*mut i16>, pfdesc: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttributesA(hstmt : *mut core::ffi::c_void, icol : u16, fdesctype : u16, rgbdesc : *mut core::ffi::c_void, cbdescmax : i16, pcbdesc : *mut i16, pfdesc : *mut i32) -> i16);
    SQLColAttributesA(hstmt, icol, fdesctype, core::mem::transmute(rgbdesc.unwrap_or(std::ptr::null_mut())), cbdescmax, core::mem::transmute(pcbdesc.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfdesc.unwrap_or(std::ptr::null_mut())))
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLColAttributesW(hstmt: *mut core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: Option<*mut core::ffi::c_void>, cbdescmax: i16, pcbdesc: Option<*mut i16>, pfdesc: Option<*mut i64>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttributesW(hstmt : *mut core::ffi::c_void, icol : u16, fdesctype : u16, rgbdesc : *mut core::ffi::c_void, cbdescmax : i16, pcbdesc : *mut i16, pfdesc : *mut i64) -> i16);
    SQLColAttributesW(hstmt, icol, fdesctype, core::mem::transmute(rgbdesc.unwrap_or(std::ptr::null_mut())), cbdescmax, core::mem::transmute(pcbdesc.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfdesc.unwrap_or(std::ptr::null_mut())))
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLColAttributesW(hstmt: *mut core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: Option<*mut core::ffi::c_void>, cbdescmax: i16, pcbdesc: Option<*mut i16>, pfdesc: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColAttributesW(hstmt : *mut core::ffi::c_void, icol : u16, fdesctype : u16, rgbdesc : *mut core::ffi::c_void, cbdescmax : i16, pcbdesc : *mut i16, pfdesc : *mut i32) -> i16);
    SQLColAttributesW(hstmt, icol, fdesctype, core::mem::transmute(rgbdesc.unwrap_or(std::ptr::null_mut())), cbdescmax, core::mem::transmute(pcbdesc.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfdesc.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLColumnPrivileges(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, sztablename: Option<&[u8]>, szcolumnname: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColumnPrivileges(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cchcatalogname : i16, szschemaname : *const u8, cchschemaname : i16, sztablename : *const u8, cchtablename : i16, szcolumnname : *const u8, cchcolumnname : i16) -> i16);
    SQLColumnPrivileges(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szcolumnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolumnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLColumnPrivilegesA(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, sztablename: Option<&[u8]>, szcolumnname: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColumnPrivilegesA(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cbcatalogname : i16, szschemaname : *const u8, cbschemaname : i16, sztablename : *const u8, cbtablename : i16, szcolumnname : *const u8, cbcolumnname : i16) -> i16);
    SQLColumnPrivilegesA(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szcolumnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolumnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLColumnPrivilegesW(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u16]>, szschemaname: Option<&[u16]>, sztablename: Option<&[u16]>, szcolumnname: Option<&[u16]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColumnPrivilegesW(hstmt : *mut core::ffi::c_void, szcatalogname : *const u16, cchcatalogname : i16, szschemaname : *const u16, cchschemaname : i16, sztablename : *const u16, cchtablename : i16, szcolumnname : *const u16, cchcolumnname : i16) -> i16);
    SQLColumnPrivilegesW(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szcolumnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolumnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLColumns(statementhandle: *mut core::ffi::c_void, catalogname: Option<&[u8]>, schemaname: Option<&[u8]>, tablename: Option<&[u8]>, columnname: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColumns(statementhandle : *mut core::ffi::c_void, catalogname : *const u8, namelength1 : i16, schemaname : *const u8, namelength2 : i16, tablename : *const u8, namelength3 : i16, columnname : *const u8, namelength4 : i16) -> i16);
    SQLColumns(
        statementhandle,
        core::mem::transmute(catalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        catalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(schemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        schemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(tablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        tablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(columnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        columnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLColumnsA(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, sztablename: Option<&[u8]>, szcolumnname: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColumnsA(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cbcatalogname : i16, szschemaname : *const u8, cbschemaname : i16, sztablename : *const u8, cbtablename : i16, szcolumnname : *const u8, cbcolumnname : i16) -> i16);
    SQLColumnsA(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szcolumnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolumnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLColumnsW(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u16]>, szschemaname: Option<&[u16]>, sztablename: Option<&[u16]>, szcolumnname: Option<&[u16]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLColumnsW(hstmt : *mut core::ffi::c_void, szcatalogname : *const u16, cchcatalogname : i16, szschemaname : *const u16, cchschemaname : i16, sztablename : *const u16, cchtablename : i16, szcolumnname : *const u16, cchcolumnname : i16) -> i16);
    SQLColumnsW(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szcolumnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolumnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLCompleteAsync(handletype: i16, handle: *mut core::ffi::c_void, asyncretcodeptr: *mut i16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLCompleteAsync(handletype : i16, handle : *mut core::ffi::c_void, asyncretcodeptr : *mut i16) -> i16);
    SQLCompleteAsync(handletype, handle, asyncretcodeptr)
}
#[inline]
pub unsafe fn SQLConnect(connectionhandle: *mut core::ffi::c_void, servername: &[u8], username: &[u8], authentication: &[u8]) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLConnect(connectionhandle : *mut core::ffi::c_void, servername : *const u8, namelength1 : i16, username : *const u8, namelength2 : i16, authentication : *const u8, namelength3 : i16) -> i16);
    SQLConnect(connectionhandle, core::mem::transmute(servername.as_ptr()), servername.len().try_into().unwrap(), core::mem::transmute(username.as_ptr()), username.len().try_into().unwrap(), core::mem::transmute(authentication.as_ptr()), authentication.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SQLConnectA(hdbc: *mut core::ffi::c_void, szdsn: &[u8], szuid: &[u8], szauthstr: &[u8]) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLConnectA(hdbc : *mut core::ffi::c_void, szdsn : *const u8, cbdsn : i16, szuid : *const u8, cbuid : i16, szauthstr : *const u8, cbauthstr : i16) -> i16);
    SQLConnectA(hdbc, core::mem::transmute(szdsn.as_ptr()), szdsn.len().try_into().unwrap(), core::mem::transmute(szuid.as_ptr()), szuid.len().try_into().unwrap(), core::mem::transmute(szauthstr.as_ptr()), szauthstr.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SQLConnectW(hdbc: *mut core::ffi::c_void, szdsn: &[u16], szuid: &[u16], szauthstr: &[u16]) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLConnectW(hdbc : *mut core::ffi::c_void, szdsn : *const u16, cchdsn : i16, szuid : *const u16, cchuid : i16, szauthstr : *const u16, cchauthstr : i16) -> i16);
    SQLConnectW(hdbc, core::mem::transmute(szdsn.as_ptr()), szdsn.len().try_into().unwrap(), core::mem::transmute(szuid.as_ptr()), szuid.len().try_into().unwrap(), core::mem::transmute(szauthstr.as_ptr()), szauthstr.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SQLCopyDesc(sourcedeschandle: *mut core::ffi::c_void, targetdeschandle: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLCopyDesc(sourcedeschandle : *mut core::ffi::c_void, targetdeschandle : *mut core::ffi::c_void) -> i16);
    SQLCopyDesc(sourcedeschandle, targetdeschandle)
}
#[inline]
pub unsafe fn SQLDataSources(environmenthandle: *mut core::ffi::c_void, direction: u16, servername: Option<&mut [u8]>, namelength1ptr: Option<*mut i16>, description: Option<&mut [u8]>, namelength2ptr: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDataSources(environmenthandle : *mut core::ffi::c_void, direction : u16, servername : *mut u8, bufferlength1 : i16, namelength1ptr : *mut i16, description : *mut u8, bufferlength2 : i16, namelength2ptr : *mut i16) -> i16);
    SQLDataSources(
        environmenthandle,
        direction,
        core::mem::transmute(servername.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        servername.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(namelength1ptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(description.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        description.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(namelength2ptr.unwrap_or(std::ptr::null_mut())),
    )
}
#[inline]
pub unsafe fn SQLDataSourcesA(henv: *mut core::ffi::c_void, fdirection: u16, szdsn: Option<&mut [u8]>, pcbdsn: *mut i16, szdescription: Option<&mut [u8]>, pcbdescription: *mut i16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDataSourcesA(henv : *mut core::ffi::c_void, fdirection : u16, szdsn : *mut u8, cbdsnmax : i16, pcbdsn : *mut i16, szdescription : *mut u8, cbdescriptionmax : i16, pcbdescription : *mut i16) -> i16);
    SQLDataSourcesA(henv, fdirection, core::mem::transmute(szdsn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szdsn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbdsn, core::mem::transmute(szdescription.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbdescription)
}
#[inline]
pub unsafe fn SQLDataSourcesW(henv: *mut core::ffi::c_void, fdirection: u16, szdsn: Option<&mut [u16]>, pcchdsn: Option<*mut i16>, wszdescription: Option<&mut [u16]>, pcchdescription: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDataSourcesW(henv : *mut core::ffi::c_void, fdirection : u16, szdsn : *mut u16, cchdsnmax : i16, pcchdsn : *mut i16, wszdescription : *mut u16, cchdescriptionmax : i16, pcchdescription : *mut i16) -> i16);
    SQLDataSourcesW(
        henv,
        fdirection,
        core::mem::transmute(szdsn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szdsn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcchdsn.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(wszdescription.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        wszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcchdescription.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLDescribeCol(statementhandle: *mut core::ffi::c_void, columnnumber: u16, columnname: Option<&mut [u8]>, namelength: Option<*mut i16>, datatype: Option<*mut i16>, columnsize: Option<*mut u64>, decimaldigits: Option<*mut i16>, nullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDescribeCol(statementhandle : *mut core::ffi::c_void, columnnumber : u16, columnname : *mut u8, bufferlength : i16, namelength : *mut i16, datatype : *mut i16, columnsize : *mut u64, decimaldigits : *mut i16, nullable : *mut i16) -> i16);
    SQLDescribeCol(
        statementhandle,
        columnnumber,
        core::mem::transmute(columnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        columnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(namelength.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(datatype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(columnsize.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(decimaldigits.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(nullable.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLDescribeCol(statementhandle: *mut core::ffi::c_void, columnnumber: u16, columnname: Option<&mut [u8]>, namelength: Option<*mut i16>, datatype: Option<*mut i16>, columnsize: Option<*mut u32>, decimaldigits: Option<*mut i16>, nullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDescribeCol(statementhandle : *mut core::ffi::c_void, columnnumber : u16, columnname : *mut u8, bufferlength : i16, namelength : *mut i16, datatype : *mut i16, columnsize : *mut u32, decimaldigits : *mut i16, nullable : *mut i16) -> i16);
    SQLDescribeCol(
        statementhandle,
        columnnumber,
        core::mem::transmute(columnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        columnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(namelength.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(datatype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(columnsize.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(decimaldigits.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(nullable.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLDescribeColA(hstmt: *mut core::ffi::c_void, icol: u16, szcolname: Option<&mut [u8]>, pcbcolname: Option<*mut i16>, pfsqltype: Option<*mut i16>, pcbcoldef: Option<*mut u64>, pibscale: Option<*mut i16>, pfnullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDescribeColA(hstmt : *mut core::ffi::c_void, icol : u16, szcolname : *mut u8, cbcolnamemax : i16, pcbcolname : *mut i16, pfsqltype : *mut i16, pcbcoldef : *mut u64, pibscale : *mut i16, pfnullable : *mut i16) -> i16);
    SQLDescribeColA(
        hstmt,
        icol,
        core::mem::transmute(szcolname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcbcolname.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfsqltype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pcbcoldef.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pibscale.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfnullable.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLDescribeColA(hstmt: *mut core::ffi::c_void, icol: u16, szcolname: Option<&mut [u8]>, pcbcolname: Option<*mut i16>, pfsqltype: Option<*mut i16>, pcbcoldef: Option<*mut u32>, pibscale: Option<*mut i16>, pfnullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDescribeColA(hstmt : *mut core::ffi::c_void, icol : u16, szcolname : *mut u8, cbcolnamemax : i16, pcbcolname : *mut i16, pfsqltype : *mut i16, pcbcoldef : *mut u32, pibscale : *mut i16, pfnullable : *mut i16) -> i16);
    SQLDescribeColA(
        hstmt,
        icol,
        core::mem::transmute(szcolname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcbcolname.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfsqltype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pcbcoldef.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pibscale.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfnullable.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLDescribeColW(hstmt: *mut core::ffi::c_void, icol: u16, szcolname: Option<&mut [u16]>, pcchcolname: Option<*mut i16>, pfsqltype: Option<*mut i16>, pcbcoldef: Option<*mut u64>, pibscale: Option<*mut i16>, pfnullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDescribeColW(hstmt : *mut core::ffi::c_void, icol : u16, szcolname : *mut u16, cchcolnamemax : i16, pcchcolname : *mut i16, pfsqltype : *mut i16, pcbcoldef : *mut u64, pibscale : *mut i16, pfnullable : *mut i16) -> i16);
    SQLDescribeColW(
        hstmt,
        icol,
        core::mem::transmute(szcolname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcchcolname.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfsqltype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pcbcoldef.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pibscale.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfnullable.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLDescribeColW(hstmt: *mut core::ffi::c_void, icol: u16, szcolname: Option<&mut [u16]>, pcchcolname: Option<*mut i16>, pfsqltype: Option<*mut i16>, pcbcoldef: Option<*mut u32>, pibscale: Option<*mut i16>, pfnullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDescribeColW(hstmt : *mut core::ffi::c_void, icol : u16, szcolname : *mut u16, cchcolnamemax : i16, pcchcolname : *mut i16, pfsqltype : *mut i16, pcbcoldef : *mut u32, pibscale : *mut i16, pfnullable : *mut i16) -> i16);
    SQLDescribeColW(
        hstmt,
        icol,
        core::mem::transmute(szcolname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcchcolname.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfsqltype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pcbcoldef.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pibscale.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfnullable.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLDescribeParam(hstmt: *mut core::ffi::c_void, ipar: u16, pfsqltype: Option<*mut i16>, pcbparamdef: Option<*mut u64>, pibscale: Option<*mut i16>, pfnullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDescribeParam(hstmt : *mut core::ffi::c_void, ipar : u16, pfsqltype : *mut i16, pcbparamdef : *mut u64, pibscale : *mut i16, pfnullable : *mut i16) -> i16);
    SQLDescribeParam(hstmt, ipar, core::mem::transmute(pfsqltype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcbparamdef.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pibscale.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfnullable.unwrap_or(std::ptr::null_mut())))
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLDescribeParam(hstmt: *mut core::ffi::c_void, ipar: u16, pfsqltype: Option<*mut i16>, pcbparamdef: Option<*mut u32>, pibscale: Option<*mut i16>, pfnullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDescribeParam(hstmt : *mut core::ffi::c_void, ipar : u16, pfsqltype : *mut i16, pcbparamdef : *mut u32, pibscale : *mut i16, pfnullable : *mut i16) -> i16);
    SQLDescribeParam(hstmt, ipar, core::mem::transmute(pfsqltype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcbparamdef.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pibscale.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfnullable.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLDisconnect(connectionhandle: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDisconnect(connectionhandle : *mut core::ffi::c_void) -> i16);
    SQLDisconnect(connectionhandle)
}
#[inline]
pub unsafe fn SQLDriverConnect(hdbc: *mut core::ffi::c_void, hwnd: isize, szconnstrin: &[u8], szconnstrout: Option<&mut [u8]>, pcchconnstrout: Option<*mut i16>, fdrivercompletion: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDriverConnect(hdbc : *mut core::ffi::c_void, hwnd : isize, szconnstrin : *const u8, cchconnstrin : i16, szconnstrout : *mut u8, cchconnstroutmax : i16, pcchconnstrout : *mut i16, fdrivercompletion : u16) -> i16);
    SQLDriverConnect(hdbc, hwnd, core::mem::transmute(szconnstrin.as_ptr()), szconnstrin.len().try_into().unwrap(), core::mem::transmute(szconnstrout.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szconnstrout.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcchconnstrout.unwrap_or(std::ptr::null_mut())), fdrivercompletion)
}
#[inline]
pub unsafe fn SQLDriverConnectA(hdbc: *mut core::ffi::c_void, hwnd: isize, szconnstrin: &[u8], szconnstrout: Option<&mut [u8]>, pcbconnstrout: Option<*mut i16>, fdrivercompletion: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDriverConnectA(hdbc : *mut core::ffi::c_void, hwnd : isize, szconnstrin : *const u8, cbconnstrin : i16, szconnstrout : *mut u8, cbconnstroutmax : i16, pcbconnstrout : *mut i16, fdrivercompletion : u16) -> i16);
    SQLDriverConnectA(hdbc, hwnd, core::mem::transmute(szconnstrin.as_ptr()), szconnstrin.len().try_into().unwrap(), core::mem::transmute(szconnstrout.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szconnstrout.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcbconnstrout.unwrap_or(std::ptr::null_mut())), fdrivercompletion)
}
#[inline]
pub unsafe fn SQLDriverConnectW(hdbc: *mut core::ffi::c_void, hwnd: isize, szconnstrin: &[u16], szconnstrout: Option<&mut [u16]>, pcchconnstrout: Option<*mut i16>, fdrivercompletion: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDriverConnectW(hdbc : *mut core::ffi::c_void, hwnd : isize, szconnstrin : *const u16, cchconnstrin : i16, szconnstrout : *mut u16, cchconnstroutmax : i16, pcchconnstrout : *mut i16, fdrivercompletion : u16) -> i16);
    SQLDriverConnectW(hdbc, hwnd, core::mem::transmute(szconnstrin.as_ptr()), szconnstrin.len().try_into().unwrap(), core::mem::transmute(szconnstrout.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szconnstrout.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcchconnstrout.unwrap_or(std::ptr::null_mut())), fdrivercompletion)
}
#[inline]
pub unsafe fn SQLDrivers(henv: *mut core::ffi::c_void, fdirection: u16, szdriverdesc: Option<&mut [u8]>, pcchdriverdesc: Option<*mut i16>, szdriverattributes: Option<&mut [u8]>, pcchdrvrattr: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDrivers(henv : *mut core::ffi::c_void, fdirection : u16, szdriverdesc : *mut u8, cchdriverdescmax : i16, pcchdriverdesc : *mut i16, szdriverattributes : *mut u8, cchdrvrattrmax : i16, pcchdrvrattr : *mut i16) -> i16);
    SQLDrivers(
        henv,
        fdirection,
        core::mem::transmute(szdriverdesc.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szdriverdesc.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcchdriverdesc.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(szdriverattributes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szdriverattributes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcchdrvrattr.unwrap_or(std::ptr::null_mut())),
    )
}
#[inline]
pub unsafe fn SQLDriversA(henv: *mut core::ffi::c_void, fdirection: u16, szdriverdesc: Option<&mut [u8]>, pcbdriverdesc: Option<*mut i16>, szdriverattributes: Option<&mut [u8]>, pcbdrvrattr: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDriversA(henv : *mut core::ffi::c_void, fdirection : u16, szdriverdesc : *mut u8, cbdriverdescmax : i16, pcbdriverdesc : *mut i16, szdriverattributes : *mut u8, cbdrvrattrmax : i16, pcbdrvrattr : *mut i16) -> i16);
    SQLDriversA(
        henv,
        fdirection,
        core::mem::transmute(szdriverdesc.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szdriverdesc.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcbdriverdesc.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(szdriverattributes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szdriverattributes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcbdrvrattr.unwrap_or(std::ptr::null_mut())),
    )
}
#[inline]
pub unsafe fn SQLDriversW(henv: *mut core::ffi::c_void, fdirection: u16, szdriverdesc: Option<&mut [u16]>, pcchdriverdesc: Option<*mut i16>, szdriverattributes: Option<&mut [u16]>, pcchdrvrattr: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLDriversW(henv : *mut core::ffi::c_void, fdirection : u16, szdriverdesc : *mut u16, cchdriverdescmax : i16, pcchdriverdesc : *mut i16, szdriverattributes : *mut u16, cchdrvrattrmax : i16, pcchdrvrattr : *mut i16) -> i16);
    SQLDriversW(
        henv,
        fdirection,
        core::mem::transmute(szdriverdesc.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szdriverdesc.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcchdriverdesc.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(szdriverattributes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szdriverattributes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcchdrvrattr.unwrap_or(std::ptr::null_mut())),
    )
}
#[inline]
pub unsafe fn SQLEndTran(handletype: i16, handle: *mut core::ffi::c_void, completiontype: i16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLEndTran(handletype : i16, handle : *mut core::ffi::c_void, completiontype : i16) -> i16);
    SQLEndTran(handletype, handle, completiontype)
}
#[inline]
pub unsafe fn SQLError(environmenthandle: *mut core::ffi::c_void, connectionhandle: *mut core::ffi::c_void, statementhandle: *mut core::ffi::c_void, sqlstate: &mut [u8; 6], nativeerror: Option<*mut i32>, messagetext: Option<&mut [u8]>, textlength: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLError(environmenthandle : *mut core::ffi::c_void, connectionhandle : *mut core::ffi::c_void, statementhandle : *mut core::ffi::c_void, sqlstate : *mut u8, nativeerror : *mut i32, messagetext : *mut u8, bufferlength : i16, textlength : *mut i16) -> i16);
    SQLError(environmenthandle, connectionhandle, statementhandle, core::mem::transmute(sqlstate.as_ptr()), core::mem::transmute(nativeerror.unwrap_or(std::ptr::null_mut())), core::mem::transmute(messagetext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), messagetext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(textlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLErrorA(henv: *mut core::ffi::c_void, hdbc: *mut core::ffi::c_void, hstmt: *mut core::ffi::c_void, szsqlstate: *mut u8, pfnativeerror: Option<*mut i32>, szerrormsg: Option<&mut [u8]>, pcberrormsg: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLErrorA(henv : *mut core::ffi::c_void, hdbc : *mut core::ffi::c_void, hstmt : *mut core::ffi::c_void, szsqlstate : *mut u8, pfnativeerror : *mut i32, szerrormsg : *mut u8, cberrormsgmax : i16, pcberrormsg : *mut i16) -> i16);
    SQLErrorA(henv, hdbc, hstmt, szsqlstate, core::mem::transmute(pfnativeerror.unwrap_or(std::ptr::null_mut())), core::mem::transmute(szerrormsg.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szerrormsg.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcberrormsg.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLErrorW(henv: *mut core::ffi::c_void, hdbc: *mut core::ffi::c_void, hstmt: *mut core::ffi::c_void, wszsqlstate: &mut [u16; 6], pfnativeerror: Option<*mut i32>, wszerrormsg: Option<&mut [u16]>, pccherrormsg: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLErrorW(henv : *mut core::ffi::c_void, hdbc : *mut core::ffi::c_void, hstmt : *mut core::ffi::c_void, wszsqlstate : *mut u16, pfnativeerror : *mut i32, wszerrormsg : *mut u16, ccherrormsgmax : i16, pccherrormsg : *mut i16) -> i16);
    SQLErrorW(henv, hdbc, hstmt, core::mem::transmute(wszsqlstate.as_ptr()), core::mem::transmute(pfnativeerror.unwrap_or(std::ptr::null_mut())), core::mem::transmute(wszerrormsg.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), wszerrormsg.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pccherrormsg.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLExecDirect(statementhandle: *mut core::ffi::c_void, statementtext: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLExecDirect(statementhandle : *mut core::ffi::c_void, statementtext : *const u8, textlength : i32) -> i16);
    SQLExecDirect(statementhandle, core::mem::transmute(statementtext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), statementtext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn SQLExecDirectA(hstmt: *mut core::ffi::c_void, szsqlstr: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLExecDirectA(hstmt : *mut core::ffi::c_void, szsqlstr : *const u8, cbsqlstr : i32) -> i16);
    SQLExecDirectA(hstmt, core::mem::transmute(szsqlstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szsqlstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn SQLExecDirectW(hstmt: *mut core::ffi::c_void, szsqlstr: Option<&[u16]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLExecDirectW(hstmt : *mut core::ffi::c_void, szsqlstr : *const u16, textlength : i32) -> i16);
    SQLExecDirectW(hstmt, core::mem::transmute(szsqlstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szsqlstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn SQLExecute(statementhandle: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLExecute(statementhandle : *mut core::ffi::c_void) -> i16);
    SQLExecute(statementhandle)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLExtendedFetch(hstmt: *mut core::ffi::c_void, ffetchtype: u16, irow: i64, pcrow: Option<*mut u64>, rgfrowstatus: Option<*mut u16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLExtendedFetch(hstmt : *mut core::ffi::c_void, ffetchtype : u16, irow : i64, pcrow : *mut u64, rgfrowstatus : *mut u16) -> i16);
    SQLExtendedFetch(hstmt, ffetchtype, irow, core::mem::transmute(pcrow.unwrap_or(std::ptr::null_mut())), core::mem::transmute(rgfrowstatus.unwrap_or(std::ptr::null_mut())))
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLExtendedFetch(hstmt: *mut core::ffi::c_void, ffetchtype: u16, irow: i32, pcrow: Option<*mut u32>, rgfrowstatus: Option<*mut u16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLExtendedFetch(hstmt : *mut core::ffi::c_void, ffetchtype : u16, irow : i32, pcrow : *mut u32, rgfrowstatus : *mut u16) -> i16);
    SQLExtendedFetch(hstmt, ffetchtype, irow, core::mem::transmute(pcrow.unwrap_or(std::ptr::null_mut())), core::mem::transmute(rgfrowstatus.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLFetch(statementhandle: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLFetch(statementhandle : *mut core::ffi::c_void) -> i16);
    SQLFetch(statementhandle)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLFetchScroll(statementhandle: *mut core::ffi::c_void, fetchorientation: i16, fetchoffset: i64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLFetchScroll(statementhandle : *mut core::ffi::c_void, fetchorientation : i16, fetchoffset : i64) -> i16);
    SQLFetchScroll(statementhandle, fetchorientation, fetchoffset)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLFetchScroll(statementhandle: *mut core::ffi::c_void, fetchorientation: i16, fetchoffset: i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLFetchScroll(statementhandle : *mut core::ffi::c_void, fetchorientation : i16, fetchoffset : i32) -> i16);
    SQLFetchScroll(statementhandle, fetchorientation, fetchoffset)
}
#[inline]
pub unsafe fn SQLForeignKeys(hstmt: *mut core::ffi::c_void, szpkcatalogname: Option<&[u8]>, szpkschemaname: Option<&[u8]>, szpktablename: Option<&[u8]>, szfkcatalogname: Option<&[u8]>, szfkschemaname: Option<&[u8]>, szfktablename: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLForeignKeys(hstmt : *mut core::ffi::c_void, szpkcatalogname : *const u8, cchpkcatalogname : i16, szpkschemaname : *const u8, cchpkschemaname : i16, szpktablename : *const u8, cchpktablename : i16, szfkcatalogname : *const u8, cchfkcatalogname : i16, szfkschemaname : *const u8, cchfkschemaname : i16, szfktablename : *const u8, cchfktablename : i16) -> i16);
    SQLForeignKeys(
        hstmt,
        core::mem::transmute(szpkcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szpkcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szpkschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szpkschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szpktablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szpktablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szfkcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szfkcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szfkschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szfkschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szfktablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szfktablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLForeignKeysA(hstmt: *mut core::ffi::c_void, szpkcatalogname: Option<&[u8]>, szpkschemaname: Option<&[u8]>, szpktablename: Option<&[u8]>, szfkcatalogname: Option<&[u8]>, szfkschemaname: Option<&[u8]>, szfktablename: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLForeignKeysA(hstmt : *mut core::ffi::c_void, szpkcatalogname : *const u8, cbpkcatalogname : i16, szpkschemaname : *const u8, cbpkschemaname : i16, szpktablename : *const u8, cbpktablename : i16, szfkcatalogname : *const u8, cbfkcatalogname : i16, szfkschemaname : *const u8, cbfkschemaname : i16, szfktablename : *const u8, cbfktablename : i16) -> i16);
    SQLForeignKeysA(
        hstmt,
        core::mem::transmute(szpkcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szpkcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szpkschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szpkschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szpktablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szpktablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szfkcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szfkcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szfkschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szfkschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szfktablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szfktablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLForeignKeysW(hstmt: *mut core::ffi::c_void, szpkcatalogname: Option<&[u16]>, szpkschemaname: Option<&[u16]>, szpktablename: Option<&[u16]>, szfkcatalogname: Option<&[u16]>, szfkschemaname: Option<&[u16]>, szfktablename: Option<&[u16]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLForeignKeysW(hstmt : *mut core::ffi::c_void, szpkcatalogname : *const u16, cchpkcatalogname : i16, szpkschemaname : *const u16, cchpkschemaname : i16, szpktablename : *const u16, cchpktablename : i16, szfkcatalogname : *const u16, cchfkcatalogname : i16, szfkschemaname : *const u16, cchfkschemaname : i16, szfktablename : *const u16, cchfktablename : i16) -> i16);
    SQLForeignKeysW(
        hstmt,
        core::mem::transmute(szpkcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szpkcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szpkschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szpkschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szpktablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szpktablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szfkcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szfkcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szfkschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szfkschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szfktablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szfktablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLFreeConnect(connectionhandle: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLFreeConnect(connectionhandle : *mut core::ffi::c_void) -> i16);
    SQLFreeConnect(connectionhandle)
}
#[inline]
pub unsafe fn SQLFreeEnv(environmenthandle: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLFreeEnv(environmenthandle : *mut core::ffi::c_void) -> i16);
    SQLFreeEnv(environmenthandle)
}
#[inline]
pub unsafe fn SQLFreeHandle(handletype: i16, handle: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLFreeHandle(handletype : i16, handle : *mut core::ffi::c_void) -> i16);
    SQLFreeHandle(handletype, handle)
}
#[inline]
pub unsafe fn SQLFreeStmt(statementhandle: *mut core::ffi::c_void, option: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLFreeStmt(statementhandle : *mut core::ffi::c_void, option : u16) -> i16);
    SQLFreeStmt(statementhandle, option)
}
#[inline]
pub unsafe fn SQLGetConnectAttr(connectionhandle: *mut core::ffi::c_void, attribute: i32, value: Option<*mut core::ffi::c_void>, bufferlength: i32, stringlengthptr: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetConnectAttr(connectionhandle : *mut core::ffi::c_void, attribute : i32, value : *mut core::ffi::c_void, bufferlength : i32, stringlengthptr : *mut i32) -> i16);
    SQLGetConnectAttr(connectionhandle, attribute, core::mem::transmute(value.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(stringlengthptr.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetConnectAttrA(hdbc: *mut core::ffi::c_void, fattribute: i32, rgbvalue: Option<*mut core::ffi::c_void>, cbvaluemax: i32, pcbvalue: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetConnectAttrA(hdbc : *mut core::ffi::c_void, fattribute : i32, rgbvalue : *mut core::ffi::c_void, cbvaluemax : i32, pcbvalue : *mut i32) -> i16);
    SQLGetConnectAttrA(hdbc, fattribute, core::mem::transmute(rgbvalue.unwrap_or(std::ptr::null_mut())), cbvaluemax, core::mem::transmute(pcbvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetConnectAttrW(hdbc: *mut core::ffi::c_void, fattribute: i32, rgbvalue: Option<*mut core::ffi::c_void>, cbvaluemax: i32, pcbvalue: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetConnectAttrW(hdbc : *mut core::ffi::c_void, fattribute : i32, rgbvalue : *mut core::ffi::c_void, cbvaluemax : i32, pcbvalue : *mut i32) -> i16);
    SQLGetConnectAttrW(hdbc, fattribute, core::mem::transmute(rgbvalue.unwrap_or(std::ptr::null_mut())), cbvaluemax, core::mem::transmute(pcbvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetConnectOption(connectionhandle: *mut core::ffi::c_void, option: u16, value: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetConnectOption(connectionhandle : *mut core::ffi::c_void, option : u16, value : *mut core::ffi::c_void) -> i16);
    SQLGetConnectOption(connectionhandle, option, value)
}
#[inline]
pub unsafe fn SQLGetConnectOptionA(hdbc: *mut core::ffi::c_void, foption: u16, pvparam: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetConnectOptionA(hdbc : *mut core::ffi::c_void, foption : u16, pvparam : *mut core::ffi::c_void) -> i16);
    SQLGetConnectOptionA(hdbc, foption, pvparam)
}
#[inline]
pub unsafe fn SQLGetConnectOptionW(hdbc: *mut core::ffi::c_void, foption: u16, pvparam: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetConnectOptionW(hdbc : *mut core::ffi::c_void, foption : u16, pvparam : *mut core::ffi::c_void) -> i16);
    SQLGetConnectOptionW(hdbc, foption, pvparam)
}
#[inline]
pub unsafe fn SQLGetCursorName(statementhandle: *mut core::ffi::c_void, cursorname: Option<&mut [u8]>, namelengthptr: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetCursorName(statementhandle : *mut core::ffi::c_void, cursorname : *mut u8, bufferlength : i16, namelengthptr : *mut i16) -> i16);
    SQLGetCursorName(statementhandle, core::mem::transmute(cursorname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cursorname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(namelengthptr.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetCursorNameA(hstmt: *mut core::ffi::c_void, szcursor: Option<&mut [u8]>, pcbcursor: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetCursorNameA(hstmt : *mut core::ffi::c_void, szcursor : *mut u8, cbcursormax : i16, pcbcursor : *mut i16) -> i16);
    SQLGetCursorNameA(hstmt, core::mem::transmute(szcursor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szcursor.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcbcursor.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetCursorNameW(hstmt: *mut core::ffi::c_void, szcursor: Option<&mut [u16]>, pcchcursor: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetCursorNameW(hstmt : *mut core::ffi::c_void, szcursor : *mut u16, cchcursormax : i16, pcchcursor : *mut i16) -> i16);
    SQLGetCursorNameW(hstmt, core::mem::transmute(szcursor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szcursor.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcchcursor.unwrap_or(std::ptr::null_mut())))
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLGetData(statementhandle: *mut core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: Option<*mut core::ffi::c_void>, bufferlength: i64, strlen_or_indptr: Option<*mut i64>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetData(statementhandle : *mut core::ffi::c_void, columnnumber : u16, targettype : i16, targetvalue : *mut core::ffi::c_void, bufferlength : i64, strlen_or_indptr : *mut i64) -> i16);
    SQLGetData(statementhandle, columnnumber, targettype, core::mem::transmute(targetvalue.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(strlen_or_indptr.unwrap_or(std::ptr::null_mut())))
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLGetData(statementhandle: *mut core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: Option<*mut core::ffi::c_void>, bufferlength: i32, strlen_or_indptr: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetData(statementhandle : *mut core::ffi::c_void, columnnumber : u16, targettype : i16, targetvalue : *mut core::ffi::c_void, bufferlength : i32, strlen_or_indptr : *mut i32) -> i16);
    SQLGetData(statementhandle, columnnumber, targettype, core::mem::transmute(targetvalue.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(strlen_or_indptr.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetDescField(descriptorhandle: *mut core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: Option<*mut core::ffi::c_void>, bufferlength: i32, stringlength: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDescField(descriptorhandle : *mut core::ffi::c_void, recnumber : i16, fieldidentifier : i16, value : *mut core::ffi::c_void, bufferlength : i32, stringlength : *mut i32) -> i16);
    SQLGetDescField(descriptorhandle, recnumber, fieldidentifier, core::mem::transmute(value.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(stringlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetDescFieldA(hdesc: *mut core::ffi::c_void, irecord: i16, ifield: i16, rgbvalue: Option<*mut core::ffi::c_void>, cbbufferlength: i32, stringlength: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDescFieldA(hdesc : *mut core::ffi::c_void, irecord : i16, ifield : i16, rgbvalue : *mut core::ffi::c_void, cbbufferlength : i32, stringlength : *mut i32) -> i16);
    SQLGetDescFieldA(hdesc, irecord, ifield, core::mem::transmute(rgbvalue.unwrap_or(std::ptr::null_mut())), cbbufferlength, core::mem::transmute(stringlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetDescFieldW(hdesc: *mut core::ffi::c_void, irecord: i16, ifield: i16, rgbvalue: Option<*mut core::ffi::c_void>, cbbufferlength: i32, stringlength: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDescFieldW(hdesc : *mut core::ffi::c_void, irecord : i16, ifield : i16, rgbvalue : *mut core::ffi::c_void, cbbufferlength : i32, stringlength : *mut i32) -> i16);
    SQLGetDescFieldW(hdesc, irecord, ifield, core::mem::transmute(rgbvalue.unwrap_or(std::ptr::null_mut())), cbbufferlength, core::mem::transmute(stringlength.unwrap_or(std::ptr::null_mut())))
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLGetDescRec(descriptorhandle: *mut core::ffi::c_void, recnumber: i16, name: Option<&mut [u8]>, stringlengthptr: Option<*mut i16>, typeptr: Option<*mut i16>, subtypeptr: Option<*mut i16>, lengthptr: Option<*mut i64>, precisionptr: Option<*mut i16>, scaleptr: Option<*mut i16>, nullableptr: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDescRec(descriptorhandle : *mut core::ffi::c_void, recnumber : i16, name : *mut u8, bufferlength : i16, stringlengthptr : *mut i16, typeptr : *mut i16, subtypeptr : *mut i16, lengthptr : *mut i64, precisionptr : *mut i16, scaleptr : *mut i16, nullableptr : *mut i16) -> i16);
    SQLGetDescRec(
        descriptorhandle,
        recnumber,
        core::mem::transmute(name.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        name.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(stringlengthptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(typeptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(subtypeptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lengthptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(precisionptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(scaleptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(nullableptr.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLGetDescRec(descriptorhandle: *mut core::ffi::c_void, recnumber: i16, name: Option<&mut [u8]>, stringlengthptr: Option<*mut i16>, typeptr: Option<*mut i16>, subtypeptr: Option<*mut i16>, lengthptr: Option<*mut i32>, precisionptr: Option<*mut i16>, scaleptr: Option<*mut i16>, nullableptr: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDescRec(descriptorhandle : *mut core::ffi::c_void, recnumber : i16, name : *mut u8, bufferlength : i16, stringlengthptr : *mut i16, typeptr : *mut i16, subtypeptr : *mut i16, lengthptr : *mut i32, precisionptr : *mut i16, scaleptr : *mut i16, nullableptr : *mut i16) -> i16);
    SQLGetDescRec(
        descriptorhandle,
        recnumber,
        core::mem::transmute(name.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        name.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(stringlengthptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(typeptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(subtypeptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lengthptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(precisionptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(scaleptr.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(nullableptr.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLGetDescRecA(hdesc: *mut core::ffi::c_void, irecord: i16, szname: Option<&mut [u8]>, pcbname: Option<*mut i16>, pftype: Option<*mut i16>, pfsubtype: Option<*mut i16>, plength: Option<*mut i64>, pprecision: Option<*mut i16>, pscale: Option<*mut i16>, pnullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDescRecA(hdesc : *mut core::ffi::c_void, irecord : i16, szname : *mut u8, cbnamemax : i16, pcbname : *mut i16, pftype : *mut i16, pfsubtype : *mut i16, plength : *mut i64, pprecision : *mut i16, pscale : *mut i16, pnullable : *mut i16) -> i16);
    SQLGetDescRecA(
        hdesc,
        irecord,
        core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcbname.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pftype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfsubtype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(plength.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pprecision.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pscale.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pnullable.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLGetDescRecA(hdesc: *mut core::ffi::c_void, irecord: i16, szname: Option<&mut [u8]>, pcbname: Option<*mut i16>, pftype: Option<*mut i16>, pfsubtype: Option<*mut i16>, plength: Option<*mut i32>, pprecision: Option<*mut i16>, pscale: Option<*mut i16>, pnullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDescRecA(hdesc : *mut core::ffi::c_void, irecord : i16, szname : *mut u8, cbnamemax : i16, pcbname : *mut i16, pftype : *mut i16, pfsubtype : *mut i16, plength : *mut i32, pprecision : *mut i16, pscale : *mut i16, pnullable : *mut i16) -> i16);
    SQLGetDescRecA(
        hdesc,
        irecord,
        core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcbname.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pftype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfsubtype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(plength.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pprecision.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pscale.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pnullable.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLGetDescRecW(hdesc: *mut core::ffi::c_void, irecord: i16, szname: Option<&mut [u16]>, pcchname: Option<*mut i16>, pftype: Option<*mut i16>, pfsubtype: Option<*mut i16>, plength: Option<*mut i64>, pprecision: Option<*mut i16>, pscale: Option<*mut i16>, pnullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDescRecW(hdesc : *mut core::ffi::c_void, irecord : i16, szname : *mut u16, cchnamemax : i16, pcchname : *mut i16, pftype : *mut i16, pfsubtype : *mut i16, plength : *mut i64, pprecision : *mut i16, pscale : *mut i16, pnullable : *mut i16) -> i16);
    SQLGetDescRecW(
        hdesc,
        irecord,
        core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcchname.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pftype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfsubtype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(plength.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pprecision.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pscale.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pnullable.unwrap_or(std::ptr::null_mut())),
    )
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLGetDescRecW(hdesc: *mut core::ffi::c_void, irecord: i16, szname: Option<&mut [u16]>, pcchname: Option<*mut i16>, pftype: Option<*mut i16>, pfsubtype: Option<*mut i16>, plength: Option<*mut i32>, pprecision: Option<*mut i16>, pscale: Option<*mut i16>, pnullable: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDescRecW(hdesc : *mut core::ffi::c_void, irecord : i16, szname : *mut u16, cchnamemax : i16, pcchname : *mut i16, pftype : *mut i16, pfsubtype : *mut i16, plength : *mut i32, pprecision : *mut i16, pscale : *mut i16, pnullable : *mut i16) -> i16);
    SQLGetDescRecW(
        hdesc,
        irecord,
        core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pcchname.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pftype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfsubtype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(plength.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pprecision.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pscale.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pnullable.unwrap_or(std::ptr::null_mut())),
    )
}
#[inline]
pub unsafe fn SQLGetDiagField(handletype: i16, handle: *mut core::ffi::c_void, recnumber: i16, diagidentifier: i16, diaginfo: Option<*mut core::ffi::c_void>, bufferlength: i16, stringlength: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDiagField(handletype : i16, handle : *mut core::ffi::c_void, recnumber : i16, diagidentifier : i16, diaginfo : *mut core::ffi::c_void, bufferlength : i16, stringlength : *mut i16) -> i16);
    SQLGetDiagField(handletype, handle, recnumber, diagidentifier, core::mem::transmute(diaginfo.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(stringlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetDiagFieldA(fhandletype: i16, handle: *mut core::ffi::c_void, irecord: i16, fdiagfield: i16, rgbdiaginfo: Option<*mut core::ffi::c_void>, cbdiaginfomax: i16, pcbdiaginfo: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDiagFieldA(fhandletype : i16, handle : *mut core::ffi::c_void, irecord : i16, fdiagfield : i16, rgbdiaginfo : *mut core::ffi::c_void, cbdiaginfomax : i16, pcbdiaginfo : *mut i16) -> i16);
    SQLGetDiagFieldA(fhandletype, handle, irecord, fdiagfield, core::mem::transmute(rgbdiaginfo.unwrap_or(std::ptr::null_mut())), cbdiaginfomax, core::mem::transmute(pcbdiaginfo.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetDiagFieldW(fhandletype: i16, handle: *mut core::ffi::c_void, irecord: i16, fdiagfield: i16, rgbdiaginfo: Option<*mut core::ffi::c_void>, cbbufferlength: i16, pcbstringlength: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDiagFieldW(fhandletype : i16, handle : *mut core::ffi::c_void, irecord : i16, fdiagfield : i16, rgbdiaginfo : *mut core::ffi::c_void, cbbufferlength : i16, pcbstringlength : *mut i16) -> i16);
    SQLGetDiagFieldW(fhandletype, handle, irecord, fdiagfield, core::mem::transmute(rgbdiaginfo.unwrap_or(std::ptr::null_mut())), cbbufferlength, core::mem::transmute(pcbstringlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetDiagRec(handletype: i16, handle: *mut core::ffi::c_void, recnumber: i16, sqlstate: Option<&mut [u8; 6]>, nativeerror: *mut i32, messagetext: Option<&mut [u8]>, textlength: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDiagRec(handletype : i16, handle : *mut core::ffi::c_void, recnumber : i16, sqlstate : *mut u8, nativeerror : *mut i32, messagetext : *mut u8, bufferlength : i16, textlength : *mut i16) -> i16);
    SQLGetDiagRec(handletype, handle, recnumber, core::mem::transmute(sqlstate.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), nativeerror, core::mem::transmute(messagetext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), messagetext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(textlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetDiagRecA(fhandletype: i16, handle: *mut core::ffi::c_void, irecord: i16, szsqlstate: Option<&mut [u8; 6]>, pfnativeerror: *mut i32, szerrormsg: Option<&mut [u8]>, pcberrormsg: *mut i16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDiagRecA(fhandletype : i16, handle : *mut core::ffi::c_void, irecord : i16, szsqlstate : *mut u8, pfnativeerror : *mut i32, szerrormsg : *mut u8, cberrormsgmax : i16, pcberrormsg : *mut i16) -> i16);
    SQLGetDiagRecA(fhandletype, handle, irecord, core::mem::transmute(szsqlstate.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pfnativeerror, core::mem::transmute(szerrormsg.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szerrormsg.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcberrormsg)
}
#[inline]
pub unsafe fn SQLGetDiagRecW(fhandletype: i16, handle: *mut core::ffi::c_void, irecord: i16, szsqlstate: Option<&mut [u16; 6]>, pfnativeerror: *mut i32, szerrormsg: Option<&mut [u16]>, pccherrormsg: *mut i16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetDiagRecW(fhandletype : i16, handle : *mut core::ffi::c_void, irecord : i16, szsqlstate : *mut u16, pfnativeerror : *mut i32, szerrormsg : *mut u16, ccherrormsgmax : i16, pccherrormsg : *mut i16) -> i16);
    SQLGetDiagRecW(fhandletype, handle, irecord, core::mem::transmute(szsqlstate.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pfnativeerror, core::mem::transmute(szerrormsg.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szerrormsg.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pccherrormsg)
}
#[inline]
pub unsafe fn SQLGetEnvAttr(environmenthandle: *mut core::ffi::c_void, attribute: i32, value: *mut core::ffi::c_void, bufferlength: i32, stringlength: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetEnvAttr(environmenthandle : *mut core::ffi::c_void, attribute : i32, value : *mut core::ffi::c_void, bufferlength : i32, stringlength : *mut i32) -> i16);
    SQLGetEnvAttr(environmenthandle, attribute, value, bufferlength, core::mem::transmute(stringlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetFunctions(connectionhandle: *mut core::ffi::c_void, functionid: u16, supported: Option<*mut u16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetFunctions(connectionhandle : *mut core::ffi::c_void, functionid : u16, supported : *mut u16) -> i16);
    SQLGetFunctions(connectionhandle, functionid, core::mem::transmute(supported.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetInfo(connectionhandle: *mut core::ffi::c_void, infotype: u16, infovalue: Option<*mut core::ffi::c_void>, bufferlength: i16, stringlengthptr: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetInfo(connectionhandle : *mut core::ffi::c_void, infotype : u16, infovalue : *mut core::ffi::c_void, bufferlength : i16, stringlengthptr : *mut i16) -> i16);
    SQLGetInfo(connectionhandle, infotype, core::mem::transmute(infovalue.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(stringlengthptr.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetInfoA(hdbc: *mut core::ffi::c_void, finfotype: u16, rgbinfovalue: Option<*mut core::ffi::c_void>, cbinfovaluemax: i16, pcbinfovalue: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetInfoA(hdbc : *mut core::ffi::c_void, finfotype : u16, rgbinfovalue : *mut core::ffi::c_void, cbinfovaluemax : i16, pcbinfovalue : *mut i16) -> i16);
    SQLGetInfoA(hdbc, finfotype, core::mem::transmute(rgbinfovalue.unwrap_or(std::ptr::null_mut())), cbinfovaluemax, core::mem::transmute(pcbinfovalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetInfoW(hdbc: *mut core::ffi::c_void, finfotype: u16, rgbinfovalue: Option<*mut core::ffi::c_void>, cbinfovaluemax: i16, pcbinfovalue: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetInfoW(hdbc : *mut core::ffi::c_void, finfotype : u16, rgbinfovalue : *mut core::ffi::c_void, cbinfovaluemax : i16, pcbinfovalue : *mut i16) -> i16);
    SQLGetInfoW(hdbc, finfotype, core::mem::transmute(rgbinfovalue.unwrap_or(std::ptr::null_mut())), cbinfovaluemax, core::mem::transmute(pcbinfovalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetNextEnumeration<P0>(henumhandle: P0, prgenumdata: *mut u8, pienumlength: *mut i32) -> i16
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn SQLGetNextEnumeration(henumhandle : super::super::Foundation:: HANDLE, prgenumdata : *mut u8, pienumlength : *mut i32) -> i16);
    SQLGetNextEnumeration(henumhandle.param().abi(), prgenumdata, pienumlength)
}
#[inline]
pub unsafe fn SQLGetStmtAttr(statementhandle: *mut core::ffi::c_void, attribute: i32, value: Option<*mut core::ffi::c_void>, bufferlength: i32, stringlength: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetStmtAttr(statementhandle : *mut core::ffi::c_void, attribute : i32, value : *mut core::ffi::c_void, bufferlength : i32, stringlength : *mut i32) -> i16);
    SQLGetStmtAttr(statementhandle, attribute, core::mem::transmute(value.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(stringlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLGetStmtAttrA(hstmt: *mut core::ffi::c_void, fattribute: i32, rgbvalue: *mut core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetStmtAttrA(hstmt : *mut core::ffi::c_void, fattribute : i32, rgbvalue : *mut core::ffi::c_void, cbvaluemax : i32, pcbvalue : *mut i32) -> i16);
    SQLGetStmtAttrA(hstmt, fattribute, rgbvalue, cbvaluemax, pcbvalue)
}
#[inline]
pub unsafe fn SQLGetStmtAttrW(hstmt: *mut core::ffi::c_void, fattribute: i32, rgbvalue: *mut core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetStmtAttrW(hstmt : *mut core::ffi::c_void, fattribute : i32, rgbvalue : *mut core::ffi::c_void, cbvaluemax : i32, pcbvalue : *mut i32) -> i16);
    SQLGetStmtAttrW(hstmt, fattribute, rgbvalue, cbvaluemax, pcbvalue)
}
#[inline]
pub unsafe fn SQLGetStmtOption(statementhandle: *mut core::ffi::c_void, option: u16, value: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetStmtOption(statementhandle : *mut core::ffi::c_void, option : u16, value : *mut core::ffi::c_void) -> i16);
    SQLGetStmtOption(statementhandle, option, value)
}
#[inline]
pub unsafe fn SQLGetTypeInfo(statementhandle: *mut core::ffi::c_void, datatype: i16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetTypeInfo(statementhandle : *mut core::ffi::c_void, datatype : i16) -> i16);
    SQLGetTypeInfo(statementhandle, datatype)
}
#[inline]
pub unsafe fn SQLGetTypeInfoA(statementhandle: *mut core::ffi::c_void, datatype: i16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetTypeInfoA(statementhandle : *mut core::ffi::c_void, datatype : i16) -> i16);
    SQLGetTypeInfoA(statementhandle, datatype)
}
#[inline]
pub unsafe fn SQLGetTypeInfoW(statementhandle: *mut core::ffi::c_void, datatype: i16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLGetTypeInfoW(statementhandle : *mut core::ffi::c_void, datatype : i16) -> i16);
    SQLGetTypeInfoW(statementhandle, datatype)
}
#[inline]
pub unsafe fn SQLInitEnumServers<P0, P1>(pwchservername: P0, pwchinstancename: P1) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn SQLInitEnumServers(pwchservername : windows_core::PCWSTR, pwchinstancename : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    SQLInitEnumServers(pwchservername.param().abi(), pwchinstancename.param().abi())
}
#[inline]
pub unsafe fn SQLLinkedCatalogsA<P0>(param0: *mut core::ffi::c_void, param1: P0, param2: i16) -> i16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn SQLLinkedCatalogsA(param0 : *mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : i16) -> i16);
    SQLLinkedCatalogsA(param0, param1.param().abi(), param2)
}
#[inline]
pub unsafe fn SQLLinkedCatalogsW<P0>(param0: *mut core::ffi::c_void, param1: P0, param2: i16) -> i16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn SQLLinkedCatalogsW(param0 : *mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : i16) -> i16);
    SQLLinkedCatalogsW(param0, param1.param().abi(), param2)
}
#[inline]
pub unsafe fn SQLLinkedServers(param0: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn SQLLinkedServers(param0 : *mut core::ffi::c_void) -> i16);
    SQLLinkedServers(param0)
}
#[inline]
pub unsafe fn SQLMoreResults(hstmt: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLMoreResults(hstmt : *mut core::ffi::c_void) -> i16);
    SQLMoreResults(hstmt)
}
#[inline]
pub unsafe fn SQLNativeSql(hdbc: *mut core::ffi::c_void, szsqlstrin: &[u8], szsqlstr: Option<&mut [u8]>, pcbsqlstr: *mut i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLNativeSql(hdbc : *mut core::ffi::c_void, szsqlstrin : *const u8, cchsqlstrin : i32, szsqlstr : *mut u8, cchsqlstrmax : i32, pcbsqlstr : *mut i32) -> i16);
    SQLNativeSql(hdbc, core::mem::transmute(szsqlstrin.as_ptr()), szsqlstrin.len().try_into().unwrap(), core::mem::transmute(szsqlstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szsqlstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbsqlstr)
}
#[inline]
pub unsafe fn SQLNativeSqlA(hdbc: *mut core::ffi::c_void, szsqlstrin: &[u8], szsqlstr: Option<&mut [u8]>, pcbsqlstr: *mut i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLNativeSqlA(hdbc : *mut core::ffi::c_void, szsqlstrin : *const u8, cbsqlstrin : i32, szsqlstr : *mut u8, cbsqlstrmax : i32, pcbsqlstr : *mut i32) -> i16);
    SQLNativeSqlA(hdbc, core::mem::transmute(szsqlstrin.as_ptr()), szsqlstrin.len().try_into().unwrap(), core::mem::transmute(szsqlstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szsqlstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbsqlstr)
}
#[inline]
pub unsafe fn SQLNativeSqlW(hdbc: *mut core::ffi::c_void, szsqlstrin: &[u16], szsqlstr: Option<&mut [u16]>, pcchsqlstr: *mut i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLNativeSqlW(hdbc : *mut core::ffi::c_void, szsqlstrin : *const u16, cchsqlstrin : i32, szsqlstr : *mut u16, cchsqlstrmax : i32, pcchsqlstr : *mut i32) -> i16);
    SQLNativeSqlW(hdbc, core::mem::transmute(szsqlstrin.as_ptr()), szsqlstrin.len().try_into().unwrap(), core::mem::transmute(szsqlstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szsqlstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcchsqlstr)
}
#[inline]
pub unsafe fn SQLNumParams(hstmt: *mut core::ffi::c_void, pcpar: Option<*mut i16>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLNumParams(hstmt : *mut core::ffi::c_void, pcpar : *mut i16) -> i16);
    SQLNumParams(hstmt, core::mem::transmute(pcpar.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLNumResultCols(statementhandle: *mut core::ffi::c_void, columncount: *mut i16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLNumResultCols(statementhandle : *mut core::ffi::c_void, columncount : *mut i16) -> i16);
    SQLNumResultCols(statementhandle, columncount)
}
#[inline]
pub unsafe fn SQLParamData(statementhandle: *mut core::ffi::c_void, value: Option<*mut *mut core::ffi::c_void>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLParamData(statementhandle : *mut core::ffi::c_void, value : *mut *mut core::ffi::c_void) -> i16);
    SQLParamData(statementhandle, core::mem::transmute(value.unwrap_or(std::ptr::null_mut())))
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLParamOptions(hstmt: *mut core::ffi::c_void, crow: u64, pirow: *mut u64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLParamOptions(hstmt : *mut core::ffi::c_void, crow : u64, pirow : *mut u64) -> i16);
    SQLParamOptions(hstmt, crow, pirow)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLParamOptions(hstmt: *mut core::ffi::c_void, crow: u32, pirow: *mut u32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLParamOptions(hstmt : *mut core::ffi::c_void, crow : u32, pirow : *mut u32) -> i16);
    SQLParamOptions(hstmt, crow, pirow)
}
#[inline]
pub unsafe fn SQLPrepare(statementhandle: *mut core::ffi::c_void, statementtext: &[u8]) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLPrepare(statementhandle : *mut core::ffi::c_void, statementtext : *const u8, textlength : i32) -> i16);
    SQLPrepare(statementhandle, core::mem::transmute(statementtext.as_ptr()), statementtext.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SQLPrepareA(hstmt: *mut core::ffi::c_void, szsqlstr: &[u8]) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLPrepareA(hstmt : *mut core::ffi::c_void, szsqlstr : *const u8, cbsqlstr : i32) -> i16);
    SQLPrepareA(hstmt, core::mem::transmute(szsqlstr.as_ptr()), szsqlstr.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SQLPrepareW(hstmt: *mut core::ffi::c_void, szsqlstr: &[u16]) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLPrepareW(hstmt : *mut core::ffi::c_void, szsqlstr : *const u16, cchsqlstr : i32) -> i16);
    SQLPrepareW(hstmt, core::mem::transmute(szsqlstr.as_ptr()), szsqlstr.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SQLPrimaryKeys(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, sztablename: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLPrimaryKeys(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cchcatalogname : i16, szschemaname : *const u8, cchschemaname : i16, sztablename : *const u8, cchtablename : i16) -> i16);
    SQLPrimaryKeys(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLPrimaryKeysA(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, sztablename: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLPrimaryKeysA(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cbcatalogname : i16, szschemaname : *const u8, cbschemaname : i16, sztablename : *const u8, cbtablename : i16) -> i16);
    SQLPrimaryKeysA(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLPrimaryKeysW(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u16]>, szschemaname: Option<&[u16]>, sztablename: Option<&[u16]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLPrimaryKeysW(hstmt : *mut core::ffi::c_void, szcatalogname : *const u16, cchcatalogname : i16, szschemaname : *const u16, cchschemaname : i16, sztablename : *const u16, cchtablename : i16) -> i16);
    SQLPrimaryKeysW(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLProcedureColumns(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, szprocname: Option<&[u8]>, szcolumnname: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLProcedureColumns(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cchcatalogname : i16, szschemaname : *const u8, cchschemaname : i16, szprocname : *const u8, cchprocname : i16, szcolumnname : *const u8, cchcolumnname : i16) -> i16);
    SQLProcedureColumns(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szprocname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szprocname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szcolumnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolumnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLProcedureColumnsA(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, szprocname: Option<&[u8]>, szcolumnname: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLProcedureColumnsA(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cbcatalogname : i16, szschemaname : *const u8, cbschemaname : i16, szprocname : *const u8, cbprocname : i16, szcolumnname : *const u8, cbcolumnname : i16) -> i16);
    SQLProcedureColumnsA(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szprocname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szprocname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szcolumnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolumnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLProcedureColumnsW(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u16]>, szschemaname: Option<&[u16]>, szprocname: Option<&[u16]>, szcolumnname: Option<&[u16]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLProcedureColumnsW(hstmt : *mut core::ffi::c_void, szcatalogname : *const u16, cchcatalogname : i16, szschemaname : *const u16, cchschemaname : i16, szprocname : *const u16, cchprocname : i16, szcolumnname : *const u16, cchcolumnname : i16) -> i16);
    SQLProcedureColumnsW(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szprocname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szprocname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szcolumnname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcolumnname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLProcedures(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, szprocname: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLProcedures(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cchcatalogname : i16, szschemaname : *const u8, cchschemaname : i16, szprocname : *const u8, cchprocname : i16) -> i16);
    SQLProcedures(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szprocname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szprocname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLProceduresA(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, szprocname: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLProceduresA(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cbcatalogname : i16, szschemaname : *const u8, cbschemaname : i16, szprocname : *const u8, cbprocname : i16) -> i16);
    SQLProceduresA(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szprocname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szprocname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLProceduresW(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u16]>, szschemaname: Option<&[u16]>, szprocname: Option<&[u16]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLProceduresW(hstmt : *mut core::ffi::c_void, szcatalogname : *const u16, cchcatalogname : i16, szschemaname : *const u16, cchschemaname : i16, szprocname : *const u16, cchprocname : i16) -> i16);
    SQLProceduresW(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szprocname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szprocname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLPutData(statementhandle: *mut core::ffi::c_void, data: *const core::ffi::c_void, strlen_or_ind: i64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLPutData(statementhandle : *mut core::ffi::c_void, data : *const core::ffi::c_void, strlen_or_ind : i64) -> i16);
    SQLPutData(statementhandle, data, strlen_or_ind)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLPutData(statementhandle: *mut core::ffi::c_void, data: *const core::ffi::c_void, strlen_or_ind: i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLPutData(statementhandle : *mut core::ffi::c_void, data : *const core::ffi::c_void, strlen_or_ind : i32) -> i16);
    SQLPutData(statementhandle, data, strlen_or_ind)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLRowCount(statementhandle: *const core::ffi::c_void, rowcount: *mut i64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLRowCount(statementhandle : *const core::ffi::c_void, rowcount : *mut i64) -> i16);
    SQLRowCount(statementhandle, rowcount)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLRowCount(statementhandle: *const core::ffi::c_void, rowcount: *mut i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLRowCount(statementhandle : *const core::ffi::c_void, rowcount : *mut i32) -> i16);
    SQLRowCount(statementhandle, rowcount)
}
#[inline]
pub unsafe fn SQLSetConnectAttr(connectionhandle: *mut core::ffi::c_void, attribute: i32, value: Option<*const core::ffi::c_void>, stringlength: i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetConnectAttr(connectionhandle : *mut core::ffi::c_void, attribute : i32, value : *const core::ffi::c_void, stringlength : i32) -> i16);
    SQLSetConnectAttr(connectionhandle, attribute, core::mem::transmute(value.unwrap_or(std::ptr::null())), stringlength)
}
#[inline]
pub unsafe fn SQLSetConnectAttrA(hdbc: *mut core::ffi::c_void, fattribute: i32, rgbvalue: Option<*const core::ffi::c_void>, cbvalue: i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetConnectAttrA(hdbc : *mut core::ffi::c_void, fattribute : i32, rgbvalue : *const core::ffi::c_void, cbvalue : i32) -> i16);
    SQLSetConnectAttrA(hdbc, fattribute, core::mem::transmute(rgbvalue.unwrap_or(std::ptr::null())), cbvalue)
}
#[inline]
pub unsafe fn SQLSetConnectAttrW(hdbc: *mut core::ffi::c_void, fattribute: i32, rgbvalue: Option<*const core::ffi::c_void>, cbvalue: i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetConnectAttrW(hdbc : *mut core::ffi::c_void, fattribute : i32, rgbvalue : *const core::ffi::c_void, cbvalue : i32) -> i16);
    SQLSetConnectAttrW(hdbc, fattribute, core::mem::transmute(rgbvalue.unwrap_or(std::ptr::null())), cbvalue)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLSetConnectOption(connectionhandle: *mut core::ffi::c_void, option: u16, value: u64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetConnectOption(connectionhandle : *mut core::ffi::c_void, option : u16, value : u64) -> i16);
    SQLSetConnectOption(connectionhandle, option, value)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLSetConnectOption(connectionhandle: *mut core::ffi::c_void, option: u16, value: u32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetConnectOption(connectionhandle : *mut core::ffi::c_void, option : u16, value : u32) -> i16);
    SQLSetConnectOption(connectionhandle, option, value)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLSetConnectOptionA(hdbc: *mut core::ffi::c_void, foption: u16, vparam: u64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetConnectOptionA(hdbc : *mut core::ffi::c_void, foption : u16, vparam : u64) -> i16);
    SQLSetConnectOptionA(hdbc, foption, vparam)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLSetConnectOptionA(hdbc: *mut core::ffi::c_void, foption: u16, vparam: u32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetConnectOptionA(hdbc : *mut core::ffi::c_void, foption : u16, vparam : u32) -> i16);
    SQLSetConnectOptionA(hdbc, foption, vparam)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLSetConnectOptionW(hdbc: *mut core::ffi::c_void, foption: u16, vparam: u64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetConnectOptionW(hdbc : *mut core::ffi::c_void, foption : u16, vparam : u64) -> i16);
    SQLSetConnectOptionW(hdbc, foption, vparam)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLSetConnectOptionW(hdbc: *mut core::ffi::c_void, foption: u16, vparam: u32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetConnectOptionW(hdbc : *mut core::ffi::c_void, foption : u16, vparam : u32) -> i16);
    SQLSetConnectOptionW(hdbc, foption, vparam)
}
#[inline]
pub unsafe fn SQLSetCursorName(statementhandle: *mut core::ffi::c_void, cursorname: &[u8]) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetCursorName(statementhandle : *mut core::ffi::c_void, cursorname : *const u8, namelength : i16) -> i16);
    SQLSetCursorName(statementhandle, core::mem::transmute(cursorname.as_ptr()), cursorname.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SQLSetCursorNameA(hstmt: *mut core::ffi::c_void, szcursor: &[u8]) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetCursorNameA(hstmt : *mut core::ffi::c_void, szcursor : *const u8, cbcursor : i16) -> i16);
    SQLSetCursorNameA(hstmt, core::mem::transmute(szcursor.as_ptr()), szcursor.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SQLSetCursorNameW(hstmt: *mut core::ffi::c_void, szcursor: &[u16]) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetCursorNameW(hstmt : *mut core::ffi::c_void, szcursor : *const u16, cchcursor : i16) -> i16);
    SQLSetCursorNameW(hstmt, core::mem::transmute(szcursor.as_ptr()), szcursor.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SQLSetDescField(descriptorhandle: *mut core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *const core::ffi::c_void, bufferlength: i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetDescField(descriptorhandle : *mut core::ffi::c_void, recnumber : i16, fieldidentifier : i16, value : *const core::ffi::c_void, bufferlength : i32) -> i16);
    SQLSetDescField(descriptorhandle, recnumber, fieldidentifier, value, bufferlength)
}
#[inline]
pub unsafe fn SQLSetDescFieldW(descriptorhandle: *mut core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *mut core::ffi::c_void, bufferlength: i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetDescFieldW(descriptorhandle : *mut core::ffi::c_void, recnumber : i16, fieldidentifier : i16, value : *mut core::ffi::c_void, bufferlength : i32) -> i16);
    SQLSetDescFieldW(descriptorhandle, recnumber, fieldidentifier, value, bufferlength)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLSetDescRec(descriptorhandle: *mut core::ffi::c_void, recnumber: i16, r#type: i16, subtype: i16, length: i64, precision: i16, scale: i16, data: Option<*mut core::ffi::c_void>, stringlength: Option<*mut i64>, indicator: Option<*mut i64>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetDescRec(descriptorhandle : *mut core::ffi::c_void, recnumber : i16, r#type : i16, subtype : i16, length : i64, precision : i16, scale : i16, data : *mut core::ffi::c_void, stringlength : *mut i64, indicator : *mut i64) -> i16);
    SQLSetDescRec(descriptorhandle, recnumber, r#type, subtype, length, precision, scale, core::mem::transmute(data.unwrap_or(std::ptr::null_mut())), core::mem::transmute(stringlength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(indicator.unwrap_or(std::ptr::null_mut())))
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLSetDescRec(descriptorhandle: *mut core::ffi::c_void, recnumber: i16, r#type: i16, subtype: i16, length: i32, precision: i16, scale: i16, data: Option<*mut core::ffi::c_void>, stringlength: Option<*mut i32>, indicator: Option<*mut i32>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetDescRec(descriptorhandle : *mut core::ffi::c_void, recnumber : i16, r#type : i16, subtype : i16, length : i32, precision : i16, scale : i16, data : *mut core::ffi::c_void, stringlength : *mut i32, indicator : *mut i32) -> i16);
    SQLSetDescRec(descriptorhandle, recnumber, r#type, subtype, length, precision, scale, core::mem::transmute(data.unwrap_or(std::ptr::null_mut())), core::mem::transmute(stringlength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(indicator.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SQLSetEnvAttr(environmenthandle: *mut core::ffi::c_void, attribute: i32, value: Option<*const core::ffi::c_void>, stringlength: i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetEnvAttr(environmenthandle : *mut core::ffi::c_void, attribute : i32, value : *const core::ffi::c_void, stringlength : i32) -> i16);
    SQLSetEnvAttr(environmenthandle, attribute, core::mem::transmute(value.unwrap_or(std::ptr::null())), stringlength)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLSetParam(statementhandle: *mut core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u64, parameterscale: i16, parametervalue: *const core::ffi::c_void, strlen_or_ind: *mut i64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetParam(statementhandle : *mut core::ffi::c_void, parameternumber : u16, valuetype : i16, parametertype : i16, lengthprecision : u64, parameterscale : i16, parametervalue : *const core::ffi::c_void, strlen_or_ind : *mut i64) -> i16);
    SQLSetParam(statementhandle, parameternumber, valuetype, parametertype, lengthprecision, parameterscale, parametervalue, strlen_or_ind)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLSetParam(statementhandle: *mut core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u32, parameterscale: i16, parametervalue: *const core::ffi::c_void, strlen_or_ind: *mut i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetParam(statementhandle : *mut core::ffi::c_void, parameternumber : u16, valuetype : i16, parametertype : i16, lengthprecision : u32, parameterscale : i16, parametervalue : *const core::ffi::c_void, strlen_or_ind : *mut i32) -> i16);
    SQLSetParam(statementhandle, parameternumber, valuetype, parametertype, lengthprecision, parameterscale, parametervalue, strlen_or_ind)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLSetPos(hstmt: *mut core::ffi::c_void, irow: u64, foption: u16, flock: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetPos(hstmt : *mut core::ffi::c_void, irow : u64, foption : u16, flock : u16) -> i16);
    SQLSetPos(hstmt, irow, foption, flock)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLSetPos(hstmt: *mut core::ffi::c_void, irow: u16, foption: u16, flock: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetPos(hstmt : *mut core::ffi::c_void, irow : u16, foption : u16, flock : u16) -> i16);
    SQLSetPos(hstmt, irow, foption, flock)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLSetScrollOptions(hstmt: *mut core::ffi::c_void, fconcurrency: u16, crowkeyset: i64, crowrowset: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetScrollOptions(hstmt : *mut core::ffi::c_void, fconcurrency : u16, crowkeyset : i64, crowrowset : u16) -> i16);
    SQLSetScrollOptions(hstmt, fconcurrency, crowkeyset, crowrowset)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLSetScrollOptions(hstmt: *mut core::ffi::c_void, fconcurrency: u16, crowkeyset: i32, crowrowset: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetScrollOptions(hstmt : *mut core::ffi::c_void, fconcurrency : u16, crowkeyset : i32, crowrowset : u16) -> i16);
    SQLSetScrollOptions(hstmt, fconcurrency, crowkeyset, crowrowset)
}
#[inline]
pub unsafe fn SQLSetStmtAttr(statementhandle: *mut core::ffi::c_void, attribute: i32, value: *const core::ffi::c_void, stringlength: i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetStmtAttr(statementhandle : *mut core::ffi::c_void, attribute : i32, value : *const core::ffi::c_void, stringlength : i32) -> i16);
    SQLSetStmtAttr(statementhandle, attribute, value, stringlength)
}
#[inline]
pub unsafe fn SQLSetStmtAttrW(hstmt: *mut core::ffi::c_void, fattribute: i32, rgbvalue: *mut core::ffi::c_void, cbvaluemax: i32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetStmtAttrW(hstmt : *mut core::ffi::c_void, fattribute : i32, rgbvalue : *mut core::ffi::c_void, cbvaluemax : i32) -> i16);
    SQLSetStmtAttrW(hstmt, fattribute, rgbvalue, cbvaluemax)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SQLSetStmtOption(statementhandle: *mut core::ffi::c_void, option: u16, value: u64) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetStmtOption(statementhandle : *mut core::ffi::c_void, option : u16, value : u64) -> i16);
    SQLSetStmtOption(statementhandle, option, value)
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SQLSetStmtOption(statementhandle: *mut core::ffi::c_void, option: u16, value: u32) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSetStmtOption(statementhandle : *mut core::ffi::c_void, option : u16, value : u32) -> i16);
    SQLSetStmtOption(statementhandle, option, value)
}
#[inline]
pub unsafe fn SQLSpecialColumns(statementhandle: *mut core::ffi::c_void, identifiertype: u16, catalogname: Option<&[u8]>, schemaname: Option<&[u8]>, tablename: Option<&[u8]>, scope: u16, nullable: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSpecialColumns(statementhandle : *mut core::ffi::c_void, identifiertype : u16, catalogname : *const u8, namelength1 : i16, schemaname : *const u8, namelength2 : i16, tablename : *const u8, namelength3 : i16, scope : u16, nullable : u16) -> i16);
    SQLSpecialColumns(
        statementhandle,
        identifiertype,
        core::mem::transmute(catalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        catalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(schemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        schemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(tablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        tablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        scope,
        nullable,
    )
}
#[inline]
pub unsafe fn SQLSpecialColumnsA(hstmt: *mut core::ffi::c_void, fcoltype: u16, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, sztablename: Option<&[u8]>, fscope: u16, fnullable: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSpecialColumnsA(hstmt : *mut core::ffi::c_void, fcoltype : u16, szcatalogname : *const u8, cbcatalogname : i16, szschemaname : *const u8, cbschemaname : i16, sztablename : *const u8, cbtablename : i16, fscope : u16, fnullable : u16) -> i16);
    SQLSpecialColumnsA(
        hstmt,
        fcoltype,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        fscope,
        fnullable,
    )
}
#[inline]
pub unsafe fn SQLSpecialColumnsW(hstmt: *mut core::ffi::c_void, fcoltype: u16, szcatalogname: Option<&[u16]>, szschemaname: Option<&[u16]>, sztablename: Option<&[u16]>, fscope: u16, fnullable: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLSpecialColumnsW(hstmt : *mut core::ffi::c_void, fcoltype : u16, szcatalogname : *const u16, cchcatalogname : i16, szschemaname : *const u16, cchschemaname : i16, sztablename : *const u16, cchtablename : i16, fscope : u16, fnullable : u16) -> i16);
    SQLSpecialColumnsW(
        hstmt,
        fcoltype,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        fscope,
        fnullable,
    )
}
#[inline]
pub unsafe fn SQLStatistics(statementhandle: *mut core::ffi::c_void, catalogname: Option<&[u8]>, schemaname: Option<&[u8]>, tablename: Option<&[u8]>, unique: u16, reserved: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLStatistics(statementhandle : *mut core::ffi::c_void, catalogname : *const u8, namelength1 : i16, schemaname : *const u8, namelength2 : i16, tablename : *const u8, namelength3 : i16, unique : u16, reserved : u16) -> i16);
    SQLStatistics(
        statementhandle,
        core::mem::transmute(catalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        catalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(schemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        schemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(tablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        tablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        unique,
        reserved,
    )
}
#[inline]
pub unsafe fn SQLStatisticsA(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, sztablename: Option<&[u8]>, funique: u16, faccuracy: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLStatisticsA(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cbcatalogname : i16, szschemaname : *const u8, cbschemaname : i16, sztablename : *const u8, cbtablename : i16, funique : u16, faccuracy : u16) -> i16);
    SQLStatisticsA(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        funique,
        faccuracy,
    )
}
#[inline]
pub unsafe fn SQLStatisticsW(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u16]>, szschemaname: Option<&[u16]>, sztablename: Option<&[u16]>, funique: u16, faccuracy: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLStatisticsW(hstmt : *mut core::ffi::c_void, szcatalogname : *const u16, cchcatalogname : i16, szschemaname : *const u16, cchschemaname : i16, sztablename : *const u16, cchtablename : i16, funique : u16, faccuracy : u16) -> i16);
    SQLStatisticsW(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        funique,
        faccuracy,
    )
}
#[inline]
pub unsafe fn SQLTablePrivileges(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, sztablename: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLTablePrivileges(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cchcatalogname : i16, szschemaname : *const u8, cchschemaname : i16, sztablename : *const u8, cchtablename : i16) -> i16);
    SQLTablePrivileges(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLTablePrivilegesA(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, sztablename: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLTablePrivilegesA(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cbcatalogname : i16, szschemaname : *const u8, cbschemaname : i16, sztablename : *const u8, cbtablename : i16) -> i16);
    SQLTablePrivilegesA(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLTablePrivilegesW(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u16]>, szschemaname: Option<&[u16]>, sztablename: Option<&[u16]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLTablePrivilegesW(hstmt : *mut core::ffi::c_void, szcatalogname : *const u16, cchcatalogname : i16, szschemaname : *const u16, cchschemaname : i16, sztablename : *const u16, cchtablename : i16) -> i16);
    SQLTablePrivilegesW(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLTables(statementhandle: *mut core::ffi::c_void, catalogname: Option<&[u8]>, schemaname: Option<&[u8]>, tablename: Option<&[u8]>, tabletype: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLTables(statementhandle : *mut core::ffi::c_void, catalogname : *const u8, namelength1 : i16, schemaname : *const u8, namelength2 : i16, tablename : *const u8, namelength3 : i16, tabletype : *const u8, namelength4 : i16) -> i16);
    SQLTables(
        statementhandle,
        core::mem::transmute(catalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        catalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(schemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        schemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(tablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        tablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(tabletype.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        tabletype.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLTablesA(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u8]>, szschemaname: Option<&[u8]>, sztablename: Option<&[u8]>, sztabletype: Option<&[u8]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLTablesA(hstmt : *mut core::ffi::c_void, szcatalogname : *const u8, cbcatalogname : i16, szschemaname : *const u8, cbschemaname : i16, sztablename : *const u8, cbtablename : i16, sztabletype : *const u8, cbtabletype : i16) -> i16);
    SQLTablesA(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztabletype.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztabletype.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLTablesW(hstmt: *mut core::ffi::c_void, szcatalogname: Option<&[u16]>, szschemaname: Option<&[u16]>, sztablename: Option<&[u16]>, sztabletype: Option<&[u16]>) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLTablesW(hstmt : *mut core::ffi::c_void, szcatalogname : *const u16, cchcatalogname : i16, szschemaname : *const u16, cchschemaname : i16, sztablename : *const u16, cchtablename : i16, sztabletype : *const u16, cchtabletype : i16) -> i16);
    SQLTablesW(
        hstmt,
        core::mem::transmute(szcatalogname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szcatalogname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(szschemaname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        szschemaname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztablename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztablename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(sztabletype.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        sztabletype.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn SQLTransact(environmenthandle: *mut core::ffi::c_void, connectionhandle: *mut core::ffi::c_void, completiontype: u16) -> i16 {
    windows_targets::link!("odbc32.dll" "system" fn SQLTransact(environmenthandle : *mut core::ffi::c_void, connectionhandle : *mut core::ffi::c_void, completiontype : u16) -> i16);
    SQLTransact(environmenthandle, connectionhandle, completiontype)
}
#[inline]
pub unsafe fn bcp_batch(param0: *mut core::ffi::c_void) -> i32 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_batch(param0 : *mut core::ffi::c_void) -> i32);
    bcp_batch(param0)
}
#[inline]
pub unsafe fn bcp_bind(param0: *mut core::ffi::c_void, param1: *mut u8, param2: i32, param3: i32, param4: *mut u8, param5: i32, param6: i32, param7: i32) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_bind(param0 : *mut core::ffi::c_void, param1 : *mut u8, param2 : i32, param3 : i32, param4 : *mut u8, param5 : i32, param6 : i32, param7 : i32) -> i16);
    bcp_bind(param0, param1, param2, param3, param4, param5, param6, param7)
}
#[inline]
pub unsafe fn bcp_colfmt(param0: *mut core::ffi::c_void, param1: i32, param2: u8, param3: i32, param4: i32, param5: *mut u8, param6: i32, param7: i32) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_colfmt(param0 : *mut core::ffi::c_void, param1 : i32, param2 : u8, param3 : i32, param4 : i32, param5 : *mut u8, param6 : i32, param7 : i32) -> i16);
    bcp_colfmt(param0, param1, param2, param3, param4, param5, param6, param7)
}
#[inline]
pub unsafe fn bcp_collen(param0: *mut core::ffi::c_void, param1: i32, param2: i32) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_collen(param0 : *mut core::ffi::c_void, param1 : i32, param2 : i32) -> i16);
    bcp_collen(param0, param1, param2)
}
#[inline]
pub unsafe fn bcp_colptr(param0: *mut core::ffi::c_void, param1: *mut u8, param2: i32) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_colptr(param0 : *mut core::ffi::c_void, param1 : *mut u8, param2 : i32) -> i16);
    bcp_colptr(param0, param1, param2)
}
#[inline]
pub unsafe fn bcp_columns(param0: *mut core::ffi::c_void, param1: i32) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_columns(param0 : *mut core::ffi::c_void, param1 : i32) -> i16);
    bcp_columns(param0, param1)
}
#[inline]
pub unsafe fn bcp_control(param0: *mut core::ffi::c_void, param1: i32, param2: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_control(param0 : *mut core::ffi::c_void, param1 : i32, param2 : *mut core::ffi::c_void) -> i16);
    bcp_control(param0, param1, param2)
}
#[inline]
pub unsafe fn bcp_done(param0: *mut core::ffi::c_void) -> i32 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_done(param0 : *mut core::ffi::c_void) -> i32);
    bcp_done(param0)
}
#[inline]
pub unsafe fn bcp_exec(param0: *mut core::ffi::c_void, param1: *mut i32) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_exec(param0 : *mut core::ffi::c_void, param1 : *mut i32) -> i16);
    bcp_exec(param0, param1)
}
#[inline]
pub unsafe fn bcp_getcolfmt(param0: *mut core::ffi::c_void, param1: i32, param2: i32, param3: *mut core::ffi::c_void, param4: i32, param5: *mut i32) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_getcolfmt(param0 : *mut core::ffi::c_void, param1 : i32, param2 : i32, param3 : *mut core::ffi::c_void, param4 : i32, param5 : *mut i32) -> i16);
    bcp_getcolfmt(param0, param1, param2, param3, param4, param5)
}
#[inline]
pub unsafe fn bcp_initA<P0, P1, P2>(param0: *mut core::ffi::c_void, param1: P0, param2: P1, param3: P2, param4: i32) -> i16
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_initA(param0 : *mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : windows_core::PCSTR, param3 : windows_core::PCSTR, param4 : i32) -> i16);
    bcp_initA(param0, param1.param().abi(), param2.param().abi(), param3.param().abi(), param4)
}
#[inline]
pub unsafe fn bcp_initW<P0, P1, P2>(param0: *mut core::ffi::c_void, param1: P0, param2: P1, param3: P2, param4: i32) -> i16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_initW(param0 : *mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : windows_core::PCWSTR, param3 : windows_core::PCWSTR, param4 : i32) -> i16);
    bcp_initW(param0, param1.param().abi(), param2.param().abi(), param3.param().abi(), param4)
}
#[inline]
pub unsafe fn bcp_moretext(param0: *mut core::ffi::c_void, param1: i32, param2: *mut u8) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_moretext(param0 : *mut core::ffi::c_void, param1 : i32, param2 : *mut u8) -> i16);
    bcp_moretext(param0, param1, param2)
}
#[inline]
pub unsafe fn bcp_readfmtA<P0>(param0: *mut core::ffi::c_void, param1: P0) -> i16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_readfmtA(param0 : *mut core::ffi::c_void, param1 : windows_core::PCSTR) -> i16);
    bcp_readfmtA(param0, param1.param().abi())
}
#[inline]
pub unsafe fn bcp_readfmtW<P0>(param0: *mut core::ffi::c_void, param1: P0) -> i16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_readfmtW(param0 : *mut core::ffi::c_void, param1 : windows_core::PCWSTR) -> i16);
    bcp_readfmtW(param0, param1.param().abi())
}
#[inline]
pub unsafe fn bcp_sendrow(param0: *mut core::ffi::c_void) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_sendrow(param0 : *mut core::ffi::c_void) -> i16);
    bcp_sendrow(param0)
}
#[inline]
pub unsafe fn bcp_setcolfmt(param0: *mut core::ffi::c_void, param1: i32, param2: i32, param3: *mut core::ffi::c_void, param4: i32) -> i16 {
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_setcolfmt(param0 : *mut core::ffi::c_void, param1 : i32, param2 : i32, param3 : *mut core::ffi::c_void, param4 : i32) -> i16);
    bcp_setcolfmt(param0, param1, param2, param3, param4)
}
#[inline]
pub unsafe fn bcp_writefmtA<P0>(param0: *mut core::ffi::c_void, param1: P0) -> i16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_writefmtA(param0 : *mut core::ffi::c_void, param1 : windows_core::PCSTR) -> i16);
    bcp_writefmtA(param0, param1.param().abi())
}
#[inline]
pub unsafe fn bcp_writefmtW<P0>(param0: *mut core::ffi::c_void, param1: P0) -> i16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("odbcbcp.dll" "system" fn bcp_writefmtW(param0 : *mut core::ffi::c_void, param1 : windows_core::PCWSTR) -> i16);
    bcp_writefmtW(param0, param1.param().abi())
}
#[inline]
pub unsafe fn dbprtypeA(param0: i32) -> windows_core::PSTR {
    windows_targets::link!("odbcbcp.dll" "system" fn dbprtypeA(param0 : i32) -> windows_core::PSTR);
    dbprtypeA(param0)
}
#[inline]
pub unsafe fn dbprtypeW(param0: i32) -> windows_core::PWSTR {
    windows_targets::link!("odbcbcp.dll" "system" fn dbprtypeW(param0 : i32) -> windows_core::PWSTR);
    dbprtypeW(param0)
}
windows_core::imp::define_interface!(DataSource, DataSource_Vtbl, 0x7c0ffab3_cd84_11d0_949a_00a0c91110ed);
impl core::ops::Deref for DataSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(DataSource, windows_core::IUnknown);
impl DataSource {
    pub unsafe fn getDataMember(&self, bstrdm: *const u16, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getDataMember)(windows_core::Interface::as_raw(self), bstrdm, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn getDataMemberName(&self, lindex: i32) -> windows_core::Result<*mut u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getDataMemberName)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| result__)
    }
    pub unsafe fn getDataMemberCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getDataMemberCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn addDataSourceListener<P0>(&self, pdsl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DataSourceListener>,
    {
        (windows_core::Interface::vtable(self).addDataSourceListener)(windows_core::Interface::as_raw(self), pdsl.param().abi()).ok()
    }
    pub unsafe fn removeDataSourceListener<P0>(&self, pdsl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DataSourceListener>,
    {
        (windows_core::Interface::vtable(self).removeDataSourceListener)(windows_core::Interface::as_raw(self), pdsl.param().abi()).ok()
    }
}
#[repr(C)]
pub struct DataSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub getDataMember: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getDataMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut u16) -> windows_core::HRESULT,
    pub getDataMemberCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub addDataSourceListener: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removeDataSourceListener: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(DataSourceListener, DataSourceListener_Vtbl, 0x7c0ffab2_cd84_11d0_949a_00a0c91110ed);
impl core::ops::Deref for DataSourceListener {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(DataSourceListener, windows_core::IUnknown);
impl DataSourceListener {
    pub unsafe fn dataMemberChanged(&self, bstrdm: *const u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).dataMemberChanged)(windows_core::Interface::as_raw(self), bstrdm).ok()
    }
    pub unsafe fn dataMemberAdded(&self, bstrdm: *const u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).dataMemberAdded)(windows_core::Interface::as_raw(self), bstrdm).ok()
    }
    pub unsafe fn dataMemberRemoved(&self, bstrdm: *const u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).dataMemberRemoved)(windows_core::Interface::as_raw(self), bstrdm).ok()
    }
}
#[repr(C)]
pub struct DataSourceListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub dataMemberChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
    pub dataMemberAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
    pub dataMemberRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DataSourceObject, DataSourceObject_Vtbl, 0x0ae9a4e4_18d4_11d1_b3b3_00aa00c1a924);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DataSourceObject {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DataSourceObject, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DataSourceObject {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DataSourceObject_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
windows_core::imp::define_interface!(IAccessor, IAccessor_Vtbl, 0x0c733a8c_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IAccessor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccessor, windows_core::IUnknown);
impl IAccessor {
    pub unsafe fn AddRefAccessor<P0>(&self, haccessor: P0, pcrefcount: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).AddRefAccessor)(windows_core::Interface::as_raw(self), haccessor.param().abi(), core::mem::transmute(pcrefcount.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAccessor(&self, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut HACCESSOR, rgstatus: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateAccessor)(windows_core::Interface::as_raw(self), dwaccessorflags, cbindings, rgbindings, cbrowsize, phaccessor, core::mem::transmute(rgstatus.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBindings<P0>(&self, haccessor: P0, pdwaccessorflags: *mut u32, pcbindings: Option<*mut usize>, prgbindings: *mut *mut DBBINDING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).GetBindings)(windows_core::Interface::as_raw(self), haccessor.param().abi(), pdwaccessorflags, core::mem::transmute(pcbindings.unwrap_or(std::ptr::null_mut())), prgbindings).ok()
    }
    pub unsafe fn ReleaseAccessor<P0>(&self, haccessor: P0, pcrefcount: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).ReleaseAccessor)(windows_core::Interface::as_raw(self), haccessor.param().abi(), core::mem::transmute(pcrefcount.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IAccessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRefAccessor: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAccessor: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, *const DBBINDING, usize, *mut HACCESSOR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAccessor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBindings: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, *mut u32, *mut usize, *mut *mut DBBINDING) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBindings: usize,
    pub ReleaseAccessor: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAlterIndex, IAlterIndex_Vtbl, 0x0c733aa6_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IAlterIndex {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAlterIndex, windows_core::IUnknown);
impl IAlterIndex {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn AlterIndex(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, pnewindexid: *const super::super::Storage::IndexServer::DBID, rgpropertysets: &mut [DBPROPSET]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AlterIndex)(windows_core::Interface::as_raw(self), ptableid, pindexid, pnewindexid, rgpropertysets.len().try_into().unwrap(), core::mem::transmute(rgpropertysets.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IAlterIndex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub AlterIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    AlterIndex: usize,
}
windows_core::imp::define_interface!(IAlterTable, IAlterTable_Vtbl, 0x0c733aa5_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IAlterTable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAlterTable, windows_core::IUnknown);
impl IAlterTable {
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn AlterColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *const DBCOLUMNDESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AlterColumn)(windows_core::Interface::as_raw(self), ptableid, pcolumnid, dwcolumndescflags, pcolumndesc).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn AlterTable(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pnewtableid: *const super::super::Storage::IndexServer::DBID, rgpropertysets: &mut [DBPROPSET]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AlterTable)(windows_core::Interface::as_raw(self), ptableid, pnewtableid, rgpropertysets.len().try_into().unwrap(), core::mem::transmute(rgpropertysets.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IAlterTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub AlterColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID, u32, *const DBCOLUMNDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    AlterColumn: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub AlterTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    AlterTable: usize,
}
windows_core::imp::define_interface!(IBindResource, IBindResource_Vtbl, 0x0c733ab1_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IBindResource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBindResource, windows_core::IUnknown);
impl IBindResource {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Bind<P0, P1, P2>(&self, punkouter: P0, pwszurl: P1, dwbindurlflags: u32, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, pauthenticate: P2, pimplsession: Option<*mut DBIMPLICITSESSION>, pdwbindstatus: Option<*mut u32>, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::Com::IAuthenticate>,
    {
        (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), punkouter.param().abi(), pwszurl.param().abi(), dwbindurlflags, rguid, riid, pauthenticate.param().abi(), core::mem::transmute(pimplsession.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwbindstatus.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppunk)).ok()
    }
}
#[repr(C)]
pub struct IBindResource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void, *mut DBIMPLICITSESSION, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Bind: usize,
}
windows_core::imp::define_interface!(IChapteredRowset, IChapteredRowset_Vtbl, 0x0c733a93_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IChapteredRowset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IChapteredRowset, windows_core::IUnknown);
impl IChapteredRowset {
    pub unsafe fn AddRefChapter(&self, hchapter: usize, pcrefcount: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddRefChapter)(windows_core::Interface::as_raw(self), hchapter, core::mem::transmute(pcrefcount.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ReleaseChapter(&self, hchapter: usize, pcrefcount: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseChapter)(windows_core::Interface::as_raw(self), hchapter, core::mem::transmute(pcrefcount.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IChapteredRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRefChapter: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32) -> windows_core::HRESULT,
    pub ReleaseChapter: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IColumnMapper, IColumnMapper_Vtbl, 0x0b63e37a_9ccc_11d0_bcdb_00805fccce04);
impl core::ops::Deref for IColumnMapper {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IColumnMapper, windows_core::IUnknown);
impl IColumnMapper {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetPropInfoFromName<P0>(&self, wcspropname: P0, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetPropInfoFromName)(windows_core::Interface::as_raw(self), wcspropname.param().abi(), pppropid, pproptype, puiwidth).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetPropInfoFromId(&self, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropInfoFromId)(windows_core::Interface::as_raw(self), ppropid, pwcsname, pproptype, puiwidth).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn EnumPropInfo(&self, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumPropInfo)(windows_core::Interface::as_raw(self), ientry, pwcsname, pppropid, pproptype, puiwidth).ok()
    }
    pub unsafe fn IsMapUpToDate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsMapUpToDate)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IColumnMapper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetPropInfoFromName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut super::super::Storage::IndexServer::DBID, *mut u16, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetPropInfoFromName: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetPropInfoFromId: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *mut *mut u16, *mut u16, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetPropInfoFromId: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub EnumPropInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const u16, *mut *mut super::super::Storage::IndexServer::DBID, *mut u16, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    EnumPropInfo: usize,
    pub IsMapUpToDate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IColumnMapperCreator, IColumnMapperCreator_Vtbl, 0x0b63e37b_9ccc_11d0_bcdb_00805fccce04);
impl core::ops::Deref for IColumnMapperCreator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IColumnMapperCreator, windows_core::IUnknown);
impl IColumnMapperCreator {
    pub unsafe fn GetColumnMapper<P0, P1>(&self, wcsmachinename: P0, wcscatalogname: P1) -> windows_core::Result<IColumnMapper>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumnMapper)(windows_core::Interface::as_raw(self), wcsmachinename.param().abi(), wcscatalogname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IColumnMapperCreator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetColumnMapper: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IColumnsInfo, IColumnsInfo_Vtbl, 0x0c733a11_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IColumnsInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IColumnsInfo, windows_core::IUnknown);
impl IColumnsInfo {
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn GetColumnInfo(&self, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: Option<*mut *mut u16>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumnInfo)(windows_core::Interface::as_raw(self), pccolumns, prginfo, core::mem::transmute(ppstringsbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn MapColumnIDs(&self, ccolumnids: usize, rgcolumnids: Option<*const super::super::Storage::IndexServer::DBID>, rgcolumns: Option<*mut usize>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MapColumnIDs)(windows_core::Interface::as_raw(self), ccolumnids, core::mem::transmute(rgcolumnids.unwrap_or(std::ptr::null())), core::mem::transmute(rgcolumns.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IColumnsInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub GetColumnInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut *mut DBCOLUMNINFO, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    GetColumnInfo: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub MapColumnIDs: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const super::super::Storage::IndexServer::DBID, *mut usize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    MapColumnIDs: usize,
}
windows_core::imp::define_interface!(IColumnsInfo2, IColumnsInfo2_Vtbl, 0x0c733ab8_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IColumnsInfo2 {
    type Target = IColumnsInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IColumnsInfo2, windows_core::IUnknown, IColumnsInfo);
impl IColumnsInfo2 {
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn GetRestrictedColumnInfo(&self, rgcolumnidmasks: &[super::super::Storage::IndexServer::DBID], dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: Option<*mut *mut u16>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRestrictedColumnInfo)(windows_core::Interface::as_raw(self), rgcolumnidmasks.len().try_into().unwrap(), core::mem::transmute(rgcolumnidmasks.as_ptr()), dwflags, pccolumns, prgcolumnids, prgcolumninfo, core::mem::transmute(ppstringsbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IColumnsInfo2_Vtbl {
    pub base__: IColumnsInfo_Vtbl,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub GetRestrictedColumnInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const super::super::Storage::IndexServer::DBID, u32, *mut usize, *mut *mut super::super::Storage::IndexServer::DBID, *mut *mut DBCOLUMNINFO, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    GetRestrictedColumnInfo: usize,
}
windows_core::imp::define_interface!(IColumnsRowset, IColumnsRowset_Vtbl, 0x0c733a10_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IColumnsRowset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IColumnsRowset, windows_core::IUnknown);
impl IColumnsRowset {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetAvailableColumns(&self, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAvailableColumns)(windows_core::Interface::as_raw(self), pcoptcolumns, prgoptcolumns).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetColumnsRowset<P0>(&self, punkouter: P0, rgoptcolumns: &[super::super::Storage::IndexServer::DBID], riid: *const windows_core::GUID, rgpropertysets: Option<&mut [DBPROPSET]>, ppcolrowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetColumnsRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), rgoptcolumns.len().try_into().unwrap(), core::mem::transmute(rgoptcolumns.as_ptr()), riid, rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertysets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(ppcolrowset)).ok()
    }
}
#[repr(C)]
pub struct IColumnsRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetAvailableColumns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetAvailableColumns: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetColumnsRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, *const super::super::Storage::IndexServer::DBID, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetColumnsRowset: usize,
}
windows_core::imp::define_interface!(ICommand, ICommand_Vtbl, 0x0c733a63_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICommand {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommand, windows_core::IUnknown);
impl ICommand {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Execute<P0>(&self, punkouter: P0, riid: *const windows_core::GUID, pparams: Option<*mut DBPARAMS>, pcrowsaffected: Option<*mut isize>, pprowset: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, core::mem::transmute(pparams.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcrowsaffected.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pprowset.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDBSession(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDBSession)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut DBPARAMS, *mut isize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDBSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommandCost, ICommandCost_Vtbl, 0x0c733a4e_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICommandCost {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommandCost, windows_core::IUnknown);
impl ICommandCost {
    pub unsafe fn GetAccumulatedCost<P0>(&self, pwszrowsetname: P0, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetAccumulatedCost)(windows_core::Interface::as_raw(self), pwszrowsetname.param().abi(), pccostlimits, prgcostlimits).ok()
    }
    pub unsafe fn GetCostEstimate<P0>(&self, pwszrowsetname: P0, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetCostEstimate)(windows_core::Interface::as_raw(self), pwszrowsetname.param().abi(), pccostestimates, prgcostestimates).ok()
    }
    pub unsafe fn GetCostGoals<P0>(&self, pwszrowsetname: P0, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetCostGoals)(windows_core::Interface::as_raw(self), pwszrowsetname.param().abi(), pccostgoals, prgcostgoals).ok()
    }
    pub unsafe fn GetCostLimits<P0>(&self, pwszrowsetname: P0, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetCostLimits)(windows_core::Interface::as_raw(self), pwszrowsetname.param().abi(), pccostlimits, prgcostlimits).ok()
    }
    pub unsafe fn SetCostGoals<P0>(&self, pwszrowsetname: P0, rgcostgoals: &[DBCOST]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCostGoals)(windows_core::Interface::as_raw(self), pwszrowsetname.param().abi(), rgcostgoals.len().try_into().unwrap(), core::mem::transmute(rgcostgoals.as_ptr())).ok()
    }
    pub unsafe fn SetCostLimits<P0>(&self, pwszrowsetname: P0, ccostlimits: u32, prgcostlimits: *const DBCOST, dwexecutionflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCostLimits)(windows_core::Interface::as_raw(self), pwszrowsetname.param().abi(), ccostlimits, prgcostlimits, dwexecutionflags).ok()
    }
}
#[repr(C)]
pub struct ICommandCost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAccumulatedCost: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut DBCOST) -> windows_core::HRESULT,
    pub GetCostEstimate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut DBCOST) -> windows_core::HRESULT,
    pub GetCostGoals: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut DBCOST) -> windows_core::HRESULT,
    pub GetCostLimits: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut DBCOST) -> windows_core::HRESULT,
    pub SetCostGoals: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const DBCOST) -> windows_core::HRESULT,
    pub SetCostLimits: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const DBCOST, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommandPersist, ICommandPersist_Vtbl, 0x0c733aa7_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICommandPersist {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommandPersist, windows_core::IUnknown);
impl ICommandPersist {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn DeleteCommand(&self, pcommandid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteCommand)(windows_core::Interface::as_raw(self), pcommandid).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetCurrentCommand(&self) -> windows_core::Result<*mut super::super::Storage::IndexServer::DBID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentCommand)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn LoadCommand(&self, pcommandid: *const super::super::Storage::IndexServer::DBID, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadCommand)(windows_core::Interface::as_raw(self), pcommandid, dwflags).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn SaveCommand(&self, pcommandid: *const super::super::Storage::IndexServer::DBID, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveCommand)(windows_core::Interface::as_raw(self), pcommandid, dwflags).ok()
    }
}
#[repr(C)]
pub struct ICommandPersist_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DeleteCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DeleteCommand: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetCurrentCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetCurrentCommand: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub LoadCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    LoadCommand: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub SaveCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    SaveCommand: usize,
}
windows_core::imp::define_interface!(ICommandPrepare, ICommandPrepare_Vtbl, 0x0c733a26_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICommandPrepare {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommandPrepare, windows_core::IUnknown);
impl ICommandPrepare {
    pub unsafe fn Prepare(&self, cexpectedruns: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self), cexpectedruns).ok()
    }
    pub unsafe fn Unprepare(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unprepare)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICommandPrepare_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Unprepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommandProperties, ICommandProperties_Vtbl, 0x0c733a79_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICommandProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommandProperties, windows_core::IUnknown);
impl ICommandProperties {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetProperties(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), rgpropertyidsets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertyidsets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcpropertysets, prgpropertysets).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn SetProperties(&self, rgpropertysets: &[DBPROPSET]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProperties)(windows_core::Interface::as_raw(self), rgpropertysets.len().try_into().unwrap(), core::mem::transmute(rgpropertysets.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ICommandProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetProperties: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub SetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    SetProperties: usize,
}
windows_core::imp::define_interface!(ICommandStream, ICommandStream_Vtbl, 0x0c733abf_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICommandStream {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommandStream, windows_core::IUnknown);
impl ICommandStream {
    pub unsafe fn GetCommandStream(&self, piid: Option<*mut windows_core::GUID>, pguiddialect: Option<*mut windows_core::GUID>, ppcommandstream: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCommandStream)(windows_core::Interface::as_raw(self), core::mem::transmute(piid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pguiddialect.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppcommandstream)).ok()
    }
    pub unsafe fn SetCommandStream<P0>(&self, riid: *const windows_core::GUID, rguiddialect: *const windows_core::GUID, pcommandstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetCommandStream)(windows_core::Interface::as_raw(self), riid, rguiddialect, pcommandstream.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICommandStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCommandStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommandStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommandText, ICommandText_Vtbl, 0x0c733a27_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICommandText {
    type Target = ICommand;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommandText, windows_core::IUnknown, ICommand);
impl ICommandText {
    pub unsafe fn GetCommandText(&self, pguiddialect: Option<*mut windows_core::GUID>, ppwszcommand: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCommandText)(windows_core::Interface::as_raw(self), core::mem::transmute(pguiddialect.unwrap_or(std::ptr::null_mut())), ppwszcommand).ok()
    }
    pub unsafe fn SetCommandText<P0>(&self, rguiddialect: *const windows_core::GUID, pwszcommand: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCommandText)(windows_core::Interface::as_raw(self), rguiddialect, pwszcommand.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICommandText_Vtbl {
    pub base__: ICommand_Vtbl,
    pub GetCommandText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCommandText: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommandValidate, ICommandValidate_Vtbl, 0x0c733a18_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICommandValidate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommandValidate, windows_core::IUnknown);
impl ICommandValidate {
    pub unsafe fn ValidateCompletely(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ValidateCompletely)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ValidateSyntax(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ValidateSyntax)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICommandValidate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ValidateCompletely: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValidateSyntax: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommandWithParameters, ICommandWithParameters_Vtbl, 0x0c733a64_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICommandWithParameters {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommandWithParameters, windows_core::IUnknown);
impl ICommandWithParameters {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetParameterInfo(&self, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: Option<*mut *mut u16>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetParameterInfo)(windows_core::Interface::as_raw(self), pcparams, prgparaminfo, core::mem::transmute(ppnamesbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn MapParameterNames(&self, cparamnames: usize, rgparamnames: *const windows_core::PCWSTR, rgparamordinals: *mut isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MapParameterNames)(windows_core::Interface::as_raw(self), cparamnames, rgparamnames, rgparamordinals).ok()
    }
    pub unsafe fn SetParameterInfo(&self, cparams: usize, rgparamordinals: Option<*const usize>, rgparambindinfo: Option<*const DBPARAMBINDINFO>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParameterInfo)(windows_core::Interface::as_raw(self), cparams, core::mem::transmute(rgparamordinals.unwrap_or(std::ptr::null())), core::mem::transmute(rgparambindinfo.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct ICommandWithParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParameterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut *mut DBPARAMINFO, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParameterInfo: usize,
    pub MapParameterNames: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::PCWSTR, *mut isize) -> windows_core::HRESULT,
    pub SetParameterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const usize, *const DBPARAMBINDINFO) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICondition, ICondition_Vtbl, 0x0fc988d4_c935_4b97_a973_46282ea175c8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICondition {
    type Target = super::Com::IPersistStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICondition, windows_core::IUnknown, super::Com::IPersist, super::Com::IPersistStream);
#[cfg(feature = "Win32_System_Com")]
impl ICondition {
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self) -> windows_core::Result<Common::CONDITION_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConditionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSubConditions<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetSubConditions)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetComparisonInfo(&self, ppszpropertyname: Option<*mut windows_core::PWSTR>, pcop: Option<*mut Common::CONDITION_OPERATION>, ppropvar: Option<*mut windows_core::PROPVARIANT>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetComparisonInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszpropertyname.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcop.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppropvar.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetValueType(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetValueNormalization(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValueNormalization)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetInputTerms(&self, pppropertyterm: Option<*mut Option<IRichChunk>>, ppoperationterm: Option<*mut Option<IRichChunk>>, ppvalueterm: Option<*mut Option<IRichChunk>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputTerms)(windows_core::Interface::as_raw(self), core::mem::transmute(pppropertyterm.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppoperationterm.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppvalueterm.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> windows_core::Result<ICondition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICondition_Vtbl {
    pub base__: super::Com::IPersistStream_Vtbl,
    #[cfg(feature = "Win32_System_Search_Common")]
    pub GetConditionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::CONDITION_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))]
    GetConditionType: usize,
    pub GetSubConditions: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Search_Common")]
    pub GetComparisonInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut Common::CONDITION_OPERATION, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))]
    GetComparisonInfo: usize,
    pub GetValueType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetValueNormalization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetInputTerms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICondition2, ICondition2_Vtbl, 0x0db8851d_2e5b_47eb_9208_d28c325a01d7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICondition2 {
    type Target = ICondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICondition2, windows_core::IUnknown, super::Com::IPersist, super::Com::IPersistStream, ICondition);
#[cfg(feature = "Win32_System_Com")]
impl ICondition2 {
    pub unsafe fn GetLocale(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetLeafConditionInfo(&self, ppropkey: Option<*mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>, pcop: Option<*mut Common::CONDITION_OPERATION>, ppropvar: Option<*mut windows_core::PROPVARIANT>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLeafConditionInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(ppropkey.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcop.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppropvar.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICondition2_Vtbl {
    pub base__: ICondition_Vtbl,
    pub GetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetLeafConditionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut Common::CONDITION_OPERATION, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetLeafConditionInfo: usize,
}
windows_core::imp::define_interface!(IConditionFactory, IConditionFactory_Vtbl, 0xa5efe073_b16f_474f_9f3e_9f8b497a3e08);
impl core::ops::Deref for IConditionFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConditionFactory, windows_core::IUnknown);
impl IConditionFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MakeNot<P0, P1>(&self, pcsub: P0, fsimplify: P1) -> windows_core::Result<ICondition>
    where
        P0: windows_core::Param<ICondition>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MakeNot)(windows_core::Interface::as_raw(self), pcsub.param().abi(), fsimplify.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub unsafe fn MakeAndOr<P0, P1>(&self, ct: Common::CONDITION_TYPE, peusubs: P0, fsimplify: P1) -> windows_core::Result<ICondition>
    where
        P0: windows_core::Param<super::Com::IEnumUnknown>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MakeAndOr)(windows_core::Interface::as_raw(self), ct, peusubs.param().abi(), fsimplify.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub unsafe fn MakeLeaf<P0, P1, P2, P3, P4, P5>(&self, pszpropertyname: P0, cop: Common::CONDITION_OPERATION, pszvaluetype: P1, ppropvar: *const windows_core::PROPVARIANT, ppropertynameterm: P2, poperationterm: P3, pvalueterm: P4, fexpand: P5) -> windows_core::Result<ICondition>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IRichChunk>,
        P3: windows_core::Param<IRichChunk>,
        P4: windows_core::Param<IRichChunk>,
        P5: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MakeLeaf)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), cop, pszvaluetype.param().abi(), core::mem::transmute(ppropvar), ppropertynameterm.param().abi(), poperationterm.param().abi(), pvalueterm.param().abi(), fexpand.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resolve<P0>(&self, pc: P0, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: Option<*const super::super::Foundation::SYSTEMTIME>) -> windows_core::Result<ICondition>
    where
        P0: windows_core::Param<ICondition>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), pc.param().abi(), sqro, core::mem::transmute(pstreferencetime.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IConditionFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub MakeNot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MakeNot: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub MakeAndOr: unsafe extern "system" fn(*mut core::ffi::c_void, Common::CONDITION_TYPE, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common")))]
    MakeAndOr: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub MakeLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, Common::CONDITION_OPERATION, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common")))]
    MakeLeaf: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, STRUCTURED_QUERY_RESOLVE_OPTION, *const super::super::Foundation::SYSTEMTIME, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resolve: usize,
}
windows_core::imp::define_interface!(IConditionFactory2, IConditionFactory2_Vtbl, 0x71d222e1_432f_429e_8c13_b6dafde5077a);
impl core::ops::Deref for IConditionFactory2 {
    type Target = IConditionFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConditionFactory2, windows_core::IUnknown, IConditionFactory);
impl IConditionFactory2 {
    pub unsafe fn CreateTrueFalse<P0, T>(&self, fval: P0, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateTrueFalse)(windows_core::Interface::as_raw(self), fval.param().abi(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateNegation<P0, T>(&self, pcsub: P0, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        P0: windows_core::Param<ICondition>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateNegation)(windows_core::Interface::as_raw(self), pcsub.param().abi(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn CreateCompoundFromObjectArray<P0, T>(&self, ct: Common::CONDITION_TYPE, poasubs: P0, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::UI::Shell::Common::IObjectArray>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateCompoundFromObjectArray)(windows_core::Interface::as_raw(self), ct, poasubs.param().abi(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub unsafe fn CreateCompoundFromArray<T>(&self, ct: Common::CONDITION_TYPE, ppcondsubs: &[Option<ICondition>], cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateCompoundFromArray)(windows_core::Interface::as_raw(self), ct, core::mem::transmute(ppcondsubs.as_ptr()), ppcondsubs.len().try_into().unwrap(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn CreateStringLeaf<P0, P1, T>(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: P0, pszlocalename: P1, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateStringLeaf)(windows_core::Interface::as_raw(self), propkey, cop, pszvalue.param().abi(), pszlocalename.param().abi(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn CreateIntegerLeaf<T>(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateIntegerLeaf)(windows_core::Interface::as_raw(self), propkey, cop, lvalue, cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn CreateBooleanLeaf<P0, T>(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: P0, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateBooleanLeaf)(windows_core::Interface::as_raw(self), propkey, cop, fvalue.param().abi(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn CreateLeaf<P0, P1, P2, P3, P4, T>(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const windows_core::PROPVARIANT, pszsemantictype: P0, pszlocalename: P1, ppropertynameterm: P2, poperationterm: P3, pvalueterm: P4, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IRichChunk>,
        P3: windows_core::Param<IRichChunk>,
        P4: windows_core::Param<IRichChunk>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateLeaf)(windows_core::Interface::as_raw(self), propkey, cop, core::mem::transmute(propvar), pszsemantictype.param().abi(), pszlocalename.param().abi(), ppropertynameterm.param().abi(), poperationterm.param().abi(), pvalueterm.param().abi(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResolveCondition<P0, T>(&self, pc: P0, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: Option<*const super::super::Foundation::SYSTEMTIME>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<ICondition>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).ResolveCondition)(windows_core::Interface::as_raw(self), pc.param().abi(), sqro, core::mem::transmute(pstreferencetime.unwrap_or(std::ptr::null())), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IConditionFactory2_Vtbl {
    pub base__: IConditionFactory_Vtbl,
    pub CreateTrueFalse: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateNegation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateNegation: usize,
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common"))]
    pub CreateCompoundFromObjectArray: unsafe extern "system" fn(*mut core::ffi::c_void, Common::CONDITION_TYPE, *mut core::ffi::c_void, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common")))]
    CreateCompoundFromObjectArray: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub CreateCompoundFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, Common::CONDITION_TYPE, *const *mut core::ffi::c_void, u32, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common")))]
    CreateCompoundFromArray: usize,
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateStringLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, Common::CONDITION_OPERATION, windows_core::PCWSTR, windows_core::PCWSTR, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateStringLeaf: usize,
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateIntegerLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, Common::CONDITION_OPERATION, i32, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateIntegerLeaf: usize,
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateBooleanLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, Common::CONDITION_OPERATION, super::super::Foundation::BOOL, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateBooleanLeaf: usize,
    #[cfg(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, Common::CONDITION_OPERATION, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateLeaf: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResolveCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, STRUCTURED_QUERY_RESOLVE_OPTION, *const super::super::Foundation::SYSTEMTIME, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResolveCondition: usize,
}
windows_core::imp::define_interface!(IConditionGenerator, IConditionGenerator_Vtbl, 0x92d2cc58_4386_45a3_b98c_7e0ce64a4117);
impl core::ops::Deref for IConditionGenerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConditionGenerator, windows_core::IUnknown);
impl IConditionGenerator {
    pub unsafe fn Initialize<P0>(&self, pschemaprovider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISchemaProvider>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pschemaprovider.param().abi()).ok()
    }
    pub unsafe fn RecognizeNamedEntities<P0, P1, P2>(&self, pszinputstring: P0, lciduserlocale: u32, ptokencollection: P1, pnamedentities: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ITokenCollection>,
        P2: windows_core::Param<INamedEntityCollector>,
    {
        (windows_core::Interface::vtable(self).RecognizeNamedEntities)(windows_core::Interface::as_raw(self), pszinputstring.param().abi(), lciduserlocale, ptokencollection.param().abi(), pnamedentities.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub unsafe fn GenerateForLeaf<P0, P1, P2, P3, P4, P5, P6, P7, P8>(&self, pconditionfactory: P0, pszpropertyname: P1, cop: Common::CONDITION_OPERATION, pszvaluetype: P2, pszvalue: P3, pszvalue2: P4, ppropertynameterm: P5, poperationterm: P6, pvalueterm: P7, automaticwildcard: P8, pnostringquery: *mut super::super::Foundation::BOOL) -> windows_core::Result<ICondition>
    where
        P0: windows_core::Param<IConditionFactory>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<IRichChunk>,
        P6: windows_core::Param<IRichChunk>,
        P7: windows_core::Param<IRichChunk>,
        P8: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenerateForLeaf)(windows_core::Interface::as_raw(self), pconditionfactory.param().abi(), pszpropertyname.param().abi(), cop, pszvaluetype.param().abi(), pszvalue.param().abi(), pszvalue2.param().abi(), ppropertynameterm.param().abi(), poperationterm.param().abi(), pvalueterm.param().abi(), automaticwildcard.param().abi(), pnostringquery, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DefaultPhrase<P0, P1>(&self, pszvaluetype: P0, ppropvar: *const windows_core::PROPVARIANT, fuseenglish: P1, ppszphrase: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).DefaultPhrase)(windows_core::Interface::as_raw(self), pszvaluetype.param().abi(), core::mem::transmute(ppropvar), fuseenglish.param().abi(), core::mem::transmute(ppszphrase.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IConditionGenerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecognizeNamedEntities: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub GenerateForLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, Common::CONDITION_OPERATION, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common")))]
    GenerateForLeaf: usize,
    pub DefaultPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, super::super::Foundation::BOOL, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConvertType, IConvertType_Vtbl, 0x0c733a88_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IConvertType {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConvertType, windows_core::IUnknown);
impl IConvertType {
    pub unsafe fn CanConvert(&self, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CanConvert)(windows_core::Interface::as_raw(self), wfromtype, wtotype, dwconvertflags).ok()
    }
}
#[repr(C)]
pub struct IConvertType_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CanConvert: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateRow, ICreateRow_Vtbl, 0x0c733ab2_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICreateRow {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateRow, windows_core::IUnknown);
impl ICreateRow {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRow<P0, P1, P2>(&self, punkouter: P0, pwszurl: P1, dwbindurlflags: u32, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, pauthenticate: P2, pimplsession: Option<*mut DBIMPLICITSESSION>, pdwbindstatus: *mut u32, ppwsznewurl: Option<*mut windows_core::PWSTR>, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::Com::IAuthenticate>,
    {
        (windows_core::Interface::vtable(self).CreateRow)(windows_core::Interface::as_raw(self), punkouter.param().abi(), pwszurl.param().abi(), dwbindurlflags, rguid, riid, pauthenticate.param().abi(), core::mem::transmute(pimplsession.unwrap_or(std::ptr::null_mut())), pdwbindstatus, core::mem::transmute(ppwsznewurl.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppunk)).ok()
    }
}
#[repr(C)]
pub struct ICreateRow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void, *mut DBIMPLICITSESSION, *mut u32, *mut windows_core::PWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRow: usize,
}
windows_core::imp::define_interface!(IDBAsynchNotify, IDBAsynchNotify_Vtbl, 0x0c733a96_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBAsynchNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBAsynchNotify, windows_core::IUnknown);
impl IDBAsynchNotify {
    pub unsafe fn OnLowResource(&self, dwreserved: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLowResource)(windows_core::Interface::as_raw(self), dwreserved).ok()
    }
    pub unsafe fn OnProgress<P0>(&self, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), hchapter, eoperation, ulprogress, ulprogressmax, easynchphase, pwszstatustext.param().abi()).ok()
    }
    pub unsafe fn OnStop<P0>(&self, hchapter: usize, eoperation: u32, hrstatus: windows_core::HRESULT, pwszstatustext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnStop)(windows_core::Interface::as_raw(self), hchapter, eoperation, hrstatus, pwszstatustext.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDBAsynchNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLowResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, usize, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnStop: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, windows_core::HRESULT, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDBAsynchStatus, IDBAsynchStatus_Vtbl, 0x0c733a95_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBAsynchStatus {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBAsynchStatus, windows_core::IUnknown);
impl IDBAsynchStatus {
    pub unsafe fn Abort(&self, hchapter: usize, eoperation: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self), hchapter, eoperation).ok()
    }
    pub unsafe fn GetStatus(&self, hchapter: usize, eoperation: u32, pulprogress: Option<*mut usize>, pulprogressmax: Option<*mut usize>, peasynchphase: *mut u32, ppwszstatustext: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), hchapter, eoperation, core::mem::transmute(pulprogress.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pulprogressmax.unwrap_or(std::ptr::null_mut())), peasynchphase, core::mem::transmute(ppwszstatustext.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IDBAsynchStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut usize, *mut usize, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDBBinderProperties, IDBBinderProperties_Vtbl, 0x0c733ab3_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBBinderProperties {
    type Target = IDBProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBBinderProperties, windows_core::IUnknown, IDBProperties);
impl IDBBinderProperties {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDBBinderProperties_Vtbl {
    pub base__: IDBProperties_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDBCreateCommand, IDBCreateCommand_Vtbl, 0x0c733a1d_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBCreateCommand {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBCreateCommand, windows_core::IUnknown);
impl IDBCreateCommand {
    pub unsafe fn CreateCommand<P0>(&self, punkouter: P0, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCommand)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDBCreateCommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDBCreateSession, IDBCreateSession_Vtbl, 0x0c733a5d_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBCreateSession {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBCreateSession, windows_core::IUnknown);
impl IDBCreateSession {
    pub unsafe fn CreateSession<P0>(&self, punkouter: P0, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSession)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDBCreateSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDBDataSourceAdmin, IDBDataSourceAdmin_Vtbl, 0x0c733a7a_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBDataSourceAdmin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBDataSourceAdmin, windows_core::IUnknown);
impl IDBDataSourceAdmin {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn CreateDataSource<P0>(&self, rgpropertysets: Option<&mut [DBPROPSET]>, punkouter: P0, riid: *const windows_core::GUID, ppdbsession: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateDataSource)(windows_core::Interface::as_raw(self), rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertysets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), punkouter.param().abi(), riid, core::mem::transmute(ppdbsession.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn DestroyDataSource(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DestroyDataSource)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn GetCreationProperties(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: Option<*mut *mut u16>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCreationProperties)(windows_core::Interface::as_raw(self), rgpropertyidsets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertyidsets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcpropertyinfosets, prgpropertyinfosets, core::mem::transmute(ppdescbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn ModifyDataSource(&self, rgpropertysets: Option<&mut [DBPROPSET]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModifyDataSource)(windows_core::Interface::as_raw(self), rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertysets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
}
#[repr(C)]
pub struct IDBDataSourceAdmin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub CreateDataSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DBPROPSET, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    CreateDataSource: usize,
    pub DestroyDataSource: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Variant")]
    pub GetCreationProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPINFOSET, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    GetCreationProperties: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub ModifyDataSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    ModifyDataSource: usize,
}
windows_core::imp::define_interface!(IDBInfo, IDBInfo_Vtbl, 0x0c733a89_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBInfo, windows_core::IUnknown);
impl IDBInfo {
    pub unsafe fn GetKeywords(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetKeywords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLiteralInfo(&self, rgliterals: Option<&[u32]>, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLiteralInfo)(windows_core::Interface::as_raw(self), rgliterals.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgliterals.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcliteralinfo, prgliteralinfo, ppcharbuffer).ok()
    }
}
#[repr(C)]
pub struct IDBInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetKeywords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetLiteralInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *mut u32, *mut *mut DBLITERALINFO, *mut *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDBInitialize, IDBInitialize_Vtbl, 0x0c733a8b_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBInitialize {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBInitialize, windows_core::IUnknown);
impl IDBInitialize {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Uninitialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Uninitialize)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDBInitialize_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDBPromptInitialize, IDBPromptInitialize_Vtbl, 0x2206ccb0_19c1_11d1_89e0_00c04fd7a829);
impl core::ops::Deref for IDBPromptInitialize {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBPromptInitialize, windows_core::IUnknown);
impl IDBPromptInitialize {
    pub unsafe fn PromptDataSource<P0, P1, P2>(&self, punkouter: P0, hwndparent: P1, dwpromptoptions: u32, rgsourcetypefilter: Option<&[u32]>, pwszszzproviderfilter: P2, riid: *const windows_core::GUID, ppdatasource: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PromptDataSource)(windows_core::Interface::as_raw(self), punkouter.param().abi(), hwndparent.param().abi(), dwpromptoptions, rgsourcetypefilter.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgsourcetypefilter.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pwszszzproviderfilter.param().abi(), riid, core::mem::transmute(ppdatasource)).ok()
    }
    pub unsafe fn PromptFileName<P0, P1, P2>(&self, hwndparent: P0, dwpromptoptions: u32, pwszinitialdirectory: P1, pwszinitialfile: P2) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PromptFileName)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), dwpromptoptions, pwszinitialdirectory.param().abi(), pwszinitialfile.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDBPromptInitialize_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PromptDataSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND, u32, u32, *const u32, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PromptFileName: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDBProperties, IDBProperties_Vtbl, 0x0c733a8a_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBProperties, windows_core::IUnknown);
impl IDBProperties {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetProperties(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), rgpropertyidsets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertyidsets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcpropertysets, prgpropertysets).ok()
    }
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn GetPropertyInfo(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: Option<*mut *mut u16>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyInfo)(windows_core::Interface::as_raw(self), rgpropertyidsets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertyidsets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcpropertyinfosets, prgpropertyinfosets, core::mem::transmute(ppdescbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn SetProperties(&self, rgpropertysets: Option<&mut [DBPROPSET]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProperties)(windows_core::Interface::as_raw(self), rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertysets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
}
#[repr(C)]
pub struct IDBProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetProperties: usize,
    #[cfg(feature = "Win32_System_Variant")]
    pub GetPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPINFOSET, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    GetPropertyInfo: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub SetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    SetProperties: usize,
}
windows_core::imp::define_interface!(IDBSchemaCommand, IDBSchemaCommand_Vtbl, 0x0c733a50_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBSchemaCommand {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBSchemaCommand, windows_core::IUnknown);
impl IDBSchemaCommand {
    pub unsafe fn GetCommand<P0>(&self, punkouter: P0, rguidschema: *const windows_core::GUID) -> windows_core::Result<ICommand>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCommand)(windows_core::Interface::as_raw(self), punkouter.param().abi(), rguidschema, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSchemas(&self, pcschemas: *mut u32, prgschemas: *mut *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSchemas)(windows_core::Interface::as_raw(self), pcschemas, prgschemas).ok()
    }
}
#[repr(C)]
pub struct IDBSchemaCommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSchemas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDBSchemaRowset, IDBSchemaRowset_Vtbl, 0x0c733a7b_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDBSchemaRowset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDBSchemaRowset, windows_core::IUnknown);
impl IDBSchemaRowset {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetRowset<P0>(&self, punkouter: P0, rguidschema: *const windows_core::GUID, rgrestrictions: Option<&[windows_core::VARIANT]>, riid: *const windows_core::GUID, rgpropertysets: Option<&mut [DBPROPSET]>, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), rguidschema, rgrestrictions.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgrestrictions.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), riid, rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertysets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(pprowset)).ok()
    }
    pub unsafe fn GetSchemas(&self, pcschemas: *mut u32, prgschemas: *mut *mut windows_core::GUID, prgrestrictionsupport: *mut *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSchemas)(windows_core::Interface::as_raw(self), pcschemas, prgschemas, prgrestrictionsupport).ok()
    }
}
#[repr(C)]
pub struct IDBSchemaRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetRowset: usize,
    pub GetSchemas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID, *mut *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDCInfo, IDCInfo_Vtbl, 0x0c733a9c_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDCInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDCInfo, windows_core::IUnknown);
impl IDCInfo {
    pub unsafe fn GetInfo(&self, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), cinfo, rgeinfotype, prginfo).ok()
    }
    pub unsafe fn SetInfo(&self, rginfo: &[DCINFO]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInfo)(windows_core::Interface::as_raw(self), rginfo.len().try_into().unwrap(), core::mem::transmute(rginfo.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IDCInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *mut *mut DCINFO) -> windows_core::HRESULT,
    pub SetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DCINFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataConvert, IDataConvert_Vtbl, 0x0c733a8d_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IDataConvert {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDataConvert, windows_core::IUnknown);
impl IDataConvert {
    pub unsafe fn DataConvert(&self, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: Option<*mut usize>, psrc: *const core::ffi::c_void, pdst: *mut core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: Option<*mut u32>, bprecision: u8, bscale: u8, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DataConvert)(windows_core::Interface::as_raw(self), wsrctype, wdsttype, cbsrclength, core::mem::transmute(pcbdstlength.unwrap_or(std::ptr::null_mut())), psrc, pdst, cbdstmaxlength, dbssrcstatus, core::mem::transmute(pdbsstatus.unwrap_or(std::ptr::null_mut())), bprecision, bscale, dwflags).ok()
    }
    pub unsafe fn CanConvert(&self, wsrctype: u16, wdsttype: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CanConvert)(windows_core::Interface::as_raw(self), wsrctype, wdsttype).ok()
    }
    pub unsafe fn GetConversionSize(&self, wsrctype: u16, wdsttype: u16, pcbsrclength: Option<*const usize>, pcbdstlength: Option<*mut usize>, psrc: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConversionSize)(windows_core::Interface::as_raw(self), wsrctype, wdsttype, core::mem::transmute(pcbsrclength.unwrap_or(std::ptr::null())), core::mem::transmute(pcbdstlength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(psrc.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IDataConvert_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DataConvert: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, usize, *mut usize, *const core::ffi::c_void, *mut core::ffi::c_void, usize, u32, *mut u32, u8, u8, u32) -> windows_core::HRESULT,
    pub CanConvert: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    pub GetConversionSize: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *const usize, *mut usize, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataInitialize, IDataInitialize_Vtbl, 0x2206ccb1_19c1_11d1_89e0_00c04fd7a829);
impl core::ops::Deref for IDataInitialize {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDataInitialize, windows_core::IUnknown);
impl IDataInitialize {
    pub unsafe fn GetDataSource<P0, P1>(&self, punkouter: P0, dwclsctx: u32, pwszinitializationstring: P1, riid: *const windows_core::GUID, ppdatasource: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetDataSource)(windows_core::Interface::as_raw(self), punkouter.param().abi(), dwclsctx, pwszinitializationstring.param().abi(), riid, core::mem::transmute(ppdatasource)).ok()
    }
    pub unsafe fn GetInitializationString<P0>(&self, pdatasource: P0, fincludepassword: u8) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInitializationString)(windows_core::Interface::as_raw(self), pdatasource.param().abi(), fincludepassword, &mut result__).map(|| result__)
    }
    pub unsafe fn CreateDBInstance<P0, P1>(&self, clsidprovider: *const windows_core::GUID, punkouter: P0, dwclsctx: u32, pwszreserved: P1, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDBInstance)(windows_core::Interface::as_raw(self), clsidprovider, punkouter.param().abi(), dwclsctx, pwszreserved.param().abi(), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDBInstanceEx<P0, P1>(&self, clsidprovider: *const windows_core::GUID, punkouter: P0, dwclsctx: u32, pwszreserved: P1, pserverinfo: *const super::Com::COSERVERINFO, rgmqresults: &mut [super::Com::MULTI_QI]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CreateDBInstanceEx)(windows_core::Interface::as_raw(self), clsidprovider, punkouter.param().abi(), dwclsctx, pwszreserved.param().abi(), pserverinfo, rgmqresults.len().try_into().unwrap(), core::mem::transmute(rgmqresults.as_ptr())).ok()
    }
    pub unsafe fn LoadStringFromStorage<P0>(&self, pwszfilename: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadStringFromStorage)(windows_core::Interface::as_raw(self), pwszfilename.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn WriteStringToStorage<P0, P1>(&self, pwszfilename: P0, pwszinitializationstring: P1, dwcreationdisposition: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).WriteStringToStorage)(windows_core::Interface::as_raw(self), pwszfilename.param().abi(), pwszinitializationstring.param().abi(), dwcreationdisposition).ok()
    }
}
#[repr(C)]
pub struct IDataInitialize_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDataSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInitializationString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u8, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub CreateDBInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDBInstanceEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32, windows_core::PCWSTR, *const super::Com::COSERVERINFO, u32, *mut super::Com::MULTI_QI) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDBInstanceEx: usize,
    pub LoadStringFromStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub WriteStringToStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDataSourceLocator, IDataSourceLocator_Vtbl, 0x2206ccb2_19c1_11d1_89e0_00c04fd7a829);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDataSourceLocator {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDataSourceLocator, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDataSourceLocator {
    pub unsafe fn hWnd(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).hWnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SethWnd<P0>(&self, hwndparent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SethWnd)(windows_core::Interface::as_raw(self), hwndparent.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PromptNew(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PromptNew)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PromptEdit(&self, ppadoconnection: *mut Option<super::Com::IDispatch>, pbsuccess: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PromptEdit)(windows_core::Interface::as_raw(self), core::mem::transmute(ppadoconnection), pbsuccess).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataSourceLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub hWnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub SethWnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PromptNew: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PromptNew: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PromptEdit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PromptEdit: usize,
}
windows_core::imp::define_interface!(IEntity, IEntity_Vtbl, 0x24264891_e80b_4fd3_b7ce_4ff2fae8931f);
impl core::ops::Deref for IEntity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEntity, windows_core::IUnknown);
impl IEntity {
    pub unsafe fn Name(&self, ppszname: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszname.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Base(&self) -> windows_core::Result<IEntity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Base)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Relationships<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).Relationships)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRelationship<P0>(&self, pszrelationname: P0) -> windows_core::Result<IRelationship>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRelationship)(windows_core::Interface::as_raw(self), pszrelationname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MetaData<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).MetaData)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NamedEntities<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).NamedEntities)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNamedEntity<P0>(&self, pszvalue: P0) -> windows_core::Result<INamedEntity>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNamedEntity)(windows_core::Interface::as_raw(self), pszvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DefaultPhrase(&self, ppszphrase: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DefaultPhrase)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszphrase.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IEntity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Base: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Relationships: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRelationship: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamedEntities: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamedEntity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DefaultPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumItemProperties, IEnumItemProperties_Vtbl, 0xf72c8d96_6dbd_11d1_a1e8_00c04fc2fbe1);
impl core::ops::Deref for IEnumItemProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumItemProperties, windows_core::IUnknown);
impl IEnumItemProperties {
    pub unsafe fn Next(&self, rgelt: &mut [ITEMPROP], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumItemProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEnumItemProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ITEMPROP, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumSearchRoots, IEnumSearchRoots_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef52);
impl core::ops::Deref for IEnumSearchRoots {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSearchRoots, windows_core::IUnknown);
impl IEnumSearchRoots {
    pub unsafe fn Next(&self, rgelt: &mut [Option<ISearchRoot>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSearchRoots> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSearchRoots_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumSearchScopeRules, IEnumSearchScopeRules_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef54);
impl core::ops::Deref for IEnumSearchScopeRules {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSearchScopeRules, windows_core::IUnknown);
impl IEnumSearchScopeRules {
    pub unsafe fn Next(&self, pprgelt: &mut [Option<ISearchScopeRule>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pprgelt.len().try_into().unwrap(), core::mem::transmute(pprgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSearchScopeRules> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSearchScopeRules_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumSubscription, IEnumSubscription_Vtbl, 0xf72c8d97_6dbd_11d1_a1e8_00c04fc2fbe1);
impl core::ops::Deref for IEnumSubscription {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSubscription, windows_core::IUnknown);
impl IEnumSubscription {
    pub unsafe fn Next(&self, rgelt: &mut [windows_core::GUID], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSubscription> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEnumSubscription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IErrorLookup, IErrorLookup_Vtbl, 0x0c733a66_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IErrorLookup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IErrorLookup, windows_core::IUnknown);
impl IErrorLookup {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetErrorDescription(&self, hrerror: windows_core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: Option<*mut windows_core::BSTR>, pbstrdescription: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), hrerror, dwlookupid, pdispparams, lcid, core::mem::transmute(pbstrsource.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pbstrdescription.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetHelpInfo(&self, hrerror: windows_core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut windows_core::BSTR, pdwhelpcontext: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHelpInfo)(windows_core::Interface::as_raw(self), hrerror, dwlookupid, lcid, core::mem::transmute(pbstrhelpfile), pdwhelpcontext).ok()
    }
    pub unsafe fn ReleaseErrors(&self, dwdynamicerrorid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseErrors)(windows_core::Interface::as_raw(self), dwdynamicerrorid).ok()
    }
}
#[repr(C)]
pub struct IErrorLookup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, *const super::Com::DISPPARAMS, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetErrorDescription: usize,
    pub GetHelpInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32) -> windows_core::HRESULT,
    pub ReleaseErrors: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IErrorRecords, IErrorRecords_Vtbl, 0x0c733a67_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IErrorRecords {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IErrorRecords, windows_core::IUnknown);
impl IErrorRecords {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddErrorRecord<P0>(&self, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: Option<*const super::Com::DISPPARAMS>, punkcustomerror: P0, dwdynamicerrorid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).AddErrorRecord)(windows_core::Interface::as_raw(self), perrorinfo, dwlookupid, core::mem::transmute(pdispparams.unwrap_or(std::ptr::null())), punkcustomerror.param().abi(), dwdynamicerrorid).ok()
    }
    pub unsafe fn GetBasicErrorInfo(&self, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBasicErrorInfo)(windows_core::Interface::as_raw(self), ulrecordnum, perrorinfo).ok()
    }
    pub unsafe fn GetCustomErrorObject(&self, ulrecordnum: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomErrorObject)(windows_core::Interface::as_raw(self), ulrecordnum, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetErrorInfo(&self, ulrecordnum: u32, lcid: u32) -> windows_core::Result<super::Com::IErrorInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorInfo)(windows_core::Interface::as_raw(self), ulrecordnum, lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetErrorParameters(&self, ulrecordnum: u32) -> windows_core::Result<super::Com::DISPPARAMS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorParameters)(windows_core::Interface::as_raw(self), ulrecordnum, &mut result__).map(|| result__)
    }
    pub unsafe fn GetRecordCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecordCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IErrorRecords_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddErrorRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const ERRORINFO, u32, *const super::Com::DISPPARAMS, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddErrorRecord: usize,
    pub GetBasicErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ERRORINFO) -> windows_core::HRESULT,
    pub GetCustomErrorObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetErrorInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetErrorParameters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::Com::DISPPARAMS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetErrorParameters: usize,
    pub GetRecordCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetDataSource, IGetDataSource_Vtbl, 0x0c733a75_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IGetDataSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetDataSource, windows_core::IUnknown);
impl IGetDataSource {
    pub unsafe fn GetDataSource(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataSource)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IGetDataSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDataSource: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetRow, IGetRow_Vtbl, 0x0c733aaf_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IGetRow {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetRow, windows_core::IUnknown);
impl IGetRow {
    pub unsafe fn GetRowFromHROW<P0>(&self, punkouter: P0, hrow: usize, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRowFromHROW)(windows_core::Interface::as_raw(self), punkouter.param().abi(), hrow, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetURLFromHROW(&self, hrow: usize) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetURLFromHROW)(windows_core::Interface::as_raw(self), hrow, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IGetRow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRowFromHROW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetURLFromHROW: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetSession, IGetSession_Vtbl, 0x0c733aba_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IGetSession {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetSession, windows_core::IUnknown);
impl IGetSession {
    pub unsafe fn GetSession(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSession)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IGetSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetSourceRow, IGetSourceRow_Vtbl, 0x0c733abb_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IGetSourceRow {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetSourceRow, windows_core::IUnknown);
impl IGetSourceRow {
    pub unsafe fn GetSourceRow(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceRow)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IGetSourceRow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSourceRow: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIndexDefinition, IIndexDefinition_Vtbl, 0x0c733a68_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IIndexDefinition {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IIndexDefinition, windows_core::IUnknown);
impl IIndexDefinition {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn CreateIndex(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: Option<*const super::super::Storage::IndexServer::DBID>, rgindexcolumndescs: &[DBINDEXCOLUMNDESC], rgpropertysets: &mut [DBPROPSET], ppindexid: Option<*mut *mut super::super::Storage::IndexServer::DBID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateIndex)(windows_core::Interface::as_raw(self), ptableid, core::mem::transmute(pindexid.unwrap_or(std::ptr::null())), rgindexcolumndescs.len().try_into().unwrap(), core::mem::transmute(rgindexcolumndescs.as_ptr()), rgpropertysets.len().try_into().unwrap(), core::mem::transmute(rgpropertysets.as_ptr()), core::mem::transmute(ppindexid.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn DropIndex(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: Option<*const super::super::Storage::IndexServer::DBID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DropIndex)(windows_core::Interface::as_raw(self), ptableid, core::mem::transmute(pindexid.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IIndexDefinition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub CreateIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID, usize, *const DBINDEXCOLUMNDESC, u32, *mut DBPROPSET, *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    CreateIndex: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DropIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DropIndex: usize,
}
windows_core::imp::define_interface!(IInterval, IInterval_Vtbl, 0x6bf0a714_3c18_430b_8b5d_83b1c234d3db);
impl core::ops::Deref for IInterval {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInterval, windows_core::IUnknown);
impl IInterval {
    pub unsafe fn GetLimits(&self, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut windows_core::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLimits)(windows_core::Interface::as_raw(self), pilklower, core::mem::transmute(ppropvarlower), pilkupper, core::mem::transmute(ppropvarupper)).ok()
    }
}
#[repr(C)]
pub struct IInterval_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLimits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut INTERVAL_LIMIT_KIND, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut INTERVAL_LIMIT_KIND, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoadFilter, ILoadFilter_Vtbl, 0xc7310722_ac80_11d1_8df3_00c04fb6ef4f);
impl core::ops::Deref for ILoadFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILoadFilter, windows_core::IUnknown);
impl ILoadFilter {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn LoadIFilter<P0, P1, P2>(&self, pwcspath: P0, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: P1, fusedefault: P2, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut Option<super::super::Storage::IndexServer::IFilter>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).LoadIFilter)(windows_core::Interface::as_raw(self), pwcspath.param().abi(), pfilteredsources, punkouter.param().abi(), fusedefault.param().abi(), pfilterclsid, searchdecsize, pwcssearchdesc, core::mem::transmute(ppifilt)).ok()
    }
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn LoadIFilterFromStorage<P0, P1, P2, P3>(&self, pstg: P0, punkouter: P1, pwcsoverride: P2, fusedefault: P3, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut Option<super::super::Storage::IndexServer::IFilter>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::StructuredStorage::IStorage>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).LoadIFilterFromStorage)(windows_core::Interface::as_raw(self), pstg.param().abi(), punkouter.param().abi(), pwcsoverride.param().abi(), fusedefault.param().abi(), pfilterclsid, searchdecsize, pwcssearchdesc, core::mem::transmute(ppifilt)).ok()
    }
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn LoadIFilterFromStream<P0, P1, P2>(&self, pstm: P0, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: P1, fusedefault: P2, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut Option<super::super::Storage::IndexServer::IFilter>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IStream>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).LoadIFilterFromStream)(windows_core::Interface::as_raw(self), pstm.param().abi(), pfilteredsources, punkouter.param().abi(), fusedefault.param().abi(), pfilterclsid, searchdecsize, pwcssearchdesc, core::mem::transmute(ppifilt)).ok()
    }
}
#[repr(C)]
pub struct ILoadFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub LoadIFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const FILTERED_DATA_SOURCES, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut windows_core::GUID, *mut i32, *mut *mut u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    LoadIFilter: usize,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
    pub LoadIFilterFromStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL, *mut windows_core::GUID, *mut i32, *mut *mut u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage")))]
    LoadIFilterFromStorage: usize,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub LoadIFilterFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const FILTERED_DATA_SOURCES, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut windows_core::GUID, *mut i32, *mut *mut u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    LoadIFilterFromStream: usize,
}
windows_core::imp::define_interface!(ILoadFilterWithPrivateComActivation, ILoadFilterWithPrivateComActivation_Vtbl, 0x40bdbd34_780b_48d3_9bb6_12ebd4ad2e75);
impl core::ops::Deref for ILoadFilterWithPrivateComActivation {
    type Target = ILoadFilter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILoadFilterWithPrivateComActivation, windows_core::IUnknown, ILoadFilter);
impl ILoadFilterWithPrivateComActivation {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn LoadIFilterWithPrivateComActivation<P0>(&self, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: P0, filterclsid: *mut windows_core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut Option<super::super::Storage::IndexServer::IFilter>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).LoadIFilterWithPrivateComActivation)(windows_core::Interface::as_raw(self), filteredsources, usedefault.param().abi(), filterclsid, isfilterprivatecomactivated, core::mem::transmute(filterobj)).ok()
    }
}
#[repr(C)]
pub struct ILoadFilterWithPrivateComActivation_Vtbl {
    pub base__: ILoadFilter_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub LoadIFilterWithPrivateComActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *const FILTERED_DATA_SOURCES, super::super::Foundation::BOOL, *mut windows_core::GUID, *mut super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    LoadIFilterWithPrivateComActivation: usize,
}
windows_core::imp::define_interface!(IMDDataset, IMDDataset_Vtbl, 0xa07cccd1_8148_11d0_87bb_00c04fc33942);
impl core::ops::Deref for IMDDataset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDDataset, windows_core::IUnknown);
impl IMDDataset {
    pub unsafe fn FreeAxisInfo(&self, rgaxisinfo: &[MDAXISINFO]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeAxisInfo)(windows_core::Interface::as_raw(self), rgaxisinfo.len().try_into().unwrap(), core::mem::transmute(rgaxisinfo.as_ptr())).ok()
    }
    pub unsafe fn GetAxisInfo(&self, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAxisInfo)(windows_core::Interface::as_raw(self), pcaxes, prgaxisinfo).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetAxisRowset<P0>(&self, punkouter: P0, iaxis: usize, riid: *const windows_core::GUID, rgpropertysets: &mut [DBPROPSET], pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetAxisRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), iaxis, riid, rgpropertysets.len().try_into().unwrap(), core::mem::transmute(rgpropertysets.as_ptr()), core::mem::transmute(pprowset)).ok()
    }
    pub unsafe fn GetCellData<P0>(&self, haccessor: P0, ulstartcell: usize, ulendcell: usize, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).GetCellData)(windows_core::Interface::as_raw(self), haccessor.param().abi(), ulstartcell, ulendcell, pdata).ok()
    }
    pub unsafe fn GetSpecification(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSpecification)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMDDataset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FreeAxisInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const MDAXISINFO) -> windows_core::HRESULT,
    pub GetAxisInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut *mut MDAXISINFO) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetAxisRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetAxisRowset: usize,
    pub GetCellData: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, usize, usize, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpecification: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDFind, IMDFind_Vtbl, 0xa07cccd2_8148_11d0_87bb_00c04fc33942);
impl core::ops::Deref for IMDFind {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDFind, windows_core::IUnknown);
impl IMDFind {
    pub unsafe fn FindCell(&self, ulstartingordinal: usize, rgpwszmember: &[windows_core::PCWSTR]) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindCell)(windows_core::Interface::as_raw(self), ulstartingordinal, rgpwszmember.len().try_into().unwrap(), core::mem::transmute(rgpwszmember.as_ptr()), &mut result__).map(|| result__)
    }
    pub unsafe fn FindTuple(&self, ulaxisidentifier: u32, ulstartingordinal: usize, rgpwszmember: &[windows_core::PCWSTR]) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindTuple)(windows_core::Interface::as_raw(self), ulaxisidentifier, ulstartingordinal, rgpwszmember.len().try_into().unwrap(), core::mem::transmute(rgpwszmember.as_ptr()), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMDFind_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindCell: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const windows_core::PCWSTR, *mut usize) -> windows_core::HRESULT,
    pub FindTuple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, usize, *const windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDRangeRowset, IMDRangeRowset_Vtbl, 0x0c733aa0_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IMDRangeRowset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDRangeRowset, windows_core::IUnknown);
impl IMDRangeRowset {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetRangeRowset<P0>(&self, punkouter: P0, ulstartcell: usize, ulendcell: usize, riid: *const windows_core::GUID, rgpropertysets: &mut [DBPROPSET], pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetRangeRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), ulstartcell, ulendcell, riid, rgpropertysets.len().try_into().unwrap(), core::mem::transmute(rgpropertysets.as_ptr()), core::mem::transmute(pprowset)).ok()
    }
}
#[repr(C)]
pub struct IMDRangeRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetRangeRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, usize, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetRangeRowset: usize,
}
windows_core::imp::define_interface!(IMetaData, IMetaData_Vtbl, 0x780102b0_c43b_4876_bc7b_5e9ba5c88794);
impl core::ops::Deref for IMetaData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaData, windows_core::IUnknown);
impl IMetaData {
    pub unsafe fn GetData(&self, ppszkey: Option<*mut windows_core::PWSTR>, ppszvalue: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszkey.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppszvalue.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IMetaData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultipleResults, IMultipleResults_Vtbl, 0x0c733a90_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IMultipleResults {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultipleResults, windows_core::IUnknown);
impl IMultipleResults {
    pub unsafe fn GetResult<P0>(&self, punkouter: P0, lresultflag: isize, riid: *const windows_core::GUID, pcrowsaffected: Option<*mut isize>, pprowset: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self), punkouter.param().abi(), lresultflag, riid, core::mem::transmute(pcrowsaffected.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pprowset.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IMultipleResults_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, *const windows_core::GUID, *mut isize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INamedEntity, INamedEntity_Vtbl, 0xabdbd0b1_7d54_49fb_ab5c_bff4130004cd);
impl core::ops::Deref for INamedEntity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INamedEntity, windows_core::IUnknown);
impl INamedEntity {
    pub unsafe fn GetValue(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DefaultPhrase(&self, ppszphrase: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DefaultPhrase)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszphrase.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct INamedEntity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub DefaultPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INamedEntityCollector, INamedEntityCollector_Vtbl, 0xaf2440f6_8afc_47d0_9a7f_396a0acfb43d);
impl core::ops::Deref for INamedEntityCollector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INamedEntityCollector, windows_core::IUnknown);
impl INamedEntityCollector {
    pub unsafe fn Add<P0, P1>(&self, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: P0, pszvalue: P1, certainty: NAMED_ENTITY_CERTAINTY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IEntity>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), beginspan, endspan, beginactual, endactual, ptype.param().abi(), pszvalue.param().abi(), certainty).ok()
    }
}
#[repr(C)]
pub struct INamedEntityCollector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void, windows_core::PCWSTR, NAMED_ENTITY_CERTAINTY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectAccessControl, IObjectAccessControl_Vtbl, 0x0c733aa3_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IObjectAccessControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectAccessControl, windows_core::IUnknown);
impl IObjectAccessControl {
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub unsafe fn GetObjectAccessRights(&self, pobject: *const SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectAccessRights)(windows_core::Interface::as_raw(self), pobject, pcaccessentries, prgaccessentries).ok()
    }
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub unsafe fn GetObjectOwner(&self, pobject: *const SEC_OBJECT) -> windows_core::Result<*mut super::super::Security::Authorization::TRUSTEE_W> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectOwner)(windows_core::Interface::as_raw(self), pobject, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub unsafe fn IsObjectAccessAllowed(&self, pobject: *const SEC_OBJECT, paccessentry: *const super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsObjectAccessAllowed)(windows_core::Interface::as_raw(self), pobject, paccessentry, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub unsafe fn SetObjectAccessRights(&self, pobject: *const SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetObjectAccessRights)(windows_core::Interface::as_raw(self), pobject, caccessentries, prgaccessentries).ok()
    }
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub unsafe fn SetObjectOwner(&self, pobject: *const SEC_OBJECT, powner: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetObjectOwner)(windows_core::Interface::as_raw(self), pobject, powner).ok()
    }
}
#[repr(C)]
pub struct IObjectAccessControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub GetObjectAccessRights: unsafe extern "system" fn(*mut core::ffi::c_void, *const SEC_OBJECT, *mut u32, *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    GetObjectAccessRights: usize,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub GetObjectOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *const SEC_OBJECT, *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    GetObjectOwner: usize,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub IsObjectAccessAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *const SEC_OBJECT, *const super::super::Security::Authorization::EXPLICIT_ACCESS_W, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    IsObjectAccessAllowed: usize,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub SetObjectAccessRights: unsafe extern "system" fn(*mut core::ffi::c_void, *const SEC_OBJECT, u32, *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    SetObjectAccessRights: usize,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub SetObjectOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *const SEC_OBJECT, *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    SetObjectOwner: usize,
}
windows_core::imp::define_interface!(IOpLockStatus, IOpLockStatus_Vtbl, 0xc731065d_ac80_11d1_8df3_00c04fb6ef4f);
impl core::ops::Deref for IOpLockStatus {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpLockStatus, windows_core::IUnknown);
impl IOpLockStatus {
    pub unsafe fn IsOplockValid(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOplockValid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsOplockBroken(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOplockBroken)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOplockEventHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOplockEventHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOpLockStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsOplockValid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsOplockBroken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetOplockEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpenRowset, IOpenRowset_Vtbl, 0x0c733a69_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IOpenRowset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpenRowset, windows_core::IUnknown);
impl IOpenRowset {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn OpenRowset<P0>(&self, punkouter: P0, ptableid: Option<*const super::super::Storage::IndexServer::DBID>, pindexid: Option<*const super::super::Storage::IndexServer::DBID>, riid: *const windows_core::GUID, rgpropertysets: Option<&mut [DBPROPSET]>, pprowset: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OpenRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), core::mem::transmute(ptableid.unwrap_or(std::ptr::null())), core::mem::transmute(pindexid.unwrap_or(std::ptr::null())), riid, rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertysets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(pprowset.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IOpenRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub OpenRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    OpenRowset: usize,
}
windows_core::imp::define_interface!(IParentRowset, IParentRowset_Vtbl, 0x0c733aaa_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IParentRowset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IParentRowset, windows_core::IUnknown);
impl IParentRowset {
    pub unsafe fn GetChildRowset<P0>(&self, punkouter: P0, iordinal: usize, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChildRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), iordinal, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IParentRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChildRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProtocolHandlerSite, IProtocolHandlerSite_Vtbl, 0x0b63e385_9ccc_11d0_bcdb_00805fccce04);
impl core::ops::Deref for IProtocolHandlerSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProtocolHandlerSite, windows_core::IUnknown);
impl IProtocolHandlerSite {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetFilter<P0, P1>(&self, pclsidobj: *const windows_core::GUID, pcwszcontenttype: P0, pcwszextension: P1) -> windows_core::Result<super::super::Storage::IndexServer::IFilter>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilter)(windows_core::Interface::as_raw(self), pclsidobj, pcwszcontenttype.param().abi(), pcwszextension.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IProtocolHandlerSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetFilter: usize,
}
windows_core::imp::define_interface!(IProvideMoniker, IProvideMoniker_Vtbl, 0x0c733a4d_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IProvideMoniker {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProvideMoniker, windows_core::IUnknown);
impl IProvideMoniker {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMoniker(&self) -> windows_core::Result<super::Com::IMoniker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMoniker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IProvideMoniker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMoniker: usize,
}
windows_core::imp::define_interface!(IQueryParser, IQueryParser_Vtbl, 0x2ebdee67_3505_43f8_9946_ea44abc8e5b0);
impl core::ops::Deref for IQueryParser {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IQueryParser, windows_core::IUnknown);
impl IQueryParser {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parse<P0, P1>(&self, pszinputstring: P0, pcustomproperties: P1) -> windows_core::Result<IQuerySolution>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::Com::IEnumUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Parse)(windows_core::Interface::as_raw(self), pszinputstring.param().abi(), pcustomproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOption(&self, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), option, core::mem::transmute(poptionvalue)).ok()
    }
    pub unsafe fn GetOption(&self, option: STRUCTURED_QUERY_SINGLE_OPTION) -> windows_core::Result<windows_core::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOption)(windows_core::Interface::as_raw(self), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMultiOption<P0>(&self, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: P0, poptionvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetMultiOption)(windows_core::Interface::as_raw(self), option, pszoptionkey.param().abi(), core::mem::transmute(poptionvalue)).ok()
    }
    pub unsafe fn GetSchemaProvider(&self) -> windows_core::Result<ISchemaProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSchemaProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RestateToString<P0, P1>(&self, pcondition: P0, fuseenglish: P1) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<ICondition>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RestateToString)(windows_core::Interface::as_raw(self), pcondition.param().abi(), fuseenglish.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn ParsePropertyValue<P0, P1>(&self, pszpropertyname: P0, pszinputstring: P1) -> windows_core::Result<IQuerySolution>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParsePropertyValue)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), pszinputstring.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RestatePropertyValueToString<P0, P1>(&self, pcondition: P0, fuseenglish: P1, ppszpropertyname: *mut windows_core::PWSTR, ppszquerystring: *mut windows_core::PWSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICondition>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RestatePropertyValueToString)(windows_core::Interface::as_raw(self), pcondition.param().abi(), fuseenglish.param().abi(), ppszpropertyname, ppszquerystring).ok()
    }
}
#[repr(C)]
pub struct IQueryParser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parse: usize,
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, STRUCTURED_QUERY_SINGLE_OPTION, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetOption: unsafe extern "system" fn(*mut core::ffi::c_void, STRUCTURED_QUERY_SINGLE_OPTION, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub SetMultiOption: unsafe extern "system" fn(*mut core::ffi::c_void, STRUCTURED_QUERY_MULTIOPTION, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetSchemaProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RestateToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RestateToString: usize,
    pub ParsePropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RestatePropertyValueToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RestatePropertyValueToString: usize,
}
windows_core::imp::define_interface!(IQueryParserManager, IQueryParserManager_Vtbl, 0xa879e3c4_af77_44fb_8f37_ebd1487cf920);
impl core::ops::Deref for IQueryParserManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IQueryParserManager, windows_core::IUnknown);
impl IQueryParserManager {
    pub unsafe fn CreateLoadedParser<P0, T>(&self, pszcatalog: P0, langidforkeywords: u16) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateLoadedParser)(windows_core::Interface::as_raw(self), pszcatalog.param().abi(), langidforkeywords, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InitializeOptions<P0, P1, P2>(&self, funderstandnqs: P0, fautowildcard: P1, pqueryparser: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<IQueryParser>,
    {
        (windows_core::Interface::vtable(self).InitializeOptions)(windows_core::Interface::as_raw(self), funderstandnqs.param().abi(), fautowildcard.param().abi(), pqueryparser.param().abi()).ok()
    }
    pub unsafe fn SetOption(&self, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), option, core::mem::transmute(poptionvalue)).ok()
    }
}
#[repr(C)]
pub struct IQueryParserManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateLoadedParser: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u16, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeOptions: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, QUERY_PARSER_MANAGER_OPTION, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IQuerySolution, IQuerySolution_Vtbl, 0xd6ebc66b_8921_4193_afdd_a1789fb7ff57);
impl core::ops::Deref for IQuerySolution {
    type Target = IConditionFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IQuerySolution, windows_core::IUnknown, IConditionFactory);
impl IQuerySolution {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetQuery(&self, ppquerynode: Option<*mut Option<ICondition>>, ppmaintype: Option<*mut Option<IEntity>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQuery)(windows_core::Interface::as_raw(self), core::mem::transmute(ppquerynode.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppmaintype.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetErrors<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetErrors)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLexicalData(&self, ppszinputstring: Option<*mut windows_core::PWSTR>, pptokens: Option<*mut Option<ITokenCollection>>, plcid: Option<*mut u32>, ppwordbreaker: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLexicalData)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszinputstring.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pptokens.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plcid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppwordbreaker.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IQuerySolution_Vtbl {
    pub base__: IConditionFactory_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetQuery: usize,
    pub GetErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLexicalData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReadData, IReadData_Vtbl, 0x0c733a6a_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IReadData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IReadData, windows_core::IUnknown);
impl IReadData {
    pub unsafe fn ReadData<P0>(&self, hchapter: usize, pbookmark: &[u8], lrowsoffset: isize, haccessor: P0, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).ReadData)(windows_core::Interface::as_raw(self), hchapter, pbookmark.len().try_into().unwrap(), core::mem::transmute(pbookmark.as_ptr()), lrowsoffset, haccessor.param().abi(), crows, pcrowsobtained, ppfixeddata, pcbvariabletotal, ppvariabledata).ok()
    }
    pub unsafe fn ReleaseChapter(&self, hchapter: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseChapter)(windows_core::Interface::as_raw(self), hchapter).ok()
    }
}
#[repr(C)]
pub struct IReadData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReadData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const u8, isize, HACCESSOR, isize, *mut usize, *mut *mut u8, *mut usize, *mut *mut u8) -> windows_core::HRESULT,
    pub ReleaseChapter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRegisterProvider, IRegisterProvider_Vtbl, 0x0c733ab9_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRegisterProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRegisterProvider, windows_core::IUnknown);
impl IRegisterProvider {
    pub unsafe fn GetURLMapping<P0>(&self, pwszurl: P0, dwreserved: usize) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetURLMapping)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwreserved, &mut result__).map(|| result__)
    }
    pub unsafe fn SetURLMapping<P0>(&self, pwszurl: P0, dwreserved: usize, rclsidprovider: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetURLMapping)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwreserved, rclsidprovider).ok()
    }
    pub unsafe fn UnregisterProvider<P0>(&self, pwszurl: P0, dwreserved: usize, rclsidprovider: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterProvider)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwreserved, rclsidprovider).ok()
    }
}
#[repr(C)]
pub struct IRegisterProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetURLMapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, usize, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetURLMapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, usize, *const windows_core::GUID) -> windows_core::HRESULT,
    pub UnregisterProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, usize, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRelationship, IRelationship_Vtbl, 0x2769280b_5108_498c_9c7f_a51239b63147);
impl core::ops::Deref for IRelationship {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRelationship, windows_core::IUnknown);
impl IRelationship {
    pub unsafe fn Name(&self, ppszname: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszname.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn IsReal(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsReal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Destination(&self) -> windows_core::Result<IEntity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Destination)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MetaData<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).MetaData)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DefaultPhrase(&self, ppszphrase: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DefaultPhrase)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszphrase.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IRelationship_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsReal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Destination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DefaultPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRichChunk, IRichChunk_Vtbl, 0x4fdef69c_dbc9_454e_9910_b34f3c64b510);
impl core::ops::Deref for IRichChunk {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRichChunk, windows_core::IUnknown);
impl IRichChunk {
    pub unsafe fn GetData(&self, pfirstpos: Option<*mut u32>, plength: Option<*mut u32>, ppsz: Option<*mut windows_core::PWSTR>, pvalue: Option<*mut windows_core::PROPVARIANT>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), core::mem::transmute(pfirstpos.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppsz.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvalue.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IRichChunk_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut windows_core::PWSTR, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRow, IRow_Vtbl, 0x0c733ab4_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRow {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRow, windows_core::IUnknown);
impl IRow {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetColumns(&self, rgcolumns: &mut [DBCOLUMNACCESS]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumns)(windows_core::Interface::as_raw(self), rgcolumns.len().try_into().unwrap(), core::mem::transmute(rgcolumns.as_ptr())).ok()
    }
    pub unsafe fn GetSourceRowset(&self, riid: *const windows_core::GUID, pprowset: Option<*mut Option<windows_core::IUnknown>>, phrow: Option<*mut usize>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSourceRowset)(windows_core::Interface::as_raw(self), riid, core::mem::transmute(pprowset.unwrap_or(std::ptr::null_mut())), core::mem::transmute(phrow.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn Open<P0>(&self, punkouter: P0, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const windows_core::GUID, dwbindflags: u32, riid: *const windows_core::GUID, ppunk: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), punkouter.param().abi(), pcolumnid, rguidcolumntype, dwbindflags, riid, core::mem::transmute(ppunk.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IRow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetColumns: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut DBCOLUMNACCESS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetColumns: usize,
    pub GetSourceRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const windows_core::GUID, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    Open: usize,
}
windows_core::imp::define_interface!(IRowChange, IRowChange_Vtbl, 0x0c733ab5_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowChange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowChange, windows_core::IUnknown);
impl IRowChange {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn SetColumns(&self, rgcolumns: &[DBCOLUMNACCESS]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColumns)(windows_core::Interface::as_raw(self), rgcolumns.len().try_into().unwrap(), core::mem::transmute(rgcolumns.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IRowChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub SetColumns: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const DBCOLUMNACCESS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    SetColumns: usize,
}
windows_core::imp::define_interface!(IRowPosition, IRowPosition_Vtbl, 0x0c733a94_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowPosition {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowPosition, windows_core::IUnknown);
impl IRowPosition {
    pub unsafe fn ClearRowPosition(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearRowPosition)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRowPosition(&self, phchapter: Option<*mut usize>, phrow: *mut usize, pdwpositionflags: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRowPosition)(windows_core::Interface::as_raw(self), core::mem::transmute(phchapter.unwrap_or(std::ptr::null_mut())), phrow, core::mem::transmute(pdwpositionflags.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetRowset(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRowset)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Initialize<P0>(&self, prowset: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), prowset.param().abi()).ok()
    }
    pub unsafe fn SetRowPosition(&self, hchapter: usize, hrow: usize, dwpositionflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRowPosition)(windows_core::Interface::as_raw(self), hchapter, hrow, dwpositionflags).ok()
    }
}
#[repr(C)]
pub struct IRowPosition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ClearRowPosition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRowPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut usize, *mut u32) -> windows_core::HRESULT,
    pub GetRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRowPosition: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowPositionChange, IRowPositionChange_Vtbl, 0x0997a571_126e_11d0_9f8a_00a0c9a0631e);
impl core::ops::Deref for IRowPositionChange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowPositionChange, windows_core::IUnknown);
impl IRowPositionChange {
    pub unsafe fn OnRowPositionChange<P0>(&self, ereason: u32, ephase: u32, fcantdeny: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnRowPositionChange)(windows_core::Interface::as_raw(self), ereason, ephase, fcantdeny.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRowPositionChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnRowPositionChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowSchemaChange, IRowSchemaChange_Vtbl, 0x0c733aae_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowSchemaChange {
    type Target = IRowChange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowSchemaChange, windows_core::IUnknown, IRowChange);
impl IRowSchemaChange {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn DeleteColumns(&self, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteColumns)(windows_core::Interface::as_raw(self), ccolumns, rgcolumnids, rgdwstatus).ok()
    }
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn AddColumns(&self, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddColumns)(windows_core::Interface::as_raw(self), ccolumns, rgnewcolumninfo, rgcolumns).ok()
    }
}
#[repr(C)]
pub struct IRowSchemaChange_Vtbl {
    pub base__: IRowChange_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DeleteColumns: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const super::super::Storage::IndexServer::DBID, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DeleteColumns: usize,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub AddColumns: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const DBCOLUMNINFO, *mut DBCOLUMNACCESS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    AddColumns: usize,
}
windows_core::imp::define_interface!(IRowset, IRowset_Vtbl, 0x0c733a7c_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowset, windows_core::IUnknown);
impl IRowset {
    pub unsafe fn AddRefRows(&self, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddRefRows)(windows_core::Interface::as_raw(self), crows, rghrows, rgrefcounts, rgrowstatus).ok()
    }
    pub unsafe fn GetData<P0>(&self, hrow: usize, haccessor: P0, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), hrow, haccessor.param().abi(), pdata).ok()
    }
    pub unsafe fn GetNextRows(&self, hreserved: usize, lrowsoffset: isize, pcrowsobtained: *mut usize, prghrows: &mut [*mut usize]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextRows)(windows_core::Interface::as_raw(self), hreserved, lrowsoffset, prghrows.len().try_into().unwrap(), pcrowsobtained, core::mem::transmute(prghrows.as_ptr())).ok()
    }
    pub unsafe fn ReleaseRows(&self, crows: usize, rghrows: *const usize, rgrowoptions: *const u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseRows)(windows_core::Interface::as_raw(self), crows, rghrows, rgrowoptions, rgrefcounts, rgrowstatus).ok()
    }
    pub unsafe fn RestartPosition(&self, hreserved: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestartPosition)(windows_core::Interface::as_raw(self), hreserved).ok()
    }
}
#[repr(C)]
pub struct IRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRefRows: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const usize, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, HACCESSOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextRows: unsafe extern "system" fn(*mut core::ffi::c_void, usize, isize, isize, *mut usize, *mut *mut usize) -> windows_core::HRESULT,
    pub ReleaseRows: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const usize, *const u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub RestartPosition: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetAsynch, IRowsetAsynch_Vtbl, 0x0c733a0f_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetAsynch {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetAsynch, windows_core::IUnknown);
impl IRowsetAsynch {
    pub unsafe fn RatioFinished(&self, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RatioFinished)(windows_core::Interface::as_raw(self), puldenominator, pulnumerator, pcrows, pfnewrows).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRowsetAsynch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RatioFinished: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut usize, *mut usize, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetBookmark, IRowsetBookmark_Vtbl, 0x0c733ac2_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetBookmark {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetBookmark, windows_core::IUnknown);
impl IRowsetBookmark {
    pub unsafe fn PositionOnBookmark(&self, hchapter: usize, pbookmark: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PositionOnBookmark)(windows_core::Interface::as_raw(self), hchapter, pbookmark.len().try_into().unwrap(), core::mem::transmute(pbookmark.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IRowsetBookmark_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PositionOnBookmark: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetChange, IRowsetChange_Vtbl, 0x0c733a05_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetChange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetChange, windows_core::IUnknown);
impl IRowsetChange {
    pub unsafe fn DeleteRows(&self, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteRows)(windows_core::Interface::as_raw(self), hreserved, crows, rghrows, rgrowstatus).ok()
    }
    pub unsafe fn SetData<P0>(&self, hrow: usize, haccessor: P0, pdata: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), hrow, haccessor.param().abi(), pdata).ok()
    }
    pub unsafe fn InsertRow<P0>(&self, hreserved: usize, haccessor: P0, pdata: *const core::ffi::c_void) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InsertRow)(windows_core::Interface::as_raw(self), hreserved, haccessor.param().abi(), pdata, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRowsetChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeleteRows: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const usize, *mut u32) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, HACCESSOR, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertRow: unsafe extern "system" fn(*mut core::ffi::c_void, usize, HACCESSOR, *const core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetChangeExtInfo, IRowsetChangeExtInfo_Vtbl, 0x0c733a8f_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetChangeExtInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetChangeExtInfo, windows_core::IUnknown);
impl IRowsetChangeExtInfo {
    pub unsafe fn GetOriginalRow(&self, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOriginalRow)(windows_core::Interface::as_raw(self), hreserved, hrow, phroworiginal).ok()
    }
    pub unsafe fn GetPendingColumns(&self, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPendingColumns)(windows_core::Interface::as_raw(self), hreserved, hrow, ccolumnordinals, rgiordinals, rgcolumnstatus).ok()
    }
}
#[repr(C)]
pub struct IRowsetChangeExtInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOriginalRow: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut usize) -> windows_core::HRESULT,
    pub GetPendingColumns: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *const u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetChapterMember, IRowsetChapterMember_Vtbl, 0x0c733aa8_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetChapterMember {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetChapterMember, windows_core::IUnknown);
impl IRowsetChapterMember {
    pub unsafe fn IsRowInChapter(&self, hchapter: usize, hrow: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsRowInChapter)(windows_core::Interface::as_raw(self), hchapter, hrow).ok()
    }
}
#[repr(C)]
pub struct IRowsetChapterMember_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsRowInChapter: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetCopyRows, IRowsetCopyRows_Vtbl, 0x0c733a6b_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetCopyRows {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetCopyRows, windows_core::IUnknown);
impl IRowsetCopyRows {
    pub unsafe fn CloseSource(&self, hsourceid: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CloseSource)(windows_core::Interface::as_raw(self), hsourceid).ok()
    }
    pub unsafe fn CopyByHROWS(&self, hsourceid: u16, hreserved: usize, rghrows: &[usize], bflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CopyByHROWS)(windows_core::Interface::as_raw(self), hsourceid, hreserved, rghrows.len().try_into().unwrap(), core::mem::transmute(rghrows.as_ptr()), bflags).ok()
    }
    pub unsafe fn CopyRows(&self, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CopyRows)(windows_core::Interface::as_raw(self), hsourceid, hreserved, crows, bflags, &mut result__).map(|| result__)
    }
    pub unsafe fn DefineSource<P0>(&self, prowsetsource: P0, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize) -> windows_core::Result<u16>
    where
        P0: windows_core::Param<IRowset>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefineSource)(windows_core::Interface::as_raw(self), prowsetsource.param().abi(), ccolids, rgsourcecolumns, rgtargetcolumns, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRowsetCopyRows_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CloseSource: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub CopyByHROWS: unsafe extern "system" fn(*mut core::ffi::c_void, u16, usize, isize, *const usize, u32) -> windows_core::HRESULT,
    pub CopyRows: unsafe extern "system" fn(*mut core::ffi::c_void, u16, usize, isize, u32, *mut usize) -> windows_core::HRESULT,
    pub DefineSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, *const isize, *const isize, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetCurrentIndex, IRowsetCurrentIndex_Vtbl, 0x0c733abd_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetCurrentIndex {
    type Target = IRowsetIndex;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetCurrentIndex, windows_core::IUnknown, IRowsetIndex);
impl IRowsetCurrentIndex {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetIndex(&self) -> windows_core::Result<*mut super::super::Storage::IndexServer::DBID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn SetIndex(&self, pindexid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIndex)(windows_core::Interface::as_raw(self), pindexid).ok()
    }
}
#[repr(C)]
pub struct IRowsetCurrentIndex_Vtbl {
    pub base__: IRowsetIndex_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetIndex: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub SetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    SetIndex: usize,
}
windows_core::imp::define_interface!(IRowsetEvents, IRowsetEvents_Vtbl, 0x1551aea5_5d66_4b11_86f5_d5634cb211b9);
impl core::ops::Deref for IRowsetEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetEvents, windows_core::IUnknown);
impl IRowsetEvents {
    pub unsafe fn OnNewItem(&self, itemid: *const windows_core::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnNewItem)(windows_core::Interface::as_raw(self), core::mem::transmute(itemid), newitemstate).ok()
    }
    pub unsafe fn OnChangedItem(&self, itemid: *const windows_core::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnChangedItem)(windows_core::Interface::as_raw(self), core::mem::transmute(itemid), rowsetitemstate, changeditemstate).ok()
    }
    pub unsafe fn OnDeletedItem(&self, itemid: *const windows_core::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDeletedItem)(windows_core::Interface::as_raw(self), core::mem::transmute(itemid), deleteditemstate).ok()
    }
    pub unsafe fn OnRowsetEvent(&self, eventtype: ROWSETEVENT_TYPE, eventdata: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnRowsetEvent)(windows_core::Interface::as_raw(self), eventtype, core::mem::transmute(eventdata)).ok()
    }
}
#[repr(C)]
pub struct IRowsetEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNewItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT,
    pub OnChangedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, ROWSETEVENT_ITEMSTATE, ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT,
    pub OnDeletedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT,
    pub OnRowsetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, ROWSETEVENT_TYPE, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetExactScroll, IRowsetExactScroll_Vtbl, 0x0c733a7f_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetExactScroll {
    type Target = IRowsetScroll;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetExactScroll, windows_core::IUnknown, IRowset, IRowsetLocate, IRowsetScroll);
impl IRowsetExactScroll {
    pub unsafe fn GetExactPosition(&self, hchapter: usize, pbookmark: &[u8], pulposition: *mut usize, pcrows: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetExactPosition)(windows_core::Interface::as_raw(self), hchapter, pbookmark.len().try_into().unwrap(), core::mem::transmute(pbookmark.as_ptr()), pulposition, pcrows).ok()
    }
}
#[repr(C)]
pub struct IRowsetExactScroll_Vtbl {
    pub base__: IRowsetScroll_Vtbl,
    pub GetExactPosition: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const u8, *mut usize, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetFastLoad, IRowsetFastLoad_Vtbl, 0x5cf4ca13_ef21_11d0_97e7_00c04fc2ad98);
impl core::ops::Deref for IRowsetFastLoad {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetFastLoad, windows_core::IUnknown);
impl IRowsetFastLoad {
    pub unsafe fn InsertRow<P0>(&self, haccessor: P0, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).InsertRow)(windows_core::Interface::as_raw(self), haccessor.param().abi(), pdata).ok()
    }
    pub unsafe fn Commit<P0>(&self, fdone: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), fdone.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRowsetFastLoad_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InsertRow: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetFind, IRowsetFind_Vtbl, 0x0c733a9d_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetFind {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetFind, windows_core::IUnknown);
impl IRowsetFind {
    pub unsafe fn FindNextRow<P0>(&self, hchapter: usize, haccessor: P0, pfindvalue: *const core::ffi::c_void, compareop: u32, pbookmark: &[u8], lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).FindNextRow)(windows_core::Interface::as_raw(self), hchapter, haccessor.param().abi(), pfindvalue, compareop, pbookmark.len().try_into().unwrap(), core::mem::transmute(pbookmark.as_ptr()), lrowsoffset, crows, pcrowsobtained, prghrows).ok()
    }
}
#[repr(C)]
pub struct IRowsetFind_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindNextRow: unsafe extern "system" fn(*mut core::ffi::c_void, usize, HACCESSOR, *const core::ffi::c_void, u32, usize, *const u8, isize, isize, *mut usize, *mut *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetIdentity, IRowsetIdentity_Vtbl, 0x0c733a09_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetIdentity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetIdentity, windows_core::IUnknown);
impl IRowsetIdentity {
    pub unsafe fn IsSameRow(&self, hthisrow: usize, hthatrow: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsSameRow)(windows_core::Interface::as_raw(self), hthisrow, hthatrow).ok()
    }
}
#[repr(C)]
pub struct IRowsetIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsSameRow: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetIndex, IRowsetIndex_Vtbl, 0x0c733a82_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetIndex {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetIndex, windows_core::IUnknown);
impl IRowsetIndex {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetIndexInfo(&self, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIndexInfo)(windows_core::Interface::as_raw(self), pckeycolumns, prgindexcolumndesc, pcindexpropertysets, prgindexpropertysets).ok()
    }
    pub unsafe fn Seek<P0>(&self, haccessor: P0, ckeyvalues: usize, pdata: *const core::ffi::c_void, dwseekoptions: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), haccessor.param().abi(), ckeyvalues, pdata, dwseekoptions).ok()
    }
    pub unsafe fn SetRange<P0>(&self, haccessor: P0, cstartkeycolumns: usize, pstartdata: *const core::ffi::c_void, cendkeycolumns: usize, penddata: *const core::ffi::c_void, dwrangeoptions: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).SetRange)(windows_core::Interface::as_raw(self), haccessor.param().abi(), cstartkeycolumns, pstartdata, cendkeycolumns, penddata, dwrangeoptions).ok()
    }
}
#[repr(C)]
pub struct IRowsetIndex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetIndexInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut *mut DBINDEXCOLUMNDESC, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetIndexInfo: usize,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, usize, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetRange: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, usize, *const core::ffi::c_void, usize, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetInfo, IRowsetInfo_Vtbl, 0x0c733a55_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetInfo, windows_core::IUnknown);
impl IRowsetInfo {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetProperties(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), rgpropertyidsets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertyidsets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcpropertysets, prgpropertysets).ok()
    }
    pub unsafe fn GetReferencedRowset(&self, iordinal: usize, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReferencedRowset)(windows_core::Interface::as_raw(self), iordinal, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSpecification(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSpecification)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRowsetInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetProperties: usize,
    pub GetReferencedRowset: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpecification: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetKeys, IRowsetKeys_Vtbl, 0x0c733a12_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetKeys {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetKeys, windows_core::IUnknown);
impl IRowsetKeys {
    pub unsafe fn ListKeys(&self, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ListKeys)(windows_core::Interface::as_raw(self), pccolumns, prgcolumns).ok()
    }
}
#[repr(C)]
pub struct IRowsetKeys_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ListKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetLocate, IRowsetLocate_Vtbl, 0x0c733a7d_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetLocate {
    type Target = IRowset;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetLocate, windows_core::IUnknown, IRowset);
impl IRowsetLocate {
    pub unsafe fn Compare(&self, hreserved: usize, pbookmark1: &[u8], pbookmark2: &[u8]) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), hreserved, pbookmark1.len().try_into().unwrap(), core::mem::transmute(pbookmark1.as_ptr()), pbookmark2.len().try_into().unwrap(), core::mem::transmute(pbookmark2.as_ptr()), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRowsAt(&self, hreserved1: usize, hreserved2: usize, pbookmark: &[u8], lrowsoffset: isize, pcrowsobtained: *mut usize, prghrows: &mut [*mut usize]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRowsAt)(windows_core::Interface::as_raw(self), hreserved1, hreserved2, pbookmark.len().try_into().unwrap(), core::mem::transmute(pbookmark.as_ptr()), lrowsoffset, prghrows.len().try_into().unwrap(), pcrowsobtained, core::mem::transmute(prghrows.as_ptr())).ok()
    }
    pub unsafe fn GetRowsByBookmark(&self, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRowsByBookmark)(windows_core::Interface::as_raw(self), hreserved, crows, rgcbbookmarks, rgpbookmarks, rghrows, rgrowstatus).ok()
    }
    pub unsafe fn Hash(&self, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Hash)(windows_core::Interface::as_raw(self), hreserved, cbookmarks, rgcbbookmarks, rgpbookmarks, rghashedvalues, rgbookmarkstatus).ok()
    }
}
#[repr(C)]
pub struct IRowsetLocate_Vtbl {
    pub base__: IRowset_Vtbl,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const u8, usize, *const u8, *mut u32) -> windows_core::HRESULT,
    pub GetRowsAt: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, usize, *const u8, isize, isize, *mut usize, *mut *mut usize) -> windows_core::HRESULT,
    pub GetRowsByBookmark: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const usize, *const *const u8, *mut usize, *mut u32) -> windows_core::HRESULT,
    pub Hash: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const usize, *const *const u8, *mut usize, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetNewRowAfter, IRowsetNewRowAfter_Vtbl, 0x0c733a71_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetNewRowAfter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetNewRowAfter, windows_core::IUnknown);
impl IRowsetNewRowAfter {
    pub unsafe fn SetNewDataAfter<P0>(&self, hchapter: usize, pbmprevious: &[u8], haccessor: P0, pdata: *const u8) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetNewDataAfter)(windows_core::Interface::as_raw(self), hchapter, pbmprevious.len().try_into().unwrap(), core::mem::transmute(pbmprevious.as_ptr()), haccessor.param().abi(), pdata, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRowsetNewRowAfter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetNewDataAfter: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *const u8, HACCESSOR, *const u8, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetNextRowset, IRowsetNextRowset_Vtbl, 0x0c733a72_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetNextRowset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetNextRowset, windows_core::IUnknown);
impl IRowsetNextRowset {
    pub unsafe fn GetNextRowset<P0>(&self, punkouter: P0, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNextRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRowsetNextRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNextRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetNotify, IRowsetNotify_Vtbl, 0x0c733a83_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetNotify, windows_core::IUnknown);
impl IRowsetNotify {
    pub unsafe fn OnFieldChange<P0, P1>(&self, prowset: P0, hrow: usize, rgcolumns: &[usize], ereason: u32, ephase: u32, fcantdeny: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRowset>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnFieldChange)(windows_core::Interface::as_raw(self), prowset.param().abi(), hrow, rgcolumns.len().try_into().unwrap(), core::mem::transmute(rgcolumns.as_ptr()), ereason, ephase, fcantdeny.param().abi()).ok()
    }
    pub unsafe fn OnRowChange<P0, P1>(&self, prowset: P0, rghrows: &[usize], ereason: u32, ephase: u32, fcantdeny: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRowset>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnRowChange)(windows_core::Interface::as_raw(self), prowset.param().abi(), rghrows.len().try_into().unwrap(), core::mem::transmute(rghrows.as_ptr()), ereason, ephase, fcantdeny.param().abi()).ok()
    }
    pub unsafe fn OnRowsetChange<P0, P1>(&self, prowset: P0, ereason: u32, ephase: u32, fcantdeny: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRowset>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnRowsetChange)(windows_core::Interface::as_raw(self), prowset.param().abi(), ereason, ephase, fcantdeny.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRowsetNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnFieldChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, usize, *const usize, u32, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnRowChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, *const usize, u32, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnRowsetChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetPrioritization, IRowsetPrioritization_Vtbl, 0x42811652_079d_481b_87a2_09a69ecc5f44);
impl core::ops::Deref for IRowsetPrioritization {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetPrioritization, windows_core::IUnknown);
impl IRowsetPrioritization {
    pub unsafe fn SetScopePriority(&self, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScopePriority)(windows_core::Interface::as_raw(self), priority, scopestatisticseventfrequency).ok()
    }
    pub unsafe fn GetScopePriority(&self, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScopePriority)(windows_core::Interface::as_raw(self), priority, scopestatisticseventfrequency).ok()
    }
    pub unsafe fn GetScopeStatistics(&self, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScopeStatistics)(windows_core::Interface::as_raw(self), indexeddocumentcount, oustandingaddcount, oustandingmodifycount).ok()
    }
}
#[repr(C)]
pub struct IRowsetPrioritization_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetScopePriority: unsafe extern "system" fn(*mut core::ffi::c_void, PRIORITY_LEVEL, u32) -> windows_core::HRESULT,
    pub GetScopePriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PRIORITY_LEVEL, *mut u32) -> windows_core::HRESULT,
    pub GetScopeStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetQueryStatus, IRowsetQueryStatus_Vtbl, 0xa7ac77ed_f8d7_11ce_a798_0020f8008024);
impl core::ops::Deref for IRowsetQueryStatus {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetQueryStatus, windows_core::IUnknown);
impl IRowsetQueryStatus {
    pub unsafe fn GetStatus(&self, pdwstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdwstatus).ok()
    }
    pub unsafe fn GetStatusEx(&self, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatusEx)(windows_core::Interface::as_raw(self), pdwstatus, pcfiltereddocuments, pcdocumentstofilter, pdwratiofinisheddenominator, pdwratiofinishednumerator, cbbmk, pbmk, pirowbmk, pcrowstotal).ok()
    }
}
#[repr(C)]
pub struct IRowsetQueryStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStatusEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32, *mut usize, *mut usize, usize, *const u8, *mut usize, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetRefresh, IRowsetRefresh_Vtbl, 0x0c733aa9_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetRefresh {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetRefresh, windows_core::IUnknown);
impl IRowsetRefresh {
    pub unsafe fn RefreshVisibleData<P0>(&self, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: P0, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RefreshVisibleData)(windows_core::Interface::as_raw(self), hchapter, crows, rghrows, foverwrite.param().abi(), pcrowsrefreshed, prghrowsrefreshed, prgrowstatus).ok()
    }
    pub unsafe fn GetLastVisibleData<P0>(&self, hrow: usize, haccessor: P0, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).GetLastVisibleData)(windows_core::Interface::as_raw(self), hrow, haccessor.param().abi(), pdata).ok()
    }
}
#[repr(C)]
pub struct IRowsetRefresh_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RefreshVisibleData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const usize, super::super::Foundation::BOOL, *mut usize, *mut *mut usize, *mut *mut u32) -> windows_core::HRESULT,
    pub GetLastVisibleData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, HACCESSOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetResynch, IRowsetResynch_Vtbl, 0x0c733a84_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetResynch {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetResynch, windows_core::IUnknown);
impl IRowsetResynch {
    pub unsafe fn GetVisibleData<P0>(&self, hrow: usize, haccessor: P0, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).GetVisibleData)(windows_core::Interface::as_raw(self), hrow, haccessor.param().abi(), pdata).ok()
    }
    pub unsafe fn ResynchRows(&self, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResynchRows)(windows_core::Interface::as_raw(self), crows, rghrows, pcrowsresynched, prghrowsresynched, prgrowstatus).ok()
    }
}
#[repr(C)]
pub struct IRowsetResynch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVisibleData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, HACCESSOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResynchRows: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const usize, *mut usize, *mut *mut usize, *mut *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetScroll, IRowsetScroll_Vtbl, 0x0c733a7e_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetScroll {
    type Target = IRowsetLocate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetScroll, windows_core::IUnknown, IRowset, IRowsetLocate);
impl IRowsetScroll {
    pub unsafe fn GetApproximatePosition(&self, hreserved: usize, pbookmark: &[u8], pulposition: *mut usize, pcrows: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetApproximatePosition)(windows_core::Interface::as_raw(self), hreserved, pbookmark.len().try_into().unwrap(), core::mem::transmute(pbookmark.as_ptr()), pulposition, pcrows).ok()
    }
    pub unsafe fn GetRowsAtRatio(&self, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, pcrowsobtained: *mut usize, prghrows: &mut [*mut usize]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRowsAtRatio)(windows_core::Interface::as_raw(self), hreserved1, hreserved2, ulnumerator, uldenominator, prghrows.len().try_into().unwrap(), pcrowsobtained, core::mem::transmute(prghrows.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IRowsetScroll_Vtbl {
    pub base__: IRowsetLocate_Vtbl,
    pub GetApproximatePosition: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const u8, *mut usize, *mut usize) -> windows_core::HRESULT,
    pub GetRowsAtRatio: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, usize, usize, isize, *mut usize, *mut *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetUpdate, IRowsetUpdate_Vtbl, 0x0c733a6d_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetUpdate {
    type Target = IRowsetChange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetUpdate, windows_core::IUnknown, IRowsetChange);
impl IRowsetUpdate {
    pub unsafe fn GetOriginalData<P0>(&self, hrow: usize, haccessor: P0, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).GetOriginalData)(windows_core::Interface::as_raw(self), hrow, haccessor.param().abi(), pdata).ok()
    }
    pub unsafe fn GetPendingRows(&self, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPendingRows)(windows_core::Interface::as_raw(self), hreserved, dwrowstatus, pcpendingrows, prgpendingrows, prgpendingstatus).ok()
    }
    pub unsafe fn GetRowStatus(&self, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRowStatus)(windows_core::Interface::as_raw(self), hreserved, crows, rghrows, rgpendingstatus).ok()
    }
    pub unsafe fn Undo(&self, hreserved: usize, rghrows: &[usize], pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Undo)(windows_core::Interface::as_raw(self), hreserved, rghrows.len().try_into().unwrap(), core::mem::transmute(rghrows.as_ptr()), pcrowsundone, prgrowsundone, prgrowstatus).ok()
    }
    pub unsafe fn Update(&self, hreserved: usize, rghrows: &[usize], pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), hreserved, rghrows.len().try_into().unwrap(), core::mem::transmute(rghrows.as_ptr()), pcrows, prgrows, prgrowstatus).ok()
    }
}
#[repr(C)]
pub struct IRowsetUpdate_Vtbl {
    pub base__: IRowsetChange_Vtbl,
    pub GetOriginalData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, HACCESSOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPendingRows: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut usize, *mut *mut usize, *mut *mut u32) -> windows_core::HRESULT,
    pub GetRowStatus: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const usize, *mut u32) -> windows_core::HRESULT,
    pub Undo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const usize, *mut usize, *mut *mut usize, *mut *mut u32) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *const usize, *mut usize, *mut *mut usize, *mut *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetView, IRowsetView_Vtbl, 0x0c733a99_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetView {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetView, windows_core::IUnknown);
impl IRowsetView {
    pub unsafe fn CreateView<P0>(&self, punkouter: P0, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateView)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetView(&self, hchapter: usize, riid: *const windows_core::GUID, phchaptersource: *mut usize, ppview: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetView)(windows_core::Interface::as_raw(self), hchapter, riid, phchaptersource, core::mem::transmute(ppview)).ok()
    }
}
#[repr(C)]
pub struct IRowsetView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::GUID, *mut usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetWatchAll, IRowsetWatchAll_Vtbl, 0x0c733a73_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetWatchAll {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetWatchAll, windows_core::IUnknown);
impl IRowsetWatchAll {
    pub unsafe fn Acknowledge(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Acknowledge)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StopWatching(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopWatching)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRowsetWatchAll_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Acknowledge: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopWatching: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetWatchNotify, IRowsetWatchNotify_Vtbl, 0x0c733a44_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetWatchNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetWatchNotify, windows_core::IUnknown);
impl IRowsetWatchNotify {
    pub unsafe fn OnChange<P0>(&self, prowset: P0, echangereason: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRowset>,
    {
        (windows_core::Interface::vtable(self).OnChange)(windows_core::Interface::as_raw(self), prowset.param().abi(), echangereason).ok()
    }
}
#[repr(C)]
pub struct IRowsetWatchNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetWatchRegion, IRowsetWatchRegion_Vtbl, 0x0c733a45_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetWatchRegion {
    type Target = IRowsetWatchAll;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetWatchRegion, windows_core::IUnknown, IRowsetWatchAll);
impl IRowsetWatchRegion {
    pub unsafe fn CreateWatchRegion(&self, dwwatchmode: u32) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateWatchRegion)(windows_core::Interface::as_raw(self), dwwatchmode, &mut result__).map(|| result__)
    }
    pub unsafe fn ChangeWatchMode(&self, hregion: usize, dwwatchmode: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangeWatchMode)(windows_core::Interface::as_raw(self), hregion, dwwatchmode).ok()
    }
    pub unsafe fn DeleteWatchRegion(&self, hregion: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteWatchRegion)(windows_core::Interface::as_raw(self), hregion).ok()
    }
    pub unsafe fn GetWatchRegionInfo(&self, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWatchRegionInfo)(windows_core::Interface::as_raw(self), hregion, pdwwatchmode, phchapter, pcbbookmark, ppbookmark, pcrows).ok()
    }
    pub unsafe fn Refresh(&self, pcchangesobtained: *mut usize, prgchanges: *mut *mut DBROWWATCHCHANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self), pcchangesobtained, prgchanges).ok()
    }
    pub unsafe fn ShrinkWatchRegion(&self, hregion: usize, hchapter: usize, pbookmark: &[u8], crows: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShrinkWatchRegion)(windows_core::Interface::as_raw(self), hregion, hchapter, pbookmark.len().try_into().unwrap(), core::mem::transmute(pbookmark.as_ptr()), crows).ok()
    }
}
#[repr(C)]
pub struct IRowsetWatchRegion_Vtbl {
    pub base__: IRowsetWatchAll_Vtbl,
    pub CreateWatchRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut usize) -> windows_core::HRESULT,
    pub ChangeWatchMode: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32) -> windows_core::HRESULT,
    pub DeleteWatchRegion: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub GetWatchRegionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32, *mut usize, *mut usize, *mut *mut u8, *mut isize) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut *mut DBROWWATCHCHANGE) -> windows_core::HRESULT,
    pub ShrinkWatchRegion: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, usize, *const u8, isize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRowsetWithParameters, IRowsetWithParameters_Vtbl, 0x0c733a6e_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IRowsetWithParameters {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRowsetWithParameters, windows_core::IUnknown);
impl IRowsetWithParameters {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetParameterInfo(&self, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetParameterInfo)(windows_core::Interface::as_raw(self), pcparams, prgparaminfo, ppnamesbuffer).ok()
    }
    pub unsafe fn Requery(&self, pparams: *const DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Requery)(windows_core::Interface::as_raw(self), pparams, pulerrorparam, phreserved).ok()
    }
}
#[repr(C)]
pub struct IRowsetWithParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParameterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut *mut DBPARAMINFO, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParameterInfo: usize,
    pub Requery: unsafe extern "system" fn(*mut core::ffi::c_void, *const DBPARAMS, *mut u32, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISQLErrorInfo, ISQLErrorInfo_Vtbl, 0x0c733a74_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ISQLErrorInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISQLErrorInfo, windows_core::IUnknown);
impl ISQLErrorInfo {
    pub unsafe fn GetSQLInfo(&self, pbstrsqlstate: *mut windows_core::BSTR, plnativeerror: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSQLInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrsqlstate), plnativeerror).ok()
    }
}
#[repr(C)]
pub struct ISQLErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSQLInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISQLGetDiagField, ISQLGetDiagField_Vtbl, 0x228972f1_b5ff_11d0_8a80_00c04fd611cd);
impl core::ops::Deref for ISQLGetDiagField {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISQLGetDiagField, windows_core::IUnknown);
impl ISQLGetDiagField {
    pub unsafe fn GetDiagField(&self, pdiaginfo: Option<*mut KAGGETDIAG>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDiagField)(windows_core::Interface::as_raw(self), core::mem::transmute(pdiaginfo.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct ISQLGetDiagField_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDiagField: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KAGGETDIAG) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISQLRequestDiagFields, ISQLRequestDiagFields_Vtbl, 0x228972f0_b5ff_11d0_8a80_00c04fd611cd);
impl core::ops::Deref for ISQLRequestDiagFields {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISQLRequestDiagFields, windows_core::IUnknown);
impl ISQLRequestDiagFields {
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn RequestDiagFields(&self, rgdiagfields: &[KAGREQDIAG]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestDiagFields)(windows_core::Interface::as_raw(self), rgdiagfields.len().try_into().unwrap(), core::mem::transmute(rgdiagfields.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ISQLRequestDiagFields_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Variant")]
    pub RequestDiagFields: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const KAGREQDIAG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    RequestDiagFields: usize,
}
windows_core::imp::define_interface!(ISQLServerErrorInfo, ISQLServerErrorInfo_Vtbl, 0x5cf4ca12_ef21_11d0_97e7_00c04fc2ad98);
impl core::ops::Deref for ISQLServerErrorInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISQLServerErrorInfo, windows_core::IUnknown);
impl ISQLServerErrorInfo {
    pub unsafe fn GetErrorInfo(&self, pperrorinfo: *mut *mut SSERRORINFO, ppstringsbuffer: *mut *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetErrorInfo)(windows_core::Interface::as_raw(self), pperrorinfo, ppstringsbuffer).ok()
    }
}
#[repr(C)]
pub struct ISQLServerErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SSERRORINFO, *mut *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISchemaLocalizerSupport, ISchemaLocalizerSupport_Vtbl, 0xca3fdca2_bfbe_4eed_90d7_0caef0a1bda1);
impl core::ops::Deref for ISchemaLocalizerSupport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISchemaLocalizerSupport, windows_core::IUnknown);
impl ISchemaLocalizerSupport {
    pub unsafe fn Localize<P0>(&self, pszglobalstring: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Localize)(windows_core::Interface::as_raw(self), pszglobalstring.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISchemaLocalizerSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Localize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISchemaLock, ISchemaLock_Vtbl, 0x4c2389fb_2511_11d4_b258_00c04f7971ce);
impl core::ops::Deref for ISchemaLock {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISchemaLock, windows_core::IUnknown);
impl ISchemaLock {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetSchemaLock(&self, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSchemaLock)(windows_core::Interface::as_raw(self), ptableid, lmmode, phlockhandle, ptableversion).ok()
    }
    pub unsafe fn ReleaseSchemaLock<P0>(&self, hlockhandle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).ReleaseSchemaLock)(windows_core::Interface::as_raw(self), hlockhandle.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISchemaLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetSchemaLock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Storage::IndexServer::DBID, u32, *mut super::super::Foundation::HANDLE, *mut u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetSchemaLock: usize,
    pub ReleaseSchemaLock: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISchemaProvider, ISchemaProvider_Vtbl, 0x8cf89bcb_394c_49b2_ae28_a59dd4ed7f68);
impl core::ops::Deref for ISchemaProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISchemaProvider, windows_core::IUnknown);
impl ISchemaProvider {
    pub unsafe fn Entities<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).Entities)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RootEntity(&self) -> windows_core::Result<IEntity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RootEntity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetEntity<P0>(&self, pszentityname: P0) -> windows_core::Result<IEntity>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEntity)(windows_core::Interface::as_raw(self), pszentityname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MetaData<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).MetaData)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Localize<P0>(&self, lcid: u32, pschemalocalizersupport: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISchemaLocalizerSupport>,
    {
        (windows_core::Interface::vtable(self).Localize)(windows_core::Interface::as_raw(self), lcid, pschemalocalizersupport.param().abi()).ok()
    }
    pub unsafe fn SaveBinary<P0>(&self, pszschemabinarypath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SaveBinary)(windows_core::Interface::as_raw(self), pszschemabinarypath.param().abi()).ok()
    }
    pub unsafe fn LookupAuthoredNamedEntity<P0, P1, P2>(&self, pentity: P0, pszinputstring: P1, ptokencollection: P2, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut windows_core::PWSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IEntity>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<ITokenCollection>,
    {
        (windows_core::Interface::vtable(self).LookupAuthoredNamedEntity)(windows_core::Interface::as_raw(self), pentity.param().abi(), pszinputstring.param().abi(), ptokencollection.param().abi(), ctokensbegin, pctokenslength, ppszvalue).ok()
    }
}
#[repr(C)]
pub struct ISchemaProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Entities: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RootEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEntity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Localize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveBinary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub LookupAuthoredNamedEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScopedOperations, IScopedOperations_Vtbl, 0x0c733ab0_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IScopedOperations {
    type Target = IBindResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IScopedOperations, windows_core::IUnknown, IBindResource);
impl IScopedOperations {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Copy<P0>(&self, crows: usize, rgpwszsourceurls: Option<*const windows_core::PCWSTR>, rgpwszdesturls: *const windows_core::PCWSTR, dwcopyflags: u32, pauthenticate: P0, rgdwstatus: *mut u32, rgpwsznewurls: Option<*mut windows_core::PWSTR>, ppstringsbuffer: Option<*mut *mut u16>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IAuthenticate>,
    {
        (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), crows, core::mem::transmute(rgpwszsourceurls.unwrap_or(std::ptr::null())), rgpwszdesturls, dwcopyflags, pauthenticate.param().abi(), rgdwstatus, core::mem::transmute(rgpwsznewurls.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppstringsbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Move<P0>(&self, crows: usize, rgpwszsourceurls: Option<*const windows_core::PCWSTR>, rgpwszdesturls: *const windows_core::PCWSTR, dwmoveflags: u32, pauthenticate: P0, rgdwstatus: *mut u32, rgpwsznewurls: Option<*mut windows_core::PWSTR>, ppstringsbuffer: Option<*mut *mut u16>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IAuthenticate>,
    {
        (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), crows, core::mem::transmute(rgpwszsourceurls.unwrap_or(std::ptr::null())), rgpwszdesturls, dwmoveflags, pauthenticate.param().abi(), rgdwstatus, core::mem::transmute(rgpwsznewurls.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppstringsbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Delete(&self, crows: usize, rgpwszurls: *const windows_core::PCWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), crows, rgpwszurls, dwdeleteflags, rgdwstatus).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn OpenRowset<P0>(&self, punkouter: P0, ptableid: Option<*const super::super::Storage::IndexServer::DBID>, pindexid: Option<*const super::super::Storage::IndexServer::DBID>, riid: *const windows_core::GUID, rgpropertysets: &mut [DBPROPSET], pprowset: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OpenRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), core::mem::transmute(ptableid.unwrap_or(std::ptr::null())), core::mem::transmute(pindexid.unwrap_or(std::ptr::null())), riid, rgpropertysets.len().try_into().unwrap(), core::mem::transmute(rgpropertysets.as_ptr()), core::mem::transmute(pprowset.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IScopedOperations_Vtbl {
    pub base__: IBindResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut u32, *mut windows_core::PWSTR, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Copy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut u32, *mut windows_core::PWSTR, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Move: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub OpenRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    OpenRowset: usize,
}
windows_core::imp::define_interface!(ISearchCatalogManager, ISearchCatalogManager_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef50);
impl core::ops::Deref for ISearchCatalogManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchCatalogManager, windows_core::IUnknown);
impl ISearchCatalogManager {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetParameter<P0>(&self, pszname: P0) -> windows_core::Result<*mut windows_core::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParameter)(windows_core::Interface::as_raw(self), pszname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetParameter<P0>(&self, pszname: P0, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetParameter)(windows_core::Interface::as_raw(self), pszname.param().abi(), core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetCatalogStatus(&self, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCatalogStatus)(windows_core::Interface::as_raw(self), pstatus, ppausedreason).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reindex(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reindex)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReindexMatchingURLs<P0>(&self, pszpattern: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReindexMatchingURLs)(windows_core::Interface::as_raw(self), pszpattern.param().abi()).ok()
    }
    pub unsafe fn ReindexSearchRoot<P0>(&self, pszrooturl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReindexSearchRoot)(windows_core::Interface::as_raw(self), pszrooturl.param().abi()).ok()
    }
    pub unsafe fn SetConnectTimeout(&self, dwconnecttimeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConnectTimeout)(windows_core::Interface::as_raw(self), dwconnecttimeout).ok()
    }
    pub unsafe fn ConnectTimeout(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDataTimeout(&self, dwdatatimeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDataTimeout)(windows_core::Interface::as_raw(self), dwdatatimeout).ok()
    }
    pub unsafe fn DataTimeout(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfItems(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfItemsToIndex(&self, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NumberOfItemsToIndex)(windows_core::Interface::as_raw(self), plincrementalcount, plnotificationqueue, plhighpriorityqueue).ok()
    }
    pub unsafe fn URLBeingIndexed(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).URLBeingIndexed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetURLIndexingState<P0>(&self, pszurl: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetURLIndexingState)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPersistentItemsChangedSink(&self) -> windows_core::Result<ISearchPersistentItemsChangedSink> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPersistentItemsChangedSink)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterViewForNotification<P0, P1>(&self, pszview: P0, pviewchangedsink: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ISearchViewChangedSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterViewForNotification)(windows_core::Interface::as_raw(self), pszview.param().abi(), pviewchangedsink.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetItemsChangedSink<P0, T>(&self, pisearchnotifyinlinesite: P0, pguidcatalogresetsignature: *mut windows_core::GUID, pguidcheckpointsignature: *mut windows_core::GUID, pdwlastcheckpointnumber: *mut u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<ISearchNotifyInlineSite>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetItemsChangedSink)(windows_core::Interface::as_raw(self), pisearchnotifyinlinesite.param().abi(), &T::IID, &mut result__, pguidcatalogresetsignature, pguidcheckpointsignature, pdwlastcheckpointnumber).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UnregisterViewForNotification(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterViewForNotification)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn SetExtensionClusion<P0, P1>(&self, pszextension: P0, fexclude: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetExtensionClusion)(windows_core::Interface::as_raw(self), pszextension.param().abi(), fexclude.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateExcludedExtensions(&self) -> windows_core::Result<super::Com::IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateExcludedExtensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetQueryHelper(&self) -> windows_core::Result<ISearchQueryHelper> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetQueryHelper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDiacriticSensitivity<P0>(&self, fdiacriticsensitive: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDiacriticSensitivity)(windows_core::Interface::as_raw(self), fdiacriticsensitive.param().abi()).ok()
    }
    pub unsafe fn DiacriticSensitivity(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DiacriticSensitivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCrawlScopeManager(&self) -> windows_core::Result<ISearchCrawlScopeManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCrawlScopeManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISearchCatalogManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut windows_core::PROPVARIANT) -> windows_core::HRESULT,
    pub SetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetCatalogStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CatalogStatus, *mut CatalogPausedReason) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reindex: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReindexMatchingURLs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ReindexSearchRoot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetConnectTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ConnectTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDataTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DataTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NumberOfItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfItemsToIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub URLBeingIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetURLIndexingState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetPersistentItemsChangedSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterViewForNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemsChangedSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub UnregisterViewForNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetExtensionClusion: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateExcludedExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateExcludedExtensions: usize,
    pub GetQueryHelper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDiacriticSensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DiacriticSensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCrawlScopeManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchCatalogManager2, ISearchCatalogManager2_Vtbl, 0x7ac3286d_4d1d_4817_84fc_c1c85e3af0d9);
impl core::ops::Deref for ISearchCatalogManager2 {
    type Target = ISearchCatalogManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchCatalogManager2, windows_core::IUnknown, ISearchCatalogManager);
impl ISearchCatalogManager2 {
    pub unsafe fn PrioritizeMatchingURLs<P0>(&self, pszpattern: P0, dwprioritizeflags: PRIORITIZE_FLAGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PrioritizeMatchingURLs)(windows_core::Interface::as_raw(self), pszpattern.param().abi(), dwprioritizeflags).ok()
    }
}
#[repr(C)]
pub struct ISearchCatalogManager2_Vtbl {
    pub base__: ISearchCatalogManager_Vtbl,
    pub PrioritizeMatchingURLs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PRIORITIZE_FLAGS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchCrawlScopeManager, ISearchCrawlScopeManager_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef55);
impl core::ops::Deref for ISearchCrawlScopeManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchCrawlScopeManager, windows_core::IUnknown);
impl ISearchCrawlScopeManager {
    pub unsafe fn AddDefaultScopeRule<P0, P1>(&self, pszurl: P0, finclude: P1, ffollowflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddDefaultScopeRule)(windows_core::Interface::as_raw(self), pszurl.param().abi(), finclude.param().abi(), ffollowflags).ok()
    }
    pub unsafe fn AddRoot<P0>(&self, psearchroot: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISearchRoot>,
    {
        (windows_core::Interface::vtable(self).AddRoot)(windows_core::Interface::as_raw(self), psearchroot.param().abi()).ok()
    }
    pub unsafe fn RemoveRoot<P0>(&self, pszurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveRoot)(windows_core::Interface::as_raw(self), pszurl.param().abi()).ok()
    }
    pub unsafe fn EnumerateRoots(&self) -> windows_core::Result<IEnumSearchRoots> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateRoots)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddHierarchicalScope<P0, P1, P2, P3>(&self, pszurl: P0, finclude: P1, fdefault: P2, foverridechildren: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddHierarchicalScope)(windows_core::Interface::as_raw(self), pszurl.param().abi(), finclude.param().abi(), fdefault.param().abi(), foverridechildren.param().abi()).ok()
    }
    pub unsafe fn AddUserScopeRule<P0, P1, P2>(&self, pszurl: P0, finclude: P1, foverridechildren: P2, ffollowflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddUserScopeRule)(windows_core::Interface::as_raw(self), pszurl.param().abi(), finclude.param().abi(), foverridechildren.param().abi(), ffollowflags).ok()
    }
    pub unsafe fn RemoveScopeRule<P0>(&self, pszrule: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveScopeRule)(windows_core::Interface::as_raw(self), pszrule.param().abi()).ok()
    }
    pub unsafe fn EnumerateScopeRules(&self) -> windows_core::Result<IEnumSearchScopeRules> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateScopeRules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HasParentScopeRule<P0>(&self, pszurl: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasParentScopeRule)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn HasChildScopeRule<P0>(&self, pszurl: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasChildScopeRule)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn IncludedInCrawlScope<P0>(&self, pszurl: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IncludedInCrawlScope)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn IncludedInCrawlScopeEx<P0>(&self, pszurl: P0, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IncludedInCrawlScopeEx)(windows_core::Interface::as_raw(self), pszurl.param().abi(), pfisincluded, preason).ok()
    }
    pub unsafe fn RevertToDefaultScopes(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RevertToDefaultScopes)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SaveAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveAll)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetParentScopeVersionId<P0>(&self, pszurl: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParentScopeVersionId)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoveDefaultScopeRule<P0>(&self, pszurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveDefaultScopeRule)(windows_core::Interface::as_raw(self), pszurl.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISearchCrawlScopeManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddDefaultScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL, u32) -> windows_core::HRESULT,
    pub AddRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveRoot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumerateRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddHierarchicalScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub AddUserScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL, super::super::Foundation::BOOL, u32) -> windows_core::HRESULT,
    pub RemoveScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumerateScopeRules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasParentScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub HasChildScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IncludedInCrawlScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IncludedInCrawlScopeEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL, *mut CLUSION_REASON) -> windows_core::HRESULT,
    pub RevertToDefaultScopes: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParentScopeVersionId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub RemoveDefaultScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchCrawlScopeManager2, ISearchCrawlScopeManager2_Vtbl, 0x6292f7ad_4e19_4717_a534_8fc22bcd5ccd);
impl core::ops::Deref for ISearchCrawlScopeManager2 {
    type Target = ISearchCrawlScopeManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchCrawlScopeManager2, windows_core::IUnknown, ISearchCrawlScopeManager);
impl ISearchCrawlScopeManager2 {
    pub unsafe fn GetVersion(&self, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), plversion, phfilemapping).ok()
    }
}
#[repr(C)]
pub struct ISearchCrawlScopeManager2_Vtbl {
    pub base__: ISearchCrawlScopeManager_Vtbl,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut i32, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchItemsChangedSink, ISearchItemsChangedSink_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef58);
impl core::ops::Deref for ISearchItemsChangedSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchItemsChangedSink, windows_core::IUnknown);
impl ISearchItemsChangedSink {
    pub unsafe fn StartedMonitoringScope<P0>(&self, pszurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StartedMonitoringScope)(windows_core::Interface::as_raw(self), pszurl.param().abi()).ok()
    }
    pub unsafe fn StoppedMonitoringScope<P0>(&self, pszurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StoppedMonitoringScope)(windows_core::Interface::as_raw(self), pszurl.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnItemsChanged(&self, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnItemsChanged)(windows_core::Interface::as_raw(self), dwnumberofchanges, rgdatachangeentries, rgdwdocids, rghrcompletioncodes).ok()
    }
}
#[repr(C)]
pub struct ISearchItemsChangedSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartedMonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub StoppedMonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const SEARCH_ITEM_CHANGE, *mut u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnItemsChanged: usize,
}
windows_core::imp::define_interface!(ISearchLanguageSupport, ISearchLanguageSupport_Vtbl, 0x24c3cbaa_ebc1_491a_9ef1_9f6d8deb1b8f);
impl core::ops::Deref for ISearchLanguageSupport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchLanguageSupport, windows_core::IUnknown);
impl ISearchLanguageSupport {
    pub unsafe fn SetDiacriticSensitivity<P0>(&self, fdiacriticsensitive: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDiacriticSensitivity)(windows_core::Interface::as_raw(self), fdiacriticsensitive.param().abi()).ok()
    }
    pub unsafe fn GetDiacriticSensitivity(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDiacriticSensitivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LoadWordBreaker<T>(&self, lcid: u32, plcidused: *mut u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).LoadWordBreaker)(windows_core::Interface::as_raw(self), lcid, &T::IID, &mut result__, plcidused).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LoadStemmer<T>(&self, lcid: u32, plcidused: *mut u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).LoadStemmer)(windows_core::Interface::as_raw(self), lcid, &T::IID, &mut result__, plcidused).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsPrefixNormalized(&self, pwcsquerytoken: &[u16], pwcsdocumenttoken: &[u16]) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPrefixNormalized)(windows_core::Interface::as_raw(self), core::mem::transmute(pwcsquerytoken.as_ptr()), pwcsquerytoken.len().try_into().unwrap(), core::mem::transmute(pwcsdocumenttoken.as_ptr()), pwcsdocumenttoken.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISearchLanguageSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDiacriticSensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetDiacriticSensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub LoadWordBreaker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LoadStemmer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsPrefixNormalized: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchManager, ISearchManager_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef69);
impl core::ops::Deref for ISearchManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchManager, windows_core::IUnknown);
impl ISearchManager {
    pub unsafe fn GetIndexerVersionStr(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIndexerVersionStr)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetIndexerVersion(&self, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIndexerVersion)(windows_core::Interface::as_raw(self), pdwmajor, pdwminor).ok()
    }
    pub unsafe fn GetParameter<P0>(&self, pszname: P0) -> windows_core::Result<*mut windows_core::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParameter)(windows_core::Interface::as_raw(self), pszname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetParameter<P0>(&self, pszname: P0, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetParameter)(windows_core::Interface::as_raw(self), pszname.param().abi(), core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn ProxyName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProxyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BypassList(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BypassList)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProxy<P0, P1, P2>(&self, suseproxy: PROXY_ACCESS, flocalbypassproxy: P0, dwportnumber: u32, pszproxyname: P1, pszbypasslist: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProxy)(windows_core::Interface::as_raw(self), suseproxy, flocalbypassproxy.param().abi(), dwportnumber, pszproxyname.param().abi(), pszbypasslist.param().abi()).ok()
    }
    pub unsafe fn GetCatalog<P0>(&self, pszcatalog: P0) -> windows_core::Result<ISearchCatalogManager>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCatalog)(windows_core::Interface::as_raw(self), pszcatalog.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserAgent(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserAgent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUserAgent<P0>(&self, pszuseragent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetUserAgent)(windows_core::Interface::as_raw(self), pszuseragent.param().abi()).ok()
    }
    pub unsafe fn UseProxy(&self) -> windows_core::Result<PROXY_ACCESS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UseProxy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LocalBypass(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalBypass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PortNumber(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PortNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISearchManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIndexerVersionStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetIndexerVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut windows_core::PROPVARIANT) -> windows_core::HRESULT,
    pub SetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub ProxyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub BypassList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetProxy: unsafe extern "system" fn(*mut core::ffi::c_void, PROXY_ACCESS, super::super::Foundation::BOOL, u32, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCatalog: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserAgent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetUserAgent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UseProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROXY_ACCESS) -> windows_core::HRESULT,
    pub LocalBypass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub PortNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchManager2, ISearchManager2_Vtbl, 0xdbab3f73_db19_4a79_bfc0_a61a93886ddf);
impl core::ops::Deref for ISearchManager2 {
    type Target = ISearchManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchManager2, windows_core::IUnknown, ISearchManager);
impl ISearchManager2 {
    pub unsafe fn CreateCatalog<P0>(&self, pszcatalog: P0) -> windows_core::Result<ISearchCatalogManager>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCatalog)(windows_core::Interface::as_raw(self), pszcatalog.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteCatalog<P0>(&self, pszcatalog: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteCatalog)(windows_core::Interface::as_raw(self), pszcatalog.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISearchManager2_Vtbl {
    pub base__: ISearchManager_Vtbl,
    pub CreateCatalog: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteCatalog: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchNotifyInlineSite, ISearchNotifyInlineSite_Vtbl, 0xb5702e61_e75c_4b64_82a1_6cb4f832fccf);
impl core::ops::Deref for ISearchNotifyInlineSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchNotifyInlineSite, windows_core::IUnknown);
impl ISearchNotifyInlineSite {
    pub unsafe fn OnItemIndexedStatusChange(&self, sipstatus: SEARCH_INDEXING_PHASE, rgitemstatusentries: &[SEARCH_ITEM_INDEXING_STATUS]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnItemIndexedStatusChange)(windows_core::Interface::as_raw(self), sipstatus, rgitemstatusentries.len().try_into().unwrap(), core::mem::transmute(rgitemstatusentries.as_ptr())).ok()
    }
    pub unsafe fn OnCatalogStatusChange(&self, guidcatalogresetsignature: *const windows_core::GUID, guidcheckpointsignature: *const windows_core::GUID, dwlastcheckpointnumber: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCatalogStatusChange)(windows_core::Interface::as_raw(self), guidcatalogresetsignature, guidcheckpointsignature, dwlastcheckpointnumber).ok()
    }
}
#[repr(C)]
pub struct ISearchNotifyInlineSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnItemIndexedStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, SEARCH_INDEXING_PHASE, u32, *const SEARCH_ITEM_INDEXING_STATUS) -> windows_core::HRESULT,
    pub OnCatalogStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPersistentItemsChangedSink, ISearchPersistentItemsChangedSink_Vtbl, 0xa2ffdf9b_4758_4f84_b729_df81a1a0612f);
impl core::ops::Deref for ISearchPersistentItemsChangedSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchPersistentItemsChangedSink, windows_core::IUnknown);
impl ISearchPersistentItemsChangedSink {
    pub unsafe fn StartedMonitoringScope<P0>(&self, pszurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StartedMonitoringScope)(windows_core::Interface::as_raw(self), pszurl.param().abi()).ok()
    }
    pub unsafe fn StoppedMonitoringScope<P0>(&self, pszurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StoppedMonitoringScope)(windows_core::Interface::as_raw(self), pszurl.param().abi()).ok()
    }
    pub unsafe fn OnItemsChanged(&self, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnItemsChanged)(windows_core::Interface::as_raw(self), dwnumberofchanges, datachangeentries, hrcompletioncodes).ok()
    }
}
#[repr(C)]
pub struct ISearchPersistentItemsChangedSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartedMonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub StoppedMonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const SEARCH_ITEM_PERSISTENT_CHANGE, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchProtocol, ISearchProtocol_Vtbl, 0xc73106ba_ac80_11d1_8df3_00c04fb6ef4f);
impl core::ops::Deref for ISearchProtocol {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchProtocol, windows_core::IUnknown);
impl ISearchProtocol {
    pub unsafe fn Init<P0>(&self, ptimeoutinfo: *const TIMEOUT_INFO, pprotocolhandlersite: P0, pproxyinfo: *const PROXY_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IProtocolHandlerSite>,
    {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), ptimeoutinfo, pprotocolhandlersite.param().abi(), pproxyinfo).ok()
    }
    pub unsafe fn CreateAccessor<P0>(&self, pcwszurl: P0, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO) -> windows_core::Result<IUrlAccessor>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAccessor)(windows_core::Interface::as_raw(self), pcwszurl.param().abi(), pauthenticationinfo, pincrementalaccessinfo, piteminfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CloseAccessor<P0>(&self, paccessor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUrlAccessor>,
    {
        (windows_core::Interface::vtable(self).CloseAccessor)(windows_core::Interface::as_raw(self), paccessor.param().abi()).ok()
    }
    pub unsafe fn ShutDown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShutDown)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISearchProtocol_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const TIMEOUT_INFO, *mut core::ffi::c_void, *const PROXY_INFO) -> windows_core::HRESULT,
    pub CreateAccessor: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const AUTHENTICATION_INFO, *const INCREMENTAL_ACCESS_INFO, *const ITEM_INFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseAccessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchProtocol2, ISearchProtocol2_Vtbl, 0x7789f0b2_b5b2_4722_8b65_5dbd150697a9);
impl core::ops::Deref for ISearchProtocol2 {
    type Target = ISearchProtocol;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchProtocol2, windows_core::IUnknown, ISearchProtocol);
impl ISearchProtocol2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAccessorEx<P0>(&self, pcwszurl: P0, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, puserdata: *const super::Com::BLOB) -> windows_core::Result<IUrlAccessor>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAccessorEx)(windows_core::Interface::as_raw(self), pcwszurl.param().abi(), pauthenticationinfo, pincrementalaccessinfo, piteminfo, puserdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISearchProtocol2_Vtbl {
    pub base__: ISearchProtocol_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAccessorEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const AUTHENTICATION_INFO, *const INCREMENTAL_ACCESS_INFO, *const ITEM_INFO, *const super::Com::BLOB, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAccessorEx: usize,
}
windows_core::imp::define_interface!(ISearchProtocolThreadContext, ISearchProtocolThreadContext_Vtbl, 0xc73106e1_ac80_11d1_8df3_00c04fb6ef4f);
impl core::ops::Deref for ISearchProtocolThreadContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchProtocolThreadContext, windows_core::IUnknown);
impl ISearchProtocolThreadContext {
    pub unsafe fn ThreadInit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThreadInit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadShutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThreadShutdown)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadIdle(&self, dwtimeelaspedsincelastcallinms: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThreadIdle)(windows_core::Interface::as_raw(self), dwtimeelaspedsincelastcallinms).ok()
    }
}
#[repr(C)]
pub struct ISearchProtocolThreadContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThreadInit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ThreadShutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ThreadIdle: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchQueryHelper, ISearchQueryHelper_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef63);
impl core::ops::Deref for ISearchQueryHelper {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchQueryHelper, windows_core::IUnknown);
impl ISearchQueryHelper {
    pub unsafe fn ConnectionString(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectionString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQueryContentLocale(&self, lcid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQueryContentLocale)(windows_core::Interface::as_raw(self), lcid).ok()
    }
    pub unsafe fn QueryContentLocale(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryContentLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQueryKeywordLocale(&self, lcid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQueryKeywordLocale)(windows_core::Interface::as_raw(self), lcid).ok()
    }
    pub unsafe fn QueryKeywordLocale(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryKeywordLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQueryTermExpansion(&self, expandterms: SEARCH_TERM_EXPANSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQueryTermExpansion)(windows_core::Interface::as_raw(self), expandterms).ok()
    }
    pub unsafe fn QueryTermExpansion(&self) -> windows_core::Result<SEARCH_TERM_EXPANSION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryTermExpansion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuerySyntax(&self, querysyntax: SEARCH_QUERY_SYNTAX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuerySyntax)(windows_core::Interface::as_raw(self), querysyntax).ok()
    }
    pub unsafe fn QuerySyntax(&self) -> windows_core::Result<SEARCH_QUERY_SYNTAX> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuerySyntax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQueryContentProperties<P0>(&self, pszcontentproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetQueryContentProperties)(windows_core::Interface::as_raw(self), pszcontentproperties.param().abi()).ok()
    }
    pub unsafe fn QueryContentProperties(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryContentProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuerySelectColumns<P0>(&self, pszselectcolumns: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetQuerySelectColumns)(windows_core::Interface::as_raw(self), pszselectcolumns.param().abi()).ok()
    }
    pub unsafe fn QuerySelectColumns(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuerySelectColumns)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQueryWhereRestrictions<P0>(&self, pszrestrictions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetQueryWhereRestrictions)(windows_core::Interface::as_raw(self), pszrestrictions.param().abi()).ok()
    }
    pub unsafe fn QueryWhereRestrictions(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryWhereRestrictions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuerySorting<P0>(&self, pszsorting: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetQuerySorting)(windows_core::Interface::as_raw(self), pszsorting.param().abi()).ok()
    }
    pub unsafe fn QuerySorting(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuerySorting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GenerateSQLFromUserQuery<P0>(&self, pszquery: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenerateSQLFromUserQuery)(windows_core::Interface::as_raw(self), pszquery.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn WriteProperties(&self, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteProperties)(windows_core::Interface::as_raw(self), itemid, dwnumberofcolumns, pcolumns, pvalues, pftgathermodifiedtime).ok()
    }
    pub unsafe fn SetQueryMaxResults(&self, cmaxresults: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQueryMaxResults)(windows_core::Interface::as_raw(self), cmaxresults).ok()
    }
    pub unsafe fn QueryMaxResults(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryMaxResults)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISearchQueryHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetQueryContentLocale: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QueryContentLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetQueryKeywordLocale: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QueryKeywordLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetQueryTermExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, SEARCH_TERM_EXPANSION) -> windows_core::HRESULT,
    pub QueryTermExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SEARCH_TERM_EXPANSION) -> windows_core::HRESULT,
    pub SetQuerySyntax: unsafe extern "system" fn(*mut core::ffi::c_void, SEARCH_QUERY_SYNTAX) -> windows_core::HRESULT,
    pub QuerySyntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SEARCH_QUERY_SYNTAX) -> windows_core::HRESULT,
    pub SetQueryContentProperties: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub QueryContentProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetQuerySelectColumns: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub QuerySelectColumns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetQueryWhereRestrictions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub QueryWhereRestrictions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetQuerySorting: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub QuerySorting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GenerateSQLFromUserQuery: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub WriteProperties: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *const SEARCH_COLUMN_PROPERTIES, *const super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    WriteProperties: usize,
    pub SetQueryMaxResults: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub QueryMaxResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchQueryHits, ISearchQueryHits_Vtbl, 0xed8ce7e0_106c_11ce_84e2_00aa004b9986);
impl core::ops::Deref for ISearchQueryHits {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchQueryHits, windows_core::IUnknown);
impl ISearchQueryHits {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn Init<P0>(&self, pflt: P0, ulflags: u32) -> i32
    where
        P0: windows_core::Param<super::super::Storage::IndexServer::IFilter>,
    {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pflt.param().abi(), ulflags)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NextHitMoniker(&self, pcmnk: *mut u32, papmnk: *mut *mut Option<super::Com::IMoniker>) -> i32 {
        (windows_core::Interface::vtable(self).NextHitMoniker)(windows_core::Interface::as_raw(self), pcmnk, papmnk)
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn NextHitOffset(&self, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32 {
        (windows_core::Interface::vtable(self).NextHitOffset)(windows_core::Interface::as_raw(self), pcregion, paregion)
    }
}
#[repr(C)]
pub struct ISearchQueryHits_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> i32,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    Init: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NextHitMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut Option<super::Com::IMoniker>) -> i32,
    #[cfg(not(feature = "Win32_System_Com"))]
    NextHitMoniker: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub NextHitOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    NextHitOffset: usize,
}
windows_core::imp::define_interface!(ISearchRoot, ISearchRoot_Vtbl, 0x04c18ccf_1f57_4cbd_88cc_3900f5195ce3);
impl core::ops::Deref for ISearchRoot {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchRoot, windows_core::IUnknown);
impl ISearchRoot {
    pub unsafe fn SetSchedule<P0>(&self, psztaskarg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSchedule)(windows_core::Interface::as_raw(self), psztaskarg.param().abi()).ok()
    }
    pub unsafe fn Schedule(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Schedule)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRootURL<P0>(&self, pszurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRootURL)(windows_core::Interface::as_raw(self), pszurl.param().abi()).ok()
    }
    pub unsafe fn RootURL(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RootURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsHierarchical<P0>(&self, fishierarchical: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsHierarchical)(windows_core::Interface::as_raw(self), fishierarchical.param().abi()).ok()
    }
    pub unsafe fn IsHierarchical(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsHierarchical)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProvidesNotifications<P0>(&self, fprovidesnotifications: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetProvidesNotifications)(windows_core::Interface::as_raw(self), fprovidesnotifications.param().abi()).ok()
    }
    pub unsafe fn ProvidesNotifications(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProvidesNotifications)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUseNotificationsOnly<P0>(&self, fusenotificationsonly: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUseNotificationsOnly)(windows_core::Interface::as_raw(self), fusenotificationsonly.param().abi()).ok()
    }
    pub unsafe fn UseNotificationsOnly(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UseNotificationsOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnumerationDepth(&self, dwdepth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEnumerationDepth)(windows_core::Interface::as_raw(self), dwdepth).ok()
    }
    pub unsafe fn EnumerationDepth(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerationDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHostDepth(&self, dwdepth: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHostDepth)(windows_core::Interface::as_raw(self), dwdepth).ok()
    }
    pub unsafe fn HostDepth(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HostDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFollowDirectories<P0>(&self, ffollowdirectories: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFollowDirectories)(windows_core::Interface::as_raw(self), ffollowdirectories.param().abi()).ok()
    }
    pub unsafe fn FollowDirectories(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FollowDirectories)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthenticationType(&self, authtype: AUTH_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthenticationType)(windows_core::Interface::as_raw(self), authtype).ok()
    }
    pub unsafe fn AuthenticationType(&self) -> windows_core::Result<AUTH_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthenticationType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUser<P0>(&self, pszuser: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetUser)(windows_core::Interface::as_raw(self), pszuser.param().abi()).ok()
    }
    pub unsafe fn User(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).User)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPassword<P0>(&self, pszpassword: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), pszpassword.param().abi()).ok()
    }
    pub unsafe fn Password(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Password)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISearchRoot_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Schedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetRootURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RootURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetIsHierarchical: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsHierarchical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetProvidesNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ProvidesNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetUseNotificationsOnly: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub UseNotificationsOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnumerationDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumerationDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHostDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HostDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFollowDirectories: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FollowDirectories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, AUTH_TYPE) -> windows_core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AUTH_TYPE) -> windows_core::HRESULT,
    pub SetUser: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchScopeRule, ISearchScopeRule_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef53);
impl core::ops::Deref for ISearchScopeRule {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchScopeRule, windows_core::IUnknown);
impl ISearchScopeRule {
    pub unsafe fn PatternOrURL(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PatternOrURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsIncluded(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsIncluded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsDefault(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDefault)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FollowFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FollowFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISearchScopeRule_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PatternOrURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsIncluded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FollowFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchViewChangedSink, ISearchViewChangedSink_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef65);
impl core::ops::Deref for ISearchViewChangedSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchViewChangedSink, windows_core::IUnknown);
impl ISearchViewChangedSink {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnChange(&self, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnChange)(windows_core::Interface::as_raw(self), pdwdocid, pchange, pfinview).ok()
    }
}
#[repr(C)]
pub struct ISearchViewChangedSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, *const SEARCH_ITEM_CHANGE, *const super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnChange: usize,
}
windows_core::imp::define_interface!(ISecurityInfo, ISecurityInfo_Vtbl, 0x0c733aa4_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ISecurityInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISecurityInfo, windows_core::IUnknown);
impl ISecurityInfo {
    #[cfg(feature = "Win32_Security_Authorization")]
    pub unsafe fn GetCurrentTrustee(&self) -> windows_core::Result<*mut super::super::Security::Authorization::TRUSTEE_W> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentTrustee)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetObjectTypes(&self, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectTypes)(windows_core::Interface::as_raw(self), cobjecttypes, rgobjecttypes).ok()
    }
    pub unsafe fn GetPermissions(&self, objecttype: windows_core::GUID) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPermissions)(windows_core::Interface::as_raw(self), core::mem::transmute(objecttype), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISecurityInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub GetCurrentTrustee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    GetCurrentTrustee: usize,
    pub GetObjectTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IService, IService_Vtbl, 0x06210e88_01f5_11d1_b512_0080c781c384);
impl core::ops::Deref for IService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IService, windows_core::IUnknown);
impl IService {
    pub unsafe fn InvokeService<P0>(&self, punkinner: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).InvokeService)(windows_core::Interface::as_raw(self), punkinner.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InvokeService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISessionProperties, ISessionProperties_Vtbl, 0x0c733a85_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ISessionProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISessionProperties, windows_core::IUnknown);
impl ISessionProperties {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetProperties(&self, rgpropertyidsets: Option<&[DBPROPIDSET]>, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), rgpropertyidsets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertyidsets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcpropertysets, prgpropertysets).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn SetProperties(&self, rgpropertysets: Option<&mut [DBPROPSET]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProperties)(windows_core::Interface::as_raw(self), rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpropertysets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
}
#[repr(C)]
pub struct ISessionProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetProperties: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub SetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    SetProperties: usize,
}
windows_core::imp::define_interface!(ISimpleCommandCreator, ISimpleCommandCreator_Vtbl, 0x5e341ab7_02d0_11d1_900c_00a0c9063796);
impl core::ops::Deref for ISimpleCommandCreator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISimpleCommandCreator, windows_core::IUnknown);
impl ISimpleCommandCreator {
    pub unsafe fn CreateICommand<P0>(&self, ppiunknown: *mut Option<windows_core::IUnknown>, pouterunk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateICommand)(windows_core::Interface::as_raw(self), core::mem::transmute(ppiunknown), pouterunk.param().abi()).ok()
    }
    pub unsafe fn VerifyCatalog<P0, P1>(&self, pwszmachine: P0, pwszcatalogname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).VerifyCatalog)(windows_core::Interface::as_raw(self), pwszmachine.param().abi(), pwszcatalogname.param().abi()).ok()
    }
    pub unsafe fn GetDefaultCatalog<P0>(&self, pwszcatalogname: P0, cwcin: u32, pcwcout: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetDefaultCatalog)(windows_core::Interface::as_raw(self), pwszcatalogname.param().abi(), cwcin, pcwcout).ok()
    }
}
#[repr(C)]
pub struct ISimpleCommandCreator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateICommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VerifyCatalog: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDefaultCatalog: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISourcesRowset, ISourcesRowset_Vtbl, 0x0c733a1e_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ISourcesRowset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISourcesRowset, windows_core::IUnknown);
impl ISourcesRowset {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn GetSourcesRowset<P0>(&self, punkouter: P0, riid: *const windows_core::GUID, rgproperties: Option<&mut [DBPROPSET]>, ppsourcesrowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).GetSourcesRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, rgproperties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgproperties.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(ppsourcesrowset)).ok()
    }
}
#[repr(C)]
pub struct ISourcesRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub GetSourcesRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    GetSourcesRowset: usize,
}
windows_core::imp::define_interface!(IStemmer, IStemmer_Vtbl, 0xefbaf140_7f42_11ce_be57_00aa0051fe20);
impl core::ops::Deref for IStemmer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStemmer, windows_core::IUnknown);
impl IStemmer {
    pub unsafe fn Init(&self, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), ulmaxtokensize, pflicense).ok()
    }
    pub unsafe fn GenerateWordForms<P0, P1>(&self, pwcinbuf: P0, cwc: u32, pstemsink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWordFormSink>,
    {
        (windows_core::Interface::vtable(self).GenerateWordForms)(windows_core::Interface::as_raw(self), pwcinbuf.param().abi(), cwc, pstemsink.param().abi()).ok()
    }
    pub unsafe fn GetLicenseToUse(&self, ppwcslicense: *const *const u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLicenseToUse)(windows_core::Interface::as_raw(self), ppwcslicense).ok()
    }
}
#[repr(C)]
pub struct IStemmer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GenerateWordForms: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLicenseToUse: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISubscriptionItem, ISubscriptionItem_Vtbl, 0xa97559f8_6c4a_11d1_a1e8_00c04fc2fbe1);
impl core::ops::Deref for ISubscriptionItem {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISubscriptionItem, windows_core::IUnknown);
impl ISubscriptionItem {
    pub unsafe fn GetCookie(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSubscriptionItemInfo(&self, psubscriptioniteminfo: *mut SUBSCRIPTIONITEMINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSubscriptionItemInfo)(windows_core::Interface::as_raw(self), psubscriptioniteminfo).ok()
    }
    pub unsafe fn SetSubscriptionItemInfo(&self, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSubscriptionItemInfo)(windows_core::Interface::as_raw(self), psubscriptioniteminfo).ok()
    }
    pub unsafe fn ReadProperties(&self, ncount: u32, rgwszname: *const windows_core::PCWSTR, rgvalue: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReadProperties)(windows_core::Interface::as_raw(self), ncount, rgwszname, core::mem::transmute(rgvalue)).ok()
    }
    pub unsafe fn WriteProperties(&self, ncount: u32, rgwszname: *const windows_core::PCWSTR, rgvalue: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteProperties)(windows_core::Interface::as_raw(self), ncount, rgwszname, core::mem::transmute(rgvalue)).ok()
    }
    pub unsafe fn EnumProperties(&self) -> windows_core::Result<IEnumItemProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NotifyChanged(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyChanged)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISubscriptionItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetSubscriptionItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SUBSCRIPTIONITEMINFO) -> windows_core::HRESULT,
    pub SetSubscriptionItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const SUBSCRIPTIONITEMINFO) -> windows_core::HRESULT,
    pub ReadProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub WriteProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISubscriptionMgr, ISubscriptionMgr_Vtbl, 0x085fb2c0_0df8_11d1_8f4b_00a0c905413f);
impl core::ops::Deref for ISubscriptionMgr {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISubscriptionMgr, windows_core::IUnknown);
impl ISubscriptionMgr {
    pub unsafe fn DeleteSubscription<P0, P1>(&self, pwszurl: P0, hwnd: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).DeleteSubscription)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), hwnd.param().abi()).ok()
    }
    pub unsafe fn UpdateSubscription<P0>(&self, pwszurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UpdateSubscription)(windows_core::Interface::as_raw(self), pwszurl.param().abi()).ok()
    }
    pub unsafe fn UpdateAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateAll)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsSubscribed<P0>(&self, pwszurl: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSubscribed)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSubscriptionInfo<P0>(&self, pwszurl: P0, pinfo: *mut SUBSCRIPTIONINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetSubscriptionInfo)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), pinfo).ok()
    }
    pub unsafe fn GetDefaultInfo(&self, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDefaultInfo)(windows_core::Interface::as_raw(self), subtype, pinfo).ok()
    }
    pub unsafe fn ShowSubscriptionProperties<P0, P1>(&self, pwszurl: P0, hwnd: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ShowSubscriptionProperties)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), hwnd.param().abi()).ok()
    }
    pub unsafe fn CreateSubscription<P0, P1, P2>(&self, hwnd: P0, pwszurl: P1, pwszfriendlyname: P2, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CreateSubscription)(windows_core::Interface::as_raw(self), hwnd.param().abi(), pwszurl.param().abi(), pwszfriendlyname.param().abi(), dwflags, substype, pinfo).ok()
    }
}
#[repr(C)]
pub struct ISubscriptionMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeleteSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub UpdateSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UpdateAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSubscribed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSubscriptionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut SUBSCRIPTIONINFO) -> windows_core::HRESULT,
    pub GetDefaultInfo: unsafe extern "system" fn(*mut core::ffi::c_void, SUBSCRIPTIONTYPE, *mut SUBSCRIPTIONINFO) -> windows_core::HRESULT,
    pub ShowSubscriptionProperties: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub CreateSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCWSTR, windows_core::PCWSTR, u32, SUBSCRIPTIONTYPE, *mut SUBSCRIPTIONINFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISubscriptionMgr2, ISubscriptionMgr2_Vtbl, 0x614bc270_aedf_11d1_a1f9_00c04fc2fbe1);
impl core::ops::Deref for ISubscriptionMgr2 {
    type Target = ISubscriptionMgr;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISubscriptionMgr2, windows_core::IUnknown, ISubscriptionMgr);
impl ISubscriptionMgr2 {
    pub unsafe fn GetItemFromURL<P0>(&self, pwszurl: P0) -> windows_core::Result<ISubscriptionItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemFromURL)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetItemFromCookie(&self, psubscriptioncookie: *const windows_core::GUID) -> windows_core::Result<ISubscriptionItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemFromCookie)(windows_core::Interface::as_raw(self), psubscriptioncookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSubscriptionRunState(&self, dwnumcookies: u32, pcookies: *const windows_core::GUID, pdwrunstate: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSubscriptionRunState)(windows_core::Interface::as_raw(self), dwnumcookies, pcookies, pdwrunstate).ok()
    }
    pub unsafe fn EnumSubscriptions(&self, dwflags: u32) -> windows_core::Result<IEnumSubscription> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumSubscriptions)(windows_core::Interface::as_raw(self), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UpdateItems(&self, dwflags: u32, pcookies: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateItems)(windows_core::Interface::as_raw(self), dwflags, pcookies.len().try_into().unwrap(), core::mem::transmute(pcookies.as_ptr())).ok()
    }
    pub unsafe fn AbortItems(&self, pcookies: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AbortItems)(windows_core::Interface::as_raw(self), pcookies.len().try_into().unwrap(), core::mem::transmute(pcookies.as_ptr())).ok()
    }
    pub unsafe fn AbortAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AbortAll)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISubscriptionMgr2_Vtbl {
    pub base__: ISubscriptionMgr_Vtbl,
    pub GetItemFromURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemFromCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubscriptionRunState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub EnumSubscriptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateItems: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub AbortItems: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub AbortAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITableCreation, ITableCreation_Vtbl, 0x0c733abc_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ITableCreation {
    type Target = ITableDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITableCreation, windows_core::IUnknown, ITableDefinition);
impl ITableCreation {
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn GetTableDefinition(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: Option<*mut usize>, prgcolumndescs: Option<*mut *mut DBCOLUMNDESC>, pcpropertysets: Option<*mut u32>, prgpropertysets: Option<*mut *mut DBPROPSET>, pcconstraintdescs: Option<*mut u32>, prgconstraintdescs: Option<*mut *mut DBCONSTRAINTDESC>, ppwszstringbuffer: Option<*mut *mut u16>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTableDefinition)(
            windows_core::Interface::as_raw(self),
            ptableid,
            core::mem::transmute(pccolumndescs.unwrap_or(std::ptr::null_mut())),
            core::mem::transmute(prgcolumndescs.unwrap_or(std::ptr::null_mut())),
            core::mem::transmute(pcpropertysets.unwrap_or(std::ptr::null_mut())),
            core::mem::transmute(prgpropertysets.unwrap_or(std::ptr::null_mut())),
            core::mem::transmute(pcconstraintdescs.unwrap_or(std::ptr::null_mut())),
            core::mem::transmute(prgconstraintdescs.unwrap_or(std::ptr::null_mut())),
            core::mem::transmute(ppwszstringbuffer.unwrap_or(std::ptr::null_mut())),
        )
        .ok()
    }
}
#[repr(C)]
pub struct ITableCreation_Vtbl {
    pub base__: ITableDefinition_Vtbl,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub GetTableDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *mut usize, *mut *mut DBCOLUMNDESC, *mut u32, *mut *mut DBPROPSET, *mut u32, *mut *mut DBCONSTRAINTDESC, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    GetTableDefinition: usize,
}
windows_core::imp::define_interface!(ITableDefinition, ITableDefinition_Vtbl, 0x0c733a86_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ITableDefinition {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITableDefinition, windows_core::IUnknown);
impl ITableDefinition {
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn CreateTable<P0>(&self, punkouter: P0, ptableid: Option<*const super::super::Storage::IndexServer::DBID>, rgcolumndescs: Option<&[DBCOLUMNDESC]>, riid: *const windows_core::GUID, rgpropertysets: Option<&mut [DBPROPSET]>, pptableid: Option<*mut *mut super::super::Storage::IndexServer::DBID>, pprowset: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateTable)(
            windows_core::Interface::as_raw(self),
            punkouter.param().abi(),
            core::mem::transmute(ptableid.unwrap_or(std::ptr::null())),
            rgcolumndescs.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(rgcolumndescs.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            riid,
            rgpropertysets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(rgpropertysets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            core::mem::transmute(pptableid.unwrap_or(std::ptr::null_mut())),
            core::mem::transmute(pprowset.unwrap_or(std::ptr::null_mut())),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn DropTable(&self, ptableid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DropTable)(windows_core::Interface::as_raw(self), ptableid).ok()
    }
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn AddColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: Option<*mut *mut super::super::Storage::IndexServer::DBID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddColumn)(windows_core::Interface::as_raw(self), ptableid, pcolumndesc, core::mem::transmute(ppcolumnid.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn DropColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DropColumn)(windows_core::Interface::as_raw(self), ptableid, pcolumnid).ok()
    }
}
#[repr(C)]
pub struct ITableDefinition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub CreateTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, usize, *const DBCOLUMNDESC, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut super::super::Storage::IndexServer::DBID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    CreateTable: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DropTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DropTable: usize,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub AddColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const DBCOLUMNDESC, *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    AddColumn: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DropColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DropColumn: usize,
}
windows_core::imp::define_interface!(ITableDefinitionWithConstraints, ITableDefinitionWithConstraints_Vtbl, 0x0c733aab_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ITableDefinitionWithConstraints {
    type Target = ITableCreation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITableDefinitionWithConstraints, windows_core::IUnknown, ITableDefinition, ITableCreation);
impl ITableDefinitionWithConstraints {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn AddConstraint(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pconstraintdesc: *const DBCONSTRAINTDESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddConstraint)(windows_core::Interface::as_raw(self), ptableid, pconstraintdesc).ok()
    }
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn CreateTableWithConstraints<P0>(&self, punkouter: P0, ptableid: *const super::super::Storage::IndexServer::DBID, rgcolumndescs: &mut [DBCOLUMNDESC], rgconstraintdescs: &[DBCONSTRAINTDESC], riid: *const windows_core::GUID, rgpropertysets: &mut [DBPROPSET], pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateTableWithConstraints)(windows_core::Interface::as_raw(self), punkouter.param().abi(), ptableid, rgcolumndescs.len().try_into().unwrap(), core::mem::transmute(rgcolumndescs.as_ptr()), rgconstraintdescs.len().try_into().unwrap(), core::mem::transmute(rgconstraintdescs.as_ptr()), riid, rgpropertysets.len().try_into().unwrap(), core::mem::transmute(rgpropertysets.as_ptr()), pptableid, core::mem::transmute(pprowset)).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn DropConstraint(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pconstraintid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DropConstraint)(windows_core::Interface::as_raw(self), ptableid, pconstraintid).ok()
    }
}
#[repr(C)]
pub struct ITableDefinitionWithConstraints_Vtbl {
    pub base__: ITableCreation_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub AddConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const DBCONSTRAINTDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    AddConstraint: usize,
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub CreateTableWithConstraints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, usize, *mut DBCOLUMNDESC, u32, *const DBCONSTRAINTDESC, *const windows_core::GUID, u32, *mut DBPROPSET, *mut *mut super::super::Storage::IndexServer::DBID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com")))]
    CreateTableWithConstraints: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub DropConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    DropConstraint: usize,
}
windows_core::imp::define_interface!(ITableRename, ITableRename_Vtbl, 0x0c733a77_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ITableRename {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITableRename, windows_core::IUnknown);
impl ITableRename {
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn RenameColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, poldcolumnid: *const super::super::Storage::IndexServer::DBID, pnewcolumnid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RenameColumn)(windows_core::Interface::as_raw(self), ptableid, poldcolumnid, pnewcolumnid).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn RenameTable(&self, poldtableid: *const super::super::Storage::IndexServer::DBID, poldindexid: *const super::super::Storage::IndexServer::DBID, pnewtableid: *const super::super::Storage::IndexServer::DBID, pnewindexid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RenameTable)(windows_core::Interface::as_raw(self), poldtableid, poldindexid, pnewtableid, pnewindexid).ok()
    }
}
#[repr(C)]
pub struct ITableRename_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub RenameColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    RenameColumn: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub RenameTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID, *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    RenameTable: usize,
}
windows_core::imp::define_interface!(ITokenCollection, ITokenCollection_Vtbl, 0x22d8b4f2_f577_4adb_a335_c2ae88416fab);
impl core::ops::Deref for ITokenCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITokenCollection, windows_core::IUnknown);
impl ITokenCollection {
    pub unsafe fn NumberOfTokens(&self, pcount: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NumberOfTokens)(windows_core::Interface::as_raw(self), pcount).ok()
    }
    pub unsafe fn GetToken(&self, i: u32, pbegin: Option<*mut u32>, plength: Option<*mut u32>, ppsz: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetToken)(windows_core::Interface::as_raw(self), i, core::mem::transmute(pbegin.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppsz.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct ITokenCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NumberOfTokens: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub GetToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransactionJoin, ITransactionJoin_Vtbl, 0x0c733a5e_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ITransactionJoin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionJoin, windows_core::IUnknown);
impl ITransactionJoin {
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn GetOptionsObject(&self) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransactionOptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOptionsObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn JoinTransaction<P0, P1>(&self, punktransactioncoord: P0, isolevel: i32, isoflags: u32, potheroptions: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::DistributedTransactionCoordinator::ITransactionOptions>,
    {
        (windows_core::Interface::vtable(self).JoinTransaction)(windows_core::Interface::as_raw(self), punktransactioncoord.param().abi(), isolevel, isoflags, potheroptions.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITransactionJoin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub GetOptionsObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    GetOptionsObject: usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub JoinTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    JoinTransaction: usize,
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
windows_core::imp::define_interface!(ITransactionLocal, ITransactionLocal_Vtbl, 0x0c733a5f_2a1c_11ce_ade5_00aa0044773d);
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl core::ops::Deref for ITransactionLocal {
    type Target = super::DistributedTransactionCoordinator::ITransaction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
windows_core::imp::interface_hierarchy!(ITransactionLocal, windows_core::IUnknown, super::DistributedTransactionCoordinator::ITransaction);
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionLocal {
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn GetOptionsObject(&self) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransactionOptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOptionsObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn StartTransaction<P0>(&self, isolevel: i32, isoflags: u32, potheroptions: P0, pultransactionlevel: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DistributedTransactionCoordinator::ITransactionOptions>,
    {
        (windows_core::Interface::vtable(self).StartTransaction)(windows_core::Interface::as_raw(self), isolevel, isoflags, potheroptions.param().abi(), core::mem::transmute(pultransactionlevel.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
#[repr(C)]
pub struct ITransactionLocal_Vtbl {
    pub base__: super::DistributedTransactionCoordinator::ITransaction_Vtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub GetOptionsObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    GetOptionsObject: usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub StartTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    StartTransaction: usize,
}
windows_core::imp::define_interface!(ITransactionObject, ITransactionObject_Vtbl, 0x0c733a60_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ITransactionObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionObject, windows_core::IUnknown);
impl ITransactionObject {
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn GetTransactionObject(&self, ultransactionlevel: u32) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransaction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransactionObject)(windows_core::Interface::as_raw(self), ultransactionlevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITransactionObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub GetTransactionObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    GetTransactionObject: usize,
}
windows_core::imp::define_interface!(ITrusteeAdmin, ITrusteeAdmin_Vtbl, 0x0c733aa1_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ITrusteeAdmin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITrusteeAdmin, windows_core::IUnknown);
impl ITrusteeAdmin {
    #[cfg(feature = "Win32_Security_Authorization")]
    pub unsafe fn CompareTrustees(&self, ptrustee1: *const super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CompareTrustees)(windows_core::Interface::as_raw(self), ptrustee1, ptrustee2).ok()
    }
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub unsafe fn CreateTrustee(&self, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, rgpropertysets: &mut [DBPROPSET]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateTrustee)(windows_core::Interface::as_raw(self), ptrustee, rgpropertysets.len().try_into().unwrap(), core::mem::transmute(rgpropertysets.as_ptr())).ok()
    }
    #[cfg(feature = "Win32_Security_Authorization")]
    pub unsafe fn DeleteTrustee(&self, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteTrustee)(windows_core::Interface::as_raw(self), ptrustee).ok()
    }
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub unsafe fn SetTrusteeProperties(&self, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, rgpropertysets: &mut [DBPROPSET]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTrusteeProperties)(windows_core::Interface::as_raw(self), ptrustee, rgpropertysets.len().try_into().unwrap(), core::mem::transmute(rgpropertysets.as_ptr())).ok()
    }
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub unsafe fn GetTrusteeProperties(&self, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, rgpropertyidsets: &[DBPROPIDSET], pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTrusteeProperties)(windows_core::Interface::as_raw(self), ptrustee, rgpropertyidsets.len().try_into().unwrap(), core::mem::transmute(rgpropertyidsets.as_ptr()), pcpropertysets, prgpropertysets).ok()
    }
}
#[repr(C)]
pub struct ITrusteeAdmin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub CompareTrustees: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Authorization::TRUSTEE_W, *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    CompareTrustees: usize,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub CreateTrustee: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Authorization::TRUSTEE_W, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    CreateTrustee: usize,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub DeleteTrustee: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    DeleteTrustee: usize,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub SetTrusteeProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Authorization::TRUSTEE_W, u32, *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    SetTrusteeProperties: usize,
    #[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
    pub GetTrusteeProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Authorization::TRUSTEE_W, u32, *const DBPROPIDSET, *mut u32, *mut *mut DBPROPSET) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer")))]
    GetTrusteeProperties: usize,
}
windows_core::imp::define_interface!(ITrusteeGroupAdmin, ITrusteeGroupAdmin_Vtbl, 0x0c733aa2_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ITrusteeGroupAdmin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITrusteeGroupAdmin, windows_core::IUnknown);
impl ITrusteeGroupAdmin {
    #[cfg(feature = "Win32_Security_Authorization")]
    pub unsafe fn AddMember(&self, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddMember)(windows_core::Interface::as_raw(self), pmembershiptrustee, pmembertrustee).ok()
    }
    #[cfg(feature = "Win32_Security_Authorization")]
    pub unsafe fn DeleteMember(&self, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteMember)(windows_core::Interface::as_raw(self), pmembershiptrustee, pmembertrustee).ok()
    }
    #[cfg(feature = "Win32_Security_Authorization")]
    pub unsafe fn IsMember(&self, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsMember)(windows_core::Interface::as_raw(self), pmembershiptrustee, pmembertrustee, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Security_Authorization")]
    pub unsafe fn GetMembers(&self, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMembers)(windows_core::Interface::as_raw(self), pmembershiptrustee, pcmembers, prgmembers).ok()
    }
    #[cfg(feature = "Win32_Security_Authorization")]
    pub unsafe fn GetMemberships(&self, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMemberships)(windows_core::Interface::as_raw(self), ptrustee, pcmemberships, prgmemberships).ok()
    }
}
#[repr(C)]
pub struct ITrusteeGroupAdmin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub AddMember: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Authorization::TRUSTEE_W, *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    AddMember: usize,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub DeleteMember: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Authorization::TRUSTEE_W, *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    DeleteMember: usize,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub IsMember: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Authorization::TRUSTEE_W, *const super::super::Security::Authorization::TRUSTEE_W, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    IsMember: usize,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub GetMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Authorization::TRUSTEE_W, *mut u32, *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    GetMembers: usize,
    #[cfg(feature = "Win32_Security_Authorization")]
    pub GetMemberships: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Authorization::TRUSTEE_W, *mut u32, *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Authorization"))]
    GetMemberships: usize,
}
windows_core::imp::define_interface!(IUMS, IUMS_Vtbl);
impl IUMS {
    pub unsafe fn SqlUmsSuspend(&self, ticks: u32) {
        (windows_core::Interface::vtable(self).SqlUmsSuspend)(windows_core::Interface::as_raw(self), ticks)
    }
    pub unsafe fn SqlUmsYield(&self, ticks: u32) {
        (windows_core::Interface::vtable(self).SqlUmsYield)(windows_core::Interface::as_raw(self), ticks)
    }
    pub unsafe fn SqlUmsSwitchPremptive(&self) {
        (windows_core::Interface::vtable(self).SqlUmsSwitchPremptive)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SqlUmsSwitchNonPremptive(&self) {
        (windows_core::Interface::vtable(self).SqlUmsSwitchNonPremptive)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SqlUmsFIsPremptive(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).SqlUmsFIsPremptive)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IUMS_Vtbl {
    pub SqlUmsSuspend: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub SqlUmsYield: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub SqlUmsSwitchPremptive: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SqlUmsSwitchNonPremptive: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SqlUmsFIsPremptive: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(IUMSInitialize, IUMSInitialize_Vtbl, 0x5cf4ca14_ef21_11d0_97e7_00c04fc2ad98);
impl core::ops::Deref for IUMSInitialize {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUMSInitialize, windows_core::IUnknown);
impl IUMSInitialize {
    pub unsafe fn Initialize(&self, pums: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pums).ok()
    }
}
#[repr(C)]
pub struct IUMSInitialize_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUrlAccessor, IUrlAccessor_Vtbl, 0x0b63e318_9ccc_11d0_bcdb_00805fccce04);
impl core::ops::Deref for IUrlAccessor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUrlAccessor, windows_core::IUnknown);
impl IUrlAccessor {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn AddRequestParameter(&self, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddRequestParameter)(windows_core::Interface::as_raw(self), pspec, core::mem::transmute(pvar)).ok()
    }
    pub unsafe fn GetDocFormat(&self, wszdocformat: &mut [u16], pdwlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDocFormat)(windows_core::Interface::as_raw(self), core::mem::transmute(wszdocformat.as_ptr()), wszdocformat.len().try_into().unwrap(), pdwlength).ok()
    }
    pub unsafe fn GetCLSID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCLSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetHost(&self, wszhost: &mut [u16], pdwlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHost)(windows_core::Interface::as_raw(self), core::mem::transmute(wszhost.as_ptr()), wszhost.len().try_into().unwrap(), pdwlength).ok()
    }
    pub unsafe fn IsDirectory(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsDirectory)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLastModified(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFileName(&self, wszfilename: &mut [u16], pdwlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFileName)(windows_core::Interface::as_raw(self), core::mem::transmute(wszfilename.as_ptr()), wszfilename.len().try_into().unwrap(), pdwlength).ok()
    }
    pub unsafe fn GetSecurityDescriptor(&self, psd: &mut [u8], pdwlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute(psd.as_ptr()), psd.len().try_into().unwrap(), pdwlength).ok()
    }
    pub unsafe fn GetRedirectedURL(&self, wszredirectedurl: &mut [u16], pdwlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRedirectedURL)(windows_core::Interface::as_raw(self), core::mem::transmute(wszredirectedurl.as_ptr()), wszredirectedurl.len().try_into().unwrap(), pdwlength).ok()
    }
    pub unsafe fn GetSecurityProvider(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecurityProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BindToStream(&self) -> windows_core::Result<super::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BindToStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn BindToFilter(&self) -> windows_core::Result<super::super::Storage::IndexServer::IFilter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BindToFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUrlAccessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub AddRequestParameter: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::StructuredStorage::PROPSPEC, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    AddRequestParameter: usize,
    pub GetDocFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetHost: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub IsDirectory: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetLastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetRedirectedURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetSecurityProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BindToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BindToStream: usize,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub BindToFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    BindToFilter: usize,
}
windows_core::imp::define_interface!(IUrlAccessor2, IUrlAccessor2_Vtbl, 0xc7310734_ac80_11d1_8df3_00c04fb6ef4f);
impl core::ops::Deref for IUrlAccessor2 {
    type Target = IUrlAccessor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUrlAccessor2, windows_core::IUnknown, IUrlAccessor);
impl IUrlAccessor2 {
    pub unsafe fn GetDisplayUrl(&self, wszdocurl: &mut [u16], pdwlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDisplayUrl)(windows_core::Interface::as_raw(self), core::mem::transmute(wszdocurl.as_ptr()), wszdocurl.len().try_into().unwrap(), pdwlength).ok()
    }
    pub unsafe fn IsDocument(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsDocument)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCodePage(&self, wszcodepage: &mut [u16], pdwlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodePage)(windows_core::Interface::as_raw(self), core::mem::transmute(wszcodepage.as_ptr()), wszcodepage.len().try_into().unwrap(), pdwlength).ok()
    }
}
#[repr(C)]
pub struct IUrlAccessor2_Vtbl {
    pub base__: IUrlAccessor_Vtbl,
    pub GetDisplayUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub IsDocument: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUrlAccessor3, IUrlAccessor3_Vtbl, 0x6fbc7005_0455_4874_b8ff_7439450241a3);
impl core::ops::Deref for IUrlAccessor3 {
    type Target = IUrlAccessor2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUrlAccessor3, windows_core::IUnknown, IUrlAccessor, IUrlAccessor2);
impl IUrlAccessor3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetImpersonationSidBlobs<P0>(&self, pcwszurl: P0, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetImpersonationSidBlobs)(windows_core::Interface::as_raw(self), pcwszurl.param().abi(), pcsidcount, ppsidblobs).ok()
    }
}
#[repr(C)]
pub struct IUrlAccessor3_Vtbl {
    pub base__: IUrlAccessor2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetImpersonationSidBlobs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut super::Com::BLOB) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetImpersonationSidBlobs: usize,
}
windows_core::imp::define_interface!(IUrlAccessor4, IUrlAccessor4_Vtbl, 0x5cc51041_c8d2_41d7_bca3_9e9e286297dc);
impl core::ops::Deref for IUrlAccessor4 {
    type Target = IUrlAccessor3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUrlAccessor4, windows_core::IUnknown, IUrlAccessor, IUrlAccessor2, IUrlAccessor3);
impl IUrlAccessor4 {
    pub unsafe fn ShouldIndexItemContent(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShouldIndexItemContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn ShouldIndexProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShouldIndexProperty)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUrlAccessor4_Vtbl {
    pub base__: IUrlAccessor3_Vtbl,
    pub ShouldIndexItemContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub ShouldIndexProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    ShouldIndexProperty: usize,
}
windows_core::imp::define_interface!(IViewChapter, IViewChapter_Vtbl, 0x0c733a98_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IViewChapter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewChapter, windows_core::IUnknown);
impl IViewChapter {
    pub unsafe fn GetSpecification(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSpecification)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenViewChapter(&self, hsource: usize, phviewchapter: Option<*mut usize>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenViewChapter)(windows_core::Interface::as_raw(self), hsource, core::mem::transmute(phviewchapter.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IViewChapter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSpecification: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenViewChapter: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewFilter, IViewFilter_Vtbl, 0x0c733a9b_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IViewFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewFilter, windows_core::IUnknown);
impl IViewFilter {
    pub unsafe fn GetFilter<P0>(&self, haccessor: P0, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).GetFilter)(windows_core::Interface::as_raw(self), haccessor.param().abi(), pcrows, pcompareops, pcriteriadata).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFilterBindings(&self, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFilterBindings)(windows_core::Interface::as_raw(self), pcbindings, prgbindings).ok()
    }
    pub unsafe fn SetFilter<P0>(&self, haccessor: P0, compareops: &[u32], pcriteriadata: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HACCESSOR>,
    {
        (windows_core::Interface::vtable(self).SetFilter)(windows_core::Interface::as_raw(self), haccessor.param().abi(), compareops.len().try_into().unwrap(), core::mem::transmute(compareops.as_ptr()), pcriteriadata).ok()
    }
}
#[repr(C)]
pub struct IViewFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, *mut usize, *mut *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFilterBindings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut *mut DBBINDING) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFilterBindings: usize,
    pub SetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, HACCESSOR, usize, *const u32, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewRowset, IViewRowset_Vtbl, 0x0c733a97_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IViewRowset {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewRowset, windows_core::IUnknown);
impl IViewRowset {
    pub unsafe fn GetSpecification(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSpecification)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenViewRowset<P0>(&self, punkouter: P0, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenViewRowset)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IViewRowset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSpecification: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenViewRowset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewSort, IViewSort_Vtbl, 0x0c733a9a_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for IViewSort {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewSort, windows_core::IUnknown);
impl IViewSort {
    pub unsafe fn GetSortOrder(&self, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSortOrder)(windows_core::Interface::as_raw(self), pcvalues, prgcolumns, prgorders).ok()
    }
    pub unsafe fn SetSortOrder(&self, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSortOrder)(windows_core::Interface::as_raw(self), cvalues, rgcolumns, rgorders).ok()
    }
}
#[repr(C)]
pub struct IViewSort_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSortOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut *mut usize, *mut *mut u32) -> windows_core::HRESULT,
    pub SetSortOrder: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const usize, *const u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWordBreaker, IWordBreaker_Vtbl, 0xd53552c8_77e3_101a_b552_08002b33b0e6);
impl core::ops::Deref for IWordBreaker {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWordBreaker, windows_core::IUnknown);
impl IWordBreaker {
    pub unsafe fn Init<P0>(&self, fquery: P0, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), fquery.param().abi(), ulmaxtokensize, pflicense).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn BreakText<P0, P1>(&self, ptextsource: *mut TEXT_SOURCE, pwordsink: P0, pphrasesink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWordSink>,
        P1: windows_core::Param<super::super::Storage::IndexServer::IPhraseSink>,
    {
        (windows_core::Interface::vtable(self).BreakText)(windows_core::Interface::as_raw(self), ptextsource, pwordsink.param().abi(), pphrasesink.param().abi()).ok()
    }
    pub unsafe fn ComposePhrase<P0, P1, P2>(&self, pwcnoun: P0, cwcnoun: u32, pwcmodifier: P1, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: P2, pcwcphrase: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ComposePhrase)(windows_core::Interface::as_raw(self), pwcnoun.param().abi(), cwcnoun, pwcmodifier.param().abi(), cwcmodifier, ulattachmenttype, pwcphrase.param().abi(), pcwcphrase).ok()
    }
    pub unsafe fn GetLicenseToUse(&self, ppwcslicense: *const *const u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLicenseToUse)(windows_core::Interface::as_raw(self), ppwcslicense).ok()
    }
}
#[repr(C)]
pub struct IWordBreaker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub BreakText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TEXT_SOURCE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    BreakText: usize,
    pub ComposePhrase: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, u32, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetLicenseToUse: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWordFormSink, IWordFormSink_Vtbl, 0xfe77c330_7f42_11ce_be57_00aa0051fe20);
impl core::ops::Deref for IWordFormSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWordFormSink, windows_core::IUnknown);
impl IWordFormSink {
    pub unsafe fn PutAltWord<P0>(&self, pwcinbuf: P0, cwc: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PutAltWord)(windows_core::Interface::as_raw(self), pwcinbuf.param().abi(), cwc).ok()
    }
    pub unsafe fn PutWord<P0>(&self, pwcinbuf: P0, cwc: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PutWord)(windows_core::Interface::as_raw(self), pwcinbuf.param().abi(), cwc).ok()
    }
}
#[repr(C)]
pub struct IWordFormSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutAltWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub PutWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWordSink, IWordSink_Vtbl, 0xcc907054_c058_101a_b554_08002b33b0e6);
impl core::ops::Deref for IWordSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWordSink, windows_core::IUnknown);
impl IWordSink {
    pub unsafe fn PutWord<P0>(&self, cwc: u32, pwcinbuf: P0, cwcsrclen: u32, cwcsrcpos: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PutWord)(windows_core::Interface::as_raw(self), cwc, pwcinbuf.param().abi(), cwcsrclen, cwcsrcpos).ok()
    }
    pub unsafe fn PutAltWord<P0>(&self, cwc: u32, pwcinbuf: P0, cwcsrclen: u32, cwcsrcpos: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PutAltWord)(windows_core::Interface::as_raw(self), cwc, pwcinbuf.param().abi(), cwcsrclen, cwcsrcpos).ok()
    }
    pub unsafe fn StartAltPhrase(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartAltPhrase)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndAltPhrase(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndAltPhrase)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn PutBreak(&self, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PutBreak)(windows_core::Interface::as_raw(self), breaktype).ok()
    }
}
#[repr(C)]
pub struct IWordSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutWord: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub PutAltWord: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub StartAltPhrase: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndAltPhrase: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub PutBreak: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_IndexServer"))]
    PutBreak: usize,
}
windows_core::imp::define_interface!(OLEDBSimpleProvider, OLEDBSimpleProvider_Vtbl, 0xe0e270c0_c0be_11d0_8fe4_00a0c90a6341);
impl core::ops::Deref for OLEDBSimpleProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(OLEDBSimpleProvider, windows_core::IUnknown);
impl OLEDBSimpleProvider {
    pub unsafe fn getRowCount(&self) -> windows_core::Result<isize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getRowCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn getColumnCount(&self) -> windows_core::Result<isize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn getRWStatus(&self, irow: isize, icolumn: isize) -> windows_core::Result<OSPRW> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getRWStatus)(windows_core::Interface::as_raw(self), irow, icolumn, &mut result__).map(|| result__)
    }
    pub unsafe fn getVariant(&self, irow: isize, icolumn: isize, format: OSPFORMAT) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getVariant)(windows_core::Interface::as_raw(self), irow, icolumn, format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn setVariant<P0>(&self, irow: isize, icolumn: isize, format: OSPFORMAT, var: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).setVariant)(windows_core::Interface::as_raw(self), irow, icolumn, format, var.param().abi()).ok()
    }
    pub unsafe fn getLocale(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getLocale)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn deleteRows(&self, irow: isize, crows: isize) -> windows_core::Result<isize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).deleteRows)(windows_core::Interface::as_raw(self), irow, crows, &mut result__).map(|| result__)
    }
    pub unsafe fn insertRows(&self, irow: isize, crows: isize) -> windows_core::Result<isize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).insertRows)(windows_core::Interface::as_raw(self), irow, crows, &mut result__).map(|| result__)
    }
    pub unsafe fn find<P0>(&self, irowstart: isize, icolumn: isize, val: P0, findflags: OSPFIND, comptype: OSPCOMP) -> windows_core::Result<isize>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).find)(windows_core::Interface::as_raw(self), irowstart, icolumn, val.param().abi(), findflags, comptype, &mut result__).map(|| result__)
    }
    pub unsafe fn addOLEDBSimpleProviderListener<P0>(&self, pospilistener: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<OLEDBSimpleProviderListener>,
    {
        (windows_core::Interface::vtable(self).addOLEDBSimpleProviderListener)(windows_core::Interface::as_raw(self), pospilistener.param().abi()).ok()
    }
    pub unsafe fn removeOLEDBSimpleProviderListener<P0>(&self, pospilistener: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<OLEDBSimpleProviderListener>,
    {
        (windows_core::Interface::vtable(self).removeOLEDBSimpleProviderListener)(windows_core::Interface::as_raw(self), pospilistener.param().abi()).ok()
    }
    pub unsafe fn isAsync(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).isAsync)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn getEstimatedRows(&self) -> windows_core::Result<isize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getEstimatedRows)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn stopTransfer(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).stopTransfer)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct OLEDBSimpleProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub getRowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut isize) -> windows_core::HRESULT,
    pub getColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut isize) -> windows_core::HRESULT,
    pub getRWStatus: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize, *mut OSPRW) -> windows_core::HRESULT,
    pub getVariant: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize, OSPFORMAT, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub setVariant: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize, OSPFORMAT, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub getLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub deleteRows: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize, *mut isize) -> windows_core::HRESULT,
    pub insertRows: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize, *mut isize) -> windows_core::HRESULT,
    pub find: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize, core::mem::MaybeUninit<windows_core::VARIANT>, OSPFIND, OSPCOMP, *mut isize) -> windows_core::HRESULT,
    pub addOLEDBSimpleProviderListener: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removeOLEDBSimpleProviderListener: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub isAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub getEstimatedRows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut isize) -> windows_core::HRESULT,
    pub stopTransfer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(OLEDBSimpleProviderListener, OLEDBSimpleProviderListener_Vtbl, 0xe0e270c1_c0be_11d0_8fe4_00a0c90a6341);
impl core::ops::Deref for OLEDBSimpleProviderListener {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(OLEDBSimpleProviderListener, windows_core::IUnknown);
impl OLEDBSimpleProviderListener {
    pub unsafe fn aboutToChangeCell(&self, irow: isize, icolumn: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).aboutToChangeCell)(windows_core::Interface::as_raw(self), irow, icolumn).ok()
    }
    pub unsafe fn cellChanged(&self, irow: isize, icolumn: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).cellChanged)(windows_core::Interface::as_raw(self), irow, icolumn).ok()
    }
    pub unsafe fn aboutToDeleteRows(&self, irow: isize, crows: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).aboutToDeleteRows)(windows_core::Interface::as_raw(self), irow, crows).ok()
    }
    pub unsafe fn deletedRows(&self, irow: isize, crows: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).deletedRows)(windows_core::Interface::as_raw(self), irow, crows).ok()
    }
    pub unsafe fn aboutToInsertRows(&self, irow: isize, crows: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).aboutToInsertRows)(windows_core::Interface::as_raw(self), irow, crows).ok()
    }
    pub unsafe fn insertedRows(&self, irow: isize, crows: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).insertedRows)(windows_core::Interface::as_raw(self), irow, crows).ok()
    }
    pub unsafe fn rowsAvailable(&self, irow: isize, crows: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).rowsAvailable)(windows_core::Interface::as_raw(self), irow, crows).ok()
    }
    pub unsafe fn transferComplete(&self, xfer: OSPXFER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).transferComplete)(windows_core::Interface::as_raw(self), xfer).ok()
    }
}
#[repr(C)]
pub struct OLEDBSimpleProviderListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub aboutToChangeCell: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize) -> windows_core::HRESULT,
    pub cellChanged: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize) -> windows_core::HRESULT,
    pub aboutToDeleteRows: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize) -> windows_core::HRESULT,
    pub deletedRows: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize) -> windows_core::HRESULT,
    pub aboutToInsertRows: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize) -> windows_core::HRESULT,
    pub insertedRows: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize) -> windows_core::HRESULT,
    pub rowsAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, isize, isize) -> windows_core::HRESULT,
    pub transferComplete: unsafe extern "system" fn(*mut core::ffi::c_void, OSPXFER) -> windows_core::HRESULT,
}
pub const BCP6xFILEFMT: u32 = 9u32;
pub const BCPABORT: u32 = 6u32;
pub const BCPBATCH: u32 = 4u32;
pub const BCPFILECP: u32 = 12u32;
pub const BCPFILECP_ACP: u32 = 0u32;
pub const BCPFILECP_OEMCP: u32 = 1u32;
pub const BCPFILECP_RAW: i32 = -1i32;
pub const BCPFILEFMT: u32 = 15u32;
pub const BCPFIRST: u32 = 2u32;
pub const BCPHINTS: u32 = 11u32;
pub const BCPHINTSA: u32 = 10u32;
pub const BCPHINTSW: u32 = 11u32;
pub const BCPKEEPIDENTITY: u32 = 8u32;
pub const BCPKEEPNULLS: u32 = 5u32;
pub const BCPLAST: u32 = 3u32;
pub const BCPMAXERRS: u32 = 1u32;
pub const BCPODBC: u32 = 7u32;
pub const BCPTEXTFILE: u32 = 14u32;
pub const BCPUNICODEFILE: u32 = 13u32;
pub const BCP_FMT_COLLATION: u32 = 6u32;
pub const BCP_FMT_COLLATION_ID: u32 = 7u32;
pub const BCP_FMT_DATA_LEN: u32 = 3u32;
pub const BCP_FMT_INDICATOR_LEN: u32 = 2u32;
pub const BCP_FMT_SERVER_COL: u32 = 5u32;
pub const BCP_FMT_TERMINATOR: u32 = 4u32;
pub const BCP_FMT_TYPE: u32 = 1u32;
pub const BIO_BINDER: EBindInfoOptions = EBindInfoOptions(1i32);
pub const BMK_DURABILITY_INTRANSACTION: i32 = 1i32;
pub const BMK_DURABILITY_REORGANIZATION: i32 = 3i32;
pub const BMK_DURABILITY_ROWSET: i32 = 0i32;
pub const BMK_DURABILITY_XTRANSACTION: i32 = 2i32;
pub const BUCKET_EXPONENTIAL: u32 = 1u32;
pub const BUCKET_LINEAR: u32 = 0u32;
pub const CASE_REQUIREMENT_ANY: CASE_REQUIREMENT = CASE_REQUIREMENT(0i32);
pub const CASE_REQUIREMENT_UPPER_IF_AQS: CASE_REQUIREMENT = CASE_REQUIREMENT(1i32);
pub const CATALOG_PAUSED_REASON_DELAYED_RECOVERY: CatalogPausedReason = CatalogPausedReason(7i32);
pub const CATALOG_PAUSED_REASON_EXTERNAL: CatalogPausedReason = CatalogPausedReason(9i32);
pub const CATALOG_PAUSED_REASON_HIGH_CPU: CatalogPausedReason = CatalogPausedReason(2i32);
pub const CATALOG_PAUSED_REASON_HIGH_IO: CatalogPausedReason = CatalogPausedReason(1i32);
pub const CATALOG_PAUSED_REASON_HIGH_NTF_RATE: CatalogPausedReason = CatalogPausedReason(3i32);
pub const CATALOG_PAUSED_REASON_LOW_BATTERY: CatalogPausedReason = CatalogPausedReason(4i32);
pub const CATALOG_PAUSED_REASON_LOW_DISK: CatalogPausedReason = CatalogPausedReason(6i32);
pub const CATALOG_PAUSED_REASON_LOW_MEMORY: CatalogPausedReason = CatalogPausedReason(5i32);
pub const CATALOG_PAUSED_REASON_NONE: CatalogPausedReason = CatalogPausedReason(0i32);
pub const CATALOG_PAUSED_REASON_UPGRADING: CatalogPausedReason = CatalogPausedReason(10i32);
pub const CATALOG_PAUSED_REASON_USER_ACTIVE: CatalogPausedReason = CatalogPausedReason(8i32);
pub const CATALOG_STATUS_FULL_CRAWL: CatalogStatus = CatalogStatus(3i32);
pub const CATALOG_STATUS_IDLE: CatalogStatus = CatalogStatus(0i32);
pub const CATALOG_STATUS_INCREMENTAL_CRAWL: CatalogStatus = CatalogStatus(4i32);
pub const CATALOG_STATUS_PAUSED: CatalogStatus = CatalogStatus(1i32);
pub const CATALOG_STATUS_PROCESSING_NOTIFICATIONS: CatalogStatus = CatalogStatus(5i32);
pub const CATALOG_STATUS_RECOVERING: CatalogStatus = CatalogStatus(2i32);
pub const CATALOG_STATUS_SHUTTING_DOWN: CatalogStatus = CatalogStatus(6i32);
pub const CATEGORIZE_BUCKETS: u32 = 2u32;
pub const CATEGORIZE_CLUSTER: u32 = 1u32;
pub const CATEGORIZE_RANGE: u32 = 3u32;
pub const CATEGORIZE_UNIQUE: u32 = 0u32;
pub const CATEGORY_COLLATOR: i32 = 2i32;
pub const CATEGORY_GATHERER: i32 = 3i32;
pub const CATEGORY_INDEXER: i32 = 4i32;
pub const CATEGORY_SEARCH: i32 = 1i32;
pub const CDBBMKDISPIDS: u32 = 8u32;
pub const CDBCOLDISPIDS: u32 = 28u32;
pub const CDBSELFDISPIDS: u32 = 8u32;
pub const CERT_E_NOT_FOUND_OR_NO_PERMISSSION: i32 = -2147211263i32;
pub const CHANNEL_AGENT_DYNAMIC_SCHEDULE: CHANNEL_AGENT_FLAGS = CHANNEL_AGENT_FLAGS(1i32);
pub const CHANNEL_AGENT_PRECACHE_ALL: CHANNEL_AGENT_FLAGS = CHANNEL_AGENT_FLAGS(4i32);
pub const CHANNEL_AGENT_PRECACHE_SCRNSAVER: CHANNEL_AGENT_FLAGS = CHANNEL_AGENT_FLAGS(8i32);
pub const CHANNEL_AGENT_PRECACHE_SOME: CHANNEL_AGENT_FLAGS = CHANNEL_AGENT_FLAGS(2i32);
pub const CI_E_CORRUPT_FWIDX: windows_core::HRESULT = windows_core::HRESULT(0xC004182D_u32 as _);
pub const CI_E_DIACRITIC_SETTINGS_DIFFER: windows_core::HRESULT = windows_core::HRESULT(0xC004182E_u32 as _);
pub const CI_E_INCONSISTENT_TRANSACTION: windows_core::HRESULT = windows_core::HRESULT(0xC0041832_u32 as _);
pub const CI_E_INVALID_CATALOG_LIST_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8004182F_u32 as _);
pub const CI_E_MULTIPLE_PROTECTED_USERS_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0041835_u32 as _);
pub const CI_E_NO_AUXMETADATA: windows_core::HRESULT = windows_core::HRESULT(0x8004182A_u32 as _);
pub const CI_E_NO_CATALOG_MANAGER: windows_core::HRESULT = windows_core::HRESULT(0xC0041831_u32 as _);
pub const CI_E_NO_PROTECTED_USER: windows_core::HRESULT = windows_core::HRESULT(0xC0041834_u32 as _);
pub const CI_E_PROTECTED_CATALOG_NON_INTERACTIVE_USER: windows_core::HRESULT = windows_core::HRESULT(0xC0041837_u32 as _);
pub const CI_E_PROTECTED_CATALOG_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC0041833_u32 as _);
pub const CI_E_PROTECTED_CATALOG_SID_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC0041836_u32 as _);
pub const CI_S_CATALOG_RESET: windows_core::HRESULT = windows_core::HRESULT(0x41830_u32 as _);
pub const CI_S_CLIENT_REQUESTED_ABORT: windows_core::HRESULT = windows_core::HRESULT(0x4182B_u32 as _);
pub const CI_S_NEW_AUXMETADATA: windows_core::HRESULT = windows_core::HRESULT(0x41829_u32 as _);
pub const CI_S_RETRY_DOCUMENT: windows_core::HRESULT = windows_core::HRESULT(0x4182C_u32 as _);
pub const CLSID_CISimpleCommandCreator: windows_core::GUID = windows_core::GUID::from_u128(0xc7b6c04a_cbb5_11d0_bb4c_00c04fc2f410);
pub const CLSID_DataShapeProvider: windows_core::GUID = windows_core::GUID::from_u128(0x3449a1c8_c56c_11d0_ad72_00c04fc29863);
pub const CLSID_MSDASQL: windows_core::GUID = windows_core::GUID::from_u128(0xc8b522cb_5cf3_11ce_ade5_00aa0044773d);
pub const CLSID_MSDASQL_ENUMERATOR: windows_core::GUID = windows_core::GUID::from_u128(0xc8b522cd_5cf3_11ce_ade5_00aa0044773d);
pub const CLSID_MSPersist: windows_core::GUID = windows_core::GUID::from_u128(0x7c07e0d0_4418_11d2_9212_00c04fbbbfb3);
pub const CLSID_SQLOLEDB: windows_core::GUID = windows_core::GUID::from_u128(0x0c7ff16c_38e3_11d0_97ab_00c04fc2ad98);
pub const CLSID_SQLOLEDB_ENUMERATOR: windows_core::GUID = windows_core::GUID::from_u128(0xdfa22b8e_e68d_11d0_97e4_00c04fc2ad98);
pub const CLSID_SQLOLEDB_ERROR: windows_core::GUID = windows_core::GUID::from_u128(0xc0932c62_38e5_11d0_97ab_00c04fc2ad98);
pub const CLUSIONREASON_DEFAULT: CLUSION_REASON = CLUSION_REASON(1i32);
pub const CLUSIONREASON_GROUPPOLICY: CLUSION_REASON = CLUSION_REASON(3i32);
pub const CLUSIONREASON_UNKNOWNSCOPE: CLUSION_REASON = CLUSION_REASON(0i32);
pub const CLUSIONREASON_USER: CLUSION_REASON = CLUSION_REASON(2i32);
pub const CMDLINE_E_ALREADY_INIT: i32 = -2147216123i32;
pub const CMDLINE_E_NOT_INIT: i32 = -2147216124i32;
pub const CMDLINE_E_NUM_PARAMS: i32 = -2147216122i32;
pub const CMDLINE_E_PARAM_SIZE: i32 = -2147216125i32;
pub const CMDLINE_E_PAREN: i32 = -2147216126i32;
pub const CMDLINE_E_UNEXPECTED: i32 = -2147216127i32;
pub const CM_E_CONNECTIONTIMEOUT: i32 = -2147219963i32;
pub const CM_E_DATASOURCENOTAVAILABLE: i32 = -2147219964i32;
pub const CM_E_INSUFFICIENTBUFFER: i32 = -2147219957i32;
pub const CM_E_INVALIDDATASOURCE: i32 = -2147219959i32;
pub const CM_E_NOQUERYCONNECTIONS: i32 = -2147219965i32;
pub const CM_E_REGISTRY: i32 = -2147219960i32;
pub const CM_E_SERVERNOTFOUND: i32 = -2147219962i32;
pub const CM_E_TIMEOUT: i32 = -2147219958i32;
pub const CM_E_TOOMANYDATASERVERS: i32 = -2147219967i32;
pub const CM_E_TOOMANYDATASOURCES: i32 = -2147219966i32;
pub const CM_S_NODATASERVERS: i32 = 263687i32;
pub const COLL_E_BADRESULT: i32 = -2147220218i32;
pub const COLL_E_BADSEQUENCE: i32 = -2147220223i32;
pub const COLL_E_BUFFERTOOSMALL: i32 = -2147220220i32;
pub const COLL_E_DUPLICATEDBID: i32 = -2147220216i32;
pub const COLL_E_INCOMPATIBLECOLUMNS: i32 = -2147220221i32;
pub const COLL_E_MAXCONNEXCEEDED: i32 = -2147220213i32;
pub const COLL_E_NODEFAULTCATALOG: i32 = -2147220214i32;
pub const COLL_E_NOMOREDATA: i32 = -2147220222i32;
pub const COLL_E_NOSORTCOLUMN: i32 = -2147220217i32;
pub const COLL_E_TOOMANYMERGECOLUMNS: i32 = -2147220215i32;
pub const CONDITION_CREATION_DEFAULT: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(0i32);
pub const CONDITION_CREATION_NONE: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(0i32);
pub const CONDITION_CREATION_SIMPLIFY: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(1i32);
pub const CONDITION_CREATION_USE_CONTENT_LOCALE: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(16i32);
pub const CONDITION_CREATION_VECTOR_AND: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(2i32);
pub const CONDITION_CREATION_VECTOR_LEAF: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(8i32);
pub const CONDITION_CREATION_VECTOR_OR: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(4i32);
pub const CONTENT_SOURCE_E_CONTENT_CLASS_READ: i32 = -2147208188i32;
pub const CONTENT_SOURCE_E_CONTENT_SOURCE_COLUMN_TYPE: i32 = -2147208185i32;
pub const CONTENT_SOURCE_E_NULL_CONTENT_CLASS_BSTR: i32 = -2147208186i32;
pub const CONTENT_SOURCE_E_NULL_URI: i32 = -2147208183i32;
pub const CONTENT_SOURCE_E_OUT_OF_RANGE: i32 = -2147208184i32;
pub const CONTENT_SOURCE_E_PROPERTY_MAPPING_BAD_VECTOR_SIZE: i32 = -2147208189i32;
pub const CONTENT_SOURCE_E_PROPERTY_MAPPING_READ: i32 = -2147208191i32;
pub const CONTENT_SOURCE_E_UNEXPECTED_EXCEPTION: i32 = -2147208187i32;
pub const CONTENT_SOURCE_E_UNEXPECTED_NULL_POINTER: i32 = -2147208190i32;
pub const CQUERYDISPIDS: u32 = 11u32;
pub const CQUERYMETADISPIDS: u32 = 10u32;
pub const CQUERYPROPERTY: u32 = 64u32;
pub const CREATESUBS_ADDTOFAVORITES: CREATESUBSCRIPTIONFLAGS = CREATESUBSCRIPTIONFLAGS(1i32);
pub const CREATESUBS_FROMFAVORITES: CREATESUBSCRIPTIONFLAGS = CREATESUBSCRIPTIONFLAGS(2i32);
pub const CREATESUBS_NOSAVE: CREATESUBSCRIPTIONFLAGS = CREATESUBSCRIPTIONFLAGS(8i32);
pub const CREATESUBS_NOUI: CREATESUBSCRIPTIONFLAGS = CREATESUBSCRIPTIONFLAGS(4i32);
pub const CREATESUBS_SOFTWAREUPDATE: CREATESUBSCRIPTIONFLAGS = CREATESUBSCRIPTIONFLAGS(16i32);
pub const CRESTRICTIONS_DBSCHEMA_ASSERTIONS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_CATALOGS: u32 = 1u32;
pub const CRESTRICTIONS_DBSCHEMA_CHARACTER_SETS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_CHECK_CONSTRAINTS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_CHECK_CONSTRAINTS_BY_TABLE: u32 = 6u32;
pub const CRESTRICTIONS_DBSCHEMA_COLLATIONS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_COLUMNS: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_COLUMN_DOMAIN_USAGE: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_COLUMN_PRIVILEGES: u32 = 6u32;
pub const CRESTRICTIONS_DBSCHEMA_CONSTRAINT_COLUMN_USAGE: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_CONSTRAINT_TABLE_USAGE: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_FOREIGN_KEYS: u32 = 6u32;
pub const CRESTRICTIONS_DBSCHEMA_INDEXES: u32 = 5u32;
pub const CRESTRICTIONS_DBSCHEMA_KEY_COLUMN_USAGE: u32 = 7u32;
pub const CRESTRICTIONS_DBSCHEMA_LINKEDSERVERS: u32 = 1u32;
pub const CRESTRICTIONS_DBSCHEMA_OBJECTS: u32 = 1u32;
pub const CRESTRICTIONS_DBSCHEMA_OBJECT_ACTIONS: u32 = 1u32;
pub const CRESTRICTIONS_DBSCHEMA_PRIMARY_KEYS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURES: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURE_COLUMNS: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURE_PARAMETERS: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_PROVIDER_TYPES: u32 = 2u32;
pub const CRESTRICTIONS_DBSCHEMA_REFERENTIAL_CONSTRAINTS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_SCHEMATA: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_SQL_LANGUAGES: u32 = 0u32;
pub const CRESTRICTIONS_DBSCHEMA_STATISTICS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_TABLES: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_TABLES_INFO: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_TABLE_CONSTRAINTS: u32 = 7u32;
pub const CRESTRICTIONS_DBSCHEMA_TABLE_PRIVILEGES: u32 = 5u32;
pub const CRESTRICTIONS_DBSCHEMA_TABLE_STATISTICS: u32 = 7u32;
pub const CRESTRICTIONS_DBSCHEMA_TRANSLATIONS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_TRUSTEE: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_USAGE_PRIVILEGES: u32 = 6u32;
pub const CRESTRICTIONS_DBSCHEMA_VIEWS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_VIEW_COLUMN_USAGE: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_VIEW_TABLE_USAGE: u32 = 3u32;
pub const CRESTRICTIONS_MDSCHEMA_ACTIONS: u32 = 8u32;
pub const CRESTRICTIONS_MDSCHEMA_COMMANDS: u32 = 5u32;
pub const CRESTRICTIONS_MDSCHEMA_CUBES: u32 = 3u32;
pub const CRESTRICTIONS_MDSCHEMA_DIMENSIONS: u32 = 5u32;
pub const CRESTRICTIONS_MDSCHEMA_FUNCTIONS: u32 = 4u32;
pub const CRESTRICTIONS_MDSCHEMA_HIERARCHIES: u32 = 6u32;
pub const CRESTRICTIONS_MDSCHEMA_LEVELS: u32 = 7u32;
pub const CRESTRICTIONS_MDSCHEMA_MEASURES: u32 = 5u32;
pub const CRESTRICTIONS_MDSCHEMA_MEMBERS: u32 = 12u32;
pub const CRESTRICTIONS_MDSCHEMA_PROPERTIES: u32 = 9u32;
pub const CRESTRICTIONS_MDSCHEMA_SETS: u32 = 5u32;
pub const CSTORAGEPROPERTY: u32 = 23u32;
pub const DBACCESSOR_INHERITED: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(16i32);
pub const DBACCESSOR_INVALID: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(0i32);
pub const DBACCESSOR_OPTIMIZED: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(8i32);
pub const DBACCESSOR_PARAMETERDATA: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(4i32);
pub const DBACCESSOR_PASSBYREF: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(1i32);
pub const DBACCESSOR_ROWDATA: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(2i32);
pub const DBASYNCHOP_OPEN: DBASYNCHOPENUM = DBASYNCHOPENUM(0i32);
pub const DBASYNCHPHASE_CANCELED: DBASYNCHPHASEENUM = DBASYNCHPHASEENUM(3i32);
pub const DBASYNCHPHASE_COMPLETE: DBASYNCHPHASEENUM = DBASYNCHPHASEENUM(2i32);
pub const DBASYNCHPHASE_INITIALIZATION: DBASYNCHPHASEENUM = DBASYNCHPHASEENUM(0i32);
pub const DBASYNCHPHASE_POPULATION: DBASYNCHPHASEENUM = DBASYNCHPHASEENUM(1i32);
pub const DBBINDFLAG_HTML: DBBINDFLAGENUM = DBBINDFLAGENUM(1i32);
pub const DBBINDSTATUS_BADBINDINFO: DBBINDSTATUSENUM = DBBINDSTATUSENUM(3i32);
pub const DBBINDSTATUS_BADORDINAL: DBBINDSTATUSENUM = DBBINDSTATUSENUM(1i32);
pub const DBBINDSTATUS_BADSTORAGEFLAGS: DBBINDSTATUSENUM = DBBINDSTATUSENUM(4i32);
pub const DBBINDSTATUS_MULTIPLESTORAGE: DBBINDSTATUSENUM = DBBINDSTATUSENUM(6i32);
pub const DBBINDSTATUS_NOINTERFACE: DBBINDSTATUSENUM = DBBINDSTATUSENUM(5i32);
pub const DBBINDSTATUS_OK: DBBINDSTATUSENUM = DBBINDSTATUSENUM(0i32);
pub const DBBINDSTATUS_UNSUPPORTEDCONVERSION: DBBINDSTATUSENUM = DBBINDSTATUSENUM(2i32);
pub const DBBINDURLFLAG_ASYNCHRONOUS: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(4096i32);
pub const DBBINDURLFLAG_COLLECTION: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(8192i32);
pub const DBBINDURLFLAG_DELAYFETCHCOLUMNS: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(32768i32);
pub const DBBINDURLFLAG_DELAYFETCHSTREAM: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(16384i32);
pub const DBBINDURLFLAG_ISSTRUCTUREDDOCUMENT: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(134217728i32);
pub const DBBINDURLFLAG_OPENIFEXISTS: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(33554432i32);
pub const DBBINDURLFLAG_OUTPUT: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(8388608i32);
pub const DBBINDURLFLAG_OVERWRITE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(67108864i32);
pub const DBBINDURLFLAG_READ: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(1i32);
pub const DBBINDURLFLAG_READWRITE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(3i32);
pub const DBBINDURLFLAG_RECURSIVE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(4194304i32);
pub const DBBINDURLFLAG_SHARE_DENY_NONE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(16i32);
pub const DBBINDURLFLAG_SHARE_DENY_READ: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(4i32);
pub const DBBINDURLFLAG_SHARE_DENY_WRITE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(8i32);
pub const DBBINDURLFLAG_SHARE_EXCLUSIVE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(12i32);
pub const DBBINDURLFLAG_WAITFORINIT: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(16777216i32);
pub const DBBINDURLFLAG_WRITE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(2i32);
pub const DBBINDURLSTATUS_S_DENYNOTSUPPORTED: DBBINDURLSTATUSENUM = DBBINDURLSTATUSENUM(1i32);
pub const DBBINDURLSTATUS_S_DENYTYPENOTSUPPORTED: DBBINDURLSTATUSENUM = DBBINDURLSTATUSENUM(4i32);
pub const DBBINDURLSTATUS_S_OK: DBBINDURLSTATUSENUM = DBBINDURLSTATUSENUM(0i32);
pub const DBBINDURLSTATUS_S_REDIRECTED: DBBINDURLSTATUSENUM = DBBINDURLSTATUSENUM(8i32);
pub const DBBMKGUID: windows_core::GUID = windows_core::GUID::from_u128(0xc8b52232_5cf3_11ce_ade5_00aa0044773d);
pub const DBBMK_FIRST: DBBOOKMARK = DBBOOKMARK(1i32);
pub const DBBMK_INVALID: DBBOOKMARK = DBBOOKMARK(0i32);
pub const DBBMK_LAST: DBBOOKMARK = DBBOOKMARK(2i32);
pub const DBCIDGUID: windows_core::GUID = windows_core::GUID::from_u128(0x0c733a81_2a1c_11ce_ade5_00aa0044773d);
pub const DBCOLUMNDESCFLAGS_CLSID: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(8i32);
pub const DBCOLUMNDESCFLAGS_COLSIZE: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(16i32);
pub const DBCOLUMNDESCFLAGS_DBCID: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(32i32);
pub const DBCOLUMNDESCFLAGS_ITYPEINFO: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(2i32);
pub const DBCOLUMNDESCFLAGS_PRECISION: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(128i32);
pub const DBCOLUMNDESCFLAGS_PROPERTIES: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(4i32);
pub const DBCOLUMNDESCFLAGS_SCALE: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(256i32);
pub const DBCOLUMNDESCFLAGS_TYPENAME: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(1i32);
pub const DBCOLUMNDESCFLAGS_WTYPE: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(64i32);
pub const DBCOLUMNFLAGS_CACHEDEFERRED: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(4096i32);
pub const DBCOLUMNFLAGS_ISBOOKMARK: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(1i32);
pub const DBCOLUMNFLAGS_ISCHAPTER: DBCOLUMNFLAGS15ENUM = DBCOLUMNFLAGS15ENUM(8192i32);
pub const DBCOLUMNFLAGS_ISCOLLECTION: DBCOLUMNFLAGSENUM21 = DBCOLUMNFLAGSENUM21(262144i32);
pub const DBCOLUMNFLAGS_ISDEFAULTSTREAM: DBCOLUMNFLAGSENUM21 = DBCOLUMNFLAGSENUM21(131072i32);
pub const DBCOLUMNFLAGS_ISFIXEDLENGTH: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(16i32);
pub const DBCOLUMNFLAGS_ISLONG: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(128i32);
pub const DBCOLUMNFLAGS_ISNULLABLE: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(32i32);
pub const DBCOLUMNFLAGS_ISROW: DBCOLUMNFLAGSENUM26 = DBCOLUMNFLAGSENUM26(2097152i32);
pub const DBCOLUMNFLAGS_ISROWID: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(256i32);
pub const DBCOLUMNFLAGS_ISROWSET: DBCOLUMNFLAGSENUM26 = DBCOLUMNFLAGSENUM26(1048576i32);
pub const DBCOLUMNFLAGS_ISROWURL: DBCOLUMNFLAGSENUM21 = DBCOLUMNFLAGSENUM21(65536i32);
pub const DBCOLUMNFLAGS_ISROWVER: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(512i32);
pub const DBCOLUMNFLAGS_ISSTREAM: DBCOLUMNFLAGSENUM26 = DBCOLUMNFLAGSENUM26(524288i32);
pub const DBCOLUMNFLAGS_KEYCOLUMN: DBCOLUMNFLAGSDEPRECATED = DBCOLUMNFLAGSDEPRECATED(32768i32);
pub const DBCOLUMNFLAGS_MAYBENULL: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(64i32);
pub const DBCOLUMNFLAGS_MAYDEFER: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(2i32);
pub const DBCOLUMNFLAGS_RESERVED: DBCOLUMNFLAGSENUM20 = DBCOLUMNFLAGSENUM20(32768i32);
pub const DBCOLUMNFLAGS_ROWSPECIFICCOLUMN: DBCOLUMNFLAGSENUM26 = DBCOLUMNFLAGSENUM26(4194304i32);
pub const DBCOLUMNFLAGS_SCALEISNEGATIVE: DBCOLUMNFLAGSENUM20 = DBCOLUMNFLAGSENUM20(16384i32);
pub const DBCOLUMNFLAGS_WRITE: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(4i32);
pub const DBCOLUMNFLAGS_WRITEUNKNOWN: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(8i32);
pub const DBCOMMANDPERSISTFLAG_DEFAULT: DBCOMMANDPERSISTFLAGENUM21 = DBCOMMANDPERSISTFLAGENUM21(0i32);
pub const DBCOMMANDPERSISTFLAG_NOSAVE: DBCOMMANDPERSISTFLAGENUM = DBCOMMANDPERSISTFLAGENUM(1i32);
pub const DBCOMMANDPERSISTFLAG_PERSISTPROCEDURE: DBCOMMANDPERSISTFLAGENUM21 = DBCOMMANDPERSISTFLAGENUM21(4i32);
pub const DBCOMMANDPERSISTFLAG_PERSISTVIEW: DBCOMMANDPERSISTFLAGENUM21 = DBCOMMANDPERSISTFLAGENUM21(2i32);
pub const DBCOMPAREOPS_BEGINSWITH: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(5i32);
pub const DBCOMPAREOPS_CASEINSENSITIVE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(8192i32);
pub const DBCOMPAREOPS_CASESENSITIVE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(4096i32);
pub const DBCOMPAREOPS_CONTAINS: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(6i32);
pub const DBCOMPAREOPS_EQ: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(2i32);
pub const DBCOMPAREOPS_GE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(3i32);
pub const DBCOMPAREOPS_GT: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(4i32);
pub const DBCOMPAREOPS_IGNORE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(8i32);
pub const DBCOMPAREOPS_LE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(1i32);
pub const DBCOMPAREOPS_LT: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(0i32);
pub const DBCOMPAREOPS_NE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(7i32);
pub const DBCOMPAREOPS_NOTBEGINSWITH: DBCOMPAREOPSENUM20 = DBCOMPAREOPSENUM20(9i32);
pub const DBCOMPAREOPS_NOTCONTAINS: DBCOMPAREOPSENUM20 = DBCOMPAREOPSENUM20(10i32);
pub const DBCOMPARE_EQ: DBCOMPAREENUM = DBCOMPAREENUM(1i32);
pub const DBCOMPARE_GT: DBCOMPAREENUM = DBCOMPAREENUM(2i32);
pub const DBCOMPARE_LT: DBCOMPAREENUM = DBCOMPAREENUM(0i32);
pub const DBCOMPARE_NE: DBCOMPAREENUM = DBCOMPAREENUM(3i32);
pub const DBCOMPARE_NOTCOMPARABLE: DBCOMPAREENUM = DBCOMPAREENUM(4i32);
pub const DBCOMPUTEMODE_COMPUTED: u32 = 1u32;
pub const DBCOMPUTEMODE_DYNAMIC: u32 = 2u32;
pub const DBCOMPUTEMODE_NOTCOMPUTED: u32 = 3u32;
pub const DBCONSTRAINTTYPE_CHECK: DBCONSTRAINTTYPEENUM = DBCONSTRAINTTYPEENUM(3i32);
pub const DBCONSTRAINTTYPE_FOREIGNKEY: DBCONSTRAINTTYPEENUM = DBCONSTRAINTTYPEENUM(1i32);
pub const DBCONSTRAINTTYPE_PRIMARYKEY: DBCONSTRAINTTYPEENUM = DBCONSTRAINTTYPEENUM(2i32);
pub const DBCONSTRAINTTYPE_UNIQUE: DBCONSTRAINTTYPEENUM = DBCONSTRAINTTYPEENUM(0i32);
pub const DBCONVERTFLAGS_COLUMN: DBCONVERTFLAGSENUM = DBCONVERTFLAGSENUM(0i32);
pub const DBCONVERTFLAGS_FROMVARIANT: DBCONVERTFLAGSENUM20 = DBCONVERTFLAGSENUM20(8i32);
pub const DBCONVERTFLAGS_ISFIXEDLENGTH: DBCONVERTFLAGSENUM20 = DBCONVERTFLAGSENUM20(4i32);
pub const DBCONVERTFLAGS_ISLONG: DBCONVERTFLAGSENUM20 = DBCONVERTFLAGSENUM20(2i32);
pub const DBCONVERTFLAGS_PARAMETER: DBCONVERTFLAGSENUM = DBCONVERTFLAGSENUM(1i32);
pub const DBCOPY_ALLOW_EMULATION: DBCOPYFLAGSENUM = DBCOPYFLAGSENUM(1024i32);
pub const DBCOPY_ASYNC: DBCOPYFLAGSENUM = DBCOPYFLAGSENUM(256i32);
pub const DBCOPY_ATOMIC: DBCOPYFLAGSENUM = DBCOPYFLAGSENUM(4096i32);
pub const DBCOPY_NON_RECURSIVE: DBCOPYFLAGSENUM = DBCOPYFLAGSENUM(2048i32);
pub const DBCOPY_REPLACE_EXISTING: DBCOPYFLAGSENUM = DBCOPYFLAGSENUM(512i32);
pub const DBDATACONVERT_DECIMALSCALE: DBDATACONVERTENUM = DBDATACONVERTENUM(8i32);
pub const DBDATACONVERT_DEFAULT: DBDATACONVERTENUM = DBDATACONVERTENUM(0i32);
pub const DBDATACONVERT_DSTISFIXEDLENGTH: DBDATACONVERTENUM = DBDATACONVERTENUM(4i32);
pub const DBDATACONVERT_LENGTHFROMNTS: DBDATACONVERTENUM = DBDATACONVERTENUM(2i32);
pub const DBDATACONVERT_SETDATABEHAVIOR: DBDATACONVERTENUM = DBDATACONVERTENUM(1i32);
pub const DBDEFERRABILITY_DEFERRABLE: DBDEFERRABILITYENUM = DBDEFERRABILITYENUM(2i32);
pub const DBDEFERRABILITY_DEFERRED: DBDEFERRABILITYENUM = DBDEFERRABILITYENUM(1i32);
pub const DBDELETE_ASYNC: DBDELETEFLAGSENUM = DBDELETEFLAGSENUM(256i32);
pub const DBDELETE_ATOMIC: DBDELETEFLAGSENUM = DBDELETEFLAGSENUM(4096i32);
pub const DBEVENTPHASE_ABOUTTODO: DBEVENTPHASEENUM = DBEVENTPHASEENUM(1i32);
pub const DBEVENTPHASE_DIDEVENT: DBEVENTPHASEENUM = DBEVENTPHASEENUM(4i32);
pub const DBEVENTPHASE_FAILEDTODO: DBEVENTPHASEENUM = DBEVENTPHASEENUM(3i32);
pub const DBEVENTPHASE_OKTODO: DBEVENTPHASEENUM = DBEVENTPHASEENUM(0i32);
pub const DBEVENTPHASE_SYNCHAFTER: DBEVENTPHASEENUM = DBEVENTPHASEENUM(2i32);
pub const DBEXECLIMITS_ABORT: DBEXECLIMITSENUM = DBEXECLIMITSENUM(1i32);
pub const DBEXECLIMITS_STOP: DBEXECLIMITSENUM = DBEXECLIMITSENUM(2i32);
pub const DBEXECLIMITS_SUSPEND: DBEXECLIMITSENUM = DBEXECLIMITSENUM(3i32);
pub const DBGUID_MSSQLXML: windows_core::GUID = windows_core::GUID::from_u128(0x5d531cb2_e6ed_11d2_b252_00c04f681b71);
pub const DBGUID_ROWDEFAULTSTREAM: windows_core::GUID = windows_core::GUID::from_u128(0x0c733ab7_2a1c_11ce_ade5_00aa0044773d);
pub const DBGUID_ROWURL: windows_core::GUID = windows_core::GUID::from_u128(0x0c733ab6_2a1c_11ce_ade5_00aa0044773d);
pub const DBGUID_XPATH: windows_core::GUID = windows_core::GUID::from_u128(0xec2a4293_e898_11d2_b1b7_00c04f680c56);
pub const DBINDEX_COL_ORDER_ASC: DBINDEX_COL_ORDERENUM = DBINDEX_COL_ORDERENUM(0i32);
pub const DBINDEX_COL_ORDER_DESC: DBINDEX_COL_ORDERENUM = DBINDEX_COL_ORDERENUM(1i32);
pub const DBLITERAL_BINARY_LITERAL: DBLITERALENUM = DBLITERALENUM(1i32);
pub const DBLITERAL_CATALOG_NAME: DBLITERALENUM = DBLITERALENUM(2i32);
pub const DBLITERAL_CATALOG_SEPARATOR: DBLITERALENUM = DBLITERALENUM(3i32);
pub const DBLITERAL_CHAR_LITERAL: DBLITERALENUM = DBLITERALENUM(4i32);
pub const DBLITERAL_COLUMN_ALIAS: DBLITERALENUM = DBLITERALENUM(5i32);
pub const DBLITERAL_COLUMN_NAME: DBLITERALENUM = DBLITERALENUM(6i32);
pub const DBLITERAL_CORRELATION_NAME: DBLITERALENUM = DBLITERALENUM(7i32);
pub const DBLITERAL_CUBE_NAME: DBLITERALENUM20 = DBLITERALENUM20(21i32);
pub const DBLITERAL_CURSOR_NAME: DBLITERALENUM = DBLITERALENUM(8i32);
pub const DBLITERAL_DIMENSION_NAME: DBLITERALENUM20 = DBLITERALENUM20(22i32);
pub const DBLITERAL_ESCAPE_PERCENT: DBLITERALENUM = DBLITERALENUM(9i32);
pub const DBLITERAL_ESCAPE_PERCENT_SUFFIX: DBLITERALENUM21 = DBLITERALENUM21(29i32);
pub const DBLITERAL_ESCAPE_UNDERSCORE: DBLITERALENUM = DBLITERALENUM(10i32);
pub const DBLITERAL_ESCAPE_UNDERSCORE_SUFFIX: DBLITERALENUM21 = DBLITERALENUM21(30i32);
pub const DBLITERAL_HIERARCHY_NAME: DBLITERALENUM20 = DBLITERALENUM20(23i32);
pub const DBLITERAL_INDEX_NAME: DBLITERALENUM = DBLITERALENUM(11i32);
pub const DBLITERAL_INVALID: DBLITERALENUM = DBLITERALENUM(0i32);
pub const DBLITERAL_LEVEL_NAME: DBLITERALENUM20 = DBLITERALENUM20(24i32);
pub const DBLITERAL_LIKE_PERCENT: DBLITERALENUM = DBLITERALENUM(12i32);
pub const DBLITERAL_LIKE_UNDERSCORE: DBLITERALENUM = DBLITERALENUM(13i32);
pub const DBLITERAL_MEMBER_NAME: DBLITERALENUM20 = DBLITERALENUM20(25i32);
pub const DBLITERAL_PROCEDURE_NAME: DBLITERALENUM = DBLITERALENUM(14i32);
pub const DBLITERAL_PROPERTY_NAME: DBLITERALENUM20 = DBLITERALENUM20(26i32);
pub const DBLITERAL_QUOTE: DBLITERALENUM = DBLITERALENUM(15i32);
pub const DBLITERAL_QUOTE_SUFFIX: DBLITERALENUM20 = DBLITERALENUM20(28i32);
pub const DBLITERAL_SCHEMA_NAME: DBLITERALENUM = DBLITERALENUM(16i32);
pub const DBLITERAL_SCHEMA_SEPARATOR: DBLITERALENUM20 = DBLITERALENUM20(27i32);
pub const DBLITERAL_TABLE_NAME: DBLITERALENUM = DBLITERALENUM(17i32);
pub const DBLITERAL_TEXT_COMMAND: DBLITERALENUM = DBLITERALENUM(18i32);
pub const DBLITERAL_USER_NAME: DBLITERALENUM = DBLITERALENUM(19i32);
pub const DBLITERAL_VIEW_NAME: DBLITERALENUM = DBLITERALENUM(20i32);
pub const DBMATCHTYPE_FULL: DBMATCHTYPEENUM = DBMATCHTYPEENUM(0i32);
pub const DBMATCHTYPE_NONE: DBMATCHTYPEENUM = DBMATCHTYPEENUM(1i32);
pub const DBMATCHTYPE_PARTIAL: DBMATCHTYPEENUM = DBMATCHTYPEENUM(2i32);
pub const DBMAXCHAR: u32 = 8001u32;
pub const DBMEMOWNER_CLIENTOWNED: DBMEMOWNERENUM = DBMEMOWNERENUM(0i32);
pub const DBMEMOWNER_PROVIDEROWNED: DBMEMOWNERENUM = DBMEMOWNERENUM(1i32);
pub const DBMOVE_ALLOW_EMULATION: DBMOVEFLAGSENUM = DBMOVEFLAGSENUM(1024i32);
pub const DBMOVE_ASYNC: DBMOVEFLAGSENUM = DBMOVEFLAGSENUM(256i32);
pub const DBMOVE_ATOMIC: DBMOVEFLAGSENUM = DBMOVEFLAGSENUM(4096i32);
pub const DBMOVE_DONT_UPDATE_LINKS: DBMOVEFLAGSENUM = DBMOVEFLAGSENUM(512i32);
pub const DBMOVE_REPLACE_EXISTING: DBMOVEFLAGSENUM = DBMOVEFLAGSENUM(1i32);
pub const DBPARAMFLAGS_ISINPUT: DBPARAMFLAGSENUM = DBPARAMFLAGSENUM(1i32);
pub const DBPARAMFLAGS_ISLONG: DBPARAMFLAGSENUM = DBPARAMFLAGSENUM(128i32);
pub const DBPARAMFLAGS_ISNULLABLE: DBPARAMFLAGSENUM = DBPARAMFLAGSENUM(64i32);
pub const DBPARAMFLAGS_ISOUTPUT: DBPARAMFLAGSENUM = DBPARAMFLAGSENUM(2i32);
pub const DBPARAMFLAGS_ISSIGNED: DBPARAMFLAGSENUM = DBPARAMFLAGSENUM(16i32);
pub const DBPARAMFLAGS_SCALEISNEGATIVE: DBPARAMFLAGSENUM20 = DBPARAMFLAGSENUM20(256i32);
pub const DBPARAMIO_INPUT: DBPARAMIOENUM = DBPARAMIOENUM(1i32);
pub const DBPARAMIO_NOTPARAM: DBPARAMIOENUM = DBPARAMIOENUM(0i32);
pub const DBPARAMIO_OUTPUT: DBPARAMIOENUM = DBPARAMIOENUM(2i32);
pub const DBPARAMTYPE_INPUT: u32 = 1u32;
pub const DBPARAMTYPE_INPUTOUTPUT: u32 = 2u32;
pub const DBPARAMTYPE_OUTPUT: u32 = 3u32;
pub const DBPARAMTYPE_RETURNVALUE: u32 = 4u32;
pub const DBPART_INVALID: DBPARTENUM = DBPARTENUM(0i32);
pub const DBPART_LENGTH: DBPARTENUM = DBPARTENUM(2i32);
pub const DBPART_STATUS: DBPARTENUM = DBPARTENUM(4i32);
pub const DBPART_VALUE: DBPARTENUM = DBPARTENUM(1i32);
pub const DBPENDINGSTATUS_CHANGED: DBPENDINGSTATUSENUM = DBPENDINGSTATUSENUM(2i32);
pub const DBPENDINGSTATUS_DELETED: DBPENDINGSTATUSENUM = DBPENDINGSTATUSENUM(4i32);
pub const DBPENDINGSTATUS_INVALIDROW: DBPENDINGSTATUSENUM = DBPENDINGSTATUSENUM(16i32);
pub const DBPENDINGSTATUS_NEW: DBPENDINGSTATUSENUM = DBPENDINGSTATUSENUM(1i32);
pub const DBPENDINGSTATUS_UNCHANGED: DBPENDINGSTATUSENUM = DBPENDINGSTATUSENUM(8i32);
pub const DBPOSITION_BOF: DBPOSITIONFLAGSENUM = DBPOSITIONFLAGSENUM(2i32);
pub const DBPOSITION_EOF: DBPOSITIONFLAGSENUM = DBPOSITIONFLAGSENUM(3i32);
pub const DBPOSITION_NOROW: DBPOSITIONFLAGSENUM = DBPOSITIONFLAGSENUM(1i32);
pub const DBPOSITION_OK: DBPOSITIONFLAGSENUM = DBPOSITIONFLAGSENUM(0i32);
pub const DBPROMPTOPTIONS_BROWSEONLY: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(8i32);
pub const DBPROMPTOPTIONS_DISABLESAVEPASSWORD: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(32i32);
pub const DBPROMPTOPTIONS_DISABLE_PROVIDER_SELECTION: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(16i32);
pub const DBPROMPTOPTIONS_NONE: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(0i32);
pub const DBPROMPTOPTIONS_PROPERTYSHEET: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(2i32);
pub const DBPROMPTOPTIONS_WIZARDSHEET: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(1i32);
pub const DBPROMPT_COMPLETE: u32 = 2u32;
pub const DBPROMPT_COMPLETEREQUIRED: u32 = 3u32;
pub const DBPROMPT_NOPROMPT: u32 = 4u32;
pub const DBPROMPT_PROMPT: u32 = 1u32;
pub const DBPROPFLAGS_COLUMN: DBPROPFLAGSENUM = DBPROPFLAGSENUM(1i32);
pub const DBPROPFLAGS_COLUMNOK: DBPROPFLAGSENUM = DBPROPFLAGSENUM(256i32);
pub const DBPROPFLAGS_DATASOURCE: DBPROPFLAGSENUM = DBPROPFLAGSENUM(2i32);
pub const DBPROPFLAGS_DATASOURCECREATE: DBPROPFLAGSENUM = DBPROPFLAGSENUM(4i32);
pub const DBPROPFLAGS_DATASOURCEINFO: DBPROPFLAGSENUM = DBPROPFLAGSENUM(8i32);
pub const DBPROPFLAGS_DBINIT: DBPROPFLAGSENUM = DBPROPFLAGSENUM(16i32);
pub const DBPROPFLAGS_INDEX: DBPROPFLAGSENUM = DBPROPFLAGSENUM(32i32);
pub const DBPROPFLAGS_NOTSUPPORTED: DBPROPFLAGSENUM = DBPROPFLAGSENUM(0i32);
pub const DBPROPFLAGS_PERSIST: u32 = 8192u32;
pub const DBPROPFLAGS_READ: DBPROPFLAGSENUM = DBPROPFLAGSENUM(512i32);
pub const DBPROPFLAGS_REQUIRED: DBPROPFLAGSENUM = DBPROPFLAGSENUM(2048i32);
pub const DBPROPFLAGS_ROWSET: DBPROPFLAGSENUM = DBPROPFLAGSENUM(64i32);
pub const DBPROPFLAGS_SESSION: DBPROPFLAGSENUM = DBPROPFLAGSENUM(4096i32);
pub const DBPROPFLAGS_STREAM: DBPROPFLAGSENUM26 = DBPROPFLAGSENUM26(32768i32);
pub const DBPROPFLAGS_TABLE: DBPROPFLAGSENUM = DBPROPFLAGSENUM(128i32);
pub const DBPROPFLAGS_TRUSTEE: DBPROPFLAGSENUM21 = DBPROPFLAGSENUM21(8192i32);
pub const DBPROPFLAGS_VIEW: DBPROPFLAGSENUM25 = DBPROPFLAGSENUM25(16384i32);
pub const DBPROPFLAGS_WRITE: DBPROPFLAGSENUM = DBPROPFLAGSENUM(1024i32);
pub const DBPROPOPTIONS_OPTIONAL: DBPROPOPTIONSENUM = DBPROPOPTIONSENUM(1i32);
pub const DBPROPOPTIONS_REQUIRED: DBPROPOPTIONSENUM = DBPROPOPTIONSENUM(0i32);
pub const DBPROPOPTIONS_SETIFCHEAP: DBPROPOPTIONSENUM = DBPROPOPTIONSENUM(1i32);
pub const DBPROPSET_MSDAORA8_ROWSET: windows_core::GUID = windows_core::GUID::from_u128(0x7f06a375_dd6a_43db_b4e0_1fc121e5e62b);
pub const DBPROPSET_MSDAORA_ROWSET: windows_core::GUID = windows_core::GUID::from_u128(0xe8cc4cbd_fdff_11d0_b865_00a0c9081c1d);
pub const DBPROPSET_MSDSDBINIT: windows_core::GUID = windows_core::GUID::from_u128(0x55cb91a8_5c7a_11d1_adad_00c04fc29863);
pub const DBPROPSET_MSDSSESSION: windows_core::GUID = windows_core::GUID::from_u128(0xedf17536_afbf_11d1_8847_0000f879f98c);
pub const DBPROPSET_PERSIST: windows_core::GUID = windows_core::GUID::from_u128(0x4d7839a0_5b8e_11d1_a6b3_00a0c9138c66);
pub const DBPROPSET_PROVIDERCONNATTR: windows_core::GUID = windows_core::GUID::from_u128(0x497c60e4_7123_11cf_b171_00aa0057599e);
pub const DBPROPSET_PROVIDERDATASOURCEINFO: windows_core::GUID = windows_core::GUID::from_u128(0x497c60e0_7123_11cf_b171_00aa0057599e);
pub const DBPROPSET_PROVIDERDBINIT: windows_core::GUID = windows_core::GUID::from_u128(0x497c60e2_7123_11cf_b171_00aa0057599e);
pub const DBPROPSET_PROVIDERROWSET: windows_core::GUID = windows_core::GUID::from_u128(0x497c60e1_7123_11cf_b171_00aa0057599e);
pub const DBPROPSET_PROVIDERSTMTATTR: windows_core::GUID = windows_core::GUID::from_u128(0x497c60e3_7123_11cf_b171_00aa0057599e);
pub const DBPROPSET_SQLSERVERCOLUMN: windows_core::GUID = windows_core::GUID::from_u128(0x3b63fb5e_3fbb_11d3_9f29_00c04f8ee9dc);
pub const DBPROPSET_SQLSERVERDATASOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x28efaee4_2d2c_11d1_9807_00c04fc2ad98);
pub const DBPROPSET_SQLSERVERDATASOURCEINFO: windows_core::GUID = windows_core::GUID::from_u128(0xdf10cb94_35f6_11d2_9c54_00c04f7971d3);
pub const DBPROPSET_SQLSERVERDBINIT: windows_core::GUID = windows_core::GUID::from_u128(0x5cf4ca10_ef21_11d0_97e7_00c04fc2ad98);
pub const DBPROPSET_SQLSERVERROWSET: windows_core::GUID = windows_core::GUID::from_u128(0x5cf4ca11_ef21_11d0_97e7_00c04fc2ad98);
pub const DBPROPSET_SQLSERVERSESSION: windows_core::GUID = windows_core::GUID::from_u128(0x28efaee5_2d2c_11d1_9807_00c04fc2ad98);
pub const DBPROPSET_SQLSERVERSTREAM: windows_core::GUID = windows_core::GUID::from_u128(0x9f79c073_8a6d_4bca_a8a8_c9b79a9b962d);
pub const DBPROPSTATUS_BADCOLUMN: DBPROPSTATUSENUM = DBPROPSTATUSENUM(4i32);
pub const DBPROPSTATUS_BADOPTION: DBPROPSTATUSENUM = DBPROPSTATUSENUM(3i32);
pub const DBPROPSTATUS_BADVALUE: DBPROPSTATUSENUM = DBPROPSTATUSENUM(2i32);
pub const DBPROPSTATUS_CONFLICTING: DBPROPSTATUSENUM = DBPROPSTATUSENUM(8i32);
pub const DBPROPSTATUS_NOTALLSETTABLE: DBPROPSTATUSENUM = DBPROPSTATUSENUM(5i32);
pub const DBPROPSTATUS_NOTAVAILABLE: DBPROPSTATUSENUM21 = DBPROPSTATUSENUM21(9i32);
pub const DBPROPSTATUS_NOTSET: DBPROPSTATUSENUM = DBPROPSTATUSENUM(7i32);
pub const DBPROPSTATUS_NOTSETTABLE: DBPROPSTATUSENUM = DBPROPSTATUSENUM(6i32);
pub const DBPROPSTATUS_NOTSUPPORTED: DBPROPSTATUSENUM = DBPROPSTATUSENUM(1i32);
pub const DBPROPSTATUS_OK: DBPROPSTATUSENUM = DBPROPSTATUSENUM(0i32);
pub const DBPROPVAL_AO_RANDOM: i32 = 2i32;
pub const DBPROPVAL_AO_SEQUENTIAL: i32 = 0i32;
pub const DBPROPVAL_AO_SEQUENTIALSTORAGEOBJECTS: i32 = 1i32;
pub const DBPROPVAL_ASYNCH_BACKGROUNDPOPULATION: i32 = 8i32;
pub const DBPROPVAL_ASYNCH_INITIALIZE: i32 = 1i32;
pub const DBPROPVAL_ASYNCH_POPULATEONDEMAND: i32 = 32i32;
pub const DBPROPVAL_ASYNCH_PREPOPULATE: i32 = 16i32;
pub const DBPROPVAL_ASYNCH_RANDOMPOPULATION: i32 = 4i32;
pub const DBPROPVAL_ASYNCH_SEQUENTIALPOPULATION: i32 = 2i32;
pub const DBPROPVAL_BD_INTRANSACTION: i32 = 1i32;
pub const DBPROPVAL_BD_REORGANIZATION: i32 = 3i32;
pub const DBPROPVAL_BD_ROWSET: i32 = 0i32;
pub const DBPROPVAL_BD_XTRANSACTION: i32 = 2i32;
pub const DBPROPVAL_BI_CROSSROWSET: i32 = 1i32;
pub const DBPROPVAL_BMK_KEY: i32 = 2i32;
pub const DBPROPVAL_BMK_NUMERIC: i32 = 1i32;
pub const DBPROPVAL_BO_NOINDEXUPDATE: i32 = 1i32;
pub const DBPROPVAL_BO_NOLOG: i32 = 0i32;
pub const DBPROPVAL_BO_REFINTEGRITY: i32 = 2i32;
pub const DBPROPVAL_CB_DELETE: i32 = 1i32;
pub const DBPROPVAL_CB_NON_NULL: i32 = 2i32;
pub const DBPROPVAL_CB_NULL: i32 = 1i32;
pub const DBPROPVAL_CB_PRESERVE: i32 = 2i32;
pub const DBPROPVAL_CD_NOTNULL: i32 = 1i32;
pub const DBPROPVAL_CL_END: i32 = 2i32;
pub const DBPROPVAL_CL_START: i32 = 1i32;
pub const DBPROPVAL_CM_TRANSACTIONS: i32 = 1i32;
pub const DBPROPVAL_CO_BEGINSWITH: i32 = 32i32;
pub const DBPROPVAL_CO_CASEINSENSITIVE: i32 = 8i32;
pub const DBPROPVAL_CO_CASESENSITIVE: i32 = 4i32;
pub const DBPROPVAL_CO_CONTAINS: i32 = 16i32;
pub const DBPROPVAL_CO_EQUALITY: i32 = 1i32;
pub const DBPROPVAL_CO_STRING: i32 = 2i32;
pub const DBPROPVAL_CS_COMMUNICATIONFAILURE: i32 = 2i32;
pub const DBPROPVAL_CS_INITIALIZED: i32 = 1i32;
pub const DBPROPVAL_CS_UNINITIALIZED: i32 = 0i32;
pub const DBPROPVAL_CU_DML_STATEMENTS: i32 = 1i32;
pub const DBPROPVAL_CU_INDEX_DEFINITION: i32 = 4i32;
pub const DBPROPVAL_CU_PRIVILEGE_DEFINITION: i32 = 8i32;
pub const DBPROPVAL_CU_TABLE_DEFINITION: i32 = 2i32;
pub const DBPROPVAL_DF_INITIALLY_DEFERRED: u32 = 1u32;
pub const DBPROPVAL_DF_INITIALLY_IMMEDIATE: u32 = 2u32;
pub const DBPROPVAL_DF_NOT_DEFERRABLE: u32 = 3u32;
pub const DBPROPVAL_DST_DOCSOURCE: i32 = 4i32;
pub const DBPROPVAL_DST_MDP: i32 = 2i32;
pub const DBPROPVAL_DST_TDP: i32 = 1i32;
pub const DBPROPVAL_DST_TDPANDMDP: i32 = 3i32;
pub const DBPROPVAL_FU_CATALOG: i32 = 8i32;
pub const DBPROPVAL_FU_COLUMN: i32 = 2i32;
pub const DBPROPVAL_FU_NOT_SUPPORTED: i32 = 1i32;
pub const DBPROPVAL_FU_TABLE: i32 = 4i32;
pub const DBPROPVAL_GB_COLLATE: i32 = 16i32;
pub const DBPROPVAL_GB_CONTAINS_SELECT: i32 = 4i32;
pub const DBPROPVAL_GB_EQUALS_SELECT: i32 = 2i32;
pub const DBPROPVAL_GB_NOT_SUPPORTED: i32 = 1i32;
pub const DBPROPVAL_GB_NO_RELATION: i32 = 8i32;
pub const DBPROPVAL_GU_NOTSUPPORTED: i32 = 1i32;
pub const DBPROPVAL_GU_SUFFIX: i32 = 2i32;
pub const DBPROPVAL_HT_DIFFERENT_CATALOGS: i32 = 1i32;
pub const DBPROPVAL_HT_DIFFERENT_PROVIDERS: i32 = 2i32;
pub const DBPROPVAL_IC_LOWER: i32 = 2i32;
pub const DBPROPVAL_IC_MIXED: i32 = 8i32;
pub const DBPROPVAL_IC_SENSITIVE: i32 = 4i32;
pub const DBPROPVAL_IC_UPPER: i32 = 1i32;
pub const DBPROPVAL_IN_ALLOWNULL: i32 = 0i32;
pub const DBPROPVAL_IN_DISALLOWNULL: i32 = 1i32;
pub const DBPROPVAL_IN_IGNOREANYNULL: i32 = 4i32;
pub const DBPROPVAL_IN_IGNORENULL: i32 = 2i32;
pub const DBPROPVAL_IT_BTREE: i32 = 1i32;
pub const DBPROPVAL_IT_CONTENT: i32 = 3i32;
pub const DBPROPVAL_IT_HASH: i32 = 2i32;
pub const DBPROPVAL_IT_OTHER: i32 = 4i32;
pub const DBPROPVAL_LM_INTENT: i32 = 4i32;
pub const DBPROPVAL_LM_NONE: i32 = 1i32;
pub const DBPROPVAL_LM_READ: i32 = 2i32;
pub const DBPROPVAL_LM_RITE: i32 = 8i32;
pub const DBPROPVAL_LM_SINGLEROW: i32 = 2i32;
pub const DBPROPVAL_MR_CONCURRENT: i32 = 2i32;
pub const DBPROPVAL_MR_NOTSUPPORTED: i32 = 0i32;
pub const DBPROPVAL_MR_SUPPORTED: i32 = 1i32;
pub const DBPROPVAL_NC_END: i32 = 1i32;
pub const DBPROPVAL_NC_HIGH: i32 = 2i32;
pub const DBPROPVAL_NC_LOW: i32 = 4i32;
pub const DBPROPVAL_NC_START: i32 = 8i32;
pub const DBPROPVAL_NP_ABOUTTODO: i32 = 2i32;
pub const DBPROPVAL_NP_DIDEVENT: i32 = 16i32;
pub const DBPROPVAL_NP_FAILEDTODO: i32 = 8i32;
pub const DBPROPVAL_NP_OKTODO: i32 = 1i32;
pub const DBPROPVAL_NP_SYNCHAFTER: i32 = 4i32;
pub const DBPROPVAL_NT_MULTIPLEROWS: i32 = 2i32;
pub const DBPROPVAL_NT_SINGLEROW: i32 = 1i32;
pub const DBPROPVAL_OA_ATEXECUTE: i32 = 2i32;
pub const DBPROPVAL_OA_ATROWRELEASE: i32 = 4i32;
pub const DBPROPVAL_OA_NOTSUPPORTED: i32 = 1i32;
pub const DBPROPVAL_OO_BLOB: i32 = 1i32;
pub const DBPROPVAL_OO_DIRECTBIND: i32 = 16i32;
pub const DBPROPVAL_OO_IPERSIST: i32 = 2i32;
pub const DBPROPVAL_OO_ROWOBJECT: i32 = 4i32;
pub const DBPROPVAL_OO_SCOPED: i32 = 8i32;
pub const DBPROPVAL_OO_SINGLETON: i32 = 32i32;
pub const DBPROPVAL_OP_EQUAL: i32 = 1i32;
pub const DBPROPVAL_OP_RELATIVE: i32 = 2i32;
pub const DBPROPVAL_OP_STRING: i32 = 4i32;
pub const DBPROPVAL_ORS_HISTOGRAM: i32 = 8i32;
pub const DBPROPVAL_ORS_INDEX: i32 = 1i32;
pub const DBPROPVAL_ORS_INTEGRATEDINDEX: i32 = 2i32;
pub const DBPROPVAL_ORS_STOREDPROC: i32 = 4i32;
pub const DBPROPVAL_ORS_TABLE: i32 = 0i32;
pub const DBPROPVAL_OS_AGR_AFTERSESSION: i32 = 8i32;
pub const DBPROPVAL_OS_CLIENTCURSOR: i32 = 4i32;
pub const DBPROPVAL_OS_DISABLEALL: i32 = 0i32;
pub const DBPROPVAL_OS_ENABLEALL: i32 = -1i32;
pub const DBPROPVAL_OS_RESOURCEPOOLING: i32 = 1i32;
pub const DBPROPVAL_OS_TXNENLISTMENT: i32 = 2i32;
pub const DBPROPVAL_PERSIST_ADTG: u32 = 0u32;
pub const DBPROPVAL_PERSIST_XML: u32 = 1u32;
pub const DBPROPVAL_PT_GUID: i32 = 8i32;
pub const DBPROPVAL_PT_GUID_NAME: i32 = 1i32;
pub const DBPROPVAL_PT_GUID_PROPID: i32 = 2i32;
pub const DBPROPVAL_PT_NAME: i32 = 4i32;
pub const DBPROPVAL_PT_PGUID_NAME: i32 = 32i32;
pub const DBPROPVAL_PT_PGUID_PROPID: i32 = 64i32;
pub const DBPROPVAL_PT_PROPID: i32 = 16i32;
pub const DBPROPVAL_RD_RESETALL: i32 = -1i32;
pub const DBPROPVAL_RT_APTMTTHREAD: i32 = 2i32;
pub const DBPROPVAL_RT_FREETHREAD: i32 = 1i32;
pub const DBPROPVAL_RT_SINGLETHREAD: i32 = 4i32;
pub const DBPROPVAL_SQL_ANSI89_IEF: i32 = 8i32;
pub const DBPROPVAL_SQL_ANSI92_ENTRY: i32 = 16i32;
pub const DBPROPVAL_SQL_ANSI92_FULL: i32 = 128i32;
pub const DBPROPVAL_SQL_ANSI92_INTERMEDIATE: i32 = 64i32;
pub const DBPROPVAL_SQL_ESCAPECLAUSES: i32 = 256i32;
pub const DBPROPVAL_SQL_FIPS_TRANSITIONAL: i32 = 32i32;
pub const DBPROPVAL_SQL_NONE: i32 = 0i32;
pub const DBPROPVAL_SQL_ODBC_CORE: i32 = 2i32;
pub const DBPROPVAL_SQL_ODBC_EXTENDED: i32 = 4i32;
pub const DBPROPVAL_SQL_ODBC_MINIMUM: i32 = 1i32;
pub const DBPROPVAL_SQL_SUBMINIMUM: i32 = 512i32;
pub const DBPROPVAL_SQ_COMPARISON: i32 = 2i32;
pub const DBPROPVAL_SQ_CORRELATEDSUBQUERIES: i32 = 1i32;
pub const DBPROPVAL_SQ_EXISTS: i32 = 4i32;
pub const DBPROPVAL_SQ_IN: i32 = 8i32;
pub const DBPROPVAL_SQ_QUANTIFIED: i32 = 16i32;
pub const DBPROPVAL_SQ_TABLE: i32 = 32i32;
pub const DBPROPVAL_SS_ILOCKBYTES: i32 = 8i32;
pub const DBPROPVAL_SS_ISEQUENTIALSTREAM: i32 = 1i32;
pub const DBPROPVAL_SS_ISTORAGE: i32 = 4i32;
pub const DBPROPVAL_SS_ISTREAM: i32 = 2i32;
pub const DBPROPVAL_STGM_CONVERT: u32 = 262144u32;
pub const DBPROPVAL_STGM_DELETEONRELEASE: u32 = 2097152u32;
pub const DBPROPVAL_STGM_DIRECT: u32 = 65536u32;
pub const DBPROPVAL_STGM_FAILIFTHERE: u32 = 524288u32;
pub const DBPROPVAL_STGM_PRIORITY: u32 = 1048576u32;
pub const DBPROPVAL_STGM_TRANSACTED: u32 = 131072u32;
pub const DBPROPVAL_SU_DML_STATEMENTS: i32 = 1i32;
pub const DBPROPVAL_SU_INDEX_DEFINITION: i32 = 4i32;
pub const DBPROPVAL_SU_PRIVILEGE_DEFINITION: i32 = 8i32;
pub const DBPROPVAL_SU_TABLE_DEFINITION: i32 = 2i32;
pub const DBPROPVAL_TC_ALL: i32 = 8i32;
pub const DBPROPVAL_TC_DDL_COMMIT: i32 = 2i32;
pub const DBPROPVAL_TC_DDL_IGNORE: i32 = 4i32;
pub const DBPROPVAL_TC_DDL_LOCK: i32 = 16i32;
pub const DBPROPVAL_TC_DML: i32 = 1i32;
pub const DBPROPVAL_TC_NONE: i32 = 0i32;
pub const DBPROPVAL_TI_BROWSE: i32 = 256i32;
pub const DBPROPVAL_TI_CHAOS: i32 = 16i32;
pub const DBPROPVAL_TI_CURSORSTABILITY: i32 = 4096i32;
pub const DBPROPVAL_TI_ISOLATED: i32 = 1048576i32;
pub const DBPROPVAL_TI_READCOMMITTED: i32 = 4096i32;
pub const DBPROPVAL_TI_READUNCOMMITTED: i32 = 256i32;
pub const DBPROPVAL_TI_REPEATABLEREAD: i32 = 65536i32;
pub const DBPROPVAL_TI_SERIALIZABLE: i32 = 1048576i32;
pub const DBPROPVAL_TR_ABORT: i32 = 16i32;
pub const DBPROPVAL_TR_ABORT_DC: i32 = 8i32;
pub const DBPROPVAL_TR_ABORT_NO: i32 = 32i32;
pub const DBPROPVAL_TR_BOTH: i32 = 128i32;
pub const DBPROPVAL_TR_COMMIT: i32 = 2i32;
pub const DBPROPVAL_TR_COMMIT_DC: i32 = 1i32;
pub const DBPROPVAL_TR_COMMIT_NO: i32 = 4i32;
pub const DBPROPVAL_TR_DONTCARE: i32 = 64i32;
pub const DBPROPVAL_TR_NONE: i32 = 256i32;
pub const DBPROPVAL_TR_OPTIMISTIC: i32 = 512i32;
pub const DBPROPVAL_TS_CARDINALITY: i32 = 1i32;
pub const DBPROPVAL_TS_HISTOGRAM: i32 = 2i32;
pub const DBPROPVAL_UP_CHANGE: i32 = 1i32;
pub const DBPROPVAL_UP_DELETE: i32 = 2i32;
pub const DBPROPVAL_UP_INSERT: i32 = 4i32;
pub const DBPROP_ABORTPRESERVE: DBPROPENUM = DBPROPENUM(2i32);
pub const DBPROP_ACCESSORDER: DBPROPENUM20 = DBPROPENUM20(231i32);
pub const DBPROP_ACTIVESESSIONS: DBPROPENUM = DBPROPENUM(3i32);
pub const DBPROP_ALTERCOLUMN: DBPROPENUM20 = DBPROPENUM20(245i32);
pub const DBPROP_APPENDONLY: DBPROPENUM = DBPROPENUM(187i32);
pub const DBPROP_ASYNCTXNABORT: DBPROPENUM = DBPROPENUM(168i32);
pub const DBPROP_ASYNCTXNCOMMIT: DBPROPENUM = DBPROPENUM(4i32);
pub const DBPROP_AUTH_CACHE_AUTHINFO: DBPROPENUM = DBPROPENUM(5i32);
pub const DBPROP_AUTH_ENCRYPT_PASSWORD: DBPROPENUM = DBPROPENUM(6i32);
pub const DBPROP_AUTH_INTEGRATED: DBPROPENUM = DBPROPENUM(7i32);
pub const DBPROP_AUTH_MASK_PASSWORD: DBPROPENUM = DBPROPENUM(8i32);
pub const DBPROP_AUTH_PASSWORD: DBPROPENUM = DBPROPENUM(9i32);
pub const DBPROP_AUTH_PERSIST_ENCRYPTED: DBPROPENUM = DBPROPENUM(10i32);
pub const DBPROP_AUTH_PERSIST_SENSITIVE_AUTHINFO: DBPROPENUM = DBPROPENUM(11i32);
pub const DBPROP_AUTH_USERID: DBPROPENUM = DBPROPENUM(12i32);
pub const DBPROP_BLOCKINGSTORAGEOBJECTS: DBPROPENUM = DBPROPENUM(13i32);
pub const DBPROP_BOOKMARKINFO: DBPROPENUM20 = DBPROPENUM20(232i32);
pub const DBPROP_BOOKMARKS: DBPROPENUM = DBPROPENUM(14i32);
pub const DBPROP_BOOKMARKSKIPPED: DBPROPENUM = DBPROPENUM(15i32);
pub const DBPROP_BOOKMARKTYPE: DBPROPENUM = DBPROPENUM(16i32);
pub const DBPROP_BYREFACCESSORS: DBPROPENUM = DBPROPENUM(120i32);
pub const DBPROP_CACHEDEFERRED: DBPROPENUM = DBPROPENUM(17i32);
pub const DBPROP_CANFETCHBACKWARDS: DBPROPENUM = DBPROPENUM(18i32);
pub const DBPROP_CANHOLDROWS: DBPROPENUM = DBPROPENUM(19i32);
pub const DBPROP_CANSCROLLBACKWARDS: DBPROPENUM = DBPROPENUM(21i32);
pub const DBPROP_CATALOGLOCATION: DBPROPENUM = DBPROPENUM(22i32);
pub const DBPROP_CATALOGTERM: DBPROPENUM = DBPROPENUM(23i32);
pub const DBPROP_CATALOGUSAGE: DBPROPENUM = DBPROPENUM(24i32);
pub const DBPROP_CHANGEINSERTEDROWS: DBPROPENUM = DBPROPENUM(188i32);
pub const DBPROP_CLIENTCURSOR: DBPROPENUM20 = DBPROPENUM20(260i32);
pub const DBPROP_COLUMNDEFINITION: DBPROPENUM = DBPROPENUM(32i32);
pub const DBPROP_COLUMNLCID: DBPROPENUM20 = DBPROPENUM20(246i32);
pub const DBPROP_COLUMNRESTRICT: DBPROPENUM = DBPROPENUM(33i32);
pub const DBPROP_COL_AUTOINCREMENT: DBPROPENUM = DBPROPENUM(26i32);
pub const DBPROP_COL_DEFAULT: DBPROPENUM = DBPROPENUM(27i32);
pub const DBPROP_COL_DESCRIPTION: DBPROPENUM = DBPROPENUM(28i32);
pub const DBPROP_COL_FIXEDLENGTH: DBPROPENUM = DBPROPENUM(167i32);
pub const DBPROP_COL_INCREMENT: DBPROPENUM25 = DBPROPENUM25(283i32);
pub const DBPROP_COL_ISLONG: DBPROPENUM21 = DBPROPENUM21(281i32);
pub const DBPROP_COL_NULLABLE: DBPROPENUM = DBPROPENUM(29i32);
pub const DBPROP_COL_PRIMARYKEY: DBPROPENUM = DBPROPENUM(30i32);
pub const DBPROP_COL_SEED: DBPROPENUM25 = DBPROPENUM25(282i32);
pub const DBPROP_COL_UNIQUE: DBPROPENUM = DBPROPENUM(31i32);
pub const DBPROP_COMMANDTIMEOUT: DBPROPENUM = DBPROPENUM(34i32);
pub const DBPROP_COMMITPRESERVE: DBPROPENUM = DBPROPENUM(35i32);
pub const DBPROP_COMSERVICES: DBPROPENUM25 = DBPROPENUM25(285i32);
pub const DBPROP_CONCATNULLBEHAVIOR: DBPROPENUM = DBPROPENUM(36i32);
pub const DBPROP_CONNECTIONSTATUS: DBPROPENUM20 = DBPROPENUM20(244i32);
pub const DBPROP_CURRENTCATALOG: DBPROPENUM = DBPROPENUM(37i32);
pub const DBPROP_DATASOURCENAME: DBPROPENUM = DBPROPENUM(38i32);
pub const DBPROP_DATASOURCEREADONLY: DBPROPENUM = DBPROPENUM(39i32);
pub const DBPROP_DATASOURCE_TYPE: DBPROPENUM20 = DBPROPENUM20(251i32);
pub const DBPROP_DBMSNAME: DBPROPENUM = DBPROPENUM(40i32);
pub const DBPROP_DBMSVER: DBPROPENUM = DBPROPENUM(41i32);
pub const DBPROP_DEFERRED: DBPROPENUM = DBPROPENUM(42i32);
pub const DBPROP_DELAYSTORAGEOBJECTS: DBPROPENUM = DBPROPENUM(43i32);
pub const DBPROP_DSOTHREADMODEL: DBPROPENUM = DBPROPENUM(169i32);
pub const DBPROP_FILTERCOMPAREOPS: DBPROPENUM15 = DBPROPENUM15(209i32);
pub const DBPROP_FILTEROPS: DBPROPENUMDEPRECATED = DBPROPENUMDEPRECATED(208i32);
pub const DBPROP_FINDCOMPAREOPS: DBPROPENUM15 = DBPROPENUM15(210i32);
pub const DBPROP_GENERATEURL: DBPROPENUM21 = DBPROPENUM21(273i32);
pub const DBPROP_GROUPBY: DBPROPENUM = DBPROPENUM(44i32);
pub const DBPROP_HCHAPTER: u32 = 4u32;
pub const DBPROP_HETEROGENEOUSTABLES: DBPROPENUM = DBPROPENUM(45i32);
pub const DBPROP_HIDDENCOLUMNS: DBPROPENUM20 = DBPROPENUM20(258i32);
pub const DBPROP_IAccessor: DBPROPENUM = DBPROPENUM(121i32);
pub const DBPROP_IBindResource: DBPROPENUM21 = DBPROPENUM21(268i32);
pub const DBPROP_IChapteredRowset: DBPROPENUM15 = DBPROPENUM15(202i32);
pub const DBPROP_IColumnsInfo: DBPROPENUM = DBPROPENUM(122i32);
pub const DBPROP_IColumnsInfo2: DBPROPENUM21 = DBPROPENUM21(275i32);
pub const DBPROP_IColumnsRowset: DBPROPENUM = DBPROPENUM(123i32);
pub const DBPROP_ICommandCost: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(141i32);
pub const DBPROP_ICommandTree: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(142i32);
pub const DBPROP_ICommandValidate: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(143i32);
pub const DBPROP_IConnectionPointContainer: DBPROPENUM = DBPROPENUM(124i32);
pub const DBPROP_IConvertType: DBPROPENUM = DBPROPENUM(194i32);
pub const DBPROP_ICreateRow: DBPROPENUM21 = DBPROPENUM21(269i32);
pub const DBPROP_IDBAsynchStatus: DBPROPENUM15 = DBPROPENUM15(203i32);
pub const DBPROP_IDBBinderProperties: DBPROPENUM21 = DBPROPENUM21(274i32);
pub const DBPROP_IDBSchemaCommand: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(144i32);
pub const DBPROP_IDENTIFIERCASE: DBPROPENUM = DBPROPENUM(46i32);
pub const DBPROP_IGetRow: DBPROPENUM21 = DBPROPENUM21(266i32);
pub const DBPROP_IGetSession: DBPROPENUM21 = DBPROPENUM21(277i32);
pub const DBPROP_IGetSourceRow: DBPROPENUM21 = DBPROPENUM21(278i32);
pub const DBPROP_ILockBytes: DBPROPENUM = DBPROPENUM(136i32);
pub const DBPROP_IMMOBILEROWS: DBPROPENUM = DBPROPENUM(47i32);
pub const DBPROP_IMultipleResults: DBPROPENUM20 = DBPROPENUM20(217i32);
pub const DBPROP_INDEX_AUTOUPDATE: DBPROPENUM = DBPROPENUM(48i32);
pub const DBPROP_INDEX_CLUSTERED: DBPROPENUM = DBPROPENUM(49i32);
pub const DBPROP_INDEX_FILLFACTOR: DBPROPENUM = DBPROPENUM(50i32);
pub const DBPROP_INDEX_INITIALSIZE: DBPROPENUM = DBPROPENUM(51i32);
pub const DBPROP_INDEX_NULLCOLLATION: DBPROPENUM = DBPROPENUM(52i32);
pub const DBPROP_INDEX_NULLS: DBPROPENUM = DBPROPENUM(53i32);
pub const DBPROP_INDEX_PRIMARYKEY: DBPROPENUM = DBPROPENUM(54i32);
pub const DBPROP_INDEX_SORTBOOKMARKS: DBPROPENUM = DBPROPENUM(55i32);
pub const DBPROP_INDEX_TEMPINDEX: DBPROPENUM = DBPROPENUM(163i32);
pub const DBPROP_INDEX_TYPE: DBPROPENUM = DBPROPENUM(56i32);
pub const DBPROP_INDEX_UNIQUE: DBPROPENUM = DBPROPENUM(57i32);
pub const DBPROP_INIT_ASYNCH: DBPROPENUM15 = DBPROPENUM15(200i32);
pub const DBPROP_INIT_BINDFLAGS: DBPROPENUM21 = DBPROPENUM21(270i32);
pub const DBPROP_INIT_CATALOG: DBPROPENUM20 = DBPROPENUM20(233i32);
pub const DBPROP_INIT_DATASOURCE: DBPROPENUM = DBPROPENUM(59i32);
pub const DBPROP_INIT_GENERALTIMEOUT: DBPROPENUM25 = DBPROPENUM25(284i32);
pub const DBPROP_INIT_HWND: DBPROPENUM = DBPROPENUM(60i32);
pub const DBPROP_INIT_IMPERSONATION_LEVEL: DBPROPENUM = DBPROPENUM(61i32);
pub const DBPROP_INIT_LCID: DBPROPENUM = DBPROPENUM(186i32);
pub const DBPROP_INIT_LOCATION: DBPROPENUM = DBPROPENUM(62i32);
pub const DBPROP_INIT_LOCKOWNER: DBPROPENUM21 = DBPROPENUM21(271i32);
pub const DBPROP_INIT_MODE: DBPROPENUM = DBPROPENUM(63i32);
pub const DBPROP_INIT_OLEDBSERVICES: DBPROPENUM20 = DBPROPENUM20(248i32);
pub const DBPROP_INIT_PROMPT: DBPROPENUM = DBPROPENUM(64i32);
pub const DBPROP_INIT_PROTECTION_LEVEL: DBPROPENUM = DBPROPENUM(65i32);
pub const DBPROP_INIT_PROVIDERSTRING: DBPROPENUM = DBPROPENUM(160i32);
pub const DBPROP_INIT_TIMEOUT: DBPROPENUM = DBPROPENUM(66i32);
pub const DBPROP_INTERLEAVEDROWS: u32 = 8u32;
pub const DBPROP_IParentRowset: DBPROPENUM20 = DBPROPENUM20(257i32);
pub const DBPROP_IProvideMoniker: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(125i32);
pub const DBPROP_IQuery: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(146i32);
pub const DBPROP_IReadData: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(147i32);
pub const DBPROP_IRegisterProvider: DBPROPENUM21 = DBPROPENUM21(276i32);
pub const DBPROP_IRow: DBPROPENUM21 = DBPROPENUM21(263i32);
pub const DBPROP_IRowChange: DBPROPENUM21 = DBPROPENUM21(264i32);
pub const DBPROP_IRowSchemaChange: DBPROPENUM21 = DBPROPENUM21(265i32);
pub const DBPROP_IRowset: DBPROPENUM = DBPROPENUM(126i32);
pub const DBPROP_IRowsetAsynch: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(148i32);
pub const DBPROP_IRowsetBookmark: DBPROPENUM26 = DBPROPENUM26(292i32);
pub const DBPROP_IRowsetChange: DBPROPENUM = DBPROPENUM(127i32);
pub const DBPROP_IRowsetCopyRows: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(149i32);
pub const DBPROP_IRowsetCurrentIndex: DBPROPENUM21 = DBPROPENUM21(279i32);
pub const DBPROP_IRowsetExactScroll: DBPROPENUMDEPRECATED = DBPROPENUMDEPRECATED(154i32);
pub const DBPROP_IRowsetFind: DBPROPENUM15 = DBPROPENUM15(204i32);
pub const DBPROP_IRowsetIdentity: DBPROPENUM = DBPROPENUM(128i32);
pub const DBPROP_IRowsetIndex: DBPROPENUM = DBPROPENUM(159i32);
pub const DBPROP_IRowsetInfo: DBPROPENUM = DBPROPENUM(129i32);
pub const DBPROP_IRowsetKeys: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(151i32);
pub const DBPROP_IRowsetLocate: DBPROPENUM = DBPROPENUM(130i32);
pub const DBPROP_IRowsetNewRowAfter: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(152i32);
pub const DBPROP_IRowsetNextRowset: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(153i32);
pub const DBPROP_IRowsetRefresh: DBPROPENUM20 = DBPROPENUM20(249i32);
pub const DBPROP_IRowsetResynch: DBPROPENUM = DBPROPENUM(132i32);
pub const DBPROP_IRowsetScroll: DBPROPENUM = DBPROPENUM(133i32);
pub const DBPROP_IRowsetUpdate: DBPROPENUM = DBPROPENUM(134i32);
pub const DBPROP_IRowsetView: DBPROPENUM15 = DBPROPENUM15(212i32);
pub const DBPROP_IRowsetWatchAll: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(155i32);
pub const DBPROP_IRowsetWatchNotify: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(156i32);
pub const DBPROP_IRowsetWatchRegion: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(157i32);
pub const DBPROP_IRowsetWithParameters: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(158i32);
pub const DBPROP_IScopedOperations: DBPROPENUM21 = DBPROPENUM21(267i32);
pub const DBPROP_ISequentialStream: DBPROPENUM = DBPROPENUM(137i32);
pub const DBPROP_IStorage: DBPROPENUM = DBPROPENUM(138i32);
pub const DBPROP_IStream: DBPROPENUM = DBPROPENUM(139i32);
pub const DBPROP_ISupportErrorInfo: DBPROPENUM = DBPROPENUM(135i32);
pub const DBPROP_IViewChapter: DBPROPENUM15 = DBPROPENUM15(213i32);
pub const DBPROP_IViewFilter: DBPROPENUM15 = DBPROPENUM15(214i32);
pub const DBPROP_IViewRowset: DBPROPENUM15 = DBPROPENUM15(215i32);
pub const DBPROP_IViewSort: DBPROPENUM15 = DBPROPENUM15(216i32);
pub const DBPROP_LITERALBOOKMARKS: DBPROPENUM = DBPROPENUM(67i32);
pub const DBPROP_LITERALIDENTITY: DBPROPENUM = DBPROPENUM(68i32);
pub const DBPROP_LOCKMODE: DBPROPENUM20 = DBPROPENUM20(236i32);
pub const DBPROP_MAINTAINPROPS: u32 = 5u32;
pub const DBPROP_MARSHALLABLE: DBPROPENUMDEPRECATED = DBPROPENUMDEPRECATED(197i32);
pub const DBPROP_MAXINDEXSIZE: DBPROPENUM = DBPROPENUM(70i32);
pub const DBPROP_MAXOPENCHAPTERS: DBPROPENUM15 = DBPROPENUM15(199i32);
pub const DBPROP_MAXOPENROWS: DBPROPENUM = DBPROPENUM(71i32);
pub const DBPROP_MAXORSINFILTER: DBPROPENUM15 = DBPROPENUM15(205i32);
pub const DBPROP_MAXPENDINGROWS: DBPROPENUM = DBPROPENUM(72i32);
pub const DBPROP_MAXROWS: DBPROPENUM = DBPROPENUM(73i32);
pub const DBPROP_MAXROWSIZE: DBPROPENUM = DBPROPENUM(74i32);
pub const DBPROP_MAXROWSIZEINCLUDESBLOB: DBPROPENUM = DBPROPENUM(75i32);
pub const DBPROP_MAXSORTCOLUMNS: DBPROPENUM15 = DBPROPENUM15(206i32);
pub const DBPROP_MAXTABLESINSELECT: DBPROPENUM = DBPROPENUM(76i32);
pub const DBPROP_MAYWRITECOLUMN: DBPROPENUM = DBPROPENUM(77i32);
pub const DBPROP_MEMORYUSAGE: DBPROPENUM = DBPROPENUM(78i32);
pub const DBPROP_MSDAORA8_DETERMINEKEYCOLUMNS: u32 = 2u32;
pub const DBPROP_MSDAORA_DETERMINEKEYCOLUMNS: u32 = 1u32;
pub const DBPROP_MSDS_DBINIT_DATAPROVIDER: MSDSDBINITPROPENUM = MSDSDBINITPROPENUM(2i32);
pub const DBPROP_MSDS_SESS_UNIQUENAMES: MSDSSESSIONPROPENUM = MSDSSESSIONPROPENUM(2i32);
pub const DBPROP_MULTIPLECONNECTIONS: DBPROPENUM20 = DBPROPENUM20(237i32);
pub const DBPROP_MULTIPLEPARAMSETS: DBPROPENUM = DBPROPENUM(191i32);
pub const DBPROP_MULTIPLERESULTS: DBPROPENUM = DBPROPENUM(196i32);
pub const DBPROP_MULTIPLESTORAGEOBJECTS: DBPROPENUM = DBPROPENUM(80i32);
pub const DBPROP_MULTITABLEUPDATE: DBPROPENUM = DBPROPENUM(81i32);
pub const DBPROP_NOTIFICATIONGRANULARITY: DBPROPENUM = DBPROPENUM(198i32);
pub const DBPROP_NOTIFICATIONPHASES: DBPROPENUM = DBPROPENUM(82i32);
pub const DBPROP_NOTIFYCOLUMNSET: DBPROPENUM = DBPROPENUM(171i32);
pub const DBPROP_NOTIFYROWDELETE: DBPROPENUM = DBPROPENUM(173i32);
pub const DBPROP_NOTIFYROWFIRSTCHANGE: DBPROPENUM = DBPROPENUM(174i32);
pub const DBPROP_NOTIFYROWINSERT: DBPROPENUM = DBPROPENUM(175i32);
pub const DBPROP_NOTIFYROWRESYNCH: DBPROPENUM = DBPROPENUM(177i32);
pub const DBPROP_NOTIFYROWSETCHANGED: DBPROPENUM = DBPROPENUM(211i32);
pub const DBPROP_NOTIFYROWSETFETCHPOSITIONCHANGE: DBPROPENUM = DBPROPENUM(179i32);
pub const DBPROP_NOTIFYROWSETRELEASE: DBPROPENUM = DBPROPENUM(178i32);
pub const DBPROP_NOTIFYROWUNDOCHANGE: DBPROPENUM = DBPROPENUM(180i32);
pub const DBPROP_NOTIFYROWUNDODELETE: DBPROPENUM = DBPROPENUM(181i32);
pub const DBPROP_NOTIFYROWUNDOINSERT: DBPROPENUM = DBPROPENUM(182i32);
pub const DBPROP_NOTIFYROWUPDATE: DBPROPENUM = DBPROPENUM(183i32);
pub const DBPROP_NULLCOLLATION: DBPROPENUM = DBPROPENUM(83i32);
pub const DBPROP_OLEOBJECTS: DBPROPENUM = DBPROPENUM(84i32);
pub const DBPROP_OPENROWSETSUPPORT: DBPROPENUM21 = DBPROPENUM21(280i32);
pub const DBPROP_ORDERBYCOLUMNSINSELECT: DBPROPENUM = DBPROPENUM(85i32);
pub const DBPROP_ORDEREDBOOKMARKS: DBPROPENUM = DBPROPENUM(86i32);
pub const DBPROP_OTHERINSERT: DBPROPENUM = DBPROPENUM(87i32);
pub const DBPROP_OTHERUPDATEDELETE: DBPROPENUM = DBPROPENUM(88i32);
pub const DBPROP_OUTPUTENCODING: DBPROPENUM26 = DBPROPENUM26(287i32);
pub const DBPROP_OUTPUTPARAMETERAVAILABILITY: DBPROPENUM = DBPROPENUM(184i32);
pub const DBPROP_OUTPUTSTREAM: DBPROPENUM26 = DBPROPENUM26(286i32);
pub const DBPROP_OWNINSERT: DBPROPENUM = DBPROPENUM(89i32);
pub const DBPROP_OWNUPDATEDELETE: DBPROPENUM = DBPROPENUM(90i32);
pub const DBPROP_PERSISTENTIDTYPE: DBPROPENUM = DBPROPENUM(185i32);
pub const DBPROP_PREPAREABORTBEHAVIOR: DBPROPENUM = DBPROPENUM(91i32);
pub const DBPROP_PREPARECOMMITBEHAVIOR: DBPROPENUM = DBPROPENUM(92i32);
pub const DBPROP_PROCEDURETERM: DBPROPENUM = DBPROPENUM(93i32);
pub const DBPROP_PROVIDERFRIENDLYNAME: DBPROPENUM20 = DBPROPENUM20(235i32);
pub const DBPROP_PROVIDERMEMORY: DBPROPENUM20 = DBPROPENUM20(259i32);
pub const DBPROP_PROVIDERNAME: DBPROPENUM = DBPROPENUM(96i32);
pub const DBPROP_PROVIDEROLEDBVER: DBPROPENUM = DBPROPENUM(97i32);
pub const DBPROP_PROVIDERVER: DBPROPENUM = DBPROPENUM(98i32);
pub const DBPROP_PersistFormat: u32 = 2u32;
pub const DBPROP_PersistSchema: u32 = 3u32;
pub const DBPROP_QUICKRESTART: DBPROPENUM = DBPROPENUM(99i32);
pub const DBPROP_QUOTEDIDENTIFIERCASE: DBPROPENUM = DBPROPENUM(100i32);
pub const DBPROP_REENTRANTEVENTS: DBPROPENUM = DBPROPENUM(101i32);
pub const DBPROP_REMOVEDELETED: DBPROPENUM = DBPROPENUM(102i32);
pub const DBPROP_REPORTMULTIPLECHANGES: DBPROPENUM = DBPROPENUM(103i32);
pub const DBPROP_RESETDATASOURCE: DBPROPENUM20 = DBPROPENUM20(247i32);
pub const DBPROP_RETURNPENDINGINSERTS: DBPROPENUM = DBPROPENUM(189i32);
pub const DBPROP_ROWRESTRICT: DBPROPENUM = DBPROPENUM(104i32);
pub const DBPROP_ROWSETCONVERSIONSONCOMMAND: DBPROPENUM = DBPROPENUM(192i32);
pub const DBPROP_ROWSET_ASYNCH: DBPROPENUM15 = DBPROPENUM15(201i32);
pub const DBPROP_ROWTHREADMODEL: DBPROPENUM = DBPROPENUM(105i32);
pub const DBPROP_ROW_BULKOPS: DBPROPENUM20 = DBPROPENUM20(234i32);
pub const DBPROP_SCHEMATERM: DBPROPENUM = DBPROPENUM(106i32);
pub const DBPROP_SCHEMAUSAGE: DBPROPENUM = DBPROPENUM(107i32);
pub const DBPROP_SERVERCURSOR: DBPROPENUM = DBPROPENUM(108i32);
pub const DBPROP_SERVERDATAONINSERT: DBPROPENUM20 = DBPROPENUM20(239i32);
pub const DBPROP_SERVERNAME: DBPROPENUM20 = DBPROPENUM20(250i32);
pub const DBPROP_SESS_AUTOCOMMITISOLEVELS: DBPROPENUM = DBPROPENUM(190i32);
pub const DBPROP_SKIPROWCOUNTRESULTS: DBPROPENUM26 = DBPROPENUM26(291i32);
pub const DBPROP_SORTONINDEX: DBPROPENUM15 = DBPROPENUM15(207i32);
pub const DBPROP_SQLSUPPORT: DBPROPENUM = DBPROPENUM(109i32);
pub const DBPROP_STORAGEFLAGS: DBPROPENUM20 = DBPROPENUM20(240i32);
pub const DBPROP_STRONGIDENTITY: DBPROPENUM = DBPROPENUM(119i32);
pub const DBPROP_STRUCTUREDSTORAGE: DBPROPENUM = DBPROPENUM(111i32);
pub const DBPROP_SUBQUERIES: DBPROPENUM = DBPROPENUM(112i32);
pub const DBPROP_SUPPORTEDTXNDDL: DBPROPENUM = DBPROPENUM(161i32);
pub const DBPROP_SUPPORTEDTXNISOLEVELS: DBPROPENUM = DBPROPENUM(113i32);
pub const DBPROP_SUPPORTEDTXNISORETAIN: DBPROPENUM = DBPROPENUM(114i32);
pub const DBPROP_TABLESTATISTICS: DBPROPENUM26 = DBPROPENUM26(288i32);
pub const DBPROP_TABLETERM: DBPROPENUM = DBPROPENUM(115i32);
pub const DBPROP_TBL_TEMPTABLE: DBPROPENUM = DBPROPENUM(140i32);
pub const DBPROP_TRANSACTEDOBJECT: DBPROPENUM = DBPROPENUM(116i32);
pub const DBPROP_TRUSTEE_AUTHENTICATION: DBPROPENUM21 = DBPROPENUM21(242i32);
pub const DBPROP_TRUSTEE_NEWAUTHENTICATION: DBPROPENUM21 = DBPROPENUM21(243i32);
pub const DBPROP_TRUSTEE_USERNAME: DBPROPENUM21 = DBPROPENUM21(241i32);
pub const DBPROP_UNIQUEROWS: DBPROPENUM20 = DBPROPENUM20(238i32);
pub const DBPROP_UPDATABILITY: DBPROPENUM = DBPROPENUM(117i32);
pub const DBPROP_USERNAME: DBPROPENUM = DBPROPENUM(118i32);
pub const DBPROP_Unicode: u32 = 6u32;
pub const DBQUERYGUID: windows_core::GUID = windows_core::GUID::from_u128(0x49691c90_7e17_101a_a91c_08002b2ecda9);
pub const DBRANGE_EXCLUDENULLS: DBRANGEENUM = DBRANGEENUM(4i32);
pub const DBRANGE_EXCLUSIVEEND: DBRANGEENUM = DBRANGEENUM(2i32);
pub const DBRANGE_EXCLUSIVESTART: DBRANGEENUM = DBRANGEENUM(1i32);
pub const DBRANGE_INCLUSIVEEND: DBRANGEENUM = DBRANGEENUM(0i32);
pub const DBRANGE_INCLUSIVESTART: DBRANGEENUM = DBRANGEENUM(0i32);
pub const DBRANGE_MATCH: DBRANGEENUM = DBRANGEENUM(16i32);
pub const DBRANGE_MATCH_N_MASK: DBRANGEENUM20 = DBRANGEENUM20(255i32);
pub const DBRANGE_MATCH_N_SHIFT: DBRANGEENUM20 = DBRANGEENUM20(24i32);
pub const DBRANGE_PREFIX: DBRANGEENUM = DBRANGEENUM(8i32);
pub const DBREASON_COLUMN_RECALCULATED: DBREASONENUM = DBREASONENUM(3i32);
pub const DBREASON_COLUMN_SET: DBREASONENUM = DBREASONENUM(2i32);
pub const DBREASON_ROWPOSITION_CHANGED: DBREASONENUM15 = DBREASONENUM15(15i32);
pub const DBREASON_ROWPOSITION_CHAPTERCHANGED: DBREASONENUM15 = DBREASONENUM15(16i32);
pub const DBREASON_ROWPOSITION_CLEARED: DBREASONENUM15 = DBREASONENUM15(17i32);
pub const DBREASON_ROWSET_CHANGED: DBREASONENUM = DBREASONENUM(14i32);
pub const DBREASON_ROWSET_FETCHPOSITIONCHANGE: DBREASONENUM = DBREASONENUM(0i32);
pub const DBREASON_ROWSET_POPULATIONCOMPLETE: DBREASONENUM25 = DBREASONENUM25(20i32);
pub const DBREASON_ROWSET_POPULATIONSTOPPED: DBREASONENUM25 = DBREASONENUM25(21i32);
pub const DBREASON_ROWSET_RELEASE: DBREASONENUM = DBREASONENUM(1i32);
pub const DBREASON_ROWSET_ROWSADDED: DBREASONENUM25 = DBREASONENUM25(19i32);
pub const DBREASON_ROW_ACTIVATE: DBREASONENUM = DBREASONENUM(4i32);
pub const DBREASON_ROW_ASYNCHINSERT: DBREASONENUM15 = DBREASONENUM15(18i32);
pub const DBREASON_ROW_DELETE: DBREASONENUM = DBREASONENUM(6i32);
pub const DBREASON_ROW_FIRSTCHANGE: DBREASONENUM = DBREASONENUM(7i32);
pub const DBREASON_ROW_INSERT: DBREASONENUM = DBREASONENUM(8i32);
pub const DBREASON_ROW_RELEASE: DBREASONENUM = DBREASONENUM(5i32);
pub const DBREASON_ROW_RESYNCH: DBREASONENUM = DBREASONENUM(9i32);
pub const DBREASON_ROW_UNDOCHANGE: DBREASONENUM = DBREASONENUM(10i32);
pub const DBREASON_ROW_UNDODELETE: DBREASONENUM = DBREASONENUM(12i32);
pub const DBREASON_ROW_UNDOINSERT: DBREASONENUM = DBREASONENUM(11i32);
pub const DBREASON_ROW_UPDATE: DBREASONENUM = DBREASONENUM(13i32);
pub const DBRESOURCE_CPU: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(2i32);
pub const DBRESOURCE_DISK: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(4i32);
pub const DBRESOURCE_INVALID: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(0i32);
pub const DBRESOURCE_MEMORY: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(3i32);
pub const DBRESOURCE_NETWORK: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(5i32);
pub const DBRESOURCE_OTHER: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(8i32);
pub const DBRESOURCE_RESPONSE: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(6i32);
pub const DBRESOURCE_ROWS: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(7i32);
pub const DBRESOURCE_TOTAL: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(1i32);
pub const DBRESULTFLAG_DEFAULT: DBRESULTFLAGENUM = DBRESULTFLAGENUM(0i32);
pub const DBRESULTFLAG_ROW: DBRESULTFLAGENUM = DBRESULTFLAGENUM(2i32);
pub const DBRESULTFLAG_ROWSET: DBRESULTFLAGENUM = DBRESULTFLAGENUM(1i32);
pub const DBROWCHANGEKIND_COUNT: DBROWCHANGEKINDENUM = DBROWCHANGEKINDENUM(3i32);
pub const DBROWCHANGEKIND_DELETE: DBROWCHANGEKINDENUM = DBROWCHANGEKINDENUM(1i32);
pub const DBROWCHANGEKIND_INSERT: DBROWCHANGEKINDENUM = DBROWCHANGEKINDENUM(0i32);
pub const DBROWCHANGEKIND_UPDATE: DBROWCHANGEKINDENUM = DBROWCHANGEKINDENUM(2i32);
pub const DBROWSTATUS_E_CANCELED: DBROWSTATUSENUM = DBROWSTATUSENUM(4i32);
pub const DBROWSTATUS_E_CANTRELEASE: DBROWSTATUSENUM = DBROWSTATUSENUM(6i32);
pub const DBROWSTATUS_E_CONCURRENCYVIOLATION: DBROWSTATUSENUM = DBROWSTATUSENUM(7i32);
pub const DBROWSTATUS_E_DELETED: DBROWSTATUSENUM = DBROWSTATUSENUM(8i32);
pub const DBROWSTATUS_E_FAIL: DBROWSTATUSENUM = DBROWSTATUSENUM(19i32);
pub const DBROWSTATUS_E_INTEGRITYVIOLATION: DBROWSTATUSENUM = DBROWSTATUSENUM(11i32);
pub const DBROWSTATUS_E_INVALID: DBROWSTATUSENUM = DBROWSTATUSENUM(12i32);
pub const DBROWSTATUS_E_LIMITREACHED: DBROWSTATUSENUM = DBROWSTATUSENUM(17i32);
pub const DBROWSTATUS_E_MAXPENDCHANGESEXCEEDED: DBROWSTATUSENUM = DBROWSTATUSENUM(13i32);
pub const DBROWSTATUS_E_NEWLYINSERTED: DBROWSTATUSENUM = DBROWSTATUSENUM(10i32);
pub const DBROWSTATUS_E_OBJECTOPEN: DBROWSTATUSENUM = DBROWSTATUSENUM(14i32);
pub const DBROWSTATUS_E_OUTOFMEMORY: DBROWSTATUSENUM = DBROWSTATUSENUM(15i32);
pub const DBROWSTATUS_E_PENDINGINSERT: DBROWSTATUSENUM = DBROWSTATUSENUM(9i32);
pub const DBROWSTATUS_E_PERMISSIONDENIED: DBROWSTATUSENUM = DBROWSTATUSENUM(16i32);
pub const DBROWSTATUS_E_SCHEMAVIOLATION: DBROWSTATUSENUM = DBROWSTATUSENUM(18i32);
pub const DBROWSTATUS_S_MULTIPLECHANGES: DBROWSTATUSENUM = DBROWSTATUSENUM(2i32);
pub const DBROWSTATUS_S_NOCHANGE: DBROWSTATUSENUM20 = DBROWSTATUSENUM20(20i32);
pub const DBROWSTATUS_S_OK: DBROWSTATUSENUM = DBROWSTATUSENUM(0i32);
pub const DBROWSTATUS_S_PENDINGCHANGES: DBROWSTATUSENUM = DBROWSTATUSENUM(3i32);
pub const DBSCHEMA_LINKEDSERVERS: windows_core::GUID = windows_core::GUID::from_u128(0x9093caf4_2eac_11d1_9809_00c04fc2ad98);
pub const DBSEEK_AFTER: DBSEEKENUM = DBSEEKENUM(8i32);
pub const DBSEEK_AFTEREQ: DBSEEKENUM = DBSEEKENUM(4i32);
pub const DBSEEK_BEFORE: DBSEEKENUM = DBSEEKENUM(32i32);
pub const DBSEEK_BEFOREEQ: DBSEEKENUM = DBSEEKENUM(16i32);
pub const DBSEEK_FIRSTEQ: DBSEEKENUM = DBSEEKENUM(1i32);
pub const DBSEEK_INVALID: DBSEEKENUM = DBSEEKENUM(0i32);
pub const DBSEEK_LASTEQ: DBSEEKENUM = DBSEEKENUM(2i32);
pub const DBSELFGUID: windows_core::GUID = windows_core::GUID::from_u128(0xc8b52231_5cf3_11ce_ade5_00aa0044773d);
pub const DBSORT_ASCENDING: DBSORTENUM = DBSORTENUM(0i32);
pub const DBSORT_DESCENDING: DBSORTENUM = DBSORTENUM(1i32);
pub const DBSOURCETYPE_BINDER: DBSOURCETYPEENUM25 = DBSOURCETYPEENUM25(4i32);
pub const DBSOURCETYPE_DATASOURCE: DBSOURCETYPEENUM = DBSOURCETYPEENUM(1i32);
pub const DBSOURCETYPE_DATASOURCE_MDP: DBSOURCETYPEENUM20 = DBSOURCETYPEENUM20(3i32);
pub const DBSOURCETYPE_DATASOURCE_TDP: DBSOURCETYPEENUM20 = DBSOURCETYPEENUM20(1i32);
pub const DBSOURCETYPE_ENUMERATOR: DBSOURCETYPEENUM = DBSOURCETYPEENUM(2i32);
pub const DBSTATUS_E_BADACCESSOR: DBSTATUSENUM = DBSTATUSENUM(1i32);
pub const DBSTATUS_E_BADSTATUS: DBSTATUSENUM = DBSTATUSENUM(12i32);
pub const DBSTATUS_E_CANCELED: DBSTATUSENUM25 = DBSTATUSENUM25(27i32);
pub const DBSTATUS_E_CANNOTCOMPLETE: DBSTATUSENUM21 = DBSTATUSENUM21(20i32);
pub const DBSTATUS_E_CANTCONVERTVALUE: DBSTATUSENUM = DBSTATUSENUM(2i32);
pub const DBSTATUS_E_CANTCREATE: DBSTATUSENUM = DBSTATUSENUM(7i32);
pub const DBSTATUS_E_DATAOVERFLOW: DBSTATUSENUM = DBSTATUSENUM(6i32);
pub const DBSTATUS_E_DOESNOTEXIST: DBSTATUSENUM21 = DBSTATUSENUM21(16i32);
pub const DBSTATUS_E_INTEGRITYVIOLATION: DBSTATUSENUM = DBSTATUSENUM(10i32);
pub const DBSTATUS_E_INVALIDURL: DBSTATUSENUM21 = DBSTATUSENUM21(17i32);
pub const DBSTATUS_E_NOTCOLLECTION: DBSTATUSENUM25 = DBSTATUSENUM25(28i32);
pub const DBSTATUS_E_OUTOFSPACE: DBSTATUSENUM21 = DBSTATUSENUM21(22i32);
pub const DBSTATUS_E_PERMISSIONDENIED: DBSTATUSENUM = DBSTATUSENUM(9i32);
pub const DBSTATUS_E_READONLY: DBSTATUSENUM21 = DBSTATUSENUM21(24i32);
pub const DBSTATUS_E_RESOURCEEXISTS: DBSTATUSENUM21 = DBSTATUSENUM21(19i32);
pub const DBSTATUS_E_RESOURCELOCKED: DBSTATUSENUM21 = DBSTATUSENUM21(18i32);
pub const DBSTATUS_E_RESOURCEOUTOFSCOPE: DBSTATUSENUM21 = DBSTATUSENUM21(25i32);
pub const DBSTATUS_E_SCHEMAVIOLATION: DBSTATUSENUM = DBSTATUSENUM(11i32);
pub const DBSTATUS_E_SIGNMISMATCH: DBSTATUSENUM = DBSTATUSENUM(5i32);
pub const DBSTATUS_E_UNAVAILABLE: DBSTATUSENUM = DBSTATUSENUM(8i32);
pub const DBSTATUS_E_VOLUMENOTFOUND: DBSTATUSENUM21 = DBSTATUSENUM21(21i32);
pub const DBSTATUS_S_ALREADYEXISTS: DBSTATUSENUM21 = DBSTATUSENUM21(26i32);
pub const DBSTATUS_S_CANNOTDELETESOURCE: DBSTATUSENUM21 = DBSTATUSENUM21(23i32);
pub const DBSTATUS_S_DEFAULT: DBSTATUSENUM = DBSTATUSENUM(13i32);
pub const DBSTATUS_S_IGNORE: DBSTATUSENUM20 = DBSTATUSENUM20(15i32);
pub const DBSTATUS_S_ISNULL: DBSTATUSENUM = DBSTATUSENUM(3i32);
pub const DBSTATUS_S_OK: DBSTATUSENUM = DBSTATUSENUM(0i32);
pub const DBSTATUS_S_ROWSETCOLUMN: DBSTATUSENUM26 = DBSTATUSENUM26(29i32);
pub const DBSTATUS_S_TRUNCATED: DBSTATUSENUM = DBSTATUSENUM(4i32);
pub const DBSTAT_COLUMN_CARDINALITY: DBTABLESTATISTICSTYPE26 = DBTABLESTATISTICSTYPE26(2i32);
pub const DBSTAT_HISTOGRAM: DBTABLESTATISTICSTYPE26 = DBTABLESTATISTICSTYPE26(1i32);
pub const DBSTAT_TUPLE_CARDINALITY: DBTABLESTATISTICSTYPE26 = DBTABLESTATISTICSTYPE26(4i32);
pub const DBTYPE_ARRAY: DBTYPEENUM = DBTYPEENUM(8192i32);
pub const DBTYPE_BOOL: DBTYPEENUM = DBTYPEENUM(11i32);
pub const DBTYPE_BSTR: DBTYPEENUM = DBTYPEENUM(8i32);
pub const DBTYPE_BYREF: DBTYPEENUM = DBTYPEENUM(16384i32);
pub const DBTYPE_BYTES: DBTYPEENUM = DBTYPEENUM(128i32);
pub const DBTYPE_CY: DBTYPEENUM = DBTYPEENUM(6i32);
pub const DBTYPE_DATE: DBTYPEENUM = DBTYPEENUM(7i32);
pub const DBTYPE_DBDATE: DBTYPEENUM = DBTYPEENUM(133i32);
pub const DBTYPE_DBTIME: DBTYPEENUM = DBTYPEENUM(134i32);
pub const DBTYPE_DBTIMESTAMP: DBTYPEENUM = DBTYPEENUM(135i32);
pub const DBTYPE_DECIMAL: DBTYPEENUM = DBTYPEENUM(14i32);
pub const DBTYPE_EMPTY: DBTYPEENUM = DBTYPEENUM(0i32);
pub const DBTYPE_ERROR: DBTYPEENUM = DBTYPEENUM(10i32);
pub const DBTYPE_FILETIME: DBTYPEENUM20 = DBTYPEENUM20(64i32);
pub const DBTYPE_GUID: DBTYPEENUM = DBTYPEENUM(72i32);
pub const DBTYPE_HCHAPTER: DBTYPEENUM15 = DBTYPEENUM15(136i32);
pub const DBTYPE_I1: DBTYPEENUM = DBTYPEENUM(16i32);
pub const DBTYPE_I2: DBTYPEENUM = DBTYPEENUM(2i32);
pub const DBTYPE_I4: DBTYPEENUM = DBTYPEENUM(3i32);
pub const DBTYPE_I8: DBTYPEENUM = DBTYPEENUM(20i32);
pub const DBTYPE_IDISPATCH: DBTYPEENUM = DBTYPEENUM(9i32);
pub const DBTYPE_IUNKNOWN: DBTYPEENUM = DBTYPEENUM(13i32);
pub const DBTYPE_NULL: DBTYPEENUM = DBTYPEENUM(1i32);
pub const DBTYPE_NUMERIC: DBTYPEENUM = DBTYPEENUM(131i32);
pub const DBTYPE_PROPVARIANT: DBTYPEENUM20 = DBTYPEENUM20(138i32);
pub const DBTYPE_R4: DBTYPEENUM = DBTYPEENUM(4i32);
pub const DBTYPE_R8: DBTYPEENUM = DBTYPEENUM(5i32);
pub const DBTYPE_RESERVED: DBTYPEENUM = DBTYPEENUM(32768i32);
pub const DBTYPE_SQLVARIANT: u32 = 144u32;
pub const DBTYPE_STR: DBTYPEENUM = DBTYPEENUM(129i32);
pub const DBTYPE_UDT: DBTYPEENUM = DBTYPEENUM(132i32);
pub const DBTYPE_UI1: DBTYPEENUM = DBTYPEENUM(17i32);
pub const DBTYPE_UI2: DBTYPEENUM = DBTYPEENUM(18i32);
pub const DBTYPE_UI4: DBTYPEENUM = DBTYPEENUM(19i32);
pub const DBTYPE_UI8: DBTYPEENUM = DBTYPEENUM(21i32);
pub const DBTYPE_VARIANT: DBTYPEENUM = DBTYPEENUM(12i32);
pub const DBTYPE_VARNUMERIC: DBTYPEENUM20 = DBTYPEENUM20(139i32);
pub const DBTYPE_VECTOR: DBTYPEENUM = DBTYPEENUM(4096i32);
pub const DBTYPE_WSTR: DBTYPEENUM = DBTYPEENUM(130i32);
pub const DBUNIT_BYTE: DBCOSTUNITENUM = DBCOSTUNITENUM(512i32);
pub const DBUNIT_GIGA_BYTE: DBCOSTUNITENUM = DBCOSTUNITENUM(4096i32);
pub const DBUNIT_HOUR: DBCOSTUNITENUM = DBCOSTUNITENUM(256i32);
pub const DBUNIT_INVALID: DBCOSTUNITENUM = DBCOSTUNITENUM(0i32);
pub const DBUNIT_KILO_BYTE: DBCOSTUNITENUM = DBCOSTUNITENUM(1024i32);
pub const DBUNIT_MAXIMUM: DBCOSTUNITENUM = DBCOSTUNITENUM(4i32);
pub const DBUNIT_MEGA_BYTE: DBCOSTUNITENUM = DBCOSTUNITENUM(2048i32);
pub const DBUNIT_MICRO_SECOND: DBCOSTUNITENUM = DBCOSTUNITENUM(16i32);
pub const DBUNIT_MILLI_SECOND: DBCOSTUNITENUM = DBCOSTUNITENUM(32i32);
pub const DBUNIT_MINIMUM: DBCOSTUNITENUM = DBCOSTUNITENUM(8i32);
pub const DBUNIT_MINUTE: DBCOSTUNITENUM = DBCOSTUNITENUM(128i32);
pub const DBUNIT_NUM_LOCKS: DBCOSTUNITENUM = DBCOSTUNITENUM(16384i32);
pub const DBUNIT_NUM_MSGS: DBCOSTUNITENUM = DBCOSTUNITENUM(8192i32);
pub const DBUNIT_NUM_ROWS: DBCOSTUNITENUM = DBCOSTUNITENUM(32768i32);
pub const DBUNIT_OTHER: DBCOSTUNITENUM = DBCOSTUNITENUM(65536i32);
pub const DBUNIT_PERCENT: DBCOSTUNITENUM = DBCOSTUNITENUM(2i32);
pub const DBUNIT_SECOND: DBCOSTUNITENUM = DBCOSTUNITENUM(64i32);
pub const DBUNIT_WEIGHT: DBCOSTUNITENUM = DBCOSTUNITENUM(1i32);
pub const DBUPDELRULE_CASCADE: DBUPDELRULEENUM = DBUPDELRULEENUM(1i32);
pub const DBUPDELRULE_NOACTION: DBUPDELRULEENUM = DBUPDELRULEENUM(0i32);
pub const DBUPDELRULE_SETDEFAULT: DBUPDELRULEENUM = DBUPDELRULEENUM(3i32);
pub const DBUPDELRULE_SETNULL: DBUPDELRULEENUM = DBUPDELRULEENUM(2i32);
pub const DBWATCHMODE_ALL: DBWATCHMODEENUM = DBWATCHMODEENUM(1i32);
pub const DBWATCHMODE_COUNT: DBWATCHMODEENUM = DBWATCHMODEENUM(8i32);
pub const DBWATCHMODE_EXTEND: DBWATCHMODEENUM = DBWATCHMODEENUM(2i32);
pub const DBWATCHMODE_MOVE: DBWATCHMODEENUM = DBWATCHMODEENUM(4i32);
pub const DBWATCHNOTIFY_QUERYDONE: DBWATCHNOTIFYENUM = DBWATCHNOTIFYENUM(2i32);
pub const DBWATCHNOTIFY_QUERYREEXECUTED: DBWATCHNOTIFYENUM = DBWATCHNOTIFYENUM(3i32);
pub const DBWATCHNOTIFY_ROWSCHANGED: DBWATCHNOTIFYENUM = DBWATCHNOTIFYENUM(1i32);
pub const DB_ALL_EXCEPT_LIKE: u32 = 3u32;
pub const DB_BINDFLAGS_COLLECTION: i32 = 16i32;
pub const DB_BINDFLAGS_DELAYFETCHCOLUMNS: i32 = 1i32;
pub const DB_BINDFLAGS_DELAYFETCHSTREAM: i32 = 2i32;
pub const DB_BINDFLAGS_ISSTRUCTUREDDOCUMENT: i32 = 128i32;
pub const DB_BINDFLAGS_OPENIFEXISTS: i32 = 32i32;
pub const DB_BINDFLAGS_OUTPUT: i32 = 8i32;
pub const DB_BINDFLAGS_OVERWRITE: i32 = 64i32;
pub const DB_BINDFLAGS_RECURSIVE: i32 = 4i32;
pub const DB_COLLATION_ASC: u32 = 1u32;
pub const DB_COLLATION_DESC: u32 = 2u32;
pub const DB_COUNTUNAVAILABLE: i32 = -1i32;
pub const DB_E_ABORTLIMITREACHED: windows_core::HRESULT = windows_core::HRESULT(0x80040E31_u32 as _);
pub const DB_E_ALREADYINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80040E52_u32 as _);
pub const DB_E_ALTERRESTRICTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E9D_u32 as _);
pub const DB_E_ASYNCNOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E95_u32 as _);
pub const DB_E_BADACCESSORFLAGS: windows_core::HRESULT = windows_core::HRESULT(0x80040E46_u32 as _);
pub const DB_E_BADACCESSORHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80040E00_u32 as _);
pub const DB_E_BADACCESSORTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040E4B_u32 as _);
pub const DB_E_BADBINDINFO: windows_core::HRESULT = windows_core::HRESULT(0x80040E08_u32 as _);
pub const DB_E_BADBOOKMARK: windows_core::HRESULT = windows_core::HRESULT(0x80040E0E_u32 as _);
pub const DB_E_BADCHAPTER: windows_core::HRESULT = windows_core::HRESULT(0x80040E06_u32 as _);
pub const DB_E_BADCOLUMNID: windows_core::HRESULT = windows_core::HRESULT(0x80040E11_u32 as _);
pub const DB_E_BADCOMMANDFLAGS: windows_core::HRESULT = windows_core::HRESULT(0x80040E8C_u32 as _);
pub const DB_E_BADCOMMANDID: windows_core::HRESULT = windows_core::HRESULT(0x80040E76_u32 as _);
pub const DB_E_BADCOMPAREOP: windows_core::HRESULT = windows_core::HRESULT(0x80040E27_u32 as _);
pub const DB_E_BADCONSTRAINTFORM: windows_core::HRESULT = windows_core::HRESULT(0x80040E78_u32 as _);
pub const DB_E_BADCONSTRAINTID: windows_core::HRESULT = windows_core::HRESULT(0x80040E8B_u32 as _);
pub const DB_E_BADCONSTRAINTTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040E77_u32 as _);
pub const DB_E_BADCONVERTFLAG: windows_core::HRESULT = windows_core::HRESULT(0x80040E5C_u32 as _);
pub const DB_E_BADCOPY: windows_core::HRESULT = windows_core::HRESULT(0x80040E39_u32 as _);
pub const DB_E_BADDEFERRABILITY: windows_core::HRESULT = windows_core::HRESULT(0x80040E79_u32 as _);
pub const DB_E_BADDYNAMICERRORID: windows_core::HRESULT = windows_core::HRESULT(0x80040E5A_u32 as _);
pub const DB_E_BADHRESULT: windows_core::HRESULT = windows_core::HRESULT(0x80040E58_u32 as _);
pub const DB_E_BADID: i32 = -2147217860i32;
pub const DB_E_BADINDEXID: windows_core::HRESULT = windows_core::HRESULT(0x80040E72_u32 as _);
pub const DB_E_BADINITSTRING: windows_core::HRESULT = windows_core::HRESULT(0x80040E73_u32 as _);
pub const DB_E_BADLOCKMODE: windows_core::HRESULT = windows_core::HRESULT(0x80040E0F_u32 as _);
pub const DB_E_BADLOOKUPID: windows_core::HRESULT = windows_core::HRESULT(0x80040E59_u32 as _);
pub const DB_E_BADMATCHTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040E80_u32 as _);
pub const DB_E_BADORDINAL: windows_core::HRESULT = windows_core::HRESULT(0x80040E55_u32 as _);
pub const DB_E_BADPARAMETERNAME: windows_core::HRESULT = windows_core::HRESULT(0x80040E5D_u32 as _);
pub const DB_E_BADPRECISION: windows_core::HRESULT = windows_core::HRESULT(0x80040E3A_u32 as _);
pub const DB_E_BADPROPERTYVALUE: windows_core::HRESULT = windows_core::HRESULT(0x80040E44_u32 as _);
pub const DB_E_BADRATIO: windows_core::HRESULT = windows_core::HRESULT(0x80040E12_u32 as _);
pub const DB_E_BADRECORDNUM: windows_core::HRESULT = windows_core::HRESULT(0x80040E42_u32 as _);
pub const DB_E_BADREGIONHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80040E2A_u32 as _);
pub const DB_E_BADROWHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80040E04_u32 as _);
pub const DB_E_BADSCALE: windows_core::HRESULT = windows_core::HRESULT(0x80040E3B_u32 as _);
pub const DB_E_BADSOURCEHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80040E50_u32 as _);
pub const DB_E_BADSTARTPOSITION: windows_core::HRESULT = windows_core::HRESULT(0x80040E1E_u32 as _);
pub const DB_E_BADSTATUSVALUE: windows_core::HRESULT = windows_core::HRESULT(0x80040E28_u32 as _);
pub const DB_E_BADSTORAGEFLAG: windows_core::HRESULT = windows_core::HRESULT(0x80040E26_u32 as _);
pub const DB_E_BADSTORAGEFLAGS: windows_core::HRESULT = windows_core::HRESULT(0x80040E47_u32 as _);
pub const DB_E_BADTABLEID: windows_core::HRESULT = windows_core::HRESULT(0x80040E3C_u32 as _);
pub const DB_E_BADTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040E3D_u32 as _);
pub const DB_E_BADTYPENAME: windows_core::HRESULT = windows_core::HRESULT(0x80040E30_u32 as _);
pub const DB_E_BADUPDATEDELETERULE: windows_core::HRESULT = windows_core::HRESULT(0x80040E8A_u32 as _);
pub const DB_E_BADVALUES: windows_core::HRESULT = windows_core::HRESULT(0x80040E13_u32 as _);
pub const DB_E_BOGUS: windows_core::HRESULT = windows_core::HRESULT(0x80040EFF_u32 as _);
pub const DB_E_BOOKMARKSKIPPED: windows_core::HRESULT = windows_core::HRESULT(0x80040E43_u32 as _);
pub const DB_E_BYREFACCESSORNOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E48_u32 as _);
pub const DB_E_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x80040E4E_u32 as _);
pub const DB_E_CANNOTCONNECT: windows_core::HRESULT = windows_core::HRESULT(0x80040E96_u32 as _);
pub const DB_E_CANNOTFREE: windows_core::HRESULT = windows_core::HRESULT(0x80040E1A_u32 as _);
pub const DB_E_CANNOTRESTART: windows_core::HRESULT = windows_core::HRESULT(0x80040E18_u32 as _);
pub const DB_E_CANTCANCEL: windows_core::HRESULT = windows_core::HRESULT(0x80040E15_u32 as _);
pub const DB_E_CANTCONVERTVALUE: windows_core::HRESULT = windows_core::HRESULT(0x80040E07_u32 as _);
pub const DB_E_CANTFETCHBACKWARDS: windows_core::HRESULT = windows_core::HRESULT(0x80040E24_u32 as _);
pub const DB_E_CANTFILTER: windows_core::HRESULT = windows_core::HRESULT(0x80040E5F_u32 as _);
pub const DB_E_CANTORDER: windows_core::HRESULT = windows_core::HRESULT(0x80040E60_u32 as _);
pub const DB_E_CANTSCROLLBACKWARDS: windows_core::HRESULT = windows_core::HRESULT(0x80040E29_u32 as _);
pub const DB_E_CANTTRANSLATE: windows_core::HRESULT = windows_core::HRESULT(0x80040E33_u32 as _);
pub const DB_E_CHAPTERNOTRELEASED: windows_core::HRESULT = windows_core::HRESULT(0x80040E4F_u32 as _);
pub const DB_E_COLUMNUNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80040EA0_u32 as _);
pub const DB_E_COMMANDNOTPERSISTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E67_u32 as _);
pub const DB_E_CONCURRENCYVIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x80040E38_u32 as _);
pub const DB_E_COSTLIMIT: windows_core::HRESULT = windows_core::HRESULT(0x80040E0D_u32 as _);
pub const DB_E_DATAOVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0x80040E57_u32 as _);
pub const DB_E_DELETEDROW: windows_core::HRESULT = windows_core::HRESULT(0x80040E23_u32 as _);
pub const DB_E_DIALECTNOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E16_u32 as _);
pub const DB_E_DROPRESTRICTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E90_u32 as _);
pub const DB_E_DUPLICATECOLUMNID: windows_core::HRESULT = windows_core::HRESULT(0x80040E3E_u32 as _);
pub const DB_E_DUPLICATECONSTRAINTID: windows_core::HRESULT = windows_core::HRESULT(0x80040E99_u32 as _);
pub const DB_E_DUPLICATEDATASOURCE: windows_core::HRESULT = windows_core::HRESULT(0x80040E17_u32 as _);
pub const DB_E_DUPLICATEID: windows_core::HRESULT = windows_core::HRESULT(0x80040E68_u32 as _);
pub const DB_E_DUPLICATEINDEXID: windows_core::HRESULT = windows_core::HRESULT(0x80040E34_u32 as _);
pub const DB_E_DUPLICATETABLEID: windows_core::HRESULT = windows_core::HRESULT(0x80040E3F_u32 as _);
pub const DB_E_ERRORSINCOMMAND: windows_core::HRESULT = windows_core::HRESULT(0x80040E14_u32 as _);
pub const DB_E_ERRORSOCCURRED: windows_core::HRESULT = windows_core::HRESULT(0x80040E21_u32 as _);
pub const DB_E_GOALREJECTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E1C_u32 as _);
pub const DB_E_INDEXINUSE: windows_core::HRESULT = windows_core::HRESULT(0x80040E36_u32 as _);
pub const DB_E_INTEGRITYVIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x80040E2F_u32 as _);
pub const DB_E_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80040E45_u32 as _);
pub const DB_E_INVALIDTRANSITION: windows_core::HRESULT = windows_core::HRESULT(0x80040E2C_u32 as _);
pub const DB_E_LIMITREJECTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E0B_u32 as _);
pub const DB_E_MAXPENDCHANGESEXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80040E54_u32 as _);
pub const DB_E_MISMATCHEDPROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x80040E75_u32 as _);
pub const DB_E_MULTIPLESTATEMENTS: windows_core::HRESULT = windows_core::HRESULT(0x80040E2E_u32 as _);
pub const DB_E_MULTIPLESTORAGE: windows_core::HRESULT = windows_core::HRESULT(0x80040E5E_u32 as _);
pub const DB_E_NEWLYINSERTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E1B_u32 as _);
pub const DB_E_NOAGGREGATION: windows_core::HRESULT = windows_core::HRESULT(0x80040E22_u32 as _);
pub const DB_E_NOCOLUMN: windows_core::HRESULT = windows_core::HRESULT(0x80040E65_u32 as _);
pub const DB_E_NOCOMMAND: windows_core::HRESULT = windows_core::HRESULT(0x80040E0C_u32 as _);
pub const DB_E_NOCONSTRAINT: windows_core::HRESULT = windows_core::HRESULT(0x80040E9F_u32 as _);
pub const DB_E_NOINDEX: windows_core::HRESULT = windows_core::HRESULT(0x80040E35_u32 as _);
pub const DB_E_NOLOCALE: windows_core::HRESULT = windows_core::HRESULT(0x80040E41_u32 as _);
pub const DB_E_NONCONTIGUOUSRANGE: windows_core::HRESULT = windows_core::HRESULT(0x80040E2B_u32 as _);
pub const DB_E_NOPROVIDERSREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80040E74_u32 as _);
pub const DB_E_NOQUERY: windows_core::HRESULT = windows_core::HRESULT(0x80040E1F_u32 as _);
pub const DB_E_NOSOURCEOBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80040E91_u32 as _);
pub const DB_E_NOSTATISTIC: windows_core::HRESULT = windows_core::HRESULT(0x80040E9C_u32 as _);
pub const DB_E_NOTABLE: windows_core::HRESULT = windows_core::HRESULT(0x80040E37_u32 as _);
pub const DB_E_NOTAREFERENCECOLUMN: windows_core::HRESULT = windows_core::HRESULT(0x80040E0A_u32 as _);
pub const DB_E_NOTASUBREGION: windows_core::HRESULT = windows_core::HRESULT(0x80040E2D_u32 as _);
pub const DB_E_NOTCOLLECTION: windows_core::HRESULT = windows_core::HRESULT(0x80040E93_u32 as _);
pub const DB_E_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80040E19_u32 as _);
pub const DB_E_NOTPREPARED: windows_core::HRESULT = windows_core::HRESULT(0x80040E4A_u32 as _);
pub const DB_E_NOTREENTRANT: windows_core::HRESULT = windows_core::HRESULT(0x80040E20_u32 as _);
pub const DB_E_NOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E53_u32 as _);
pub const DB_E_NULLACCESSORNOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E49_u32 as _);
pub const DB_E_OBJECTCREATIONLIMITREACHED: windows_core::HRESULT = windows_core::HRESULT(0x80040E69_u32 as _);
pub const DB_E_OBJECTMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80040E8D_u32 as _);
pub const DB_E_OBJECTOPEN: windows_core::HRESULT = windows_core::HRESULT(0x80040E05_u32 as _);
pub const DB_E_OUTOFSPACE: windows_core::HRESULT = windows_core::HRESULT(0x80040E9A_u32 as _);
pub const DB_E_PARAMNOTOPTIONAL: windows_core::HRESULT = windows_core::HRESULT(0x80040E10_u32 as _);
pub const DB_E_PARAMUNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80040E51_u32 as _);
pub const DB_E_PENDINGCHANGES: windows_core::HRESULT = windows_core::HRESULT(0x80040E56_u32 as _);
pub const DB_E_PENDINGINSERT: windows_core::HRESULT = windows_core::HRESULT(0x80040E5B_u32 as _);
pub const DB_E_READONLY: windows_core::HRESULT = windows_core::HRESULT(0x80040E94_u32 as _);
pub const DB_E_READONLYACCESSOR: windows_core::HRESULT = windows_core::HRESULT(0x80040E02_u32 as _);
pub const DB_E_RESOURCEEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80040E98_u32 as _);
pub const DB_E_RESOURCELOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80040E92_u32 as _);
pub const DB_E_RESOURCENOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80040E9E_u32 as _);
pub const DB_E_RESOURCEOUTOFSCOPE: windows_core::HRESULT = windows_core::HRESULT(0x80040E8E_u32 as _);
pub const DB_E_ROWLIMITEXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80040E01_u32 as _);
pub const DB_E_ROWSETINCOMMAND: windows_core::HRESULT = windows_core::HRESULT(0x80040E32_u32 as _);
pub const DB_E_ROWSNOTRELEASED: windows_core::HRESULT = windows_core::HRESULT(0x80040E25_u32 as _);
pub const DB_E_SCHEMAVIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x80040E03_u32 as _);
pub const DB_E_TABLEINUSE: windows_core::HRESULT = windows_core::HRESULT(0x80040E40_u32 as _);
pub const DB_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80040E97_u32 as _);
pub const DB_E_UNSUPPORTEDCONVERSION: windows_core::HRESULT = windows_core::HRESULT(0x80040E1D_u32 as _);
pub const DB_E_WRITEONLYACCESSOR: windows_core::HRESULT = windows_core::HRESULT(0x80040E4C_u32 as _);
pub const DB_IMP_LEVEL_ANONYMOUS: u32 = 0u32;
pub const DB_IMP_LEVEL_DELEGATE: u32 = 3u32;
pub const DB_IMP_LEVEL_IDENTIFY: u32 = 1u32;
pub const DB_IMP_LEVEL_IMPERSONATE: u32 = 2u32;
pub const DB_IN: u32 = 1u32;
pub const DB_INVALID_HACCESSOR: u32 = 0u32;
pub const DB_INVALID_HCHAPTER: u32 = 0u32;
pub const DB_LIKE_ONLY: u32 = 2u32;
pub const DB_LOCAL_EXCLUSIVE: u32 = 3u32;
pub const DB_LOCAL_SHARED: u32 = 2u32;
pub const DB_MODE_READ: u32 = 1u32;
pub const DB_MODE_READWRITE: u32 = 3u32;
pub const DB_MODE_SHARE_DENY_NONE: u32 = 16u32;
pub const DB_MODE_SHARE_DENY_READ: u32 = 4u32;
pub const DB_MODE_SHARE_DENY_WRITE: u32 = 8u32;
pub const DB_MODE_SHARE_EXCLUSIVE: u32 = 12u32;
pub const DB_MODE_WRITE: u32 = 2u32;
pub const DB_NULLGUID: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const DB_NULL_HACCESSOR: u32 = 0u32;
pub const DB_NULL_HCHAPTER: u32 = 0u32;
pub const DB_NULL_HROW: u32 = 0u32;
pub const DB_OUT: u32 = 2u32;
pub const DB_PROT_LEVEL_CALL: u32 = 2u32;
pub const DB_PROT_LEVEL_CONNECT: u32 = 1u32;
pub const DB_PROT_LEVEL_NONE: u32 = 0u32;
pub const DB_PROT_LEVEL_PKT: u32 = 3u32;
pub const DB_PROT_LEVEL_PKT_INTEGRITY: u32 = 4u32;
pub const DB_PROT_LEVEL_PKT_PRIVACY: u32 = 5u32;
pub const DB_PT_FUNCTION: u32 = 3u32;
pub const DB_PT_PROCEDURE: u32 = 2u32;
pub const DB_PT_UNKNOWN: u32 = 1u32;
pub const DB_REMOTE: u32 = 1u32;
pub const DB_SEARCHABLE: u32 = 4u32;
pub const DB_SEC_E_AUTH_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80040E4D_u32 as _);
pub const DB_SEC_E_PERMISSIONDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80040E09_u32 as _);
pub const DB_SEC_E_SAFEMODE_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x80040E9B_u32 as _);
pub const DB_S_ASYNCHRONOUS: windows_core::HRESULT = windows_core::HRESULT(0x40ED0_u32 as _);
pub const DB_S_BADROWHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x40ED3_u32 as _);
pub const DB_S_BOOKMARKSKIPPED: windows_core::HRESULT = windows_core::HRESULT(0x40EC3_u32 as _);
pub const DB_S_BUFFERFULL: windows_core::HRESULT = windows_core::HRESULT(0x40EC8_u32 as _);
pub const DB_S_CANTRELEASE: windows_core::HRESULT = windows_core::HRESULT(0x40ECA_u32 as _);
pub const DB_S_COLUMNSCHANGED: windows_core::HRESULT = windows_core::HRESULT(0x40ED1_u32 as _);
pub const DB_S_COLUMNTYPEMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x40EC1_u32 as _);
pub const DB_S_COMMANDREEXECUTED: windows_core::HRESULT = windows_core::HRESULT(0x40EC7_u32 as _);
pub const DB_S_DELETEDROW: windows_core::HRESULT = windows_core::HRESULT(0x40ED4_u32 as _);
pub const DB_S_DIALECTIGNORED: windows_core::HRESULT = windows_core::HRESULT(0x40ECD_u32 as _);
pub const DB_S_ENDOFROWSET: windows_core::HRESULT = windows_core::HRESULT(0x40EC6_u32 as _);
pub const DB_S_ERRORSOCCURRED: windows_core::HRESULT = windows_core::HRESULT(0x40EDA_u32 as _);
pub const DB_S_ERRORSRETURNED: windows_core::HRESULT = windows_core::HRESULT(0x40ED2_u32 as _);
pub const DB_S_GOALCHANGED: windows_core::HRESULT = windows_core::HRESULT(0x40ECB_u32 as _);
pub const DB_S_LOCKUPGRADED: windows_core::HRESULT = windows_core::HRESULT(0x40ED8_u32 as _);
pub const DB_S_MULTIPLECHANGES: windows_core::HRESULT = windows_core::HRESULT(0x40EDC_u32 as _);
pub const DB_S_NONEXTROWSET: windows_core::HRESULT = windows_core::HRESULT(0x40EC5_u32 as _);
pub const DB_S_NORESULT: windows_core::HRESULT = windows_core::HRESULT(0x40EC9_u32 as _);
pub const DB_S_NOROWSPECIFICCOLUMNS: windows_core::HRESULT = windows_core::HRESULT(0x40EDD_u32 as _);
pub const DB_S_NOTSINGLETON: windows_core::HRESULT = windows_core::HRESULT(0x40ED7_u32 as _);
pub const DB_S_PARAMUNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x40EDB_u32 as _);
pub const DB_S_PROPERTIESCHANGED: windows_core::HRESULT = windows_core::HRESULT(0x40ED9_u32 as _);
pub const DB_S_ROWLIMITEXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x40EC0_u32 as _);
pub const DB_S_STOPLIMITREACHED: windows_core::HRESULT = windows_core::HRESULT(0x40ED6_u32 as _);
pub const DB_S_TOOMANYCHANGES: windows_core::HRESULT = windows_core::HRESULT(0x40ED5_u32 as _);
pub const DB_S_TYPEINFOOVERRIDDEN: windows_core::HRESULT = windows_core::HRESULT(0x40EC2_u32 as _);
pub const DB_S_UNWANTEDOPERATION: windows_core::HRESULT = windows_core::HRESULT(0x40ECC_u32 as _);
pub const DB_S_UNWANTEDPHASE: windows_core::HRESULT = windows_core::HRESULT(0x40ECE_u32 as _);
pub const DB_S_UNWANTEDREASON: windows_core::HRESULT = windows_core::HRESULT(0x40ECF_u32 as _);
pub const DB_UNSEARCHABLE: u32 = 1u32;
pub const DCINFOTYPE_VERSION: DCINFOTYPEENUM = DCINFOTYPEENUM(1i32);
pub const DELIVERY_AGENT_FLAG_NO_BROADCAST: DELIVERY_AGENT_FLAGS = DELIVERY_AGENT_FLAGS(4i32);
pub const DELIVERY_AGENT_FLAG_NO_RESTRICTIONS: DELIVERY_AGENT_FLAGS = DELIVERY_AGENT_FLAGS(8i32);
pub const DELIVERY_AGENT_FLAG_SILENT_DIAL: DELIVERY_AGENT_FLAGS = DELIVERY_AGENT_FLAGS(16i32);
pub const DISPID_QUERY_ALL: u32 = 6u32;
pub const DISPID_QUERY_HITCOUNT: u32 = 4u32;
pub const DISPID_QUERY_LASTSEENTIME: u32 = 10u32;
pub const DISPID_QUERY_METADATA_PROPDISPID: u32 = 6u32;
pub const DISPID_QUERY_METADATA_PROPGUID: u32 = 5u32;
pub const DISPID_QUERY_METADATA_PROPMODIFIABLE: u32 = 9u32;
pub const DISPID_QUERY_METADATA_PROPNAME: u32 = 7u32;
pub const DISPID_QUERY_METADATA_STORELEVEL: u32 = 8u32;
pub const DISPID_QUERY_METADATA_VROOTAUTOMATIC: u32 = 3u32;
pub const DISPID_QUERY_METADATA_VROOTMANUAL: u32 = 4u32;
pub const DISPID_QUERY_METADATA_VROOTUSED: u32 = 2u32;
pub const DISPID_QUERY_RANK: u32 = 3u32;
pub const DISPID_QUERY_RANKVECTOR: u32 = 2u32;
pub const DISPID_QUERY_REVNAME: u32 = 8u32;
pub const DISPID_QUERY_UNFILTERED: u32 = 7u32;
pub const DISPID_QUERY_VIRTUALPATH: u32 = 9u32;
pub const DISPID_QUERY_WORKID: u32 = 5u32;
pub const DS_E_ALREADYDISABLED: i32 = -2147220447i32;
pub const DS_E_ALREADYENABLED: i32 = -2147220454i32;
pub const DS_E_BADREQUEST: i32 = -2147220475i32;
pub const DS_E_BADRESULT: i32 = -2147220445i32;
pub const DS_E_BADSEQUENCE: i32 = -2147220473i32;
pub const DS_E_BUFFERTOOSMALL: i32 = -2147220449i32;
pub const DS_E_CANNOTREMOVECONCURRENT: i32 = -2147220443i32;
pub const DS_E_CANNOTWRITEREGISTRY: i32 = -2147220444i32;
pub const DS_E_CONFIGBAD: i32 = -2147220470i32;
pub const DS_E_CONFIGNOTRIGHTTYPE: i32 = -2147220456i32;
pub const DS_E_DATANOTPRESENT: i32 = -2147220464i32;
pub const DS_E_DATASOURCENOTAVAILABLE: i32 = -2147220478i32;
pub const DS_E_DATASOURCENOTDISABLED: i32 = -2147220459i32;
pub const DS_E_DUPLICATEID: i32 = -2147220462i32;
pub const DS_E_INDEXDIRECTORY: i32 = -2147220452i32;
pub const DS_E_INVALIDCATALOGNAME: i32 = -2147220457i32;
pub const DS_E_INVALIDDATASOURCE: i32 = -2147220479i32;
pub const DS_E_INVALIDTAGDB: i32 = -2147220458i32;
pub const DS_E_MESSAGETOOLONG: i32 = -2147220472i32;
pub const DS_E_MISSINGCATALOG: i32 = -2147220440i32;
pub const DS_E_NOMOREDATA: i32 = -2147220480i32;
pub const DS_E_PARAMOUTOFRANGE: i32 = -2147220448i32;
pub const DS_E_PROPVERSIONMISMATCH: i32 = -2147220441i32;
pub const DS_E_PROTOCOLVERSION: i32 = -2147220455i32;
pub const DS_E_QUERYCANCELED: i32 = -2147220477i32;
pub const DS_E_QUERYHUNG: i32 = -2147220446i32;
pub const DS_E_REGISTRY: i32 = -2147220460i32;
pub const DS_E_SEARCHCATNAMECOLLISION: i32 = -2147220442i32;
pub const DS_E_SERVERCAPACITY: i32 = -2147220474i32;
pub const DS_E_SERVERERROR: i32 = -2147220471i32;
pub const DS_E_SETSTATUSINPROGRESS: i32 = -2147220463i32;
pub const DS_E_TOOMANYDATASOURCES: i32 = -2147220461i32;
pub const DS_E_UNKNOWNPARAM: i32 = -2147220450i32;
pub const DS_E_UNKNOWNREQUEST: i32 = -2147220476i32;
pub const DS_E_VALUETOOLARGE: i32 = -2147220451i32;
pub const ERROR_FTE: u32 = 13824u32;
pub const ERROR_FTE_CB: u32 = 51968u32;
pub const ERROR_FTE_FD: u32 = 64768u32;
pub const ERROR_SOURCE_CMDLINE: u32 = 5376u32;
pub const ERROR_SOURCE_COLLATOR: u32 = 1280u32;
pub const ERROR_SOURCE_CONNMGR: u32 = 1536u32;
pub const ERROR_SOURCE_CONTENT_SOURCE: u32 = 13312u32;
pub const ERROR_SOURCE_DATASOURCE: u32 = 1024u32;
pub const ERROR_SOURCE_DAV: u32 = 8960u32;
pub const ERROR_SOURCE_EXSTOREPH: u32 = 9984u32;
pub const ERROR_SOURCE_FLTRDMN: u32 = 9216u32;
pub const ERROR_SOURCE_GATHERER: u32 = 3328u32;
pub const ERROR_SOURCE_INDEXER: u32 = 4352u32;
pub const ERROR_SOURCE_MSS: u32 = 8448u32;
pub const ERROR_SOURCE_NETWORKING: u32 = 768u32;
pub const ERROR_SOURCE_NLADMIN: u32 = 6400u32;
pub const ERROR_SOURCE_NOTESPH: u32 = 9728u32;
pub const ERROR_SOURCE_OLEDB_BINDER: u32 = 9472u32;
pub const ERROR_SOURCE_PEOPLE_IMPORT: u32 = 16384u32;
pub const ERROR_SOURCE_PROTHNDLR: u32 = 4608u32;
pub const ERROR_SOURCE_QUERY: u32 = 1792u32;
pub const ERROR_SOURCE_REMOTE_EXSTOREPH: u32 = 13568u32;
pub const ERROR_SOURCE_SCHEMA: u32 = 3072u32;
pub const ERROR_SOURCE_SCRIPTPI: u32 = 8192u32;
pub const ERROR_SOURCE_SECURITY: u32 = 5120u32;
pub const ERROR_SOURCE_SETUP: u32 = 4864u32;
pub const ERROR_SOURCE_SRCH_SCHEMA_CACHE: u32 = 13056u32;
pub const ERROR_SOURCE_XML: u32 = 8704u32;
pub const EVENT_AUDIENCECOMPUTATION_CANNOTSTART: i32 = -1073738223i32;
pub const EVENT_AUTOCAT_CANT_CREATE_FILE_SHARE: i32 = -1073738726i32;
pub const EVENT_AUTOCAT_PERFMON: i32 = -1073738753i32;
pub const EVENT_CONFIG_ERROR: i32 = -1073738821i32;
pub const EVENT_CONFIG_SYNTAX: i32 = -2147482604i32;
pub const EVENT_CRAWL_SCHEDULED: i32 = 1073744884i32;
pub const EVENT_DETAILED_FILTERPOOL_ADD_FAILED: i32 = -1073738719i32;
pub const EVENT_DSS_NOT_ENABLED: i32 = -2147476572i32;
pub const EVENT_ENUMERATE_SESSIONS_FAILED: i32 = -1073738720i32;
pub const EVENT_EXCEPTION: i32 = -1073740815i32;
pub const EVENT_FAILED_CREATE_GATHERER_LOG: i32 = -2147480587i32;
pub const EVENT_FAILED_INITIALIZE_CRAWL: i32 = -1073738765i32;
pub const EVENT_FILTERPOOL_ADD_FAILED: i32 = -1073738722i32;
pub const EVENT_FILTERPOOL_DELETE_FAILED: i32 = -1073738721i32;
pub const EVENT_FILTER_HOST_FORCE_TERMINATE: i32 = -2147473624i32;
pub const EVENT_FILTER_HOST_NOT_INITIALIZED: i32 = -1073738724i32;
pub const EVENT_FILTER_HOST_NOT_TERMINATED: i32 = -1073738723i32;
pub const EVENT_GATHERER_DATASOURCE: i32 = -1073738727i32;
pub const EVENT_GATHERER_PERFMON: i32 = -1073738817i32;
pub const EVENT_GATHERSVC_PERFMON: i32 = -1073738818i32;
pub const EVENT_GATHER_ADVISE_FAILED: i32 = -1073738798i32;
pub const EVENT_GATHER_APP_INIT_FAILED: i32 = -1073738766i32;
pub const EVENT_GATHER_AUTODESCENCODE_INVALID: i32 = -2147480592i32;
pub const EVENT_GATHER_AUTODESCLEN_ADJUSTED: i32 = -2147480603i32;
pub const EVENT_GATHER_BACKUPAPP_COMPLETE: i32 = 3077i32;
pub const EVENT_GATHER_BACKUPAPP_ERROR: i32 = -1073738748i32;
pub const EVENT_GATHER_CANT_CREATE_DOCID: i32 = -1073738793i32;
pub const EVENT_GATHER_CANT_DELETE_DOCID: i32 = -1073738792i32;
pub const EVENT_GATHER_CHECKPOINT_CORRUPT: i32 = -1073738732i32;
pub const EVENT_GATHER_CHECKPOINT_FAILED: i32 = -1073738736i32;
pub const EVENT_GATHER_CHECKPOINT_FILE_MISSING: i32 = -1073738731i32;
pub const EVENT_GATHER_CRAWL_IN_PROGRESS: i32 = -2147480609i32;
pub const EVENT_GATHER_CRAWL_NOT_STARTED: i32 = -2147480625i32;
pub const EVENT_GATHER_CRAWL_SEED_ERROR: i32 = -2147480624i32;
pub const EVENT_GATHER_CRAWL_SEED_FAILED: i32 = -2147480612i32;
pub const EVENT_GATHER_CRAWL_SEED_FAILED_INIT: i32 = -2147480611i32;
pub const EVENT_GATHER_CRITICAL_ERROR: i32 = -1073738799i32;
pub const EVENT_GATHER_DAEMON_TERMINATED: i32 = -2147480570i32;
pub const EVENT_GATHER_DELETING_HISTORY_ITEMS: i32 = -1073738774i32;
pub const EVENT_GATHER_DIRTY_STARTUP: i32 = -2147480576i32;
pub const EVENT_GATHER_DISK_FULL: i32 = -2147480594i32;
pub const EVENT_GATHER_END_ADAPTIVE: i32 = 1073744891i32;
pub const EVENT_GATHER_END_CRAWL: i32 = 1073744842i32;
pub const EVENT_GATHER_END_INCREMENTAL: i32 = 1073744871i32;
pub const EVENT_GATHER_EXCEPTION: i32 = -1073738810i32;
pub const EVENT_GATHER_FLUSH_FAILED: i32 = -1073738737i32;
pub const EVENT_GATHER_FROM_NOT_SET: i32 = -1073738776i32;
pub const EVENT_GATHER_HISTORY_CORRUPTION_DETECTED: i32 = -2147480575i32;
pub const EVENT_GATHER_INPLACE_INDEX_REBUILD: i32 = 1073745427i32;
pub const EVENT_GATHER_INTERNAL: i32 = -1073738804i32;
pub const EVENT_GATHER_INVALID_NETWORK_ACCESS_ACCOUNT: i32 = -1073738739i32;
pub const EVENT_GATHER_LOCK_FAILED: i32 = -1073738784i32;
pub const EVENT_GATHER_NO_CRAWL_SEEDS: i32 = -2147480602i32;
pub const EVENT_GATHER_NO_SCHEMA: i32 = -2147480593i32;
pub const EVENT_GATHER_OBJ_INIT_FAILED: i32 = -1073738796i32;
pub const EVENT_GATHER_PLUGINMGR_INIT_FAILED: i32 = -1073738767i32;
pub const EVENT_GATHER_PLUGIN_INIT_FAILED: i32 = -1073738795i32;
pub const EVENT_GATHER_PROTOCOLHANDLER_INIT_FAILED: i32 = -1073738740i32;
pub const EVENT_GATHER_PROTOCOLHANDLER_LOAD_FAILED: i32 = -1073738741i32;
pub const EVENT_GATHER_READ_CHECKPOINT_FAILED: i32 = -1073738733i32;
pub const EVENT_GATHER_RECOVERY_FAILURE: i32 = -1073738222i32;
pub const EVENT_GATHER_REG_MISSING: i32 = -2147480610i32;
pub const EVENT_GATHER_RESET_START: i32 = 1073744865i32;
pub const EVENT_GATHER_RESTOREAPP_COMPLETE: i32 = 3075i32;
pub const EVENT_GATHER_RESTOREAPP_ERROR: i32 = -1073738750i32;
pub const EVENT_GATHER_RESTORE_CHECKPOINT_FAILED: i32 = -1073738734i32;
pub const EVENT_GATHER_RESTORE_COMPLETE: i32 = 3069i32;
pub const EVENT_GATHER_RESTORE_ERROR: i32 = -1073738754i32;
pub const EVENT_GATHER_RESUME: i32 = 1073744868i32;
pub const EVENT_GATHER_SAVE_FAILED: i32 = -1073738735i32;
pub const EVENT_GATHER_SERVICE_INIT: i32 = -1073738794i32;
pub const EVENT_GATHER_START_CRAWL: i32 = 1073744843i32;
pub const EVENT_GATHER_START_CRAWL_IF_RESET: i32 = -2147480595i32;
pub const EVENT_GATHER_START_PAUSE: i32 = -2147480606i32;
pub const EVENT_GATHER_STOP_START: i32 = 1073744876i32;
pub const EVENT_GATHER_SYSTEM_LCID_CHANGED: i32 = -2147480562i32;
pub const EVENT_GATHER_THROTTLE: i32 = 1073744867i32;
pub const EVENT_GATHER_TRANSACTION_FAIL: i32 = -1073738797i32;
pub const EVENT_HASHMAP_INSERT: i32 = -1073738816i32;
pub const EVENT_HASHMAP_UPDATE: i32 = -1073738811i32;
pub const EVENT_INDEXER_ADD_DSS_DISCONNECT: i32 = -2147476585i32;
pub const EVENT_INDEXER_ADD_DSS_FAILED: i32 = -2147476627i32;
pub const EVENT_INDEXER_ADD_DSS_SUCCEEDED: i32 = 7019i32;
pub const EVENT_INDEXER_BUILD_ENDED: i32 = 1073748873i32;
pub const EVENT_INDEXER_BUILD_FAILED: i32 = -1073734797i32;
pub const EVENT_INDEXER_BUILD_START: i32 = 1073748872i32;
pub const EVENT_INDEXER_CI_LOAD_ERROR: i32 = -1073734785i32;
pub const EVENT_INDEXER_DSS_ALREADY_ADDED: i32 = 1073748870i32;
pub const EVENT_INDEXER_DSS_CONTACT_FAILED: i32 = -1073734800i32;
pub const EVENT_INDEXER_DSS_UNABLE_TO_REMOVE: i32 = -1073734755i32;
pub const EVENT_INDEXER_FAIL_TO_CREATE_PER_USER_CATALOG: i32 = -1073731797i32;
pub const EVENT_INDEXER_FAIL_TO_SET_MAX_JETINSTANCE: i32 = -1073731798i32;
pub const EVENT_INDEXER_FAIL_TO_UNLOAD_PER_USER_CATALOG: i32 = -1073731796i32;
pub const EVENT_INDEXER_INIT_ERROR: i32 = -1073734814i32;
pub const EVENT_INDEXER_INVALID_DIRECTORY: i32 = -1073734813i32;
pub const EVENT_INDEXER_LOAD_FAIL: i32 = -1073734781i32;
pub const EVENT_INDEXER_MISSING_APP_DIRECTORY: i32 = -1073734758i32;
pub const EVENT_INDEXER_NEW_PROJECT: i32 = -1073734754i32;
pub const EVENT_INDEXER_NO_SEARCH_SERVERS: i32 = -2147476630i32;
pub const EVENT_INDEXER_OUT_OF_DATABASE_INSTANCE: i32 = -1073731799i32;
pub const EVENT_INDEXER_PAUSED_FOR_DISKFULL: i32 = -1073734811i32;
pub const EVENT_INDEXER_PERFMON: i32 = -1073734760i32;
pub const EVENT_INDEXER_PROPSTORE_INIT_FAILED: i32 = -1073734787i32;
pub const EVENT_INDEXER_PROP_ABORTED: i32 = 1073748899i32;
pub const EVENT_INDEXER_PROP_COMMITTED: i32 = 1073748898i32;
pub const EVENT_INDEXER_PROP_COMMIT_FAILED: i32 = -1073734747i32;
pub const EVENT_INDEXER_PROP_ERROR: i32 = -1073734812i32;
pub const EVENT_INDEXER_PROP_STARTED: i32 = 1073748841i32;
pub const EVENT_INDEXER_PROP_STATE_CORRUPT: i32 = -1073734780i32;
pub const EVENT_INDEXER_PROP_STOPPED: i32 = -2147476633i32;
pub const EVENT_INDEXER_PROP_SUCCEEDED: i32 = 7016i32;
pub const EVENT_INDEXER_REG_ERROR: i32 = -1073734756i32;
pub const EVENT_INDEXER_REG_MISSING: i32 = -1073734796i32;
pub const EVENT_INDEXER_REMOVED_PROJECT: i32 = -1073734753i32;
pub const EVENT_INDEXER_REMOVE_DSS_FAILED: i32 = -1073734801i32;
pub const EVENT_INDEXER_REMOVE_DSS_SUCCEEDED: i32 = 7020i32;
pub const EVENT_INDEXER_RESET_FOR_CORRUPTION: i32 = -1073734784i32;
pub const EVENT_INDEXER_SCHEMA_COPY_ERROR: i32 = -1073734823i32;
pub const EVENT_INDEXER_SHUTDOWN: i32 = 1073748866i32;
pub const EVENT_INDEXER_STARTED: i32 = 1073748824i32;
pub const EVENT_INDEXER_VERIFY_PROP_ACCOUNT: i32 = -1073734768i32;
pub const EVENT_LEARN_COMPILE_FAILED: i32 = -2147480583i32;
pub const EVENT_LEARN_CREATE_DB_FAILED: i32 = -2147480584i32;
pub const EVENT_LEARN_PROPAGATION_COPY_FAILED: i32 = -2147480585i32;
pub const EVENT_LEARN_PROPAGATION_FAILED: i32 = -2147480582i32;
pub const EVENT_LOCAL_GROUPS_CACHE_FLUSHED: i32 = 1073744920i32;
pub const EVENT_LOCAL_GROUP_NOT_EXPANDED: i32 = 1073744919i32;
pub const EVENT_NOTIFICATION_FAILURE: i32 = -1073738745i32;
pub const EVENT_NOTIFICATION_FAILURE_SCOPE_EXCEEDED_LOGGING: i32 = -2147480568i32;
pub const EVENT_NOTIFICATION_RESTORED: i32 = 1073744905i32;
pub const EVENT_NOTIFICATION_RESTORED_SCOPE_EXCEEDED_LOGGING: i32 = -2147480566i32;
pub const EVENT_NOTIFICATION_THREAD_EXIT_FAILED: i32 = -1073738725i32;
pub const EVENT_OUTOFMEMORY: i32 = -1073740817i32;
pub const EVENT_PERF_COUNTERS_ALREADY_EXISTS: i32 = -2147473626i32;
pub const EVENT_PERF_COUNTERS_NOT_LOADED: i32 = -2147473628i32;
pub const EVENT_PERF_COUNTERS_REGISTRY_TROUBLE: i32 = -2147473627i32;
pub const EVENT_PROTOCOL_HOST_FORCE_TERMINATE: i32 = -2147473625i32;
pub const EVENT_REG_VERSION: i32 = -1073738790i32;
pub const EVENT_SSSEARCH_CREATE_PATH_RULES_FAILED: i32 = -2147482634i32;
pub const EVENT_SSSEARCH_CSM_SAVE_FAILED: i32 = -1073740805i32;
pub const EVENT_SSSEARCH_DATAFILES_MOVE_FAILED: i32 = -1073740808i32;
pub const EVENT_SSSEARCH_DATAFILES_MOVE_ROLLBACK_ERRORS: i32 = -2147482630i32;
pub const EVENT_SSSEARCH_DATAFILES_MOVE_SUCCEEDED: i32 = 1073742841i32;
pub const EVENT_SSSEARCH_DROPPED_EVENTS: i32 = -2147482633i32;
pub const EVENT_SSSEARCH_SETUP_CLEANUP_FAILED: i32 = -1073740813i32;
pub const EVENT_SSSEARCH_SETUP_CLEANUP_STARTED: i32 = -2147482640i32;
pub const EVENT_SSSEARCH_SETUP_CLEANUP_SUCCEEDED: i32 = 1073742834i32;
pub const EVENT_SSSEARCH_SETUP_FAILED: i32 = -1073740818i32;
pub const EVENT_SSSEARCH_SETUP_SUCCEEDED: i32 = 1073742829i32;
pub const EVENT_SSSEARCH_STARTED: i32 = 1073742827i32;
pub const EVENT_SSSEARCH_STARTING_SETUP: i32 = 1073742828i32;
pub const EVENT_SSSEARCH_STOPPED: i32 = 1073742837i32;
pub const EVENT_STS_INIT_SECURITY_FAILED: i32 = -2147480554i32;
pub const EVENT_SYSTEM_EXCEPTION: i32 = -2147482595i32;
pub const EVENT_TRANSACTION_READ: i32 = -1073738809i32;
pub const EVENT_TRANSLOG_APPEND: i32 = -1073738814i32;
pub const EVENT_TRANSLOG_CREATE: i32 = -1073738791i32;
pub const EVENT_TRANSLOG_CREATE_TRX: i32 = -1073738815i32;
pub const EVENT_TRANSLOG_UPDATE: i32 = -1073738813i32;
pub const EVENT_UNPRIVILEGED_SERVICE_ACCOUNT: i32 = -2147482596i32;
pub const EVENT_USING_DIFFERENT_WORD_BREAKER: i32 = -2147480580i32;
pub const EVENT_WARNING_CANNOT_UPGRADE_NOISE_FILE: i32 = -2147473634i32;
pub const EVENT_WARNING_CANNOT_UPGRADE_NOISE_FILES: i32 = -2147473635i32;
pub const EVENT_WBREAKER_NOT_LOADED: i32 = -2147480586i32;
pub const EVENT_WIN32_ERROR: i32 = -2147473633i32;
pub const EXCI_E_ACCESS_DENIED: i32 = -2147216990i32;
pub const EXCI_E_BADCONFIG_OR_ACCESSDENIED: i32 = -2147216988i32;
pub const EXCI_E_INVALID_ACCOUNT_INFO: i32 = -2147216984i32;
pub const EXCI_E_INVALID_EXCHANGE_SERVER: i32 = -2147216989i32;
pub const EXCI_E_INVALID_SERVER_CONFIG: i32 = -2147216991i32;
pub const EXCI_E_NOT_ADMIN_OR_WRONG_SITE: i32 = -2147216986i32;
pub const EXCI_E_NO_CONFIG: i32 = -2147216992i32;
pub const EXCI_E_NO_MAPI: i32 = -2147216985i32;
pub const EXCI_E_WRONG_SERVER_OR_ACCT: i32 = -2147216987i32;
pub const EXSTOREPH_E_UNEXPECTED: i32 = -2147211519i32;
pub const EX_ANY: u32 = 0u32;
pub const EX_CMDFATAL: u32 = 20u32;
pub const EX_CONTROL: u32 = 25u32;
pub const EX_DBCORRUPT: u32 = 23u32;
pub const EX_DBFATAL: u32 = 21u32;
pub const EX_DEADLOCK: u32 = 13u32;
pub const EX_HARDWARE: u32 = 24u32;
pub const EX_INFO: u32 = 10u32;
pub const EX_INTOK: u32 = 18u32;
pub const EX_LIMIT: u32 = 19u32;
pub const EX_MAXISEVERITY: u32 = 10u32;
pub const EX_MISSING: u32 = 11u32;
pub const EX_PERMIT: u32 = 14u32;
pub const EX_RESOURCE: u32 = 17u32;
pub const EX_SYNTAX: u32 = 15u32;
pub const EX_TABCORRUPT: u32 = 22u32;
pub const EX_TYPE: u32 = 12u32;
pub const EX_USER: u32 = 16u32;
pub const FAIL: u32 = 0u32;
pub const FF_INDEXCOMPLEXURLS: FOLLOW_FLAGS = FOLLOW_FLAGS(1i32);
pub const FF_SUPPRESSINDEXING: FOLLOW_FLAGS = FOLLOW_FLAGS(2i32);
pub const FLTRDMN_E_CANNOT_DECRYPT_PASSWORD: i32 = -2147212282i32;
pub const FLTRDMN_E_ENCRYPTED_DOCUMENT: i32 = -2147212283i32;
pub const FLTRDMN_E_FILTER_INIT_FAILED: i32 = -2147212284i32;
pub const FLTRDMN_E_QI_FILTER_FAILED: i32 = -2147212286i32;
pub const FLTRDMN_E_UNEXPECTED: i32 = -2147212287i32;
pub const FTE_E_ADMIN_BLOB_CORRUPT: i32 = -2147207676i32;
pub const FTE_E_AFFINITY_MASK: i32 = -2147207651i32;
pub const FTE_E_ALREADY_INITIALIZED: i32 = -2147207604i32;
pub const FTE_E_ANOTHER_STATUS_CHANGE_IS_ALREADY_ACTIVE: i32 = -2147207635i32;
pub const FTE_E_BATCH_ABORTED: i32 = -2147207636i32;
pub const FTE_E_CATALOG_ALREADY_EXISTS: i32 = -2147207656i32;
pub const FTE_E_CATALOG_DOES_NOT_EXIST: i32 = -2147207639i32;
pub const FTE_E_CB_CBID_OUT_OF_BOUND: i32 = -2147169535i32;
pub const FTE_E_CB_NOT_ENOUGH_AVAIL_PHY_MEM: i32 = -2147169534i32;
pub const FTE_E_CB_NOT_ENOUGH_OCC_BUFFER: i32 = -2147169533i32;
pub const FTE_E_CB_OUT_OF_MEMORY: i32 = -2147169536i32;
pub const FTE_E_COM_SIGNATURE_VALIDATION: i32 = -2147207652i32;
pub const FTE_E_CORRUPT_GATHERER_HASH_MAP: i32 = -2147207619i32;
pub const FTE_E_CORRUPT_PROPERTY_STORE: i32 = -2147207622i32;
pub const FTE_E_CORRUPT_WORDLIST: i32 = -2147169532i32;
pub const FTE_E_DATATYPE_MISALIGNMENT: i32 = -2147207605i32;
pub const FTE_E_DEPENDENT_TRAN_FAILED_TO_PERSIST: i32 = -2147207641i32;
pub const FTE_E_DOC_TOO_HUGE: i32 = -2147207606i32;
pub const FTE_E_DUPLICATE_OBJECT: i32 = -2147207644i32;
pub const FTE_E_ERROR_WRITING_REGISTRY: i32 = -2147207674i32;
pub const FTE_E_EXCEEDED_MAX_PLUGINS: i32 = -2147207647i32;
pub const FTE_E_FAILED_TO_CREATE_ACCESSOR: i32 = -2147207625i32;
pub const FTE_E_FAILURE_TO_POST_SETCOMPLETION_STATUS: i32 = -2147207597i32;
pub const FTE_E_FD_DID_NOT_CONNECT: i32 = -2147207660i32;
pub const FTE_E_FD_DOC_TIMEOUT: i32 = -2147156733i32;
pub const FTE_E_FD_DOC_UNEXPECTED_EXIT: i32 = -2147156731i32;
pub const FTE_E_FD_FAILED_TO_LOAD_IFILTER: i32 = -2147156734i32;
pub const FTE_E_FD_FILTER_CAUSED_SHARING_VIOLATION: i32 = -2147156725i32;
pub const FTE_E_FD_IDLE: i32 = -2147207595i32;
pub const FTE_E_FD_IFILTER_INIT_FAILED: i32 = -2147156735i32;
pub const FTE_E_FD_NOISE_NO_IPERSISTSTREAM_ON_TEXT_FILTER: i32 = -2147156729i32;
pub const FTE_E_FD_NOISE_NO_TEXT_FILTER: i32 = -2147156730i32;
pub const FTE_E_FD_NOISE_TEXT_FILTER_INIT_FAILED: i32 = -2147156727i32;
pub const FTE_E_FD_NOISE_TEXT_FILTER_LOAD_FAILED: i32 = -2147156728i32;
pub const FTE_E_FD_NO_IPERSIST_INTERFACE: i32 = -2147156736i32;
pub const FTE_E_FD_OCCURRENCE_OVERFLOW: i32 = -2147156726i32;
pub const FTE_E_FD_OWNERSHIP_OBSOLETE: i32 = -2147207650i32;
pub const FTE_E_FD_SHUTDOWN: i32 = -2147207640i32;
pub const FTE_E_FD_TIMEOUT: i32 = -2147207632i32;
pub const FTE_E_FD_UNEXPECTED_EXIT: i32 = -2147156732i32;
pub const FTE_E_FD_UNRESPONSIVE: i32 = -2147207594i32;
pub const FTE_E_FD_USED_TOO_MUCH_MEMORY: i32 = -2147207603i32;
pub const FTE_E_FILTER_SINGLE_THREADED: i32 = -2147207675i32;
pub const FTE_E_HIGH_MEMORY_PRESSURE: i32 = -2147207601i32;
pub const FTE_E_INVALID_CODEPAGE: i32 = -2147207596i32;
pub const FTE_E_INVALID_DOCID: i32 = -2147207663i32;
pub const FTE_E_INVALID_ISOLATE_ERROR_BATCH: i32 = -2147207600i32;
pub const FTE_E_INVALID_PROG_ID: i32 = -2147207614i32;
pub const FTE_E_INVALID_PROJECT_ID: i32 = -2147207598i32;
pub const FTE_E_INVALID_PROPERTY: i32 = -2147207630i32;
pub const FTE_E_INVALID_TYPE: i32 = -2147207624i32;
pub const FTE_E_KEY_NOT_CACHED: i32 = -2147207618i32;
pub const FTE_E_LIBRARY_NOT_LOADED: i32 = -2147207627i32;
pub const FTE_E_NOT_PROCESSED_DUE_TO_PREVIOUS_ERRORS: i32 = -2147207633i32;
pub const FTE_E_NO_MORE_PROPERTIES: i32 = -2147207629i32;
pub const FTE_E_NO_PLUGINS: i32 = -2147207638i32;
pub const FTE_E_NO_PROPERTY_STORE: i32 = -1073465766i32;
pub const FTE_E_OUT_OF_RANGE: i32 = -2147207623i32;
pub const FTE_E_PATH_TOO_LONG: i32 = -2147207654i32;
pub const FTE_E_PAUSE_EXTERNAL: i32 = -2147207662i32;
pub const FTE_E_PERFMON_FULL: i32 = -2147207626i32;
pub const FTE_E_PERF_NOT_LOADED: i32 = -2147207611i32;
pub const FTE_E_PIPE_DATA_CORRUPTED: i32 = -2147207671i32;
pub const FTE_E_PIPE_NOT_CONNECTED: i32 = -2147207677i32;
pub const FTE_E_PROGID_REQUIRED: i32 = -2147207658i32;
pub const FTE_E_PROJECT_NOT_INITALIZED: i32 = -2147207672i32;
pub const FTE_E_PROJECT_SHUTDOWN: i32 = -2147207673i32;
pub const FTE_E_PROPERTY_STORE_WORKID_NOTVALID: i32 = -2147207621i32;
pub const FTE_E_READONLY_CATALOG: i32 = -2147207612i32;
pub const FTE_E_REDUNDANT_TRAN_FAILURE: i32 = -2147207642i32;
pub const FTE_E_REJECTED_DUE_TO_PROJECT_STATUS: i32 = -2147207661i32;
pub const FTE_E_RESOURCE_SHUTDOWN: i32 = -2147207631i32;
pub const FTE_E_RETRY_HUGE_DOC: i32 = -2147207608i32;
pub const FTE_E_RETRY_SINGLE_DOC_PER_BATCH: i32 = -2147207599i32;
pub const FTE_E_SECRET_NOT_FOUND: i32 = -2147207678i32;
pub const FTE_E_SERIAL_STREAM_CORRUPT: i32 = -2147207613i32;
pub const FTE_E_STACK_CORRUPTED: i32 = -2147207615i32;
pub const FTE_E_STATIC_THREAD_INVALID_ARGUMENTS: i32 = -2147207657i32;
pub const FTE_E_UNEXPECTED_EXIT: i32 = -2147207602i32;
pub const FTE_E_UNKNOWN_FD_TYPE: i32 = -2147207607i32;
pub const FTE_E_UNKNOWN_PLUGIN: i32 = -2147207628i32;
pub const FTE_E_UPGRADE_INTERFACE_ALREADY_INSTANTIATED: i32 = -2147207616i32;
pub const FTE_E_UPGRADE_INTERFACE_ALREADY_SHUTDOWN: i32 = -2147207617i32;
pub const FTE_E_URB_TOO_BIG: i32 = -2147207664i32;
pub const FTE_INVALID_ADMIN_CLIENT: i32 = -2147207653i32;
pub const FTE_S_BEYOND_QUOTA: i32 = 276002i32;
pub const FTE_S_CATALOG_BLOB_MISMATCHED: i32 = 276056i32;
pub const FTE_S_PROPERTY_RESET: i32 = 276057i32;
pub const FTE_S_PROPERTY_STORE_END_OF_ENUMERATION: i32 = 276028i32;
pub const FTE_S_READONLY_CATALOG: i32 = 276038i32;
pub const FTE_S_REDUNDANT: i32 = 276005i32;
pub const FTE_S_RESOURCES_STARTING_TO_GET_LOW: i32 = 275993i32;
pub const FTE_S_RESUME: i32 = 276014i32;
pub const FTE_S_STATUS_CHANGE_REQUEST: i32 = 276011i32;
pub const FTE_S_TRY_TO_FLUSH: i32 = 276055i32;
pub const GENERATE_METHOD_PREFIXMATCH: u32 = 1u32;
pub const GENERATE_METHOD_STEMMED: u32 = 2u32;
pub const GHTR_E_INSUFFICIENT_DISK_SPACE: i32 = -2147218037i32;
pub const GHTR_E_LOCAL_SERVER_UNAVAILABLE: i32 = -2147218055i32;
pub const GTHR_E_ADDLINKS_FAILED_WILL_RETRY_PARENT: i32 = -2147217989i32;
pub const GTHR_E_APPLICATION_NOT_FOUND: i32 = -2147218079i32;
pub const GTHR_E_AUTOCAT_UNEXPECTED: i32 = -2147218012i32;
pub const GTHR_E_BACKUP_VALIDATION_FAIL: i32 = -2147217994i32;
pub const GTHR_E_BAD_FILTER_DAEMON: i32 = -2147218119i32;
pub const GTHR_E_BAD_FILTER_HOST: i32 = -2147217993i32;
pub const GTHR_E_CANNOT_ENABLE_CHECKPOINT: i32 = -2147218002i32;
pub const GTHR_E_CANNOT_REMOVE_PLUGINMGR: i32 = -2147218078i32;
pub const GTHR_E_CONFIG_DUP_EXTENSION: i32 = -2147218165i32;
pub const GTHR_E_CONFIG_DUP_PROJECT: i32 = -2147218166i32;
pub const GTHR_E_CONTENT_ID_CONFLICT: i32 = -2147218062i32;
pub const GTHR_E_DIRMON_NOT_INITIALZED: i32 = -2147218019i32;
pub const GTHR_E_DUPLICATE_OBJECT: i32 = -2147218174i32;
pub const GTHR_E_DUPLICATE_PROJECT: i32 = -2147218094i32;
pub const GTHR_E_DUPLICATE_URL: i32 = -2147218163i32;
pub const GTHR_E_DUP_PROPERTY_MAPPING: i32 = -2147218134i32;
pub const GTHR_E_EMPTY_DACL: i32 = -2147218006i32;
pub const GTHR_E_ERROR_INITIALIZING_PERFMON: i32 = -2147218171i32;
pub const GTHR_E_ERROR_OBJECT_NOT_FOUND: i32 = -2147218170i32;
pub const GTHR_E_ERROR_WRITING_REGISTRY: i32 = -2147218172i32;
pub const GTHR_E_FILTERPOOL_NOTFOUND: i32 = -2147217990i32;
pub const GTHR_E_FILTER_FAULT: i32 = -2147218075i32;
pub const GTHR_E_FILTER_INIT: i32 = -2147218130i32;
pub const GTHR_E_FILTER_INTERRUPTED: i32 = -2147218092i32;
pub const GTHR_E_FILTER_INVALID_MESSAGE: i32 = -2147218158i32;
pub const GTHR_E_FILTER_NOT_FOUND: i32 = -2147218154i32;
pub const GTHR_E_FILTER_NO_CODEPAGE: i32 = -2147218123i32;
pub const GTHR_E_FILTER_NO_MORE_THREADS: i32 = -2147218153i32;
pub const GTHR_E_FILTER_PROCESS_TERMINATED: i32 = -2147218159i32;
pub const GTHR_E_FILTER_PROCESS_TERMINATED_QUOTA: i32 = -2147218151i32;
pub const GTHR_E_FILTER_SINGLE_THREADED: i32 = -2147218069i32;
pub const GTHR_E_FOLDER_CRAWLED_BY_ANOTHER_WORKSPACE: i32 = -2147218007i32;
pub const GTHR_E_FORCE_NOTIFICATION_RESET: i32 = -2147218065i32;
pub const GTHR_E_FROM_NOT_SPECIFIED: i32 = -2147218109i32;
pub const GTHR_E_IE_OFFLINE: i32 = -2147218120i32;
pub const GTHR_E_INSUFFICIENT_EXAMPLE_CATEGORIES: i32 = -2147218014i32;
pub const GTHR_E_INSUFFICIENT_EXAMPLE_DOCUMENTS: i32 = -2147218013i32;
pub const GTHR_E_INSUFFICIENT_FEATURE_TERMS: i32 = -2147218015i32;
pub const GTHR_E_INVALIDFUNCTION: i32 = -2147218161i32;
pub const GTHR_E_INVALID_ACCOUNT: i32 = -2147218132i32;
pub const GTHR_E_INVALID_ACCOUNT_SYNTAX: i32 = -2147218129i32;
pub const GTHR_E_INVALID_APPLICATION_NAME: i32 = -2147218077i32;
pub const GTHR_E_INVALID_CALL_FROM_WBREAKER: i32 = -2147218058i32;
pub const GTHR_E_INVALID_DIRECTORY: i32 = -2147218093i32;
pub const GTHR_E_INVALID_EXTENSION: i32 = -2147218107i32;
pub const GTHR_E_INVALID_GROW_FACTOR: i32 = -2147218106i32;
pub const GTHR_E_INVALID_HOST_NAME: i32 = -2147218096i32;
pub const GTHR_E_INVALID_LOG_FILE_NAME: i32 = -2147218103i32;
pub const GTHR_E_INVALID_MAPPING: i32 = -2147218112i32;
pub const GTHR_E_INVALID_PATH: i32 = -2147218124i32;
pub const GTHR_E_INVALID_PATH_EXPRESSION: i32 = -2147218088i32;
pub const GTHR_E_INVALID_PATH_SPEC: i32 = -2147218016i32;
pub const GTHR_E_INVALID_PROJECT_NAME: i32 = -2147218142i32;
pub const GTHR_E_INVALID_PROXY_PORT: i32 = -2147218091i32;
pub const GTHR_E_INVALID_RESOURCE_ID: i32 = -2147218035i32;
pub const GTHR_E_INVALID_RETRIES: i32 = -2147218104i32;
pub const GTHR_E_INVALID_START_ADDRESS: i32 = -2147217998i32;
pub const GTHR_E_INVALID_START_PAGE: i32 = -2147218095i32;
pub const GTHR_E_INVALID_START_PAGE_HOST: i32 = -2147218087i32;
pub const GTHR_E_INVALID_START_PAGE_PATH: i32 = -2147218080i32;
pub const GTHR_E_INVALID_STREAM_LOGS_COUNT: i32 = -2147218108i32;
pub const GTHR_E_INVALID_TIME_OUT: i32 = -2147218105i32;
pub const GTHR_E_JET_BACKUP_ERROR: i32 = -2147218026i32;
pub const GTHR_E_JET_RESTORE_ERROR: i32 = -2147218025i32;
pub const GTHR_E_LOCAL_GROUPS_EXPANSION_INTERNAL_ERROR: i32 = -2147216867i32;
pub const GTHR_E_NAME_TOO_LONG: i32 = -2147218156i32;
pub const GTHR_E_NESTED_HIERARCHICAL_START_ADDRESSES: i32 = -2147218034i32;
pub const GTHR_E_NOFILTERSINK: i32 = -2147218160i32;
pub const GTHR_E_NON_FIXED_DRIVE: i32 = -2147218074i32;
pub const GTHR_E_NOTIFICATION_FILE_SHARE_INFO_NOT_AVAILABLE: i32 = -2147218040i32;
pub const GTHR_E_NOTIFICATION_LOCAL_PATH_MUST_USE_FIXED_DRIVE: i32 = -2147218039i32;
pub const GTHR_E_NOTIFICATION_START_ADDRESS_INVALID: i32 = -2147218042i32;
pub const GTHR_E_NOTIFICATION_START_PAGE: i32 = -2147218137i32;
pub const GTHR_E_NOTIFICATION_TYPE_NOT_SUPPORTED: i32 = -2147218041i32;
pub const GTHR_E_NOTIF_ACCESS_TOKEN_UPDATED: i32 = -2147218020i32;
pub const GTHR_E_NOTIF_BEING_REMOVED: i32 = -2147218018i32;
pub const GTHR_E_NOTIF_EXCESSIVE_THROUGHPUT: i32 = -2147218017i32;
pub const GTHR_E_NO_IDENTITY: i32 = -2147218155i32;
pub const GTHR_E_NO_PRTCLHNLR: i32 = -2147218121i32;
pub const GTHR_E_NTF_CLIENT_NOT_SUBSCRIBED: i32 = -1073476167i32;
pub const GTHR_E_OBJECT_NOT_VALID: i32 = -2147218005i32;
pub const GTHR_E_OUT_OF_DOC_ID: i32 = -2147218138i32;
pub const GTHR_E_PIPE_NOT_CONNECTTED: i32 = -2147217996i32;
pub const GTHR_E_PLUGIN_NOT_REGISTERED: i32 = -2147218021i32;
pub const GTHR_E_PROJECT_NOT_INITIALIZED: i32 = -2147218149i32;
pub const GTHR_E_PROPERTIES_EXCEEDED: i32 = -2147218000i32;
pub const GTHR_E_PROPERTY_LIST_NOT_INITIALIZED: i32 = -2147218057i32;
pub const GTHR_E_PROXY_NAME: i32 = -2147218127i32;
pub const GTHR_E_PRT_HNDLR_PROGID_MISSING: i32 = -2147218152i32;
pub const GTHR_E_RECOVERABLE_EXOLEDB_ERROR: i32 = -2147218060i32;
pub const GTHR_E_RETRY: i32 = -2147218027i32;
pub const GTHR_E_SCHEMA_ERRORS_OCCURRED: i32 = -2147218054i32;
pub const GTHR_E_SCOPES_EXCEEDED: i32 = -2147218001i32;
pub const GTHR_E_SECRET_NOT_FOUND: i32 = -2147218089i32;
pub const GTHR_E_SERVER_UNAVAILABLE: i32 = -2147218126i32;
pub const GTHR_E_SHUTTING_DOWN: i32 = -2147218141i32;
pub const GTHR_E_SINGLE_THREADED_EMBEDDING: i32 = -2147218011i32;
pub const GTHR_E_TIMEOUT: i32 = -2147218053i32;
pub const GTHR_E_TOO_MANY_PLUGINS: i32 = -2147218162i32;
pub const GTHR_E_UNABLE_TO_READ_EXCHANGE_STORE: i32 = -2147218061i32;
pub const GTHR_E_UNABLE_TO_READ_REGISTRY: i32 = -2147218173i32;
pub const GTHR_E_UNKNOWN_PROTOCOL: i32 = -2147218150i32;
pub const GTHR_E_UNSUPPORTED_PROPERTY_TYPE: i32 = -2147218157i32;
pub const GTHR_E_URL_EXCLUDED: i32 = -2147218169i32;
pub const GTHR_E_URL_UNIDENTIFIED: i32 = -2147218067i32;
pub const GTHR_E_USER_AGENT_NOT_SPECIFIED: i32 = -2147218111i32;
pub const GTHR_E_VALUE_NOT_AVAILABLE: i32 = -2147218139i32;
pub const GTHR_S_BAD_FILE_LINK: i32 = 265580i32;
pub const GTHR_S_CANNOT_FILTER: i32 = 265520i32;
pub const GTHR_S_CANNOT_WORDBREAK: i32 = 265638i32;
pub const GTHR_S_CONFIG_HAS_ACCOUNTS: i32 = 265558i32;
pub const GTHR_S_CRAWL_ADAPTIVE: i32 = 265605i32;
pub const GTHR_S_CRAWL_FULL: i32 = 265603i32;
pub const GTHR_S_CRAWL_INCREMENTAL: i32 = 265604i32;
pub const GTHR_S_CRAWL_SCHEDULED: i32 = 265576i32;
pub const GTHR_S_END_PROCESS_LOOP_NOTIFY_QUEUE: i32 = 265584i32;
pub const GTHR_S_END_STD_CHUNKS: i32 = 265508i32;
pub const GTHR_S_MODIFIED_PARTS: i32 = 265592i32;
pub const GTHR_S_NOT_ALL_PARTS: i32 = 265582i32;
pub const GTHR_S_NO_CRAWL_SEEDS: i32 = 265515i32;
pub const GTHR_S_NO_INDEX: i32 = 265616i32;
pub const GTHR_S_OFFICE_CHILD: i32 = 265626i32;
pub const GTHR_S_PAUSE_REASON_BACKOFF: i32 = 265620i32;
pub const GTHR_S_PAUSE_REASON_EXTERNAL: i32 = 265618i32;
pub const GTHR_S_PAUSE_REASON_PROFILE_IMPORT: i32 = 265651i32;
pub const GTHR_S_PAUSE_REASON_UPGRADING: i32 = 265619i32;
pub const GTHR_S_PROB_NOT_MODIFIED: i32 = 265575i32;
pub const GTHR_S_START_FILTER_FROM_BODY: i32 = 265585i32;
pub const GTHR_S_START_FILTER_FROM_PROTOCOL: i32 = 265578i32;
pub const GTHR_S_STATUS_CHANGE_IGNORED: i32 = 265500i32;
pub const GTHR_S_STATUS_END_CRAWL: i32 = 265501i32;
pub const GTHR_S_STATUS_PAUSE: i32 = 265505i32;
pub const GTHR_S_STATUS_RESET: i32 = 265502i32;
pub const GTHR_S_STATUS_RESUME: i32 = 265504i32;
pub const GTHR_S_STATUS_START: i32 = 265526i32;
pub const GTHR_S_STATUS_STOP: i32 = 265523i32;
pub const GTHR_S_STATUS_THROTTLE: i32 = 265503i32;
pub const GTHR_S_TRANSACTION_IGNORED: i32 = 265577i32;
pub const GTHR_S_USE_MIME_FILTER: i32 = 265639i32;
pub const IDENTIFIER_SDK_ERROR: u32 = 268435456u32;
pub const IDENTIFIER_SDK_MASK: u32 = 4026531840u32;
pub const IDS_MON_BUILTIN_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x4093F_u32 as _);
pub const IDS_MON_BUILTIN_VIEW: windows_core::HRESULT = windows_core::HRESULT(0x40937_u32 as _);
pub const IDS_MON_CANNOT_CAST: windows_core::HRESULT = windows_core::HRESULT(0x40946_u32 as _);
pub const IDS_MON_CANNOT_CONVERT: windows_core::HRESULT = windows_core::HRESULT(0x4093B_u32 as _);
pub const IDS_MON_COLUMN_NOT_DEFINED: windows_core::HRESULT = windows_core::HRESULT(0x40936_u32 as _);
pub const IDS_MON_DATE_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x40947_u32 as _);
pub const IDS_MON_DEFAULT_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x4092F_u32 as _);
pub const IDS_MON_ILLEGAL_PASSTHROUGH: windows_core::HRESULT = windows_core::HRESULT(0x40930_u32 as _);
pub const IDS_MON_INVALIDSELECT_COALESCE: windows_core::HRESULT = windows_core::HRESULT(0x40945_u32 as _);
pub const IDS_MON_INVALID_CATALOG: windows_core::HRESULT = windows_core::HRESULT(0x40944_u32 as _);
pub const IDS_MON_INVALID_IN_GROUP_CLAUSE: windows_core::HRESULT = windows_core::HRESULT(0x40948_u32 as _);
pub const IDS_MON_MATCH_STRING: windows_core::HRESULT = windows_core::HRESULT(0x40941_u32 as _);
pub const IDS_MON_NOT_COLUMN_OF_VIEW: windows_core::HRESULT = windows_core::HRESULT(0x4093E_u32 as _);
pub const IDS_MON_ORDINAL_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x40934_u32 as _);
pub const IDS_MON_OR_NOT: windows_core::HRESULT = windows_core::HRESULT(0x4093A_u32 as _);
pub const IDS_MON_OUT_OF_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x40938_u32 as _);
pub const IDS_MON_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x4093C_u32 as _);
pub const IDS_MON_PARSE_ERR_1_PARAM: windows_core::HRESULT = windows_core::HRESULT(0x40931_u32 as _);
pub const IDS_MON_PARSE_ERR_2_PARAM: windows_core::HRESULT = windows_core::HRESULT(0x40932_u32 as _);
pub const IDS_MON_PROPERTY_NAME_IN_VIEW: windows_core::HRESULT = windows_core::HRESULT(0x40942_u32 as _);
pub const IDS_MON_RELATIVE_INTERVAL: windows_core::HRESULT = windows_core::HRESULT(0x4093D_u32 as _);
pub const IDS_MON_SELECT_STAR: windows_core::HRESULT = windows_core::HRESULT(0x40939_u32 as _);
pub const IDS_MON_SEMI_COLON: windows_core::HRESULT = windows_core::HRESULT(0x40933_u32 as _);
pub const IDS_MON_VIEW_ALREADY_DEFINED: windows_core::HRESULT = windows_core::HRESULT(0x40943_u32 as _);
pub const IDS_MON_VIEW_NOT_DEFINED: windows_core::HRESULT = windows_core::HRESULT(0x40935_u32 as _);
pub const IDS_MON_WEIGHT_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x40940_u32 as _);
pub const IDX_E_BUILD_IN_PROGRESS: i32 = -2147217147i32;
pub const IDX_E_CATALOG_DISMOUNTED: i32 = -2147217124i32;
pub const IDX_E_CORRUPT_INDEX: i32 = -2147217136i32;
pub const IDX_E_DISKFULL: i32 = -2147217138i32;
pub const IDX_E_DOCUMENT_ABORTED: i32 = -2147217125i32;
pub const IDX_E_DSS_NOT_CONNECTED: i32 = -2147217126i32;
pub const IDX_E_IDXLSTFILE_CORRUPT: i32 = -2147217146i32;
pub const IDX_E_INVALIDTAG: i32 = -2147217151i32;
pub const IDX_E_INVALID_INDEX: i32 = -2147217137i32;
pub const IDX_E_METAFILE_CORRUPT: i32 = -2147217150i32;
pub const IDX_E_NOISELIST_NOTFOUND: i32 = -2147217141i32;
pub const IDX_E_NOT_LOADED: i32 = -2147217129i32;
pub const IDX_E_OBJECT_NOT_FOUND: i32 = -2147217144i32;
pub const IDX_E_PROPSTORE_INIT_FAILED: i32 = -2147217134i32;
pub const IDX_E_PROP_MAJOR_VERSION_MISMATCH: i32 = -2147217128i32;
pub const IDX_E_PROP_MINOR_VERSION_MISMATCH: i32 = -2147217127i32;
pub const IDX_E_PROP_STATE_CORRUPT: i32 = -2147217133i32;
pub const IDX_E_PROP_STOPPED: i32 = -2147217139i32;
pub const IDX_E_REGISTRY_ENTRY: i32 = -2147217145i32;
pub const IDX_E_SEARCH_SERVER_ALREADY_EXISTS: i32 = -2147217148i32;
pub const IDX_E_SEARCH_SERVER_NOT_FOUND: i32 = -2147217143i32;
pub const IDX_E_STEMMER_NOTFOUND: i32 = -2147217140i32;
pub const IDX_E_TOO_MANY_SEARCH_SERVERS: i32 = -2147217149i32;
pub const IDX_E_USE_APPGLOBAL_PROPTABLE: i32 = -2147217120i32;
pub const IDX_E_USE_DEFAULT_CONTENTCLASS: i32 = -2147217121i32;
pub const IDX_E_WB_NOTFOUND: i32 = -2147217142i32;
pub const IDX_S_DSS_NOT_AVAILABLE: i32 = 266525i32;
pub const IDX_S_NO_BUILD_IN_PROGRESS: i32 = 266516i32;
pub const IDX_S_SEARCH_SERVER_ALREADY_EXISTS: i32 = 266517i32;
pub const IDX_S_SEARCH_SERVER_DOES_NOT_EXIST: i32 = 266518i32;
pub const ILK_EXPLICIT_EXCLUDED: INTERVAL_LIMIT_KIND = INTERVAL_LIMIT_KIND(1i32);
pub const ILK_EXPLICIT_INCLUDED: INTERVAL_LIMIT_KIND = INTERVAL_LIMIT_KIND(0i32);
pub const ILK_NEGATIVE_INFINITY: INTERVAL_LIMIT_KIND = INTERVAL_LIMIT_KIND(2i32);
pub const ILK_POSITIVE_INFINITY: INTERVAL_LIMIT_KIND = INTERVAL_LIMIT_KIND(3i32);
pub const INET_E_AGENT_CACHE_SIZE_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x800C0F82_u32 as _);
pub const INET_E_AGENT_CONNECTION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x800C0F83_u32 as _);
pub const INET_E_AGENT_EXCEEDING_CACHE_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x800C0F90_u32 as _);
pub const INET_E_AGENT_MAX_SIZE_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x800C0F80_u32 as _);
pub const INET_E_SCHEDULED_EXCLUDE_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x800C0F87_u32 as _);
pub const INET_E_SCHEDULED_UPDATES_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x800C0F84_u32 as _);
pub const INET_E_SCHEDULED_UPDATES_RESTRICTED: windows_core::HRESULT = windows_core::HRESULT(0x800C0F85_u32 as _);
pub const INET_E_SCHEDULED_UPDATE_INTERVAL: windows_core::HRESULT = windows_core::HRESULT(0x800C0F86_u32 as _);
pub const INET_S_AGENT_INCREASED_CACHE_SIZE: windows_core::HRESULT = windows_core::HRESULT(0xC0F90_u32 as _);
pub const INET_S_AGENT_PART_FAIL: windows_core::HRESULT = windows_core::HRESULT(0xC0F81_u32 as _);
pub const JET_GET_PROP_STORE_ERROR: i32 = -1073732822i32;
pub const JET_INIT_ERROR: i32 = -1073732824i32;
pub const JET_MULTIINSTANCE_DISABLED: i32 = -2147474645i32;
pub const JET_NEW_PROP_STORE_ERROR: i32 = -1073732823i32;
pub const JPS_E_CATALOG_DECSRIPTION_MISSING: i32 = -2147217023i32;
pub const JPS_E_INSUFFICIENT_DATABASE_RESOURCES: i32 = -2147217019i32;
pub const JPS_E_INSUFFICIENT_DATABASE_SESSIONS: i32 = -2147217020i32;
pub const JPS_E_INSUFFICIENT_VERSION_STORAGE: i32 = -2147217021i32;
pub const JPS_E_JET_ERR: i32 = -2147217025i32;
pub const JPS_E_MISSING_INFORMATION: i32 = -2147217022i32;
pub const JPS_E_PROPAGATION_CORRUPTION: i32 = -2147217016i32;
pub const JPS_E_PROPAGATION_FILE: i32 = -2147217017i32;
pub const JPS_E_PROPAGATION_VERSION_MISMATCH: i32 = -2147217015i32;
pub const JPS_E_SCHEMA_ERROR: i32 = -2147217018i32;
pub const JPS_E_SHARING_VIOLATION: i32 = -2147217014i32;
pub const JPS_S_DUPLICATE_DOC_DETECTED: i32 = 266624i32;
pub const KAGPROPVAL_CONCUR_LOCK: u32 = 4u32;
pub const KAGPROPVAL_CONCUR_READ_ONLY: u32 = 8u32;
pub const KAGPROPVAL_CONCUR_ROWVER: u32 = 1u32;
pub const KAGPROPVAL_CONCUR_VALUES: u32 = 2u32;
pub const KAGPROP_ACCESSIBLEPROCEDURES: u32 = 2u32;
pub const KAGPROP_ACCESSIBLETABLES: u32 = 3u32;
pub const KAGPROP_ACTIVESTATEMENTS: u32 = 24u32;
pub const KAGPROP_AUTH_SERVERINTEGRATED: u32 = 3u32;
pub const KAGPROP_AUTH_TRUSTEDCONNECTION: u32 = 2u32;
pub const KAGPROP_BLOBSONFOCURSOR: u32 = 8u32;
pub const KAGPROP_CONCURRENCY: u32 = 7u32;
pub const KAGPROP_CURSOR: u32 = 6u32;
pub const KAGPROP_DRIVERNAME: u32 = 7u32;
pub const KAGPROP_DRIVERODBCVER: u32 = 9u32;
pub const KAGPROP_DRIVERVER: u32 = 8u32;
pub const KAGPROP_FILEUSAGE: u32 = 23u32;
pub const KAGPROP_FORCENOPARAMETERREBIND: u32 = 11u32;
pub const KAGPROP_FORCENOPREPARE: u32 = 12u32;
pub const KAGPROP_FORCENOREEXECUTE: u32 = 13u32;
pub const KAGPROP_FORCESSFIREHOSEMODE: u32 = 10u32;
pub const KAGPROP_INCLUDENONEXACT: u32 = 9u32;
pub const KAGPROP_IRowsetChangeExtInfo: u32 = 5u32;
pub const KAGPROP_LIKEESCAPECLAUSE: u32 = 10u32;
pub const KAGPROP_MARSHALLABLE: u32 = 3u32;
pub const KAGPROP_MAXCOLUMNSINGROUPBY: u32 = 12u32;
pub const KAGPROP_MAXCOLUMNSININDEX: u32 = 13u32;
pub const KAGPROP_MAXCOLUMNSINORDERBY: u32 = 14u32;
pub const KAGPROP_MAXCOLUMNSINSELECT: u32 = 15u32;
pub const KAGPROP_MAXCOLUMNSINTABLE: u32 = 16u32;
pub const KAGPROP_NUMERICFUNCTIONS: u32 = 17u32;
pub const KAGPROP_ODBCSQLCONFORMANCE: u32 = 18u32;
pub const KAGPROP_ODBCSQLOPTIEF: u32 = 4u32;
pub const KAGPROP_OJCAPABILITY: u32 = 5u32;
pub const KAGPROP_OUTERJOINS: u32 = 19u32;
pub const KAGPROP_POSITIONONNEWROW: u32 = 4u32;
pub const KAGPROP_PROCEDURES: u32 = 6u32;
pub const KAGPROP_QUERYBASEDUPDATES: u32 = 2u32;
pub const KAGPROP_SPECIALCHARACTERS: u32 = 11u32;
pub const KAGPROP_STRINGFUNCTIONS: u32 = 20u32;
pub const KAGPROP_SYSTEMFUNCTIONS: u32 = 21u32;
pub const KAGPROP_TIMEDATEFUNCTIONS: u32 = 22u32;
pub const KAGREQDIAGFLAGS_HEADER: KAGREQDIAGFLAGSENUM = KAGREQDIAGFLAGSENUM(1i32);
pub const KAGREQDIAGFLAGS_RECORD: KAGREQDIAGFLAGSENUM = KAGREQDIAGFLAGSENUM(2i32);
pub const LOCKMODE_EXCLUSIVE: LOCKMODEENUM = LOCKMODEENUM(1i32);
pub const LOCKMODE_INVALID: LOCKMODEENUM = LOCKMODEENUM(0i32);
pub const LOCKMODE_SHARED: LOCKMODEENUM = LOCKMODEENUM(2i32);
pub const MAXNAME: u32 = 129u32;
pub const MAXNUMERICLEN: u32 = 16u32;
pub const MAXUSEVERITY: u32 = 18u32;
pub const MAX_QUERY_RANK: u32 = 1000u32;
pub const MDAXIS_CHAPTERS: u32 = 4u32;
pub const MDAXIS_COLUMNS: u32 = 0u32;
pub const MDAXIS_PAGES: u32 = 2u32;
pub const MDAXIS_ROWS: u32 = 1u32;
pub const MDAXIS_SECTIONS: u32 = 3u32;
pub const MDAXIS_SLICERS: u32 = 4294967295u32;
pub const MDDISPINFO_DRILLED_DOWN: u32 = 65536u32;
pub const MDDISPINFO_PARENT_SAME_AS_PREV: u32 = 131072u32;
pub const MDFF_BOLD: u32 = 1u32;
pub const MDFF_ITALIC: u32 = 2u32;
pub const MDFF_STRIKEOUT: u32 = 8u32;
pub const MDFF_UNDERLINE: u32 = 4u32;
pub const MDLEVEL_TYPE_ALL: u32 = 1u32;
pub const MDLEVEL_TYPE_CALCULATED: u32 = 2u32;
pub const MDLEVEL_TYPE_REGULAR: u32 = 0u32;
pub const MDLEVEL_TYPE_RESERVED1: u32 = 8u32;
pub const MDLEVEL_TYPE_TIME: u32 = 4u32;
pub const MDLEVEL_TYPE_TIME_DAYS: u32 = 516u32;
pub const MDLEVEL_TYPE_TIME_HALF_YEAR: u32 = 36u32;
pub const MDLEVEL_TYPE_TIME_HOURS: u32 = 772u32;
pub const MDLEVEL_TYPE_TIME_MINUTES: u32 = 1028u32;
pub const MDLEVEL_TYPE_TIME_MONTHS: u32 = 132u32;
pub const MDLEVEL_TYPE_TIME_QUARTERS: u32 = 68u32;
pub const MDLEVEL_TYPE_TIME_SECONDS: u32 = 2052u32;
pub const MDLEVEL_TYPE_TIME_UNDEFINED: u32 = 4100u32;
pub const MDLEVEL_TYPE_TIME_WEEKS: u32 = 260u32;
pub const MDLEVEL_TYPE_TIME_YEARS: u32 = 20u32;
pub const MDLEVEL_TYPE_UNKNOWN: u32 = 0u32;
pub const MDMEASURE_AGGR_AVG: u32 = 5u32;
pub const MDMEASURE_AGGR_CALCULATED: u32 = 127u32;
pub const MDMEASURE_AGGR_COUNT: u32 = 2u32;
pub const MDMEASURE_AGGR_MAX: u32 = 4u32;
pub const MDMEASURE_AGGR_MIN: u32 = 3u32;
pub const MDMEASURE_AGGR_STD: u32 = 7u32;
pub const MDMEASURE_AGGR_SUM: u32 = 1u32;
pub const MDMEASURE_AGGR_UNKNOWN: u32 = 0u32;
pub const MDMEASURE_AGGR_VAR: u32 = 6u32;
pub const MDMEMBER_TYPE_ALL: u32 = 2u32;
pub const MDMEMBER_TYPE_FORMULA: u32 = 4u32;
pub const MDMEMBER_TYPE_MEASURE: u32 = 3u32;
pub const MDMEMBER_TYPE_REGULAR: u32 = 1u32;
pub const MDMEMBER_TYPE_RESERVE1: u32 = 5u32;
pub const MDMEMBER_TYPE_RESERVE2: u32 = 6u32;
pub const MDMEMBER_TYPE_RESERVE3: u32 = 7u32;
pub const MDMEMBER_TYPE_RESERVE4: u32 = 8u32;
pub const MDMEMBER_TYPE_UNKNOWN: u32 = 0u32;
pub const MDPROPVAL_AU_UNCHANGED: i32 = 1i32;
pub const MDPROPVAL_AU_UNKNOWN: i32 = 2i32;
pub const MDPROPVAL_AU_UNSUPPORTED: i32 = 0i32;
pub const MDPROPVAL_FS_FULL_SUPPORT: i32 = 1i32;
pub const MDPROPVAL_FS_GENERATED_COLUMN: i32 = 2i32;
pub const MDPROPVAL_FS_GENERATED_DIMENSION: i32 = 3i32;
pub const MDPROPVAL_FS_NO_SUPPORT: i32 = 4i32;
pub const MDPROPVAL_MC_SEARCHEDCASE: i32 = 2i32;
pub const MDPROPVAL_MC_SINGLECASE: i32 = 1i32;
pub const MDPROPVAL_MD_AFTER: i32 = 4i32;
pub const MDPROPVAL_MD_BEFORE: i32 = 2i32;
pub const MDPROPVAL_MD_SELF: i32 = 1i32;
pub const MDPROPVAL_MF_CREATE_CALCMEMBERS: i32 = 4i32;
pub const MDPROPVAL_MF_CREATE_NAMEDSETS: i32 = 8i32;
pub const MDPROPVAL_MF_SCOPE_GLOBAL: i32 = 32i32;
pub const MDPROPVAL_MF_SCOPE_SESSION: i32 = 16i32;
pub const MDPROPVAL_MF_WITH_CALCMEMBERS: i32 = 1i32;
pub const MDPROPVAL_MF_WITH_NAMEDSETS: i32 = 2i32;
pub const MDPROPVAL_MJC_IMPLICITCUBE: i32 = 4i32;
pub const MDPROPVAL_MJC_MULTICUBES: i32 = 2i32;
pub const MDPROPVAL_MJC_SINGLECUBE: i32 = 1i32;
pub const MDPROPVAL_MMF_CLOSINGPERIOD: i32 = 8i32;
pub const MDPROPVAL_MMF_COUSIN: i32 = 1i32;
pub const MDPROPVAL_MMF_OPENINGPERIOD: i32 = 4i32;
pub const MDPROPVAL_MMF_PARALLELPERIOD: i32 = 2i32;
pub const MDPROPVAL_MNF_AGGREGATE: i32 = 16i32;
pub const MDPROPVAL_MNF_CORRELATION: i32 = 64i32;
pub const MDPROPVAL_MNF_COVARIANCE: i32 = 32i32;
pub const MDPROPVAL_MNF_DRILLDOWNLEVEL: i32 = 2048i32;
pub const MDPROPVAL_MNF_DRILLDOWNLEVELBOTTOM: i32 = 32768i32;
pub const MDPROPVAL_MNF_DRILLDOWNLEVELTOP: i32 = 16384i32;
pub const MDPROPVAL_MNF_DRILLDOWNMEMBERBOTTOM: i32 = 8192i32;
pub const MDPROPVAL_MNF_DRILLDOWNMEMBERTOP: i32 = 4096i32;
pub const MDPROPVAL_MNF_DRILLUPLEVEL: i32 = 131072i32;
pub const MDPROPVAL_MNF_DRILLUPMEMBER: i32 = 65536i32;
pub const MDPROPVAL_MNF_LINREG2: i32 = 512i32;
pub const MDPROPVAL_MNF_LINREGPOINT: i32 = 1024i32;
pub const MDPROPVAL_MNF_LINREGSLOPE: i32 = 128i32;
pub const MDPROPVAL_MNF_LINREGVARIANCE: i32 = 256i32;
pub const MDPROPVAL_MNF_MEDIAN: i32 = 1i32;
pub const MDPROPVAL_MNF_RANK: i32 = 8i32;
pub const MDPROPVAL_MNF_STDDEV: i32 = 4i32;
pub const MDPROPVAL_MNF_VAR: i32 = 2i32;
pub const MDPROPVAL_MOQ_CATALOG_CUBE: i32 = 2i32;
pub const MDPROPVAL_MOQ_CUBE_DIM: i32 = 8i32;
pub const MDPROPVAL_MOQ_DATASOURCE_CUBE: i32 = 1i32;
pub const MDPROPVAL_MOQ_DIMHIER_LEVEL: i32 = 32i32;
pub const MDPROPVAL_MOQ_DIMHIER_MEMBER: i32 = 256i32;
pub const MDPROPVAL_MOQ_DIM_HIER: i32 = 16i32;
pub const MDPROPVAL_MOQ_LEVEL_MEMBER: i32 = 64i32;
pub const MDPROPVAL_MOQ_MEMBER_MEMBER: i32 = 128i32;
pub const MDPROPVAL_MOQ_OUTERREFERENCE: i32 = 1i32;
pub const MDPROPVAL_MOQ_SCHEMA_CUBE: i32 = 4i32;
pub const MDPROPVAL_MSC_GREATERTHAN: i32 = 2i32;
pub const MDPROPVAL_MSC_GREATERTHANEQUAL: i32 = 8i32;
pub const MDPROPVAL_MSC_LESSTHAN: i32 = 1i32;
pub const MDPROPVAL_MSC_LESSTHANEQUAL: i32 = 4i32;
pub const MDPROPVAL_MSF_BOTTOMPERCENT: i32 = 2i32;
pub const MDPROPVAL_MSF_BOTTOMSUM: i32 = 8i32;
pub const MDPROPVAL_MSF_DRILLDOWNLEVEL: i32 = 2048i32;
pub const MDPROPVAL_MSF_DRILLDOWNLEVELBOTTOM: i32 = 32768i32;
pub const MDPROPVAL_MSF_DRILLDOWNLEVELTOP: i32 = 16384i32;
pub const MDPROPVAL_MSF_DRILLDOWNMEMBBER: i32 = 1024i32;
pub const MDPROPVAL_MSF_DRILLDOWNMEMBERBOTTOM: i32 = 8192i32;
pub const MDPROPVAL_MSF_DRILLDOWNMEMBERTOP: i32 = 4096i32;
pub const MDPROPVAL_MSF_DRILLUPLEVEL: i32 = 131072i32;
pub const MDPROPVAL_MSF_DRILLUPMEMBER: i32 = 65536i32;
pub const MDPROPVAL_MSF_LASTPERIODS: i32 = 32i32;
pub const MDPROPVAL_MSF_MTD: i32 = 256i32;
pub const MDPROPVAL_MSF_PERIODSTODATE: i32 = 16i32;
pub const MDPROPVAL_MSF_QTD: i32 = 128i32;
pub const MDPROPVAL_MSF_TOGGLEDRILLSTATE: i32 = 262144i32;
pub const MDPROPVAL_MSF_TOPPERCENT: i32 = 1i32;
pub const MDPROPVAL_MSF_TOPSUM: i32 = 4i32;
pub const MDPROPVAL_MSF_WTD: i32 = 512i32;
pub const MDPROPVAL_MSF_YTD: i32 = 64i32;
pub const MDPROPVAL_MS_MULTIPLETUPLES: i32 = 1i32;
pub const MDPROPVAL_MS_SINGLETUPLE: i32 = 2i32;
pub const MDPROPVAL_NL_NAMEDLEVELS: i32 = 1i32;
pub const MDPROPVAL_NL_NUMBEREDLEVELS: i32 = 2i32;
pub const MDPROPVAL_NL_SCHEMAONLY: i32 = 4i32;
pub const MDPROPVAL_NME_ALLDIMENSIONS: i32 = 0i32;
pub const MDPROPVAL_NME_MEASURESONLY: i32 = 1i32;
pub const MDPROPVAL_RR_NORANGEROWSET: i32 = 1i32;
pub const MDPROPVAL_RR_READONLY: i32 = 2i32;
pub const MDPROPVAL_RR_UPDATE: i32 = 4i32;
pub const MDPROPVAL_VISUAL_MODE_DEFAULT: i32 = 0i32;
pub const MDPROPVAL_VISUAL_MODE_VISUAL: i32 = 1i32;
pub const MDPROPVAL_VISUAL_MODE_VISUAL_OFF: i32 = 2i32;
pub const MDPROP_AGGREGATECELL_UPDATE: DBPROPENUM20 = DBPROPENUM20(230i32);
pub const MDPROP_AXES: DBPROPENUM20 = DBPROPENUM20(252i32);
pub const MDPROP_CELL: u32 = 2u32;
pub const MDPROP_FLATTENING_SUPPORT: DBPROPENUM20 = DBPROPENUM20(253i32);
pub const MDPROP_MDX_AGGREGATECELL_UPDATE: DBPROPENUM20 = DBPROPENUM20(230i32);
pub const MDPROP_MDX_CASESUPPORT: DBPROPENUM20 = DBPROPENUM20(222i32);
pub const MDPROP_MDX_CUBEQUALIFICATION: DBPROPENUM20 = DBPROPENUM20(219i32);
pub const MDPROP_MDX_DESCFLAGS: DBPROPENUM20 = DBPROPENUM20(225i32);
pub const MDPROP_MDX_FORMULAS: DBPROPENUM20 = DBPROPENUM20(229i32);
pub const MDPROP_MDX_JOINCUBES: DBPROPENUM20 = DBPROPENUM20(254i32);
pub const MDPROP_MDX_MEMBER_FUNCTIONS: DBPROPENUM20 = DBPROPENUM20(227i32);
pub const MDPROP_MDX_NONMEASURE_EXPRESSIONS: DBPROPENUM20 = DBPROPENUM20(262i32);
pub const MDPROP_MDX_NUMERIC_FUNCTIONS: DBPROPENUM20 = DBPROPENUM20(228i32);
pub const MDPROP_MDX_OBJQUALIFICATION: DBPROPENUM20 = DBPROPENUM20(261i32);
pub const MDPROP_MDX_OUTERREFERENCE: DBPROPENUM20 = DBPROPENUM20(220i32);
pub const MDPROP_MDX_QUERYBYPROPERTY: DBPROPENUM20 = DBPROPENUM20(221i32);
pub const MDPROP_MDX_SET_FUNCTIONS: DBPROPENUM20 = DBPROPENUM20(226i32);
pub const MDPROP_MDX_SLICER: DBPROPENUM20 = DBPROPENUM20(218i32);
pub const MDPROP_MDX_STRING_COMPOP: DBPROPENUM20 = DBPROPENUM20(224i32);
pub const MDPROP_MEMBER: u32 = 1u32;
pub const MDPROP_NAMED_LEVELS: DBPROPENUM20 = DBPROPENUM20(255i32);
pub const MDPROP_RANGEROWSET: DBPROPENUM20 = DBPROPENUM20(256i32);
pub const MDPROP_VISUALMODE: DBPROPENUM26 = DBPROPENUM26(293i32);
pub const MDSTATUS_S_CELLEMPTY: DBSTATUSENUM20 = DBSTATUSENUM20(14i32);
pub const MDTREEOP_ANCESTORS: u32 = 32u32;
pub const MDTREEOP_CHILDREN: u32 = 1u32;
pub const MDTREEOP_DESCENDANTS: u32 = 16u32;
pub const MDTREEOP_PARENT: u32 = 4u32;
pub const MDTREEOP_SELF: u32 = 8u32;
pub const MDTREEOP_SIBLINGS: u32 = 2u32;
pub const MD_DIMTYPE_MEASURE: u32 = 2u32;
pub const MD_DIMTYPE_OTHER: u32 = 3u32;
pub const MD_DIMTYPE_TIME: u32 = 1u32;
pub const MD_DIMTYPE_UNKNOWN: u32 = 0u32;
pub const MD_E_BADCOORDINATE: windows_core::HRESULT = windows_core::HRESULT(0x80040E62_u32 as _);
pub const MD_E_BADTUPLE: windows_core::HRESULT = windows_core::HRESULT(0x80040E61_u32 as _);
pub const MD_E_INVALIDAXIS: windows_core::HRESULT = windows_core::HRESULT(0x80040E63_u32 as _);
pub const MD_E_INVALIDCELLRANGE: windows_core::HRESULT = windows_core::HRESULT(0x80040E64_u32 as _);
pub const MINFATALERR: u32 = 20u32;
pub const MIN_USER_DATATYPE: u32 = 256u32;
pub const MSG_CI_CORRUPT_INDEX_COMPONENT: windows_core::HRESULT = windows_core::HRESULT(0x4000102A_u32 as _);
pub const MSG_CI_CREATE_SEVER_ITEM_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80001048_u32 as _);
pub const MSG_CI_MASTER_MERGE_ABORTED: windows_core::HRESULT = windows_core::HRESULT(0x40001008_u32 as _);
pub const MSG_CI_MASTER_MERGE_ABORTED_LOW_DISK: windows_core::HRESULT = windows_core::HRESULT(0x40001043_u32 as _);
pub const MSG_CI_MASTER_MERGE_CANT_RESTART: windows_core::HRESULT = windows_core::HRESULT(0xC000100A_u32 as _);
pub const MSG_CI_MASTER_MERGE_CANT_START: windows_core::HRESULT = windows_core::HRESULT(0xC0001009_u32 as _);
pub const MSG_CI_MASTER_MERGE_COMPLETED: windows_core::HRESULT = windows_core::HRESULT(0x40001007_u32 as _);
pub const MSG_CI_MASTER_MERGE_REASON_EXPECTED_DOCS: windows_core::HRESULT = windows_core::HRESULT(0x40001046_u32 as _);
pub const MSG_CI_MASTER_MERGE_REASON_EXTERNAL: windows_core::HRESULT = windows_core::HRESULT(0x40001044_u32 as _);
pub const MSG_CI_MASTER_MERGE_REASON_INDEX_LIMIT: windows_core::HRESULT = windows_core::HRESULT(0x40001045_u32 as _);
pub const MSG_CI_MASTER_MERGE_REASON_NUMBER: windows_core::HRESULT = windows_core::HRESULT(0x40001047_u32 as _);
pub const MSG_CI_MASTER_MERGE_RESTARTED: windows_core::HRESULT = windows_core::HRESULT(0x40001019_u32 as _);
pub const MSG_CI_MASTER_MERGE_STARTED: windows_core::HRESULT = windows_core::HRESULT(0x40001006_u32 as _);
pub const MSG_TEST_MESSAGE: i32 = 1074008064i32;
pub const MSS_E_APPALREADYEXISTS: i32 = -2147213054i32;
pub const MSS_E_APPNOTFOUND: i32 = -2147213055i32;
pub const MSS_E_CATALOGALREADYEXISTS: i32 = -2147213050i32;
pub const MSS_E_CATALOGNOTFOUND: i32 = -2147213053i32;
pub const MSS_E_CATALOGSTOPPING: i32 = -2147213052i32;
pub const MSS_E_INVALIDAPPNAME: i32 = -2147213056i32;
pub const MSS_E_UNICODEFILEHEADERMISSING: i32 = -2147213051i32;
pub const MS_PERSIST_PROGID: windows_core::PCSTR = windows_core::s!("MSPersist");
pub const NEC_HIGH: NAMED_ENTITY_CERTAINTY = NAMED_ENTITY_CERTAINTY(2i32);
pub const NEC_LOW: NAMED_ENTITY_CERTAINTY = NAMED_ENTITY_CERTAINTY(0i32);
pub const NEC_MEDIUM: NAMED_ENTITY_CERTAINTY = NAMED_ENTITY_CERTAINTY(1i32);
pub const NET_E_DISCONNECTED: i32 = -2147220733i32;
pub const NET_E_GENERAL: i32 = -2147220736i32;
pub const NET_E_INVALIDPARAMS: i32 = -2147220728i32;
pub const NET_E_OPERATIONINPROGRESS: i32 = -2147220727i32;
pub const NLADMIN_E_BUILD_CATALOG_NOT_INITIALIZED: i32 = -2147215100i32;
pub const NLADMIN_E_DUPLICATE_CATALOG: i32 = -2147215103i32;
pub const NLADMIN_E_FAILED_TO_GIVE_ACCOUNT_PRIVILEGE: i32 = -2147215101i32;
pub const NLADMIN_S_NOT_ALL_BUILD_CATALOGS_INITIALIZED: i32 = 268546i32;
pub const NOTESPH_E_ATTACHMENTS: i32 = -2147211770i32;
pub const NOTESPH_E_DB_ACCESS_DENIED: i32 = -2147211768i32;
pub const NOTESPH_E_FAIL: i32 = -2147211759i32;
pub const NOTESPH_E_ITEM_NOT_FOUND: i32 = -2147211772i32;
pub const NOTESPH_E_NOTESSETUP_ID_MAPPING_ERROR: i32 = -2147211767i32;
pub const NOTESPH_E_NO_NTID: i32 = -2147211769i32;
pub const NOTESPH_E_SERVER_CONFIG: i32 = -2147211771i32;
pub const NOTESPH_E_UNEXPECTED_STATE: i32 = -2147211775i32;
pub const NOTESPH_E_UNSUPPORTED_CONTENT_FIELD_TYPE: i32 = -2147211773i32;
pub const NOTESPH_S_IGNORE_ID: i32 = 271874i32;
pub const NOTESPH_S_LISTKNOWNFIELDS: i32 = 271888i32;
pub const NOT_N_PARSE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8092E_u32 as _);
pub const OCC_INVALID: u32 = 4294967295u32;
pub const ODBCVER: u32 = 896u32;
pub const ODBC_ADD_DSN: u32 = 1u32;
pub const ODBC_ADD_SYS_DSN: u32 = 4u32;
pub const ODBC_BOTH_DSN: u32 = 0u32;
pub const ODBC_CONFIG_DRIVER: u32 = 3u32;
pub const ODBC_CONFIG_DRIVER_MAX: u32 = 100u32;
pub const ODBC_CONFIG_DSN: u32 = 2u32;
pub const ODBC_CONFIG_SYS_DSN: u32 = 5u32;
pub const ODBC_ERROR_COMPONENT_NOT_FOUND: u32 = 6u32;
pub const ODBC_ERROR_CREATE_DSN_FAILED: u32 = 18u32;
pub const ODBC_ERROR_GENERAL_ERR: u32 = 1u32;
pub const ODBC_ERROR_INVALID_BUFF_LEN: u32 = 2u32;
pub const ODBC_ERROR_INVALID_DSN: u32 = 9u32;
pub const ODBC_ERROR_INVALID_HWND: u32 = 3u32;
pub const ODBC_ERROR_INVALID_INF: u32 = 10u32;
pub const ODBC_ERROR_INVALID_KEYWORD_VALUE: u32 = 8u32;
pub const ODBC_ERROR_INVALID_LOG_FILE: u32 = 15u32;
pub const ODBC_ERROR_INVALID_NAME: u32 = 7u32;
pub const ODBC_ERROR_INVALID_PARAM_SEQUENCE: u32 = 14u32;
pub const ODBC_ERROR_INVALID_PATH: u32 = 12u32;
pub const ODBC_ERROR_INVALID_REQUEST_TYPE: u32 = 5u32;
pub const ODBC_ERROR_INVALID_STR: u32 = 4u32;
pub const ODBC_ERROR_LOAD_LIB_FAILED: u32 = 13u32;
pub const ODBC_ERROR_MAX: u32 = 23u32;
pub const ODBC_ERROR_NOTRANINFO: u32 = 23u32;
pub const ODBC_ERROR_OUTPUT_STRING_TRUNCATED: u32 = 22u32;
pub const ODBC_ERROR_OUT_OF_MEM: u32 = 21u32;
pub const ODBC_ERROR_REMOVE_DSN_FAILED: u32 = 20u32;
pub const ODBC_ERROR_REQUEST_FAILED: u32 = 11u32;
pub const ODBC_ERROR_USAGE_UPDATE_FAILED: u32 = 17u32;
pub const ODBC_ERROR_USER_CANCELED: u32 = 16u32;
pub const ODBC_ERROR_WRITING_SYSINFO_FAILED: u32 = 19u32;
pub const ODBC_INSTALL_COMPLETE: u32 = 2u32;
pub const ODBC_INSTALL_DRIVER: u32 = 1u32;
pub const ODBC_INSTALL_INQUIRY: u32 = 1u32;
pub const ODBC_REMOVE_DEFAULT_DSN: u32 = 7u32;
pub const ODBC_REMOVE_DRIVER: u32 = 2u32;
pub const ODBC_REMOVE_DSN: u32 = 3u32;
pub const ODBC_REMOVE_SYS_DSN: u32 = 6u32;
pub const ODBC_SYSTEM_DSN: u32 = 2u32;
pub const ODBC_USER_DSN: u32 = 1u32;
pub const ODBC_VS_FLAG_RETCODE: i32 = 4i32;
pub const ODBC_VS_FLAG_STOP: i32 = 8i32;
pub const ODBC_VS_FLAG_UNICODE_ARG: i32 = 1i32;
pub const ODBC_VS_FLAG_UNICODE_COR: i32 = 2i32;
pub const OLEDBVER: u32 = 624u32;
pub const OLEDB_BINDER_CUSTOM_ERROR: i32 = -2147212032i32;
pub const OSPCOMP_DEFAULT: OSPCOMP = OSPCOMP(1i32);
pub const OSPCOMP_EQ: OSPCOMP = OSPCOMP(1i32);
pub const OSPCOMP_GE: OSPCOMP = OSPCOMP(4i32);
pub const OSPCOMP_GT: OSPCOMP = OSPCOMP(5i32);
pub const OSPCOMP_LE: OSPCOMP = OSPCOMP(3i32);
pub const OSPCOMP_LT: OSPCOMP = OSPCOMP(2i32);
pub const OSPCOMP_NE: OSPCOMP = OSPCOMP(6i32);
pub const OSPFIND_CASESENSITIVE: OSPFIND = OSPFIND(2i32);
pub const OSPFIND_DEFAULT: OSPFIND = OSPFIND(0i32);
pub const OSPFIND_UP: OSPFIND = OSPFIND(1i32);
pub const OSPFIND_UPCASESENSITIVE: OSPFIND = OSPFIND(3i32);
pub const OSPFORMAT_DEFAULT: OSPFORMAT = OSPFORMAT(0i32);
pub const OSPFORMAT_FORMATTED: OSPFORMAT = OSPFORMAT(1i32);
pub const OSPFORMAT_HTML: OSPFORMAT = OSPFORMAT(2i32);
pub const OSPFORMAT_RAW: OSPFORMAT = OSPFORMAT(0i32);
pub const OSPRW_DEFAULT: OSPRW = OSPRW(1i32);
pub const OSPRW_MIXED: OSPRW = OSPRW(2i32);
pub const OSPRW_READONLY: OSPRW = OSPRW(0i32);
pub const OSPRW_READWRITE: OSPRW = OSPRW(1i32);
pub const OSPXFER_ABORT: OSPXFER = OSPXFER(1i32);
pub const OSPXFER_COMPLETE: OSPXFER = OSPXFER(0i32);
pub const OSPXFER_ERROR: OSPXFER = OSPXFER(2i32);
pub const OSP_IndexLabel: u32 = 0u32;
pub const PEOPLE_IMPORT_E_CANONICALURL_TOOLONG: i32 = -2147205110i32;
pub const PEOPLE_IMPORT_E_DATATYPENOTSUPPORTED: i32 = -2147205115i32;
pub const PEOPLE_IMPORT_E_DBCONNFAIL: i32 = -2147205120i32;
pub const PEOPLE_IMPORT_E_DC_NOT_AVAILABLE: i32 = -2147205108i32;
pub const PEOPLE_IMPORT_E_DIRSYNC_NOTREFRESHED: i32 = -2147205103i32;
pub const PEOPLE_IMPORT_E_DIRSYNC_ZERO_COOKIE: i32 = -2147205112i32;
pub const PEOPLE_IMPORT_E_DOMAIN_DISCOVER_FAILED: i32 = -2147205107i32;
pub const PEOPLE_IMPORT_E_DOMAIN_REMOVED: i32 = -2147205105i32;
pub const PEOPLE_IMPORT_E_ENUM_ACCESSDENIED: i32 = -2147205104i32;
pub const PEOPLE_IMPORT_E_FAILTOGETDSDEF: i32 = -2147205118i32;
pub const PEOPLE_IMPORT_E_FAILTOGETDSMAPPING: i32 = -2147205116i32;
pub const PEOPLE_IMPORT_E_FAILTOGETLCID: i32 = -2147205106i32;
pub const PEOPLE_IMPORT_E_LDAPPATH_TOOLONG: i32 = -2147205111i32;
pub const PEOPLE_IMPORT_E_NOCASTINGSUPPORTED: i32 = -2147205114i32;
pub const PEOPLE_IMPORT_E_UPDATE_DIRSYNC_COOKIE: i32 = -2147205113i32;
pub const PEOPLE_IMPORT_E_USERNAME_NOTRESOLVED: i32 = -2147205109i32;
pub const PEOPLE_IMPORT_NODSDEFINED: i32 = -2147205119i32;
pub const PEOPLE_IMPORT_NOMAPPINGDEFINED: i32 = -2147205117i32;
pub const PERM_ALL: ACCESS_MASKENUM = ACCESS_MASKENUM(268435456i32);
pub const PERM_CREATE: ACCESS_MASKENUM = ACCESS_MASKENUM(16384i32);
pub const PERM_DELETE: ACCESS_MASKENUM = ACCESS_MASKENUM(65536i32);
pub const PERM_DROP: ACCESS_MASKENUM = ACCESS_MASKENUM(256i32);
pub const PERM_EXCLUSIVE: ACCESS_MASKENUM = ACCESS_MASKENUM(512i32);
pub const PERM_EXECUTE: ACCESS_MASKENUM = ACCESS_MASKENUM(536870912i32);
pub const PERM_INSERT: ACCESS_MASKENUM = ACCESS_MASKENUM(32768i32);
pub const PERM_MAXIMUM_ALLOWED: ACCESS_MASKENUM = ACCESS_MASKENUM(33554432i32);
pub const PERM_READ: ACCESS_MASKENUM = ACCESS_MASKENUM(-2147483648i32);
pub const PERM_READCONTROL: ACCESS_MASKENUM = ACCESS_MASKENUM(131072i32);
pub const PERM_READDESIGN: ACCESS_MASKENUM = ACCESS_MASKENUM(1024i32);
pub const PERM_REFERENCE: ACCESS_MASKENUM = ACCESS_MASKENUM(8192i32);
pub const PERM_UPDATE: ACCESS_MASKENUM = ACCESS_MASKENUM(1073741824i32);
pub const PERM_WITHGRANT: ACCESS_MASKENUM = ACCESS_MASKENUM(4096i32);
pub const PERM_WRITEDESIGN: ACCESS_MASKENUM = ACCESS_MASKENUM(2048i32);
pub const PERM_WRITEOWNER: ACCESS_MASKENUM = ACCESS_MASKENUM(524288i32);
pub const PERM_WRITEPERMISSIONS: ACCESS_MASKENUM = ACCESS_MASKENUM(262144i32);
pub const PRAll: u32 = 256u32;
pub const PRAllBits: u32 = 7u32;
pub const PRAny: u32 = 512u32;
pub const PRIORITIZE_FLAG_IGNOREFAILURECOUNT: PRIORITIZE_FLAGS = PRIORITIZE_FLAGS(2i32);
pub const PRIORITIZE_FLAG_RETRYFAILEDITEMS: PRIORITIZE_FLAGS = PRIORITIZE_FLAGS(1i32);
pub const PRIORITY_LEVEL_DEFAULT: PRIORITY_LEVEL = PRIORITY_LEVEL(3i32);
pub const PRIORITY_LEVEL_FOREGROUND: PRIORITY_LEVEL = PRIORITY_LEVEL(0i32);
pub const PRIORITY_LEVEL_HIGH: PRIORITY_LEVEL = PRIORITY_LEVEL(1i32);
pub const PRIORITY_LEVEL_LOW: PRIORITY_LEVEL = PRIORITY_LEVEL(2i32);
pub const PROGID_MSPersist_Version_W: windows_core::PCWSTR = windows_core::w!("MSPersist.1");
pub const PROGID_MSPersist_W: windows_core::PCWSTR = windows_core::w!("MSPersist");
pub const PROPID_DBBMK_BOOKMARK: u32 = 2u32;
pub const PROPID_DBBMK_CHAPTER: u32 = 3u32;
pub const PROPID_DBSELF_SELF: u32 = 2u32;
pub const PROXY_ACCESS_DIRECT: PROXY_ACCESS = PROXY_ACCESS(1i32);
pub const PROXY_ACCESS_PRECONFIG: PROXY_ACCESS = PROXY_ACCESS(0i32);
pub const PROXY_ACCESS_PROXY: PROXY_ACCESS = PROXY_ACCESS(2i32);
pub const PRRE: u32 = 6u32;
pub const PRSomeBits: u32 = 8u32;
pub const PRTH_E_ACCESS_DENIED: u32 = 2147750405u32;
pub const PRTH_E_ACL_IS_READ_NONE: u32 = 2147750417u32;
pub const PRTH_E_ACL_TOO_BIG: u32 = 2147750418u32;
pub const PRTH_E_BAD_REQUEST: u32 = 2147750408u32;
pub const PRTH_E_CANT_TRANSFORM_DENIED_ACE: i32 = -2147216881i32;
pub const PRTH_E_CANT_TRANSFORM_EXTERNAL_ACL: i32 = -2147216882i32;
pub const PRTH_E_COMM_ERROR: u32 = 2147750400u32;
pub const PRTH_E_DATABASE_OPEN_ERROR: i32 = -2147216875i32;
pub const PRTH_E_HTTPS_CERTIFICATE_ERROR: i32 = -2147216861i32;
pub const PRTH_E_HTTPS_REQUIRE_CERTIFICATE: i32 = -2147216860i32;
pub const PRTH_E_HTTP_CANNOT_CONNECT: u32 = 2147750409u32;
pub const PRTH_E_INIT_FAILED: i32 = -2147216872i32;
pub const PRTH_E_INTERNAL_ERROR: i32 = -2147216892i32;
pub const PRTH_E_LOAD_FAILED: i32 = -2147216873i32;
pub const PRTH_E_MIME_EXCLUDED: i32 = -2147216883i32;
pub const PRTH_E_NOT_REDIRECTED: u32 = 2147750407u32;
pub const PRTH_E_NO_PROPERTY: i32 = -2147216877i32;
pub const PRTH_E_OBJ_NOT_FOUND: u32 = 2147750401u32;
pub const PRTH_E_OPLOCK_BROKEN: i32 = -2147216874i32;
pub const PRTH_E_REQUEST_ERROR: u32 = 2147750402u32;
pub const PRTH_E_RETRY: i32 = -2147216885i32;
pub const PRTH_E_SERVER_ERROR: u32 = 2147750406u32;
pub const PRTH_E_TRUNCATED: i32 = -2147216870i32;
pub const PRTH_E_VOLUME_MOUNT_POINT: i32 = -2147216871i32;
pub const PRTH_E_WININET: i32 = -2147216886i32;
pub const PRTH_S_ACL_IS_READ_EVERYONE: u32 = 266768u32;
pub const PRTH_S_MAX_DOWNLOAD: i32 = 266764i32;
pub const PRTH_S_MAX_GROWTH: i32 = 266761i32;
pub const PRTH_S_NOT_ALL_PARTS: u32 = 266779u32;
pub const PRTH_S_NOT_MODIFIED: u32 = 266755u32;
pub const PRTH_S_TRY_IMPERSONATING: i32 = 266789i32;
pub const PRTH_S_USE_ROSEBUD: i32 = 266772i32;
pub const PSGUID_CHARACTERIZATION: windows_core::GUID = windows_core::GUID::from_u128(0x560c36c0_503a_11cf_baa1_00004c752a9a);
pub const PSGUID_QUERY_METADATA: windows_core::GUID = windows_core::GUID::from_u128(0x624c9360_93d0_11cf_a787_00004c752752);
pub const PSGUID_STORAGE: windows_core::GUID = windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac);
pub const PWPROP_OSPVALUE: u32 = 2u32;
pub const QPMO_APPEND_LCID_TO_LOCALIZED_PATH: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(4i32);
pub const QPMO_LOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(3i32);
pub const QPMO_LOCALIZER_SUPPORT: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(5i32);
pub const QPMO_PRELOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(1i32);
pub const QPMO_SCHEMA_BINARY_NAME: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(0i32);
pub const QPMO_UNLOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(2i32);
pub const QRY_E_COLUMNNOTSEARCHABLE: i32 = -2147219700i32;
pub const QRY_E_COLUMNNOTSORTABLE: i32 = -2147219701i32;
pub const QRY_E_ENGINEFAILED: i32 = -2147219693i32;
pub const QRY_E_INFIXWILDCARD: i32 = -2147219696i32;
pub const QRY_E_INVALIDCATALOG: i32 = -2147219687i32;
pub const QRY_E_INVALIDCOLUMN: i32 = -2147219699i32;
pub const QRY_E_INVALIDINTERVAL: i32 = -2147219682i32;
pub const QRY_E_INVALIDPATH: i32 = -2147219684i32;
pub const QRY_E_INVALIDSCOPES: i32 = -2147219688i32;
pub const QRY_E_LMNOTINITIALIZED: i32 = -2147219683i32;
pub const QRY_E_NOCOLUMNS: i32 = -2147219689i32;
pub const QRY_E_NODATASOURCES: i32 = -2147219703i32;
pub const QRY_E_NOLOGMANAGER: i32 = -2147219681i32;
pub const QRY_E_NULLQUERY: i32 = -2147219691i32;
pub const QRY_E_PREFIXWILDCARD: i32 = -2147219697i32;
pub const QRY_E_QUERYCORRUPT: i32 = -2147219698i32;
pub const QRY_E_QUERYSYNTAX: i32 = -2147219711i32;
pub const QRY_E_SCOPECARDINALIDY: i32 = -2147219686i32;
pub const QRY_E_SEARCHTOOBIG: i32 = -2147219692i32;
pub const QRY_E_STARTHITTOBIG: i32 = -2147219705i32;
pub const QRY_E_TIMEOUT: i32 = -2147219702i32;
pub const QRY_E_TOOMANYCOLUMNS: i32 = -2147219707i32;
pub const QRY_E_TOOMANYDATABASES: i32 = -2147219706i32;
pub const QRY_E_TOOMANYQUERYTERMS: i32 = -2147219704i32;
pub const QRY_E_TYPEMISMATCH: i32 = -2147219710i32;
pub const QRY_E_UNEXPECTED: i32 = -2147219685i32;
pub const QRY_E_UNHANDLEDTYPE: i32 = -2147219709i32;
pub const QRY_E_WILDCARDPREFIXLENGTH: i32 = -2147219695i32;
pub const QRY_S_INEXACTRESULTS: i32 = 263958i32;
pub const QRY_S_NOROWSFOUND: i32 = 263940i32;
pub const QRY_S_TERMIGNORED: i32 = 263954i32;
pub const QUERY_E_AGGREGATE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80041619_u32 as _);
pub const QUERY_E_ALLNOISE_AND_NO_RELDOC: windows_core::HRESULT = windows_core::HRESULT(0x8004160D_u32 as _);
pub const QUERY_E_ALLNOISE_AND_NO_RELPROP: windows_core::HRESULT = windows_core::HRESULT(0x8004160F_u32 as _);
pub const QUERY_E_DUPLICATE_RANGE_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8004161B_u32 as _);
pub const QUERY_E_INCORRECT_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80041614_u32 as _);
pub const QUERY_E_INVALIDCOALESCE: windows_core::HRESULT = windows_core::HRESULT(0x80041617_u32 as _);
pub const QUERY_E_INVALIDSCOPE_COALESCE: windows_core::HRESULT = windows_core::HRESULT(0x80041615_u32 as _);
pub const QUERY_E_INVALIDSORT_COALESCE: windows_core::HRESULT = windows_core::HRESULT(0x80041616_u32 as _);
pub const QUERY_E_INVALID_DOCUMENT_IDENTIFIER: windows_core::HRESULT = windows_core::HRESULT(0x80041613_u32 as _);
pub const QUERY_E_NO_RELDOC: windows_core::HRESULT = windows_core::HRESULT(0x8004160E_u32 as _);
pub const QUERY_E_NO_RELPROP: windows_core::HRESULT = windows_core::HRESULT(0x80041610_u32 as _);
pub const QUERY_E_RELDOC_SYNTAX_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80041612_u32 as _);
pub const QUERY_E_REPEATED_RELDOC: windows_core::HRESULT = windows_core::HRESULT(0x80041611_u32 as _);
pub const QUERY_E_TOP_LEVEL_IN_GROUP: windows_core::HRESULT = windows_core::HRESULT(0x8004161A_u32 as _);
pub const QUERY_E_UPGRADEINPROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80041618_u32 as _);
pub const QUERY_SORTDEFAULT: u32 = 4u32;
pub const QUERY_SORTXASCEND: u32 = 2u32;
pub const QUERY_SORTXDESCEND: u32 = 3u32;
pub const QUERY_VALIDBITS: u32 = 3u32;
pub const REXSPH_E_DUPLICATE_PROPERTY: i32 = -2147207927i32;
pub const REXSPH_E_INVALID_CALL: i32 = -2147207936i32;
pub const REXSPH_E_MULTIPLE_REDIRECT: i32 = -2147207933i32;
pub const REXSPH_E_NO_PROPERTY_ON_ROW: i32 = -2147207932i32;
pub const REXSPH_E_REDIRECT_ON_SECURITY_UPDATE: i32 = -2147207934i32;
pub const REXSPH_E_TYPE_MISMATCH_ON_READ: i32 = -2147207931i32;
pub const REXSPH_E_UNEXPECTED_DATA_STATUS: i32 = -2147207930i32;
pub const REXSPH_E_UNEXPECTED_FILTER_STATE: i32 = -2147207928i32;
pub const REXSPH_E_UNKNOWN_DATA_TYPE: i32 = -2147207929i32;
pub const REXSPH_S_REDIRECTED: i32 = 275713i32;
pub const ROWSETEVENT_ITEMSTATE_INROWSET: ROWSETEVENT_ITEMSTATE = ROWSETEVENT_ITEMSTATE(1i32);
pub const ROWSETEVENT_ITEMSTATE_NOTINROWSET: ROWSETEVENT_ITEMSTATE = ROWSETEVENT_ITEMSTATE(0i32);
pub const ROWSETEVENT_ITEMSTATE_UNKNOWN: ROWSETEVENT_ITEMSTATE = ROWSETEVENT_ITEMSTATE(2i32);
pub const ROWSETEVENT_TYPE_DATAEXPIRED: ROWSETEVENT_TYPE = ROWSETEVENT_TYPE(0i32);
pub const ROWSETEVENT_TYPE_FOREGROUNDLOST: ROWSETEVENT_TYPE = ROWSETEVENT_TYPE(1i32);
pub const ROWSETEVENT_TYPE_SCOPESTATISTICS: ROWSETEVENT_TYPE = ROWSETEVENT_TYPE(2i32);
pub const RS_COMPLETED: u32 = 2147483648u32;
pub const RS_MAYBOTHERUSER: u32 = 131072u32;
pub const RS_READY: u32 = 1u32;
pub const RS_SUSPENDED: u32 = 2u32;
pub const RS_SUSPENDONIDLE: u32 = 65536u32;
pub const RS_UPDATING: u32 = 4u32;
pub const RTAnd: u32 = 1u32;
pub const RTContent: u32 = 4u32;
pub const RTNatLanguage: u32 = 8u32;
pub const RTNone: u32 = 0u32;
pub const RTNot: u32 = 3u32;
pub const RTOr: u32 = 2u32;
pub const RTProperty: u32 = 5u32;
pub const RTProximity: u32 = 6u32;
pub const RTVector: u32 = 7u32;
pub const SCHEMA_E_ADDSTOPWORDS: i32 = -2147218420i32;
pub const SCHEMA_E_BADATTRIBUTE: i32 = -2147218412i32;
pub const SCHEMA_E_BADCOLUMNNAME: i32 = -2147218414i32;
pub const SCHEMA_E_BADFILENAME: i32 = -2147218411i32;
pub const SCHEMA_E_BADPROPPID: i32 = -2147218413i32;
pub const SCHEMA_E_BADPROPSPEC: i32 = -2147218417i32;
pub const SCHEMA_E_CANNOTCREATEFILE: i32 = -2147218426i32;
pub const SCHEMA_E_CANNOTCREATENOISEWORDFILE: i32 = -2147218421i32;
pub const SCHEMA_E_CANNOTWRITEFILE: i32 = -2147218425i32;
pub const SCHEMA_E_DUPLICATENOISE: i32 = -2147218409i32;
pub const SCHEMA_E_EMPTYFILE: i32 = -2147218424i32;
pub const SCHEMA_E_FILECHANGED: i32 = -2147218415i32;
pub const SCHEMA_E_FILENOTFOUND: i32 = -2147218430i32;
pub const SCHEMA_E_INVALIDDATATYPE: i32 = -2147218422i32;
pub const SCHEMA_E_INVALIDFILETYPE: i32 = -2147218423i32;
pub const SCHEMA_E_INVALIDVALUE: i32 = -2147218418i32;
pub const SCHEMA_E_LOAD_SPECIAL: i32 = -2147218431i32;
pub const SCHEMA_E_NAMEEXISTS: i32 = -2147218419i32;
pub const SCHEMA_E_NESTEDTAG: i32 = -2147218429i32;
pub const SCHEMA_E_NOMORECOLUMNS: i32 = -2147218416i32;
pub const SCHEMA_E_PROPEXISTS: i32 = -2147218410i32;
pub const SCHEMA_E_UNEXPECTEDTAG: i32 = -2147218428i32;
pub const SCHEMA_E_VERSIONMISMATCH: i32 = -2147218427i32;
pub const SCRIPTPI_E_ALREADY_COMPLETED: i32 = -2147213307i32;
pub const SCRIPTPI_E_CANNOT_ALTER_CHUNK: i32 = -2147213308i32;
pub const SCRIPTPI_E_CHUNK_NOT_TEXT: i32 = -2147213312i32;
pub const SCRIPTPI_E_CHUNK_NOT_VALUE: i32 = -2147213309i32;
pub const SCRIPTPI_E_PID_NOT_NAME: i32 = -2147213311i32;
pub const SCRIPTPI_E_PID_NOT_NUMERIC: i32 = -2147213310i32;
pub const SEARCH_ADVANCED_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = SEARCH_QUERY_SYNTAX(1i32);
pub const SEARCH_CHANGE_ADD: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(0i32);
pub const SEARCH_CHANGE_DELETE: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(1i32);
pub const SEARCH_CHANGE_MODIFY: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(2i32);
pub const SEARCH_CHANGE_MOVE_RENAME: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(3i32);
pub const SEARCH_CHANGE_SEMANTICS_DIRECTORY: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(262144i32);
pub const SEARCH_CHANGE_SEMANTICS_SHALLOW: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(524288i32);
pub const SEARCH_CHANGE_SEMANTICS_UPDATE_SECURITY: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(4194304i32);
pub const SEARCH_HIGH_PRIORITY: SEARCH_NOTIFICATION_PRIORITY = SEARCH_NOTIFICATION_PRIORITY(1i32);
pub const SEARCH_INDEXING_PHASE_GATHERER: SEARCH_INDEXING_PHASE = SEARCH_INDEXING_PHASE(0i32);
pub const SEARCH_INDEXING_PHASE_PERSISTED: SEARCH_INDEXING_PHASE = SEARCH_INDEXING_PHASE(2i32);
pub const SEARCH_INDEXING_PHASE_QUERYABLE: SEARCH_INDEXING_PHASE = SEARCH_INDEXING_PHASE(1i32);
pub const SEARCH_NATURAL_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = SEARCH_QUERY_SYNTAX(2i32);
pub const SEARCH_NORMAL_PRIORITY: SEARCH_NOTIFICATION_PRIORITY = SEARCH_NOTIFICATION_PRIORITY(0i32);
pub const SEARCH_NO_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = SEARCH_QUERY_SYNTAX(0i32);
pub const SEARCH_TERM_NO_EXPANSION: SEARCH_TERM_EXPANSION = SEARCH_TERM_EXPANSION(0i32);
pub const SEARCH_TERM_PREFIX_ALL: SEARCH_TERM_EXPANSION = SEARCH_TERM_EXPANSION(1i32);
pub const SEARCH_TERM_STEM_ALL: SEARCH_TERM_EXPANSION = SEARCH_TERM_EXPANSION(2i32);
pub const SEC_E_ACCESSDENIED: i32 = -2147216129i32;
pub const SEC_E_BADTRUSTEEID: windows_core::HRESULT = windows_core::HRESULT(0x80040E6A_u32 as _);
pub const SEC_E_INITFAILED: i32 = -2147216383i32;
pub const SEC_E_INVALIDACCESSENTRY: windows_core::HRESULT = windows_core::HRESULT(0x80040E71_u32 as _);
pub const SEC_E_INVALIDACCESSENTRYLIST: windows_core::HRESULT = windows_core::HRESULT(0x80040E6F_u32 as _);
pub const SEC_E_INVALIDCONTEXT: i32 = -2147216381i32;
pub const SEC_E_INVALIDOBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80040E6D_u32 as _);
pub const SEC_E_INVALIDOWNER: windows_core::HRESULT = windows_core::HRESULT(0x80040E70_u32 as _);
pub const SEC_E_NOMEMBERSHIPSUPPORT: windows_core::HRESULT = windows_core::HRESULT(0x80040E6C_u32 as _);
pub const SEC_E_NOOWNER: windows_core::HRESULT = windows_core::HRESULT(0x80040E6E_u32 as _);
pub const SEC_E_NOTINITIALIZED: i32 = -2147216382i32;
pub const SEC_E_NOTRUSTEEID: windows_core::HRESULT = windows_core::HRESULT(0x80040E6B_u32 as _);
pub const SEC_E_PERMISSIONDENIED: i32 = -2147217911i32;
pub const SI_TEMPORARY: u32 = 2147483648u32;
pub const SPS_WS_ERROR: i32 = -2147211753i32;
pub const SQLAOPANY: u32 = 83u32;
pub const SQLAOPAVG: u32 = 79u32;
pub const SQLAOPCNT: u32 = 75u32;
pub const SQLAOPMAX: u32 = 82u32;
pub const SQLAOPMIN: u32 = 81u32;
pub const SQLAOPNOOP: u32 = 86u32;
pub const SQLAOPSTDEV: u32 = 48u32;
pub const SQLAOPSTDEVP: u32 = 49u32;
pub const SQLAOPSUM: u32 = 77u32;
pub const SQLAOPVAR: u32 = 50u32;
pub const SQLAOPVARP: u32 = 51u32;
pub const SQLBIGBINARY: u32 = 173u32;
pub const SQLBIGCHAR: u32 = 175u32;
pub const SQLBIGVARBINARY: u32 = 165u32;
pub const SQLBIGVARCHAR: u32 = 167u32;
pub const SQLBINARY: u32 = 45u32;
pub const SQLBIT: u32 = 50u32;
pub const SQLBITN: u32 = 104u32;
pub const SQLCHARACTER: u32 = 47u32;
pub const SQLDATETIM4: u32 = 58u32;
pub const SQLDATETIME: u32 = 61u32;
pub const SQLDATETIMN: u32 = 111u32;
pub const SQLDECIMAL: u32 = 106u32;
pub const SQLDECIMALN: u32 = 106u32;
pub const SQLFLT4: u32 = 59u32;
pub const SQLFLT8: u32 = 62u32;
pub const SQLFLTN: u32 = 109u32;
pub const SQLIMAGE: u32 = 34u32;
pub const SQLINT1: u32 = 48u32;
pub const SQLINT2: u32 = 52u32;
pub const SQLINT4: u32 = 56u32;
pub const SQLINT8: u32 = 127u32;
pub const SQLINTN: u32 = 38u32;
pub const SQLMONEY: u32 = 60u32;
pub const SQLMONEY4: u32 = 122u32;
pub const SQLMONEYN: u32 = 110u32;
pub const SQLNCHAR: u32 = 239u32;
pub const SQLNTEXT: u32 = 99u32;
pub const SQLNUMERIC: u32 = 108u32;
pub const SQLNUMERICN: u32 = 108u32;
pub const SQLNVARCHAR: u32 = 231u32;
pub const SQLTEXT: u32 = 35u32;
pub const SQLUNIQUEID: u32 = 36u32;
pub const SQLVARBINARY: u32 = 37u32;
pub const SQLVARCHAR: u32 = 39u32;
pub const SQLVARIANT: u32 = 98u32;
pub const SQL_AA_FALSE: i32 = 0i32;
pub const SQL_AA_TRUE: i32 = 1i32;
pub const SQL_ACCESSIBLE_PROCEDURES: u32 = 20u32;
pub const SQL_ACCESSIBLE_TABLES: u32 = 19u32;
pub const SQL_ACCESS_MODE: u32 = 101u32;
pub const SQL_ACTIVE_CONNECTIONS: u32 = 0u32;
pub const SQL_ACTIVE_ENVIRONMENTS: u32 = 116u32;
pub const SQL_ACTIVE_STATEMENTS: u32 = 1u32;
pub const SQL_ADD: u32 = 4u32;
pub const SQL_AD_ADD_CONSTRAINT_DEFERRABLE: i32 = 128i32;
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32i32;
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64i32;
pub const SQL_AD_ADD_CONSTRAINT_NON_DEFERRABLE: i32 = 256i32;
pub const SQL_AD_ADD_DOMAIN_CONSTRAINT: i32 = 2i32;
pub const SQL_AD_ADD_DOMAIN_DEFAULT: i32 = 8i32;
pub const SQL_AD_CONSTRAINT_NAME_DEFINITION: i32 = 1i32;
pub const SQL_AD_DEFAULT: i32 = 1i32;
pub const SQL_AD_DROP_DOMAIN_CONSTRAINT: i32 = 4i32;
pub const SQL_AD_DROP_DOMAIN_DEFAULT: i32 = 16i32;
pub const SQL_AD_OFF: i32 = 0i32;
pub const SQL_AD_ON: i32 = 1i32;
pub const SQL_AF_ALL: i32 = 64i32;
pub const SQL_AF_AVG: i32 = 1i32;
pub const SQL_AF_COUNT: i32 = 2i32;
pub const SQL_AF_DISTINCT: i32 = 32i32;
pub const SQL_AF_MAX: i32 = 4i32;
pub const SQL_AF_MIN: i32 = 8i32;
pub const SQL_AF_SUM: i32 = 16i32;
pub const SQL_AGGREGATE_FUNCTIONS: u32 = 169u32;
pub const SQL_ALL_CATALOGS: windows_core::PCSTR = windows_core::s!("%");
pub const SQL_ALL_EXCEPT_LIKE: u32 = 2u32;
pub const SQL_ALL_SCHEMAS: windows_core::PCSTR = windows_core::s!("%");
pub const SQL_ALL_TABLE_TYPES: windows_core::PCSTR = windows_core::s!("%");
pub const SQL_ALL_TYPES: u32 = 0u32;
pub const SQL_ALTER_DOMAIN: u32 = 117u32;
pub const SQL_ALTER_TABLE: u32 = 86u32;
pub const SQL_AM_CONNECTION: u32 = 1u32;
pub const SQL_AM_NONE: u32 = 0u32;
pub const SQL_AM_STATEMENT: u32 = 2u32;
pub const SQL_AO_DEFAULT: i32 = 0i32;
pub const SQL_AO_OFF: i32 = 0i32;
pub const SQL_AO_ON: i32 = 1i32;
pub const SQL_APD_TYPE: i32 = -100i32;
pub const SQL_API_ALL_FUNCTIONS: u32 = 0u32;
pub const SQL_API_LOADBYORDINAL: u32 = 199u32;
pub const SQL_API_ODBC3_ALL_FUNCTIONS: u32 = 999u32;
pub const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: u32 = 250u32;
pub const SQL_API_SQLALLOCCONNECT: u32 = 1u32;
pub const SQL_API_SQLALLOCENV: u32 = 2u32;
pub const SQL_API_SQLALLOCHANDLE: u32 = 1001u32;
pub const SQL_API_SQLALLOCHANDLESTD: u32 = 73u32;
pub const SQL_API_SQLALLOCSTMT: u32 = 3u32;
pub const SQL_API_SQLBINDCOL: u32 = 4u32;
pub const SQL_API_SQLBINDPARAM: u32 = 1002u32;
pub const SQL_API_SQLBINDPARAMETER: u32 = 72u32;
pub const SQL_API_SQLBROWSECONNECT: u32 = 55u32;
pub const SQL_API_SQLBULKOPERATIONS: u32 = 24u32;
pub const SQL_API_SQLCANCEL: u32 = 5u32;
pub const SQL_API_SQLCANCELHANDLE: u32 = 1550u32;
pub const SQL_API_SQLCLOSECURSOR: u32 = 1003u32;
pub const SQL_API_SQLCOLATTRIBUTE: u32 = 6u32;
pub const SQL_API_SQLCOLATTRIBUTES: u32 = 6u32;
pub const SQL_API_SQLCOLUMNPRIVILEGES: u32 = 56u32;
pub const SQL_API_SQLCOLUMNS: u32 = 40u32;
pub const SQL_API_SQLCOMPLETEASYNC: u32 = 1551u32;
pub const SQL_API_SQLCONNECT: u32 = 7u32;
pub const SQL_API_SQLCOPYDESC: u32 = 1004u32;
pub const SQL_API_SQLDATASOURCES: u32 = 57u32;
pub const SQL_API_SQLDESCRIBECOL: u32 = 8u32;
pub const SQL_API_SQLDESCRIBEPARAM: u32 = 58u32;
pub const SQL_API_SQLDISCONNECT: u32 = 9u32;
pub const SQL_API_SQLDRIVERCONNECT: u32 = 41u32;
pub const SQL_API_SQLDRIVERS: u32 = 71u32;
pub const SQL_API_SQLENDTRAN: u32 = 1005u32;
pub const SQL_API_SQLERROR: u32 = 10u32;
pub const SQL_API_SQLEXECDIRECT: u32 = 11u32;
pub const SQL_API_SQLEXECUTE: u32 = 12u32;
pub const SQL_API_SQLEXTENDEDFETCH: u32 = 59u32;
pub const SQL_API_SQLFETCH: u32 = 13u32;
pub const SQL_API_SQLFETCHSCROLL: u32 = 1021u32;
pub const SQL_API_SQLFOREIGNKEYS: u32 = 60u32;
pub const SQL_API_SQLFREECONNECT: u32 = 14u32;
pub const SQL_API_SQLFREEENV: u32 = 15u32;
pub const SQL_API_SQLFREEHANDLE: u32 = 1006u32;
pub const SQL_API_SQLFREESTMT: u32 = 16u32;
pub const SQL_API_SQLGETCONNECTATTR: u32 = 1007u32;
pub const SQL_API_SQLGETCONNECTOPTION: u32 = 42u32;
pub const SQL_API_SQLGETCURSORNAME: u32 = 17u32;
pub const SQL_API_SQLGETDATA: u32 = 43u32;
pub const SQL_API_SQLGETDESCFIELD: u32 = 1008u32;
pub const SQL_API_SQLGETDESCREC: u32 = 1009u32;
pub const SQL_API_SQLGETDIAGFIELD: u32 = 1010u32;
pub const SQL_API_SQLGETDIAGREC: u32 = 1011u32;
pub const SQL_API_SQLGETENVATTR: u32 = 1012u32;
pub const SQL_API_SQLGETFUNCTIONS: u32 = 44u32;
pub const SQL_API_SQLGETINFO: u32 = 45u32;
pub const SQL_API_SQLGETSTMTATTR: u32 = 1014u32;
pub const SQL_API_SQLGETSTMTOPTION: u32 = 46u32;
pub const SQL_API_SQLGETTYPEINFO: u32 = 47u32;
pub const SQL_API_SQLMORERESULTS: u32 = 61u32;
pub const SQL_API_SQLNATIVESQL: u32 = 62u32;
pub const SQL_API_SQLNUMPARAMS: u32 = 63u32;
pub const SQL_API_SQLNUMRESULTCOLS: u32 = 18u32;
pub const SQL_API_SQLPARAMDATA: u32 = 48u32;
pub const SQL_API_SQLPARAMOPTIONS: u32 = 64u32;
pub const SQL_API_SQLPREPARE: u32 = 19u32;
pub const SQL_API_SQLPRIMARYKEYS: u32 = 65u32;
pub const SQL_API_SQLPRIVATEDRIVERS: u32 = 79u32;
pub const SQL_API_SQLPROCEDURECOLUMNS: u32 = 66u32;
pub const SQL_API_SQLPROCEDURES: u32 = 67u32;
pub const SQL_API_SQLPUTDATA: u32 = 49u32;
pub const SQL_API_SQLROWCOUNT: u32 = 20u32;
pub const SQL_API_SQLSETCONNECTATTR: u32 = 1016u32;
pub const SQL_API_SQLSETCONNECTOPTION: u32 = 50u32;
pub const SQL_API_SQLSETCURSORNAME: u32 = 21u32;
pub const SQL_API_SQLSETDESCFIELD: u32 = 1017u32;
pub const SQL_API_SQLSETDESCREC: u32 = 1018u32;
pub const SQL_API_SQLSETENVATTR: u32 = 1019u32;
pub const SQL_API_SQLSETPARAM: u32 = 22u32;
pub const SQL_API_SQLSETPOS: u32 = 68u32;
pub const SQL_API_SQLSETSCROLLOPTIONS: u32 = 69u32;
pub const SQL_API_SQLSETSTMTATTR: u32 = 1020u32;
pub const SQL_API_SQLSETSTMTOPTION: u32 = 51u32;
pub const SQL_API_SQLSPECIALCOLUMNS: u32 = 52u32;
pub const SQL_API_SQLSTATISTICS: u32 = 53u32;
pub const SQL_API_SQLTABLEPRIVILEGES: u32 = 70u32;
pub const SQL_API_SQLTABLES: u32 = 54u32;
pub const SQL_API_SQLTRANSACT: u32 = 23u32;
pub const SQL_ARD_TYPE: i32 = -99i32;
pub const SQL_ASYNC_DBC_CAPABLE: i32 = 1i32;
pub const SQL_ASYNC_DBC_ENABLE_DEFAULT: u32 = 0u32;
pub const SQL_ASYNC_DBC_ENABLE_OFF: u32 = 0u32;
pub const SQL_ASYNC_DBC_ENABLE_ON: u32 = 1u32;
pub const SQL_ASYNC_DBC_FUNCTIONS: u32 = 10023u32;
pub const SQL_ASYNC_DBC_NOT_CAPABLE: i32 = 0i32;
pub const SQL_ASYNC_ENABLE: u32 = 4u32;
pub const SQL_ASYNC_ENABLE_DEFAULT: u32 = 0u32;
pub const SQL_ASYNC_ENABLE_OFF: u32 = 0u32;
pub const SQL_ASYNC_ENABLE_ON: u32 = 1u32;
pub const SQL_ASYNC_MODE: u32 = 10021u32;
pub const SQL_ASYNC_NOTIFICATION: u32 = 10025u32;
pub const SQL_ASYNC_NOTIFICATION_CAPABLE: i32 = 1i32;
pub const SQL_ASYNC_NOTIFICATION_NOT_CAPABLE: i32 = 0i32;
pub const SQL_ATTR_ACCESS_MODE: u32 = 101u32;
pub const SQL_ATTR_ANSI_APP: u32 = 115u32;
pub const SQL_ATTR_APPLICATION_KEY: u32 = 203u32;
pub const SQL_ATTR_APP_PARAM_DESC: u32 = 10011u32;
pub const SQL_ATTR_APP_ROW_DESC: u32 = 10010u32;
pub const SQL_ATTR_ASYNC_DBC_EVENT: u32 = 119u32;
pub const SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE: u32 = 117u32;
pub const SQL_ATTR_ASYNC_DBC_NOTIFICATION_CALLBACK: u32 = 120u32;
pub const SQL_ATTR_ASYNC_DBC_NOTIFICATION_CONTEXT: u32 = 121u32;
pub const SQL_ATTR_ASYNC_ENABLE: u32 = 4u32;
pub const SQL_ATTR_ASYNC_STMT_EVENT: u32 = 29u32;
pub const SQL_ATTR_ASYNC_STMT_NOTIFICATION_CALLBACK: u32 = 30u32;
pub const SQL_ATTR_ASYNC_STMT_NOTIFICATION_CONTEXT: u32 = 31u32;
pub const SQL_ATTR_AUTOCOMMIT: u32 = 102u32;
pub const SQL_ATTR_AUTO_IPD: u32 = 10001u32;
pub const SQL_ATTR_CONCURRENCY: u32 = 7u32;
pub const SQL_ATTR_CONNECTION_DEAD: u32 = 1209u32;
pub const SQL_ATTR_CONNECTION_POOLING: u32 = 201u32;
pub const SQL_ATTR_CONNECTION_TIMEOUT: u32 = 113u32;
pub const SQL_ATTR_CP_MATCH: u32 = 202u32;
pub const SQL_ATTR_CURRENT_CATALOG: u32 = 109u32;
pub const SQL_ATTR_CURSOR_SCROLLABLE: i32 = -1i32;
pub const SQL_ATTR_CURSOR_SENSITIVITY: i32 = -2i32;
pub const SQL_ATTR_CURSOR_TYPE: u32 = 6u32;
pub const SQL_ATTR_DBC_INFO_TOKEN: u32 = 118u32;
pub const SQL_ATTR_DISCONNECT_BEHAVIOR: u32 = 114u32;
pub const SQL_ATTR_ENABLE_AUTO_IPD: u32 = 15u32;
pub const SQL_ATTR_ENLIST_IN_DTC: u32 = 1207u32;
pub const SQL_ATTR_ENLIST_IN_XA: u32 = 1208u32;
pub const SQL_ATTR_FETCH_BOOKMARK_PTR: u32 = 16u32;
pub const SQL_ATTR_IMP_PARAM_DESC: u32 = 10013u32;
pub const SQL_ATTR_IMP_ROW_DESC: u32 = 10012u32;
pub const SQL_ATTR_KEYSET_SIZE: u32 = 8u32;
pub const SQL_ATTR_LOGIN_TIMEOUT: u32 = 103u32;
pub const SQL_ATTR_MAX_LENGTH: u32 = 3u32;
pub const SQL_ATTR_MAX_ROWS: u32 = 1u32;
pub const SQL_ATTR_METADATA_ID: u32 = 10014u32;
pub const SQL_ATTR_NOSCAN: u32 = 2u32;
pub const SQL_ATTR_ODBC_CURSORS: u32 = 110u32;
pub const SQL_ATTR_ODBC_VERSION: u32 = 200u32;
pub const SQL_ATTR_OUTPUT_NTS: u32 = 10001u32;
pub const SQL_ATTR_PACKET_SIZE: u32 = 112u32;
pub const SQL_ATTR_PARAMSET_SIZE: u32 = 22u32;
pub const SQL_ATTR_PARAMS_PROCESSED_PTR: u32 = 21u32;
pub const SQL_ATTR_PARAM_BIND_OFFSET_PTR: u32 = 17u32;
pub const SQL_ATTR_PARAM_BIND_TYPE: u32 = 18u32;
pub const SQL_ATTR_PARAM_OPERATION_PTR: u32 = 19u32;
pub const SQL_ATTR_PARAM_STATUS_PTR: u32 = 20u32;
pub const SQL_ATTR_QUERY_TIMEOUT: u32 = 0u32;
pub const SQL_ATTR_QUIET_MODE: u32 = 111u32;
pub const SQL_ATTR_READONLY: u32 = 0u32;
pub const SQL_ATTR_READWRITE_UNKNOWN: u32 = 2u32;
pub const SQL_ATTR_RESET_CONNECTION: u32 = 116u32;
pub const SQL_ATTR_RETRIEVE_DATA: u32 = 11u32;
pub const SQL_ATTR_ROWS_FETCHED_PTR: u32 = 26u32;
pub const SQL_ATTR_ROW_ARRAY_SIZE: u32 = 27u32;
pub const SQL_ATTR_ROW_BIND_OFFSET_PTR: u32 = 23u32;
pub const SQL_ATTR_ROW_BIND_TYPE: u32 = 5u32;
pub const SQL_ATTR_ROW_NUMBER: u32 = 14u32;
pub const SQL_ATTR_ROW_OPERATION_PTR: u32 = 24u32;
pub const SQL_ATTR_ROW_STATUS_PTR: u32 = 25u32;
pub const SQL_ATTR_SIMULATE_CURSOR: u32 = 10u32;
pub const SQL_ATTR_TRACE: u32 = 104u32;
pub const SQL_ATTR_TRACEFILE: u32 = 105u32;
pub const SQL_ATTR_TRANSLATE_LIB: u32 = 106u32;
pub const SQL_ATTR_TRANSLATE_OPTION: u32 = 107u32;
pub const SQL_ATTR_TXN_ISOLATION: u32 = 108u32;
pub const SQL_ATTR_USE_BOOKMARKS: u32 = 12u32;
pub const SQL_ATTR_WRITE: u32 = 1u32;
pub const SQL_AT_ADD_COLUMN: i32 = 1i32;
pub const SQL_AT_ADD_COLUMN_COLLATION: i32 = 128i32;
pub const SQL_AT_ADD_COLUMN_DEFAULT: i32 = 64i32;
pub const SQL_AT_ADD_COLUMN_SINGLE: i32 = 32i32;
pub const SQL_AT_ADD_CONSTRAINT: i32 = 8i32;
pub const SQL_AT_ADD_TABLE_CONSTRAINT: i32 = 4096i32;
pub const SQL_AT_CONSTRAINT_DEFERRABLE: i32 = 262144i32;
pub const SQL_AT_CONSTRAINT_INITIALLY_DEFERRED: i32 = 65536i32;
pub const SQL_AT_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 131072i32;
pub const SQL_AT_CONSTRAINT_NAME_DEFINITION: i32 = 32768i32;
pub const SQL_AT_CONSTRAINT_NON_DEFERRABLE: i32 = 524288i32;
pub const SQL_AT_DROP_COLUMN: i32 = 2i32;
pub const SQL_AT_DROP_COLUMN_CASCADE: i32 = 1024i32;
pub const SQL_AT_DROP_COLUMN_DEFAULT: i32 = 512i32;
pub const SQL_AT_DROP_COLUMN_RESTRICT: i32 = 2048i32;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_CASCADE: i32 = 8192i32;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_RESTRICT: i32 = 16384i32;
pub const SQL_AT_SET_COLUMN_DEFAULT: i32 = 256i32;
pub const SQL_AUTOCOMMIT: u32 = 102u32;
pub const SQL_AUTOCOMMIT_DEFAULT: u32 = 1u32;
pub const SQL_AUTOCOMMIT_OFF: u32 = 0u32;
pub const SQL_AUTOCOMMIT_ON: u32 = 1u32;
pub const SQL_BATCH_ROW_COUNT: u32 = 120u32;
pub const SQL_BATCH_SUPPORT: u32 = 121u32;
pub const SQL_BCP_DEFAULT: i32 = 0i32;
pub const SQL_BCP_OFF: i32 = 0i32;
pub const SQL_BCP_ON: i32 = 1i32;
pub const SQL_BEST_ROWID: u32 = 1u32;
pub const SQL_BIGINT: i32 = -5i32;
pub const SQL_BINARY: i32 = -2i32;
pub const SQL_BIND_BY_COLUMN: u32 = 0u32;
pub const SQL_BIND_TYPE: u32 = 5u32;
pub const SQL_BIND_TYPE_DEFAULT: u32 = 0u32;
pub const SQL_BIT: i32 = -7i32;
pub const SQL_BOOKMARK_PERSISTENCE: u32 = 82u32;
pub const SQL_BP_CLOSE: i32 = 1i32;
pub const SQL_BP_DELETE: i32 = 2i32;
pub const SQL_BP_DROP: i32 = 4i32;
pub const SQL_BP_OTHER_HSTMT: i32 = 32i32;
pub const SQL_BP_SCROLL: i32 = 64i32;
pub const SQL_BP_TRANSACTION: i32 = 8i32;
pub const SQL_BP_UPDATE: i32 = 16i32;
pub const SQL_BRC_EXPLICIT: u32 = 2u32;
pub const SQL_BRC_PROCEDURES: u32 = 1u32;
pub const SQL_BRC_ROLLED_UP: u32 = 4u32;
pub const SQL_BS_ROW_COUNT_EXPLICIT: i32 = 2i32;
pub const SQL_BS_ROW_COUNT_PROC: i32 = 8i32;
pub const SQL_BS_SELECT_EXPLICIT: i32 = 1i32;
pub const SQL_BS_SELECT_PROC: i32 = 4i32;
pub const SQL_CA1_ABSOLUTE: i32 = 2i32;
pub const SQL_CA1_BOOKMARK: i32 = 8i32;
pub const SQL_CA1_BULK_ADD: i32 = 65536i32;
pub const SQL_CA1_BULK_DELETE_BY_BOOKMARK: i32 = 262144i32;
pub const SQL_CA1_BULK_FETCH_BY_BOOKMARK: i32 = 524288i32;
pub const SQL_CA1_BULK_UPDATE_BY_BOOKMARK: i32 = 131072i32;
pub const SQL_CA1_LOCK_EXCLUSIVE: i32 = 128i32;
pub const SQL_CA1_LOCK_NO_CHANGE: i32 = 64i32;
pub const SQL_CA1_LOCK_UNLOCK: i32 = 256i32;
pub const SQL_CA1_NEXT: i32 = 1i32;
pub const SQL_CA1_POSITIONED_DELETE: i32 = 16384i32;
pub const SQL_CA1_POSITIONED_UPDATE: i32 = 8192i32;
pub const SQL_CA1_POS_DELETE: i32 = 2048i32;
pub const SQL_CA1_POS_POSITION: i32 = 512i32;
pub const SQL_CA1_POS_REFRESH: i32 = 4096i32;
pub const SQL_CA1_POS_UPDATE: i32 = 1024i32;
pub const SQL_CA1_RELATIVE: i32 = 4i32;
pub const SQL_CA1_SELECT_FOR_UPDATE: i32 = 32768i32;
pub const SQL_CA2_CRC_APPROXIMATE: i32 = 8192i32;
pub const SQL_CA2_CRC_EXACT: i32 = 4096i32;
pub const SQL_CA2_LOCK_CONCURRENCY: i32 = 2i32;
pub const SQL_CA2_MAX_ROWS_CATALOG: i32 = 2048i32;
pub const SQL_CA2_MAX_ROWS_DELETE: i32 = 512i32;
pub const SQL_CA2_MAX_ROWS_INSERT: i32 = 256i32;
pub const SQL_CA2_MAX_ROWS_SELECT: i32 = 128i32;
pub const SQL_CA2_MAX_ROWS_UPDATE: i32 = 1024i32;
pub const SQL_CA2_OPT_ROWVER_CONCURRENCY: i32 = 4i32;
pub const SQL_CA2_OPT_VALUES_CONCURRENCY: i32 = 8i32;
pub const SQL_CA2_READ_ONLY_CONCURRENCY: i32 = 1i32;
pub const SQL_CA2_SENSITIVITY_ADDITIONS: i32 = 16i32;
pub const SQL_CA2_SENSITIVITY_DELETIONS: i32 = 32i32;
pub const SQL_CA2_SENSITIVITY_UPDATES: i32 = 64i32;
pub const SQL_CA2_SIMULATE_NON_UNIQUE: i32 = 16384i32;
pub const SQL_CA2_SIMULATE_TRY_UNIQUE: i32 = 32768i32;
pub const SQL_CA2_SIMULATE_UNIQUE: i32 = 65536i32;
pub const SQL_CACHE_DATA_NO: i32 = 0i32;
pub const SQL_CACHE_DATA_YES: i32 = 1i32;
pub const SQL_CASCADE: u32 = 0u32;
pub const SQL_CATALOG_LOCATION: u32 = 114u32;
pub const SQL_CATALOG_NAME: u32 = 10003u32;
pub const SQL_CATALOG_NAME_SEPARATOR: u32 = 41u32;
pub const SQL_CATALOG_TERM: u32 = 42u32;
pub const SQL_CATALOG_USAGE: u32 = 92u32;
pub const SQL_CA_CONSTRAINT_DEFERRABLE: i32 = 64i32;
pub const SQL_CA_CONSTRAINT_INITIALLY_DEFERRED: i32 = 16i32;
pub const SQL_CA_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 32i32;
pub const SQL_CA_CONSTRAINT_NON_DEFERRABLE: i32 = 128i32;
pub const SQL_CA_CREATE_ASSERTION: i32 = 1i32;
pub const SQL_CA_SS_BASE: u32 = 1200u32;
pub const SQL_CA_SS_COLUMN_COLLATION: u32 = 1214u32;
pub const SQL_CA_SS_COLUMN_HIDDEN: u32 = 1211u32;
pub const SQL_CA_SS_COLUMN_ID: u32 = 1208u32;
pub const SQL_CA_SS_COLUMN_KEY: u32 = 1212u32;
pub const SQL_CA_SS_COLUMN_OP: u32 = 1209u32;
pub const SQL_CA_SS_COLUMN_ORDER: u32 = 1203u32;
pub const SQL_CA_SS_COLUMN_SIZE: u32 = 1210u32;
pub const SQL_CA_SS_COLUMN_SSTYPE: u32 = 1200u32;
pub const SQL_CA_SS_COLUMN_UTYPE: u32 = 1201u32;
pub const SQL_CA_SS_COLUMN_VARYLEN: u32 = 1204u32;
pub const SQL_CA_SS_COMPUTE_BYLIST: u32 = 1207u32;
pub const SQL_CA_SS_COMPUTE_ID: u32 = 1206u32;
pub const SQL_CA_SS_MAX_USED: u32 = 1218u32;
pub const SQL_CA_SS_NUM_COMPUTES: u32 = 1205u32;
pub const SQL_CA_SS_NUM_ORDERS: u32 = 1202u32;
pub const SQL_CA_SS_VARIANT_SERVER_TYPE: u32 = 1217u32;
pub const SQL_CA_SS_VARIANT_SQL_TYPE: u32 = 1216u32;
pub const SQL_CA_SS_VARIANT_TYPE: u32 = 1215u32;
pub const SQL_CB_CLOSE: u32 = 1u32;
pub const SQL_CB_DELETE: u32 = 0u32;
pub const SQL_CB_NON_NULL: u32 = 1u32;
pub const SQL_CB_NULL: u32 = 0u32;
pub const SQL_CB_PRESERVE: u32 = 2u32;
pub const SQL_CCOL_CREATE_COLLATION: i32 = 1i32;
pub const SQL_CCS_COLLATE_CLAUSE: i32 = 2i32;
pub const SQL_CCS_CREATE_CHARACTER_SET: i32 = 1i32;
pub const SQL_CCS_LIMITED_COLLATION: i32 = 4i32;
pub const SQL_CC_CLOSE: u32 = 1u32;
pub const SQL_CC_DELETE: u32 = 0u32;
pub const SQL_CC_PRESERVE: u32 = 2u32;
pub const SQL_CDO_COLLATION: i32 = 8i32;
pub const SQL_CDO_CONSTRAINT: i32 = 4i32;
pub const SQL_CDO_CONSTRAINT_DEFERRABLE: i32 = 128i32;
pub const SQL_CDO_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32i32;
pub const SQL_CDO_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64i32;
pub const SQL_CDO_CONSTRAINT_NAME_DEFINITION: i32 = 16i32;
pub const SQL_CDO_CONSTRAINT_NON_DEFERRABLE: i32 = 256i32;
pub const SQL_CDO_CREATE_DOMAIN: i32 = 1i32;
pub const SQL_CDO_DEFAULT: i32 = 2i32;
pub const SQL_CD_FALSE: i32 = 0i32;
pub const SQL_CD_TRUE: i32 = 1i32;
pub const SQL_CHAR: u32 = 1u32;
pub const SQL_CLOSE: u32 = 0u32;
pub const SQL_CL_END: u32 = 2u32;
pub const SQL_CL_START: u32 = 1u32;
pub const SQL_CN_ANY: u32 = 2u32;
pub const SQL_CN_DEFAULT: i32 = 1i32;
pub const SQL_CN_DIFFERENT: u32 = 1u32;
pub const SQL_CN_NONE: u32 = 0u32;
pub const SQL_CN_OFF: i32 = 0i32;
pub const SQL_CN_ON: i32 = 1i32;
pub const SQL_CODE_DATE: u32 = 1u32;
pub const SQL_CODE_DAY: u32 = 3u32;
pub const SQL_CODE_DAY_TO_HOUR: u32 = 8u32;
pub const SQL_CODE_DAY_TO_MINUTE: u32 = 9u32;
pub const SQL_CODE_DAY_TO_SECOND: u32 = 10u32;
pub const SQL_CODE_HOUR: u32 = 4u32;
pub const SQL_CODE_HOUR_TO_MINUTE: u32 = 11u32;
pub const SQL_CODE_HOUR_TO_SECOND: u32 = 12u32;
pub const SQL_CODE_MINUTE: u32 = 5u32;
pub const SQL_CODE_MINUTE_TO_SECOND: u32 = 13u32;
pub const SQL_CODE_MONTH: u32 = 2u32;
pub const SQL_CODE_SECOND: u32 = 6u32;
pub const SQL_CODE_TIME: u32 = 2u32;
pub const SQL_CODE_TIMESTAMP: u32 = 3u32;
pub const SQL_CODE_YEAR: u32 = 1u32;
pub const SQL_CODE_YEAR_TO_MONTH: u32 = 7u32;
pub const SQL_COLATT_OPT_MAX: u32 = 18u32;
pub const SQL_COLATT_OPT_MIN: u32 = 0u32;
pub const SQL_COLLATION_SEQ: u32 = 10004u32;
pub const SQL_COLUMN_ALIAS: u32 = 87u32;
pub const SQL_COLUMN_AUTO_INCREMENT: u32 = 11u32;
pub const SQL_COLUMN_CASE_SENSITIVE: u32 = 12u32;
pub const SQL_COLUMN_COUNT: u32 = 0u32;
pub const SQL_COLUMN_DISPLAY_SIZE: u32 = 6u32;
pub const SQL_COLUMN_DRIVER_START: u32 = 1000u32;
pub const SQL_COLUMN_IGNORE: i32 = -6i32;
pub const SQL_COLUMN_LABEL: u32 = 18u32;
pub const SQL_COLUMN_LENGTH: u32 = 3u32;
pub const SQL_COLUMN_MONEY: u32 = 9u32;
pub const SQL_COLUMN_NAME: u32 = 1u32;
pub const SQL_COLUMN_NULLABLE: u32 = 7u32;
pub const SQL_COLUMN_NUMBER_UNKNOWN: i32 = -2i32;
pub const SQL_COLUMN_OWNER_NAME: u32 = 16u32;
pub const SQL_COLUMN_PRECISION: u32 = 4u32;
pub const SQL_COLUMN_QUALIFIER_NAME: u32 = 17u32;
pub const SQL_COLUMN_SCALE: u32 = 5u32;
pub const SQL_COLUMN_SEARCHABLE: u32 = 13u32;
pub const SQL_COLUMN_TABLE_NAME: u32 = 15u32;
pub const SQL_COLUMN_TYPE: u32 = 2u32;
pub const SQL_COLUMN_TYPE_NAME: u32 = 14u32;
pub const SQL_COLUMN_UNSIGNED: u32 = 8u32;
pub const SQL_COLUMN_UPDATABLE: u32 = 10u32;
pub const SQL_COMMIT: u32 = 0u32;
pub const SQL_CONCAT_NULL_BEHAVIOR: u32 = 22u32;
pub const SQL_CONCURRENCY: u32 = 7u32;
pub const SQL_CONCUR_DEFAULT: u32 = 1u32;
pub const SQL_CONCUR_LOCK: u32 = 2u32;
pub const SQL_CONCUR_READ_ONLY: u32 = 1u32;
pub const SQL_CONCUR_ROWVER: u32 = 3u32;
pub const SQL_CONCUR_TIMESTAMP: u32 = 3u32;
pub const SQL_CONCUR_VALUES: u32 = 4u32;
pub const SQL_CONNECT_OPT_DRVR_START: u32 = 1000u32;
pub const SQL_CONN_OPT_MAX: u32 = 112u32;
pub const SQL_CONN_OPT_MIN: u32 = 101u32;
pub const SQL_CONN_POOL_RATING_BEST: u32 = 100u32;
pub const SQL_CONN_POOL_RATING_GOOD_ENOUGH: u32 = 99u32;
pub const SQL_CONN_POOL_RATING_USELESS: u32 = 0u32;
pub const SQL_CONVERT_BIGINT: u32 = 53u32;
pub const SQL_CONVERT_BINARY: u32 = 54u32;
pub const SQL_CONVERT_BIT: u32 = 55u32;
pub const SQL_CONVERT_CHAR: u32 = 56u32;
pub const SQL_CONVERT_DATE: u32 = 57u32;
pub const SQL_CONVERT_DECIMAL: u32 = 58u32;
pub const SQL_CONVERT_DOUBLE: u32 = 59u32;
pub const SQL_CONVERT_FLOAT: u32 = 60u32;
pub const SQL_CONVERT_FUNCTIONS: u32 = 48u32;
pub const SQL_CONVERT_GUID: u32 = 173u32;
pub const SQL_CONVERT_INTEGER: u32 = 61u32;
pub const SQL_CONVERT_INTERVAL_DAY_TIME: u32 = 123u32;
pub const SQL_CONVERT_INTERVAL_YEAR_MONTH: u32 = 124u32;
pub const SQL_CONVERT_LONGVARBINARY: u32 = 71u32;
pub const SQL_CONVERT_LONGVARCHAR: u32 = 62u32;
pub const SQL_CONVERT_NUMERIC: u32 = 63u32;
pub const SQL_CONVERT_REAL: u32 = 64u32;
pub const SQL_CONVERT_SMALLINT: u32 = 65u32;
pub const SQL_CONVERT_TIME: u32 = 66u32;
pub const SQL_CONVERT_TIMESTAMP: u32 = 67u32;
pub const SQL_CONVERT_TINYINT: u32 = 68u32;
pub const SQL_CONVERT_VARBINARY: u32 = 69u32;
pub const SQL_CONVERT_VARCHAR: u32 = 70u32;
pub const SQL_CONVERT_WCHAR: u32 = 122u32;
pub const SQL_CONVERT_WLONGVARCHAR: u32 = 125u32;
pub const SQL_CONVERT_WVARCHAR: u32 = 126u32;
pub const SQL_COPT_SS_ANSI_NPW: u32 = 1218u32;
pub const SQL_COPT_SS_ANSI_OEM: u32 = 1206u32;
pub const SQL_COPT_SS_ATTACHDBFILENAME: u32 = 1221u32;
pub const SQL_COPT_SS_BASE: u32 = 1200u32;
pub const SQL_COPT_SS_BASE_EX: u32 = 1240u32;
pub const SQL_COPT_SS_BCP: u32 = 1219u32;
pub const SQL_COPT_SS_BROWSE_CACHE_DATA: u32 = 1245u32;
pub const SQL_COPT_SS_BROWSE_CONNECT: u32 = 1241u32;
pub const SQL_COPT_SS_BROWSE_SERVER: u32 = 1242u32;
pub const SQL_COPT_SS_CONCAT_NULL: u32 = 1222u32;
pub const SQL_COPT_SS_CONNECTION_DEAD: u32 = 1244u32;
pub const SQL_COPT_SS_ENCRYPT: u32 = 1223u32;
pub const SQL_COPT_SS_EX_MAX_USED: u32 = 1246u32;
pub const SQL_COPT_SS_FALLBACK_CONNECT: u32 = 1210u32;
pub const SQL_COPT_SS_INTEGRATED_SECURITY: u32 = 1203u32;
pub const SQL_COPT_SS_MAX_USED: u32 = 1223u32;
pub const SQL_COPT_SS_PERF_DATA: u32 = 1211u32;
pub const SQL_COPT_SS_PERF_DATA_LOG: u32 = 1212u32;
pub const SQL_COPT_SS_PERF_DATA_LOG_NOW: u32 = 1216u32;
pub const SQL_COPT_SS_PERF_QUERY: u32 = 1215u32;
pub const SQL_COPT_SS_PERF_QUERY_INTERVAL: u32 = 1213u32;
pub const SQL_COPT_SS_PERF_QUERY_LOG: u32 = 1214u32;
pub const SQL_COPT_SS_PRESERVE_CURSORS: u32 = 1204u32;
pub const SQL_COPT_SS_QUOTED_IDENT: u32 = 1217u32;
pub const SQL_COPT_SS_REMOTE_PWD: u32 = 1201u32;
pub const SQL_COPT_SS_RESET_CONNECTION: u32 = 1246u32;
pub const SQL_COPT_SS_TRANSLATE: u32 = 1220u32;
pub const SQL_COPT_SS_USER_DATA: u32 = 1205u32;
pub const SQL_COPT_SS_USE_PROC_FOR_PREP: u32 = 1202u32;
pub const SQL_COPT_SS_WARN_ON_CP_ERROR: u32 = 1243u32;
pub const SQL_CORRELATION_NAME: u32 = 74u32;
pub const SQL_CO_AF: i32 = 2i32;
pub const SQL_CO_DEFAULT: i32 = 0i32;
pub const SQL_CO_FFO: i32 = 1i32;
pub const SQL_CO_FIREHOSE_AF: i32 = 4i32;
pub const SQL_CO_OFF: i32 = 0i32;
pub const SQL_CP_DEFAULT: u32 = 0u32;
pub const SQL_CP_DRIVER_AWARE: u32 = 3u32;
pub const SQL_CP_MATCH_DEFAULT: u32 = 0u32;
pub const SQL_CP_OFF: u32 = 0u32;
pub const SQL_CP_ONE_PER_DRIVER: u32 = 1u32;
pub const SQL_CP_ONE_PER_HENV: u32 = 2u32;
pub const SQL_CP_RELAXED_MATCH: u32 = 1u32;
pub const SQL_CP_STRICT_MATCH: u32 = 0u32;
pub const SQL_CREATE_ASSERTION: u32 = 127u32;
pub const SQL_CREATE_CHARACTER_SET: u32 = 128u32;
pub const SQL_CREATE_COLLATION: u32 = 129u32;
pub const SQL_CREATE_DOMAIN: u32 = 130u32;
pub const SQL_CREATE_SCHEMA: u32 = 131u32;
pub const SQL_CREATE_TABLE: u32 = 132u32;
pub const SQL_CREATE_TRANSLATION: u32 = 133u32;
pub const SQL_CREATE_VIEW: u32 = 134u32;
pub const SQL_CR_CLOSE: u32 = 1u32;
pub const SQL_CR_DELETE: u32 = 0u32;
pub const SQL_CR_PRESERVE: u32 = 2u32;
pub const SQL_CS_AUTHORIZATION: i32 = 2i32;
pub const SQL_CS_CREATE_SCHEMA: i32 = 1i32;
pub const SQL_CS_DEFAULT_CHARACTER_SET: i32 = 4i32;
pub const SQL_CTR_CREATE_TRANSLATION: i32 = 1i32;
pub const SQL_CT_COLUMN_COLLATION: i32 = 2048i32;
pub const SQL_CT_COLUMN_CONSTRAINT: i32 = 512i32;
pub const SQL_CT_COLUMN_DEFAULT: i32 = 1024i32;
pub const SQL_CT_COMMIT_DELETE: i32 = 4i32;
pub const SQL_CT_COMMIT_PRESERVE: i32 = 2i32;
pub const SQL_CT_CONSTRAINT_DEFERRABLE: i32 = 128i32;
pub const SQL_CT_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32i32;
pub const SQL_CT_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64i32;
pub const SQL_CT_CONSTRAINT_NAME_DEFINITION: i32 = 8192i32;
pub const SQL_CT_CONSTRAINT_NON_DEFERRABLE: i32 = 256i32;
pub const SQL_CT_CREATE_TABLE: i32 = 1i32;
pub const SQL_CT_GLOBAL_TEMPORARY: i32 = 8i32;
pub const SQL_CT_LOCAL_TEMPORARY: i32 = 16i32;
pub const SQL_CT_TABLE_CONSTRAINT: i32 = 4096i32;
pub const SQL_CURRENT_QUALIFIER: u32 = 109u32;
pub const SQL_CURSOR_COMMIT_BEHAVIOR: u32 = 23u32;
pub const SQL_CURSOR_DYNAMIC: u32 = 2u32;
pub const SQL_CURSOR_FAST_FORWARD_ONLY: u32 = 8u32;
pub const SQL_CURSOR_FORWARD_ONLY: u32 = 0u32;
pub const SQL_CURSOR_KEYSET_DRIVEN: u32 = 1u32;
pub const SQL_CURSOR_ROLLBACK_BEHAVIOR: u32 = 24u32;
pub const SQL_CURSOR_SENSITIVITY: u32 = 10001u32;
pub const SQL_CURSOR_STATIC: u32 = 3u32;
pub const SQL_CURSOR_TYPE: u32 = 6u32;
pub const SQL_CURSOR_TYPE_DEFAULT: u32 = 0u32;
pub const SQL_CUR_DEFAULT: u32 = 2u32;
pub const SQL_CUR_USE_DRIVER: u32 = 2u32;
pub const SQL_CUR_USE_IF_NEEDED: u32 = 0u32;
pub const SQL_CUR_USE_ODBC: u32 = 1u32;
pub const SQL_CU_DML_STATEMENTS: i32 = 1i32;
pub const SQL_CU_INDEX_DEFINITION: i32 = 8i32;
pub const SQL_CU_PRIVILEGE_DEFINITION: i32 = 16i32;
pub const SQL_CU_PROCEDURE_INVOCATION: i32 = 2i32;
pub const SQL_CU_TABLE_DEFINITION: i32 = 4i32;
pub const SQL_CVT_BIGINT: i32 = 16384i32;
pub const SQL_CVT_BINARY: i32 = 1024i32;
pub const SQL_CVT_BIT: i32 = 4096i32;
pub const SQL_CVT_CHAR: i32 = 1i32;
pub const SQL_CVT_DATE: i32 = 32768i32;
pub const SQL_CVT_DECIMAL: i32 = 4i32;
pub const SQL_CVT_DOUBLE: i32 = 128i32;
pub const SQL_CVT_FLOAT: i32 = 32i32;
pub const SQL_CVT_GUID: i32 = 16777216i32;
pub const SQL_CVT_INTEGER: i32 = 8i32;
pub const SQL_CVT_INTERVAL_DAY_TIME: i32 = 1048576i32;
pub const SQL_CVT_INTERVAL_YEAR_MONTH: i32 = 524288i32;
pub const SQL_CVT_LONGVARBINARY: i32 = 262144i32;
pub const SQL_CVT_LONGVARCHAR: i32 = 512i32;
pub const SQL_CVT_NUMERIC: i32 = 2i32;
pub const SQL_CVT_REAL: i32 = 64i32;
pub const SQL_CVT_SMALLINT: i32 = 16i32;
pub const SQL_CVT_TIME: i32 = 65536i32;
pub const SQL_CVT_TIMESTAMP: i32 = 131072i32;
pub const SQL_CVT_TINYINT: i32 = 8192i32;
pub const SQL_CVT_VARBINARY: i32 = 2048i32;
pub const SQL_CVT_VARCHAR: i32 = 256i32;
pub const SQL_CVT_WCHAR: i32 = 2097152i32;
pub const SQL_CVT_WLONGVARCHAR: i32 = 4194304i32;
pub const SQL_CVT_WVARCHAR: i32 = 8388608i32;
pub const SQL_CV_CASCADED: i32 = 4i32;
pub const SQL_CV_CHECK_OPTION: i32 = 2i32;
pub const SQL_CV_CREATE_VIEW: i32 = 1i32;
pub const SQL_CV_LOCAL: i32 = 8i32;
pub const SQL_C_BINARY: i32 = -2i32;
pub const SQL_C_BIT: i32 = -7i32;
pub const SQL_C_CHAR: u32 = 1u32;
pub const SQL_C_DATE: u32 = 9u32;
pub const SQL_C_DEFAULT: u32 = 99u32;
pub const SQL_C_DOUBLE: u32 = 8u32;
pub const SQL_C_FLOAT: u32 = 7u32;
pub const SQL_C_GUID: i32 = -11i32;
pub const SQL_C_INTERVAL_DAY: i32 = -83i32;
pub const SQL_C_INTERVAL_DAY_TO_HOUR: i32 = -87i32;
pub const SQL_C_INTERVAL_DAY_TO_MINUTE: i32 = -88i32;
pub const SQL_C_INTERVAL_DAY_TO_SECOND: i32 = -89i32;
pub const SQL_C_INTERVAL_HOUR: i32 = -84i32;
pub const SQL_C_INTERVAL_HOUR_TO_MINUTE: i32 = -90i32;
pub const SQL_C_INTERVAL_HOUR_TO_SECOND: i32 = -91i32;
pub const SQL_C_INTERVAL_MINUTE: i32 = -85i32;
pub const SQL_C_INTERVAL_MINUTE_TO_SECOND: i32 = -92i32;
pub const SQL_C_INTERVAL_MONTH: i32 = -81i32;
pub const SQL_C_INTERVAL_SECOND: i32 = -86i32;
pub const SQL_C_INTERVAL_YEAR: i32 = -80i32;
pub const SQL_C_INTERVAL_YEAR_TO_MONTH: i32 = -82i32;
pub const SQL_C_LONG: u32 = 4u32;
pub const SQL_C_NUMERIC: u32 = 2u32;
pub const SQL_C_SHORT: u32 = 5u32;
pub const SQL_C_TCHAR: i32 = -8i32;
pub const SQL_C_TIME: u32 = 10u32;
pub const SQL_C_TIMESTAMP: u32 = 11u32;
pub const SQL_C_TINYINT: i32 = -6i32;
pub const SQL_C_TYPE_DATE: u32 = 91u32;
pub const SQL_C_TYPE_TIME: u32 = 92u32;
pub const SQL_C_TYPE_TIMESTAMP: u32 = 93u32;
pub const SQL_C_VARBOOKMARK: i32 = -2i32;
pub const SQL_C_WCHAR: i32 = -8i32;
pub const SQL_DATABASE_NAME: u32 = 16u32;
pub const SQL_DATA_AT_EXEC: i32 = -2i32;
pub const SQL_DATA_SOURCE_NAME: u32 = 2u32;
pub const SQL_DATA_SOURCE_READ_ONLY: u32 = 25u32;
pub const SQL_DATE: u32 = 9u32;
pub const SQL_DATETIME: u32 = 9u32;
pub const SQL_DATETIME_LITERALS: u32 = 119u32;
pub const SQL_DATE_LEN: u32 = 10u32;
pub const SQL_DAY: u32 = 3u32;
pub const SQL_DAY_TO_HOUR: u32 = 8u32;
pub const SQL_DAY_TO_MINUTE: u32 = 9u32;
pub const SQL_DAY_TO_SECOND: u32 = 10u32;
pub const SQL_DA_DROP_ASSERTION: i32 = 1i32;
pub const SQL_DBMS_NAME: u32 = 17u32;
pub const SQL_DBMS_VER: u32 = 18u32;
pub const SQL_DB_DEFAULT: u32 = 0u32;
pub const SQL_DB_DISCONNECT: u32 = 1u32;
pub const SQL_DB_RETURN_TO_POOL: u32 = 0u32;
pub const SQL_DCS_DROP_CHARACTER_SET: i32 = 1i32;
pub const SQL_DC_DROP_COLLATION: i32 = 1i32;
pub const SQL_DDL_INDEX: u32 = 170u32;
pub const SQL_DD_CASCADE: i32 = 4i32;
pub const SQL_DD_DROP_DOMAIN: i32 = 1i32;
pub const SQL_DD_RESTRICT: i32 = 2i32;
pub const SQL_DECIMAL: u32 = 3u32;
pub const SQL_DEFAULT: u32 = 99u32;
pub const SQL_DEFAULT_PARAM: i32 = -5i32;
pub const SQL_DEFAULT_TXN_ISOLATION: u32 = 26u32;
pub const SQL_DELETE: u32 = 3u32;
pub const SQL_DELETE_BY_BOOKMARK: u32 = 6u32;
pub const SQL_DESCRIBE_PARAMETER: u32 = 10002u32;
pub const SQL_DESC_ALLOC_AUTO: u32 = 1u32;
pub const SQL_DESC_ALLOC_TYPE: u32 = 1099u32;
pub const SQL_DESC_ALLOC_USER: u32 = 2u32;
pub const SQL_DESC_ARRAY_SIZE: u32 = 20u32;
pub const SQL_DESC_ARRAY_STATUS_PTR: u32 = 21u32;
pub const SQL_DESC_BASE_COLUMN_NAME: u32 = 22u32;
pub const SQL_DESC_BASE_TABLE_NAME: u32 = 23u32;
pub const SQL_DESC_BIND_OFFSET_PTR: u32 = 24u32;
pub const SQL_DESC_BIND_TYPE: u32 = 25u32;
pub const SQL_DESC_COUNT: u32 = 1001u32;
pub const SQL_DESC_DATA_PTR: u32 = 1010u32;
pub const SQL_DESC_DATETIME_INTERVAL_CODE: u32 = 1007u32;
pub const SQL_DESC_DATETIME_INTERVAL_PRECISION: u32 = 26u32;
pub const SQL_DESC_INDICATOR_PTR: u32 = 1009u32;
pub const SQL_DESC_LENGTH: u32 = 1003u32;
pub const SQL_DESC_LITERAL_PREFIX: u32 = 27u32;
pub const SQL_DESC_LITERAL_SUFFIX: u32 = 28u32;
pub const SQL_DESC_LOCAL_TYPE_NAME: u32 = 29u32;
pub const SQL_DESC_MAXIMUM_SCALE: u32 = 30u32;
pub const SQL_DESC_MINIMUM_SCALE: u32 = 31u32;
pub const SQL_DESC_NAME: u32 = 1011u32;
pub const SQL_DESC_NULLABLE: u32 = 1008u32;
pub const SQL_DESC_NUM_PREC_RADIX: u32 = 32u32;
pub const SQL_DESC_OCTET_LENGTH: u32 = 1013u32;
pub const SQL_DESC_OCTET_LENGTH_PTR: u32 = 1004u32;
pub const SQL_DESC_PARAMETER_TYPE: u32 = 33u32;
pub const SQL_DESC_PRECISION: u32 = 1005u32;
pub const SQL_DESC_ROWS_PROCESSED_PTR: u32 = 34u32;
pub const SQL_DESC_ROWVER: u32 = 35u32;
pub const SQL_DESC_SCALE: u32 = 1006u32;
pub const SQL_DESC_TYPE: u32 = 1002u32;
pub const SQL_DESC_UNNAMED: u32 = 1012u32;
pub const SQL_DIAG_ALTER_DOMAIN: u32 = 3u32;
pub const SQL_DIAG_ALTER_TABLE: u32 = 4u32;
pub const SQL_DIAG_CALL: u32 = 7u32;
pub const SQL_DIAG_CLASS_ORIGIN: u32 = 8u32;
pub const SQL_DIAG_COLUMN_NUMBER: i32 = -1247i32;
pub const SQL_DIAG_CONNECTION_NAME: u32 = 10u32;
pub const SQL_DIAG_CREATE_ASSERTION: u32 = 6u32;
pub const SQL_DIAG_CREATE_CHARACTER_SET: u32 = 8u32;
pub const SQL_DIAG_CREATE_COLLATION: u32 = 10u32;
pub const SQL_DIAG_CREATE_DOMAIN: u32 = 23u32;
pub const SQL_DIAG_CREATE_INDEX: i32 = -1i32;
pub const SQL_DIAG_CREATE_SCHEMA: u32 = 64u32;
pub const SQL_DIAG_CREATE_TABLE: u32 = 77u32;
pub const SQL_DIAG_CREATE_TRANSLATION: u32 = 79u32;
pub const SQL_DIAG_CREATE_VIEW: u32 = 84u32;
pub const SQL_DIAG_CURSOR_ROW_COUNT: i32 = -1249i32;
pub const SQL_DIAG_DELETE_WHERE: u32 = 19u32;
pub const SQL_DIAG_DFC_SS_ALTER_DATABASE: i32 = -200i32;
pub const SQL_DIAG_DFC_SS_BASE: i32 = -200i32;
pub const SQL_DIAG_DFC_SS_CHECKPOINT: i32 = -201i32;
pub const SQL_DIAG_DFC_SS_CONDITION: i32 = -202i32;
pub const SQL_DIAG_DFC_SS_CREATE_DATABASE: i32 = -203i32;
pub const SQL_DIAG_DFC_SS_CREATE_DEFAULT: i32 = -204i32;
pub const SQL_DIAG_DFC_SS_CREATE_PROCEDURE: i32 = -205i32;
pub const SQL_DIAG_DFC_SS_CREATE_RULE: i32 = -206i32;
pub const SQL_DIAG_DFC_SS_CREATE_TRIGGER: i32 = -207i32;
pub const SQL_DIAG_DFC_SS_CURSOR_CLOSE: i32 = -211i32;
pub const SQL_DIAG_DFC_SS_CURSOR_DECLARE: i32 = -208i32;
pub const SQL_DIAG_DFC_SS_CURSOR_FETCH: i32 = -210i32;
pub const SQL_DIAG_DFC_SS_CURSOR_OPEN: i32 = -209i32;
pub const SQL_DIAG_DFC_SS_DBCC: i32 = -213i32;
pub const SQL_DIAG_DFC_SS_DEALLOCATE_CURSOR: i32 = -212i32;
pub const SQL_DIAG_DFC_SS_DENY: i32 = -254i32;
pub const SQL_DIAG_DFC_SS_DISK: i32 = -214i32;
pub const SQL_DIAG_DFC_SS_DROP_DATABASE: i32 = -215i32;
pub const SQL_DIAG_DFC_SS_DROP_DEFAULT: i32 = -216i32;
pub const SQL_DIAG_DFC_SS_DROP_PROCEDURE: i32 = -217i32;
pub const SQL_DIAG_DFC_SS_DROP_RULE: i32 = -218i32;
pub const SQL_DIAG_DFC_SS_DROP_TRIGGER: i32 = -219i32;
pub const SQL_DIAG_DFC_SS_DUMP_DATABASE: i32 = -220i32;
pub const SQL_DIAG_DFC_SS_DUMP_TABLE: i32 = -221i32;
pub const SQL_DIAG_DFC_SS_DUMP_TRANSACTION: i32 = -222i32;
pub const SQL_DIAG_DFC_SS_GOTO: i32 = -223i32;
pub const SQL_DIAG_DFC_SS_INSERT_BULK: i32 = -224i32;
pub const SQL_DIAG_DFC_SS_KILL: i32 = -225i32;
pub const SQL_DIAG_DFC_SS_LOAD_DATABASE: i32 = -226i32;
pub const SQL_DIAG_DFC_SS_LOAD_HEADERONLY: i32 = -227i32;
pub const SQL_DIAG_DFC_SS_LOAD_TABLE: i32 = -228i32;
pub const SQL_DIAG_DFC_SS_LOAD_TRANSACTION: i32 = -229i32;
pub const SQL_DIAG_DFC_SS_PRINT: i32 = -230i32;
pub const SQL_DIAG_DFC_SS_RAISERROR: i32 = -231i32;
pub const SQL_DIAG_DFC_SS_READTEXT: i32 = -232i32;
pub const SQL_DIAG_DFC_SS_RECONFIGURE: i32 = -233i32;
pub const SQL_DIAG_DFC_SS_RETURN: i32 = -234i32;
pub const SQL_DIAG_DFC_SS_SELECT_INTO: i32 = -235i32;
pub const SQL_DIAG_DFC_SS_SET: i32 = -236i32;
pub const SQL_DIAG_DFC_SS_SETUSER: i32 = -241i32;
pub const SQL_DIAG_DFC_SS_SET_IDENTITY_INSERT: i32 = -237i32;
pub const SQL_DIAG_DFC_SS_SET_ROW_COUNT: i32 = -238i32;
pub const SQL_DIAG_DFC_SS_SET_STATISTICS: i32 = -239i32;
pub const SQL_DIAG_DFC_SS_SET_TEXTSIZE: i32 = -240i32;
pub const SQL_DIAG_DFC_SS_SET_XCTLVL: i32 = -255i32;
pub const SQL_DIAG_DFC_SS_SHUTDOWN: i32 = -242i32;
pub const SQL_DIAG_DFC_SS_TRANS_BEGIN: i32 = -243i32;
pub const SQL_DIAG_DFC_SS_TRANS_COMMIT: i32 = -244i32;
pub const SQL_DIAG_DFC_SS_TRANS_PREPARE: i32 = -245i32;
pub const SQL_DIAG_DFC_SS_TRANS_ROLLBACK: i32 = -246i32;
pub const SQL_DIAG_DFC_SS_TRANS_SAVE: i32 = -247i32;
pub const SQL_DIAG_DFC_SS_TRUNCATE_TABLE: i32 = -248i32;
pub const SQL_DIAG_DFC_SS_UPDATETEXT: i32 = -250i32;
pub const SQL_DIAG_DFC_SS_UPDATE_STATISTICS: i32 = -249i32;
pub const SQL_DIAG_DFC_SS_USE: i32 = -251i32;
pub const SQL_DIAG_DFC_SS_WAITFOR: i32 = -252i32;
pub const SQL_DIAG_DFC_SS_WRITETEXT: i32 = -253i32;
pub const SQL_DIAG_DROP_ASSERTION: u32 = 24u32;
pub const SQL_DIAG_DROP_CHARACTER_SET: u32 = 25u32;
pub const SQL_DIAG_DROP_COLLATION: u32 = 26u32;
pub const SQL_DIAG_DROP_DOMAIN: u32 = 27u32;
pub const SQL_DIAG_DROP_INDEX: i32 = -2i32;
pub const SQL_DIAG_DROP_SCHEMA: u32 = 31u32;
pub const SQL_DIAG_DROP_TABLE: u32 = 32u32;
pub const SQL_DIAG_DROP_TRANSLATION: u32 = 33u32;
pub const SQL_DIAG_DROP_VIEW: u32 = 36u32;
pub const SQL_DIAG_DYNAMIC_DELETE_CURSOR: u32 = 38u32;
pub const SQL_DIAG_DYNAMIC_FUNCTION: u32 = 7u32;
pub const SQL_DIAG_DYNAMIC_FUNCTION_CODE: u32 = 12u32;
pub const SQL_DIAG_DYNAMIC_UPDATE_CURSOR: u32 = 81u32;
pub const SQL_DIAG_GRANT: u32 = 48u32;
pub const SQL_DIAG_INSERT: u32 = 50u32;
pub const SQL_DIAG_MESSAGE_TEXT: u32 = 6u32;
pub const SQL_DIAG_NATIVE: u32 = 5u32;
pub const SQL_DIAG_NUMBER: u32 = 2u32;
pub const SQL_DIAG_RETURNCODE: u32 = 1u32;
pub const SQL_DIAG_REVOKE: u32 = 59u32;
pub const SQL_DIAG_ROW_COUNT: u32 = 3u32;
pub const SQL_DIAG_ROW_NUMBER: i32 = -1248i32;
pub const SQL_DIAG_SELECT_CURSOR: u32 = 85u32;
pub const SQL_DIAG_SERVER_NAME: u32 = 11u32;
pub const SQL_DIAG_SQLSTATE: u32 = 4u32;
pub const SQL_DIAG_SS_BASE: i32 = -1150i32;
pub const SQL_DIAG_SS_LINE: i32 = -1154i32;
pub const SQL_DIAG_SS_MSGSTATE: i32 = -1150i32;
pub const SQL_DIAG_SS_PROCNAME: i32 = -1153i32;
pub const SQL_DIAG_SS_SEVERITY: i32 = -1151i32;
pub const SQL_DIAG_SS_SRVNAME: i32 = -1152i32;
pub const SQL_DIAG_SUBCLASS_ORIGIN: u32 = 9u32;
pub const SQL_DIAG_UNKNOWN_STATEMENT: u32 = 0u32;
pub const SQL_DIAG_UPDATE_WHERE: u32 = 82u32;
pub const SQL_DI_CREATE_INDEX: i32 = 1i32;
pub const SQL_DI_DROP_INDEX: i32 = 2i32;
pub const SQL_DL_SQL92_DATE: i32 = 1i32;
pub const SQL_DL_SQL92_INTERVAL_DAY: i32 = 32i32;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_HOUR: i32 = 1024i32;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_MINUTE: i32 = 2048i32;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_SECOND: i32 = 4096i32;
pub const SQL_DL_SQL92_INTERVAL_HOUR: i32 = 64i32;
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_MINUTE: i32 = 8192i32;
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_SECOND: i32 = 16384i32;
pub const SQL_DL_SQL92_INTERVAL_MINUTE: i32 = 128i32;
pub const SQL_DL_SQL92_INTERVAL_MINUTE_TO_SECOND: i32 = 32768i32;
pub const SQL_DL_SQL92_INTERVAL_MONTH: i32 = 16i32;
pub const SQL_DL_SQL92_INTERVAL_SECOND: i32 = 256i32;
pub const SQL_DL_SQL92_INTERVAL_YEAR: i32 = 8i32;
pub const SQL_DL_SQL92_INTERVAL_YEAR_TO_MONTH: i32 = 512i32;
pub const SQL_DL_SQL92_TIME: i32 = 2i32;
pub const SQL_DL_SQL92_TIMESTAMP: i32 = 4i32;
pub const SQL_DM_VER: u32 = 171u32;
pub const SQL_DOUBLE: u32 = 8u32;
pub const SQL_DP_OFF: i32 = 0i32;
pub const SQL_DP_ON: i32 = 1i32;
pub const SQL_DRIVER_AWARE_POOLING_CAPABLE: i32 = 1i32;
pub const SQL_DRIVER_AWARE_POOLING_NOT_CAPABLE: i32 = 0i32;
pub const SQL_DRIVER_AWARE_POOLING_SUPPORTED: u32 = 10024u32;
pub const SQL_DRIVER_COMPLETE: u32 = 1u32;
pub const SQL_DRIVER_COMPLETE_REQUIRED: u32 = 3u32;
pub const SQL_DRIVER_CONN_ATTR_BASE: u32 = 16384u32;
pub const SQL_DRIVER_C_TYPE_BASE: u32 = 16384u32;
pub const SQL_DRIVER_DESC_FIELD_BASE: u32 = 16384u32;
pub const SQL_DRIVER_DIAG_FIELD_BASE: u32 = 16384u32;
pub const SQL_DRIVER_HDBC: u32 = 3u32;
pub const SQL_DRIVER_HDESC: u32 = 135u32;
pub const SQL_DRIVER_HENV: u32 = 4u32;
pub const SQL_DRIVER_HLIB: u32 = 76u32;
pub const SQL_DRIVER_HSTMT: u32 = 5u32;
pub const SQL_DRIVER_INFO_TYPE_BASE: u32 = 16384u32;
pub const SQL_DRIVER_NAME: u32 = 6u32;
pub const SQL_DRIVER_NOPROMPT: u32 = 0u32;
pub const SQL_DRIVER_ODBC_VER: u32 = 77u32;
pub const SQL_DRIVER_PROMPT: u32 = 2u32;
pub const SQL_DRIVER_SQL_TYPE_BASE: u32 = 16384u32;
pub const SQL_DRIVER_STMT_ATTR_BASE: u32 = 16384u32;
pub const SQL_DRIVER_VER: u32 = 7u32;
pub const SQL_DROP: u32 = 1u32;
pub const SQL_DROP_ASSERTION: u32 = 136u32;
pub const SQL_DROP_CHARACTER_SET: u32 = 137u32;
pub const SQL_DROP_COLLATION: u32 = 138u32;
pub const SQL_DROP_DOMAIN: u32 = 139u32;
pub const SQL_DROP_SCHEMA: u32 = 140u32;
pub const SQL_DROP_TABLE: u32 = 141u32;
pub const SQL_DROP_TRANSLATION: u32 = 142u32;
pub const SQL_DROP_VIEW: u32 = 143u32;
pub const SQL_DS_CASCADE: i32 = 4i32;
pub const SQL_DS_DROP_SCHEMA: i32 = 1i32;
pub const SQL_DS_RESTRICT: i32 = 2i32;
pub const SQL_DTC_DONE: i32 = 0i32;
pub const SQL_DTC_ENLIST_EXPENSIVE: i32 = 1i32;
pub const SQL_DTC_TRANSITION_COST: u32 = 1750u32;
pub const SQL_DTC_UNENLIST_EXPENSIVE: i32 = 2i32;
pub const SQL_DTR_DROP_TRANSLATION: i32 = 1i32;
pub const SQL_DT_CASCADE: i32 = 4i32;
pub const SQL_DT_DROP_TABLE: i32 = 1i32;
pub const SQL_DT_RESTRICT: i32 = 2i32;
pub const SQL_DV_CASCADE: i32 = 4i32;
pub const SQL_DV_DROP_VIEW: i32 = 1i32;
pub const SQL_DV_RESTRICT: i32 = 2i32;
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES1: u32 = 144u32;
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES2: u32 = 145u32;
pub const SQL_ENSURE: u32 = 1u32;
pub const SQL_ENTIRE_ROWSET: u32 = 0u32;
pub const SQL_EN_OFF: i32 = 0i32;
pub const SQL_EN_ON: i32 = 1i32;
pub const SQL_ERROR: i32 = -1i32;
pub const SQL_EXPRESSIONS_IN_ORDERBY: u32 = 27u32;
pub const SQL_EXT_API_LAST: u32 = 72u32;
pub const SQL_EXT_API_START: u32 = 40u32;
pub const SQL_FALSE: u32 = 0u32;
pub const SQL_FAST_CONNECT: u32 = 1200u32;
pub const SQL_FB_DEFAULT: i32 = 0i32;
pub const SQL_FB_OFF: i32 = 0i32;
pub const SQL_FB_ON: i32 = 1i32;
pub const SQL_FC_DEFAULT: i32 = 0i32;
pub const SQL_FC_OFF: i32 = 0i32;
pub const SQL_FC_ON: i32 = 1i32;
pub const SQL_FD_FETCH_ABSOLUTE: i32 = 16i32;
pub const SQL_FD_FETCH_BOOKMARK: i32 = 128i32;
pub const SQL_FD_FETCH_FIRST: i32 = 2i32;
pub const SQL_FD_FETCH_LAST: i32 = 4i32;
pub const SQL_FD_FETCH_NEXT: i32 = 1i32;
pub const SQL_FD_FETCH_PREV: i32 = 8i32;
pub const SQL_FD_FETCH_PRIOR: i32 = 8i32;
pub const SQL_FD_FETCH_RELATIVE: i32 = 32i32;
pub const SQL_FD_FETCH_RESUME: i32 = 64i32;
pub const SQL_FETCH_ABSOLUTE: u32 = 5u32;
pub const SQL_FETCH_BOOKMARK: u32 = 8u32;
pub const SQL_FETCH_BY_BOOKMARK: u32 = 7u32;
pub const SQL_FETCH_DIRECTION: u32 = 8u32;
pub const SQL_FETCH_FIRST: u32 = 2u32;
pub const SQL_FETCH_FIRST_SYSTEM: u32 = 32u32;
pub const SQL_FETCH_FIRST_USER: u32 = 31u32;
pub const SQL_FETCH_LAST: u32 = 3u32;
pub const SQL_FETCH_NEXT: u32 = 1u32;
pub const SQL_FETCH_PREV: u32 = 4u32;
pub const SQL_FETCH_PRIOR: u32 = 4u32;
pub const SQL_FETCH_RELATIVE: u32 = 6u32;
pub const SQL_FETCH_RESUME: u32 = 7u32;
pub const SQL_FILE_CATALOG: u32 = 2u32;
pub const SQL_FILE_NOT_SUPPORTED: u32 = 0u32;
pub const SQL_FILE_QUALIFIER: u32 = 2u32;
pub const SQL_FILE_TABLE: u32 = 1u32;
pub const SQL_FILE_USAGE: u32 = 84u32;
pub const SQL_FLOAT: u32 = 6u32;
pub const SQL_FN_CVT_CAST: i32 = 2i32;
pub const SQL_FN_CVT_CONVERT: i32 = 1i32;
pub const SQL_FN_NUM_ABS: i32 = 1i32;
pub const SQL_FN_NUM_ACOS: i32 = 2i32;
pub const SQL_FN_NUM_ASIN: i32 = 4i32;
pub const SQL_FN_NUM_ATAN: i32 = 8i32;
pub const SQL_FN_NUM_ATAN2: i32 = 16i32;
pub const SQL_FN_NUM_CEILING: i32 = 32i32;
pub const SQL_FN_NUM_COS: i32 = 64i32;
pub const SQL_FN_NUM_COT: i32 = 128i32;
pub const SQL_FN_NUM_DEGREES: i32 = 262144i32;
pub const SQL_FN_NUM_EXP: i32 = 256i32;
pub const SQL_FN_NUM_FLOOR: i32 = 512i32;
pub const SQL_FN_NUM_LOG: i32 = 1024i32;
pub const SQL_FN_NUM_LOG10: i32 = 524288i32;
pub const SQL_FN_NUM_MOD: i32 = 2048i32;
pub const SQL_FN_NUM_PI: i32 = 65536i32;
pub const SQL_FN_NUM_POWER: i32 = 1048576i32;
pub const SQL_FN_NUM_RADIANS: i32 = 2097152i32;
pub const SQL_FN_NUM_RAND: i32 = 131072i32;
pub const SQL_FN_NUM_ROUND: i32 = 4194304i32;
pub const SQL_FN_NUM_SIGN: i32 = 4096i32;
pub const SQL_FN_NUM_SIN: i32 = 8192i32;
pub const SQL_FN_NUM_SQRT: i32 = 16384i32;
pub const SQL_FN_NUM_TAN: i32 = 32768i32;
pub const SQL_FN_NUM_TRUNCATE: i32 = 8388608i32;
pub const SQL_FN_STR_ASCII: i32 = 8192i32;
pub const SQL_FN_STR_BIT_LENGTH: i32 = 524288i32;
pub const SQL_FN_STR_CHAR: i32 = 16384i32;
pub const SQL_FN_STR_CHARACTER_LENGTH: i32 = 2097152i32;
pub const SQL_FN_STR_CHAR_LENGTH: i32 = 1048576i32;
pub const SQL_FN_STR_CONCAT: i32 = 1i32;
pub const SQL_FN_STR_DIFFERENCE: i32 = 32768i32;
pub const SQL_FN_STR_INSERT: i32 = 2i32;
pub const SQL_FN_STR_LCASE: i32 = 64i32;
pub const SQL_FN_STR_LEFT: i32 = 4i32;
pub const SQL_FN_STR_LENGTH: i32 = 16i32;
pub const SQL_FN_STR_LOCATE: i32 = 32i32;
pub const SQL_FN_STR_LOCATE_2: i32 = 65536i32;
pub const SQL_FN_STR_LTRIM: i32 = 8i32;
pub const SQL_FN_STR_OCTET_LENGTH: i32 = 4194304i32;
pub const SQL_FN_STR_POSITION: i32 = 8388608i32;
pub const SQL_FN_STR_REPEAT: i32 = 128i32;
pub const SQL_FN_STR_REPLACE: i32 = 256i32;
pub const SQL_FN_STR_RIGHT: i32 = 512i32;
pub const SQL_FN_STR_RTRIM: i32 = 1024i32;
pub const SQL_FN_STR_SOUNDEX: i32 = 131072i32;
pub const SQL_FN_STR_SPACE: i32 = 262144i32;
pub const SQL_FN_STR_SUBSTRING: i32 = 2048i32;
pub const SQL_FN_STR_UCASE: i32 = 4096i32;
pub const SQL_FN_SYS_DBNAME: i32 = 2i32;
pub const SQL_FN_SYS_IFNULL: i32 = 4i32;
pub const SQL_FN_SYS_USERNAME: i32 = 1i32;
pub const SQL_FN_TD_CURDATE: i32 = 2i32;
pub const SQL_FN_TD_CURRENT_DATE: i32 = 131072i32;
pub const SQL_FN_TD_CURRENT_TIME: i32 = 262144i32;
pub const SQL_FN_TD_CURRENT_TIMESTAMP: i32 = 524288i32;
pub const SQL_FN_TD_CURTIME: i32 = 512i32;
pub const SQL_FN_TD_DAYNAME: i32 = 32768i32;
pub const SQL_FN_TD_DAYOFMONTH: i32 = 4i32;
pub const SQL_FN_TD_DAYOFWEEK: i32 = 8i32;
pub const SQL_FN_TD_DAYOFYEAR: i32 = 16i32;
pub const SQL_FN_TD_EXTRACT: i32 = 1048576i32;
pub const SQL_FN_TD_HOUR: i32 = 1024i32;
pub const SQL_FN_TD_MINUTE: i32 = 2048i32;
pub const SQL_FN_TD_MONTH: i32 = 32i32;
pub const SQL_FN_TD_MONTHNAME: i32 = 65536i32;
pub const SQL_FN_TD_NOW: i32 = 1i32;
pub const SQL_FN_TD_QUARTER: i32 = 64i32;
pub const SQL_FN_TD_SECOND: i32 = 4096i32;
pub const SQL_FN_TD_TIMESTAMPADD: i32 = 8192i32;
pub const SQL_FN_TD_TIMESTAMPDIFF: i32 = 16384i32;
pub const SQL_FN_TD_WEEK: i32 = 128i32;
pub const SQL_FN_TD_YEAR: i32 = 256i32;
pub const SQL_FN_TSI_DAY: i32 = 16i32;
pub const SQL_FN_TSI_FRAC_SECOND: i32 = 1i32;
pub const SQL_FN_TSI_HOUR: i32 = 8i32;
pub const SQL_FN_TSI_MINUTE: i32 = 4i32;
pub const SQL_FN_TSI_MONTH: i32 = 64i32;
pub const SQL_FN_TSI_QUARTER: i32 = 128i32;
pub const SQL_FN_TSI_SECOND: i32 = 2i32;
pub const SQL_FN_TSI_WEEK: i32 = 32i32;
pub const SQL_FN_TSI_YEAR: i32 = 256i32;
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1: u32 = 146u32;
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2: u32 = 147u32;
pub const SQL_GB_COLLATE: u32 = 4u32;
pub const SQL_GB_GROUP_BY_CONTAINS_SELECT: u32 = 2u32;
pub const SQL_GB_GROUP_BY_EQUALS_SELECT: u32 = 1u32;
pub const SQL_GB_NOT_SUPPORTED: u32 = 0u32;
pub const SQL_GB_NO_RELATION: u32 = 3u32;
pub const SQL_GD_ANY_COLUMN: i32 = 1i32;
pub const SQL_GD_ANY_ORDER: i32 = 2i32;
pub const SQL_GD_BLOCK: i32 = 4i32;
pub const SQL_GD_BOUND: i32 = 8i32;
pub const SQL_GD_OUTPUT_PARAMS: i32 = 16i32;
pub const SQL_GETDATA_EXTENSIONS: u32 = 81u32;
pub const SQL_GET_BOOKMARK: u32 = 13u32;
pub const SQL_GROUP_BY: u32 = 88u32;
pub const SQL_GUID: i32 = -11i32;
pub const SQL_HANDLE_DBC: u32 = 2u32;
pub const SQL_HANDLE_DBC_INFO_TOKEN: u32 = 6u32;
pub const SQL_HANDLE_DESC: u32 = 4u32;
pub const SQL_HANDLE_ENV: u32 = 1u32;
pub const SQL_HANDLE_SENV: u32 = 5u32;
pub const SQL_HANDLE_STMT: u32 = 3u32;
pub const SQL_HC_DEFAULT: i32 = 0i32;
pub const SQL_HC_OFF: i32 = 0i32;
pub const SQL_HC_ON: i32 = 1i32;
pub const SQL_HOUR: u32 = 4u32;
pub const SQL_HOUR_TO_MINUTE: u32 = 11u32;
pub const SQL_HOUR_TO_SECOND: u32 = 12u32;
pub const SQL_IC_LOWER: u32 = 2u32;
pub const SQL_IC_MIXED: u32 = 4u32;
pub const SQL_IC_SENSITIVE: u32 = 3u32;
pub const SQL_IC_UPPER: u32 = 1u32;
pub const SQL_IDENTIFIER_CASE: u32 = 28u32;
pub const SQL_IDENTIFIER_QUOTE_CHAR: u32 = 29u32;
pub const SQL_IGNORE: i32 = -6i32;
pub const SQL_IK_ASC: i32 = 1i32;
pub const SQL_IK_DESC: i32 = 2i32;
pub const SQL_IK_NONE: i32 = 0i32;
pub const SQL_INDEX_ALL: u32 = 1u32;
pub const SQL_INDEX_CLUSTERED: u32 = 1u32;
pub const SQL_INDEX_HASHED: u32 = 2u32;
pub const SQL_INDEX_KEYWORDS: u32 = 148u32;
pub const SQL_INDEX_OTHER: u32 = 3u32;
pub const SQL_INDEX_UNIQUE: u32 = 0u32;
pub const SQL_INFO_DRIVER_START: u32 = 1000u32;
pub const SQL_INFO_FIRST: u32 = 0u32;
pub const SQL_INFO_LAST: u32 = 114u32;
pub const SQL_INFO_SCHEMA_VIEWS: u32 = 149u32;
pub const SQL_INFO_SS_FIRST: u32 = 1199u32;
pub const SQL_INFO_SS_MAX_USED: u32 = 1200u32;
pub const SQL_INFO_SS_NETLIB_NAME: u32 = 1199u32;
pub const SQL_INFO_SS_NETLIB_NAMEA: u32 = 1200u32;
pub const SQL_INFO_SS_NETLIB_NAMEW: u32 = 1199u32;
pub const SQL_INITIALLY_DEFERRED: u32 = 5u32;
pub const SQL_INITIALLY_IMMEDIATE: u32 = 6u32;
pub const SQL_INSENSITIVE: u32 = 1u32;
pub const SQL_INSERT_STATEMENT: u32 = 172u32;
pub const SQL_INTEGER: u32 = 4u32;
pub const SQL_INTEGRATED_SECURITY: u32 = 1203u32;
pub const SQL_INTEGRITY: u32 = 73u32;
pub const SQL_INTERVAL: u32 = 10u32;
pub const SQL_INTERVAL_DAY: i32 = -83i32;
pub const SQL_INTERVAL_DAY_TO_HOUR: i32 = -87i32;
pub const SQL_INTERVAL_DAY_TO_MINUTE: i32 = -88i32;
pub const SQL_INTERVAL_DAY_TO_SECOND: i32 = -89i32;
pub const SQL_INTERVAL_HOUR: i32 = -84i32;
pub const SQL_INTERVAL_HOUR_TO_MINUTE: i32 = -90i32;
pub const SQL_INTERVAL_HOUR_TO_SECOND: i32 = -91i32;
pub const SQL_INTERVAL_MINUTE: i32 = -85i32;
pub const SQL_INTERVAL_MINUTE_TO_SECOND: i32 = -92i32;
pub const SQL_INTERVAL_MONTH: i32 = -81i32;
pub const SQL_INTERVAL_SECOND: i32 = -86i32;
pub const SQL_INTERVAL_YEAR: i32 = -80i32;
pub const SQL_INTERVAL_YEAR_TO_MONTH: i32 = -82i32;
pub const SQL_INVALID_HANDLE: i32 = -2i32;
pub const SQL_ISV_ASSERTIONS: i32 = 1i32;
pub const SQL_ISV_CHARACTER_SETS: i32 = 2i32;
pub const SQL_ISV_CHECK_CONSTRAINTS: i32 = 4i32;
pub const SQL_ISV_COLLATIONS: i32 = 8i32;
pub const SQL_ISV_COLUMNS: i32 = 64i32;
pub const SQL_ISV_COLUMN_DOMAIN_USAGE: i32 = 16i32;
pub const SQL_ISV_COLUMN_PRIVILEGES: i32 = 32i32;
pub const SQL_ISV_CONSTRAINT_COLUMN_USAGE: i32 = 128i32;
pub const SQL_ISV_CONSTRAINT_TABLE_USAGE: i32 = 256i32;
pub const SQL_ISV_DOMAINS: i32 = 1024i32;
pub const SQL_ISV_DOMAIN_CONSTRAINTS: i32 = 512i32;
pub const SQL_ISV_KEY_COLUMN_USAGE: i32 = 2048i32;
pub const SQL_ISV_REFERENTIAL_CONSTRAINTS: i32 = 4096i32;
pub const SQL_ISV_SCHEMATA: i32 = 8192i32;
pub const SQL_ISV_SQL_LANGUAGES: i32 = 16384i32;
pub const SQL_ISV_TABLES: i32 = 131072i32;
pub const SQL_ISV_TABLE_CONSTRAINTS: i32 = 32768i32;
pub const SQL_ISV_TABLE_PRIVILEGES: i32 = 65536i32;
pub const SQL_ISV_TRANSLATIONS: i32 = 262144i32;
pub const SQL_ISV_USAGE_PRIVILEGES: i32 = 524288i32;
pub const SQL_ISV_VIEWS: i32 = 4194304i32;
pub const SQL_ISV_VIEW_COLUMN_USAGE: i32 = 1048576i32;
pub const SQL_ISV_VIEW_TABLE_USAGE: i32 = 2097152i32;
pub const SQL_IS_DAY: SQLINTERVAL = SQLINTERVAL(3i32);
pub const SQL_IS_DAY_TO_HOUR: SQLINTERVAL = SQLINTERVAL(8i32);
pub const SQL_IS_DAY_TO_MINUTE: SQLINTERVAL = SQLINTERVAL(9i32);
pub const SQL_IS_DAY_TO_SECOND: SQLINTERVAL = SQLINTERVAL(10i32);
pub const SQL_IS_DEFAULT: i32 = 0i32;
pub const SQL_IS_HOUR: SQLINTERVAL = SQLINTERVAL(4i32);
pub const SQL_IS_HOUR_TO_MINUTE: SQLINTERVAL = SQLINTERVAL(11i32);
pub const SQL_IS_HOUR_TO_SECOND: SQLINTERVAL = SQLINTERVAL(12i32);
pub const SQL_IS_INSERT_LITERALS: i32 = 1i32;
pub const SQL_IS_INSERT_SEARCHED: i32 = 2i32;
pub const SQL_IS_INTEGER: i32 = -6i32;
pub const SQL_IS_MINUTE: SQLINTERVAL = SQLINTERVAL(5i32);
pub const SQL_IS_MINUTE_TO_SECOND: SQLINTERVAL = SQLINTERVAL(13i32);
pub const SQL_IS_MONTH: SQLINTERVAL = SQLINTERVAL(2i32);
pub const SQL_IS_OFF: i32 = 0i32;
pub const SQL_IS_ON: i32 = 1i32;
pub const SQL_IS_POINTER: i32 = -4i32;
pub const SQL_IS_SECOND: SQLINTERVAL = SQLINTERVAL(6i32);
pub const SQL_IS_SELECT_INTO: i32 = 4i32;
pub const SQL_IS_SMALLINT: i32 = -8i32;
pub const SQL_IS_UINTEGER: i32 = -5i32;
pub const SQL_IS_USMALLINT: i32 = -7i32;
pub const SQL_IS_YEAR: SQLINTERVAL = SQLINTERVAL(1i32);
pub const SQL_IS_YEAR_TO_MONTH: SQLINTERVAL = SQLINTERVAL(7i32);
pub const SQL_KEYSET_CURSOR_ATTRIBUTES1: u32 = 150u32;
pub const SQL_KEYSET_CURSOR_ATTRIBUTES2: u32 = 151u32;
pub const SQL_KEYSET_SIZE: u32 = 8u32;
pub const SQL_KEYSET_SIZE_DEFAULT: u32 = 0u32;
pub const SQL_KEYWORDS: u32 = 89u32;
pub const SQL_LCK_EXCLUSIVE: i32 = 2i32;
pub const SQL_LCK_NO_CHANGE: i32 = 1i32;
pub const SQL_LCK_UNLOCK: i32 = 4i32;
pub const SQL_LEN_BINARY_ATTR_OFFSET: i32 = -100i32;
pub const SQL_LEN_DATA_AT_EXEC_OFFSET: i32 = -100i32;
pub const SQL_LIKE_ESCAPE_CLAUSE: u32 = 113u32;
pub const SQL_LIKE_ONLY: u32 = 1u32;
pub const SQL_LOCK_EXCLUSIVE: u32 = 1u32;
pub const SQL_LOCK_NO_CHANGE: u32 = 0u32;
pub const SQL_LOCK_TYPES: u32 = 78u32;
pub const SQL_LOCK_UNLOCK: u32 = 2u32;
pub const SQL_LOGIN_TIMEOUT: u32 = 103u32;
pub const SQL_LOGIN_TIMEOUT_DEFAULT: u32 = 15u32;
pub const SQL_LONGVARBINARY: i32 = -4i32;
pub const SQL_LONGVARCHAR: i32 = -1i32;
pub const SQL_MAXIMUM_CATALOG_NAME_LENGTH: u32 = 34u32;
pub const SQL_MAXIMUM_COLUMNS_IN_GROUP_BY: u32 = 97u32;
pub const SQL_MAXIMUM_COLUMNS_IN_INDEX: u32 = 98u32;
pub const SQL_MAXIMUM_COLUMNS_IN_ORDER_BY: u32 = 99u32;
pub const SQL_MAXIMUM_COLUMNS_IN_SELECT: u32 = 100u32;
pub const SQL_MAXIMUM_COLUMN_NAME_LENGTH: u32 = 30u32;
pub const SQL_MAXIMUM_CONCURRENT_ACTIVITIES: u32 = 1u32;
pub const SQL_MAXIMUM_CURSOR_NAME_LENGTH: u32 = 31u32;
pub const SQL_MAXIMUM_DRIVER_CONNECTIONS: u32 = 0u32;
pub const SQL_MAXIMUM_IDENTIFIER_LENGTH: u32 = 10005u32;
pub const SQL_MAXIMUM_INDEX_SIZE: u32 = 102u32;
pub const SQL_MAXIMUM_ROW_SIZE: u32 = 104u32;
pub const SQL_MAXIMUM_SCHEMA_NAME_LENGTH: u32 = 32u32;
pub const SQL_MAXIMUM_STATEMENT_LENGTH: u32 = 105u32;
pub const SQL_MAXIMUM_TABLES_IN_SELECT: u32 = 106u32;
pub const SQL_MAXIMUM_USER_NAME_LENGTH: u32 = 107u32;
pub const SQL_MAX_ASYNC_CONCURRENT_STATEMENTS: u32 = 10022u32;
pub const SQL_MAX_BINARY_LITERAL_LEN: u32 = 112u32;
pub const SQL_MAX_CATALOG_NAME_LEN: u32 = 34u32;
pub const SQL_MAX_CHAR_LITERAL_LEN: u32 = 108u32;
pub const SQL_MAX_COLUMNS_IN_GROUP_BY: u32 = 97u32;
pub const SQL_MAX_COLUMNS_IN_INDEX: u32 = 98u32;
pub const SQL_MAX_COLUMNS_IN_ORDER_BY: u32 = 99u32;
pub const SQL_MAX_COLUMNS_IN_SELECT: u32 = 100u32;
pub const SQL_MAX_COLUMNS_IN_TABLE: u32 = 101u32;
pub const SQL_MAX_COLUMN_NAME_LEN: u32 = 30u32;
pub const SQL_MAX_CONCURRENT_ACTIVITIES: u32 = 1u32;
pub const SQL_MAX_CURSOR_NAME_LEN: u32 = 31u32;
pub const SQL_MAX_DRIVER_CONNECTIONS: u32 = 0u32;
pub const SQL_MAX_DSN_LENGTH: u32 = 32u32;
pub const SQL_MAX_IDENTIFIER_LEN: u32 = 10005u32;
pub const SQL_MAX_INDEX_SIZE: u32 = 102u32;
pub const SQL_MAX_LENGTH: u32 = 3u32;
pub const SQL_MAX_LENGTH_DEFAULT: u32 = 0u32;
pub const SQL_MAX_MESSAGE_LENGTH: u32 = 512u32;
pub const SQL_MAX_NUMERIC_LEN: u32 = 16u32;
pub const SQL_MAX_OPTION_STRING_LENGTH: u32 = 256u32;
pub const SQL_MAX_OWNER_NAME_LEN: u32 = 32u32;
pub const SQL_MAX_PROCEDURE_NAME_LEN: u32 = 33u32;
pub const SQL_MAX_QUALIFIER_NAME_LEN: u32 = 34u32;
pub const SQL_MAX_ROWS: u32 = 1u32;
pub const SQL_MAX_ROWS_DEFAULT: u32 = 0u32;
pub const SQL_MAX_ROW_SIZE: u32 = 104u32;
pub const SQL_MAX_ROW_SIZE_INCLUDES_LONG: u32 = 103u32;
pub const SQL_MAX_SCHEMA_NAME_LEN: u32 = 32u32;
pub const SQL_MAX_SQLSERVERNAME: u32 = 128u32;
pub const SQL_MAX_STATEMENT_LEN: u32 = 105u32;
pub const SQL_MAX_TABLES_IN_SELECT: u32 = 106u32;
pub const SQL_MAX_TABLE_NAME_LEN: u32 = 35u32;
pub const SQL_MAX_USER_NAME_LEN: u32 = 107u32;
pub const SQL_MINUTE: u32 = 5u32;
pub const SQL_MINUTE_TO_SECOND: u32 = 13u32;
pub const SQL_MODE_DEFAULT: u32 = 0u32;
pub const SQL_MODE_READ_ONLY: u32 = 1u32;
pub const SQL_MODE_READ_WRITE: u32 = 0u32;
pub const SQL_MONTH: u32 = 2u32;
pub const SQL_MORE_INFO_NO: i32 = 0i32;
pub const SQL_MORE_INFO_YES: i32 = 1i32;
pub const SQL_MULTIPLE_ACTIVE_TXN: u32 = 37u32;
pub const SQL_MULT_RESULT_SETS: u32 = 36u32;
pub const SQL_NAMED: u32 = 0u32;
pub const SQL_NB_DEFAULT: i32 = 0i32;
pub const SQL_NB_OFF: i32 = 0i32;
pub const SQL_NB_ON: i32 = 1i32;
pub const SQL_NC_END: u32 = 4u32;
pub const SQL_NC_HIGH: u32 = 0u32;
pub const SQL_NC_LOW: u32 = 1u32;
pub const SQL_NC_OFF: i32 = 0i32;
pub const SQL_NC_ON: i32 = 1i32;
pub const SQL_NC_START: u32 = 2u32;
pub const SQL_NEED_DATA: u32 = 99u32;
pub const SQL_NEED_LONG_DATA_LEN: u32 = 111u32;
pub const SQL_NNC_NON_NULL: u32 = 1u32;
pub const SQL_NNC_NULL: u32 = 0u32;
pub const SQL_NONSCROLLABLE: u32 = 0u32;
pub const SQL_NON_NULLABLE_COLUMNS: u32 = 75u32;
pub const SQL_NOSCAN: u32 = 2u32;
pub const SQL_NOSCAN_DEFAULT: u32 = 0u32;
pub const SQL_NOSCAN_OFF: u32 = 0u32;
pub const SQL_NOSCAN_ON: u32 = 1u32;
pub const SQL_NOT_DEFERRABLE: u32 = 7u32;
pub const SQL_NO_ACTION: u32 = 3u32;
pub const SQL_NO_COLUMN_NUMBER: i32 = -1i32;
pub const SQL_NO_DATA: u32 = 100u32;
pub const SQL_NO_DATA_FOUND: u32 = 100u32;
pub const SQL_NO_NULLS: u32 = 0u32;
pub const SQL_NO_ROW_NUMBER: i32 = -1i32;
pub const SQL_NO_TOTAL: i32 = -4i32;
pub const SQL_NTS: i32 = -3i32;
pub const SQL_NTSL: i32 = -3i32;
pub const SQL_NULLABLE: u32 = 1u32;
pub const SQL_NULLABLE_UNKNOWN: u32 = 2u32;
pub const SQL_NULL_COLLATION: u32 = 85u32;
pub const SQL_NULL_DATA: i32 = -1i32;
pub const SQL_NULL_HANDLE: i32 = 0i32;
pub const SQL_NULL_HDBC: u32 = 0u32;
pub const SQL_NULL_HDESC: u32 = 0u32;
pub const SQL_NULL_HENV: u32 = 0u32;
pub const SQL_NULL_HSTMT: u32 = 0u32;
pub const SQL_NUMERIC: u32 = 2u32;
pub const SQL_NUMERIC_FUNCTIONS: u32 = 49u32;
pub const SQL_NUM_FUNCTIONS: u32 = 23u32;
pub const SQL_OAC_LEVEL1: u32 = 1u32;
pub const SQL_OAC_LEVEL2: u32 = 2u32;
pub const SQL_OAC_NONE: u32 = 0u32;
pub const SQL_ODBC_API_CONFORMANCE: u32 = 9u32;
pub const SQL_ODBC_CURSORS: u32 = 110u32;
pub const SQL_ODBC_INTERFACE_CONFORMANCE: u32 = 152u32;
pub const SQL_ODBC_KEYWORDS : windows_core::PCSTR = windows_core::s ! ( "ABSOLUTE,ACTION,ADA,ADD,ALL,ALLOCATE,ALTER,AND,ANY,ARE,AS,ASC,ASSERTION,AT,AUTHORIZATION,AVG,BEGIN,BETWEEN,BIT,BIT_LENGTH,BOTH,BY,CASCADE,CASCADED,CASE,CAST,CATALOG,CHAR,CHAR_LENGTH,CHARACTER,CHARACTER_LENGTH,CHECK,CLOSE,COALESCE,COLLATE,COLLATION,COLUMN,COMMIT,CONNECT,CONNECTION,CONSTRAINT,CONSTRAINTS,CONTINUE,CONVERT,CORRESPONDING,COUNT,CREATE,CROSS,CURRENT,CURRENT_DATE,CURRENT_TIME,CURRENT_TIMESTAMP,CURRENT_USER,CURSOR,DATE,DAY,DEALLOCATE,DEC,DECIMAL,DECLARE,DEFAULT,DEFERRABLE,DEFERRED,DELETE,DESC,DESCRIBE,DESCRIPTOR,DIAGNOSTICS,DISCONNECT,DISTINCT,DOMAIN,DOUBLE,DROP,ELSE,END,END-EXEC,ESCAPE,EXCEPT,EXCEPTION,EXEC,EXECUTE,EXISTS,EXTERNAL,EXTRACT,FALSE,FETCH,FIRST,FLOAT,FOR,FOREIGN,FORTRAN,FOUND,FROM,FULL,GET,GLOBAL,GO,GOTO,GRANT,GROUP,HAVING,HOUR,IDENTITY,IMMEDIATE,IN,INCLUDE,INDEX,INDICATOR,INITIALLY,INNER,INPUT,INSENSITIVE,INSERT,INT,INTEGER,INTERSECT,INTERVAL,INTO,IS,ISOLATION,JOIN,KEY,LANGUAGE,LAST,LEADING,LEFT,LEVEL,LIKE,LOCAL,LOWER,MATCH,MAX,MIN,MINUTE,MODULE,MONTH,NAMES,NATIONAL,NATURAL,NCHAR,NEXT,NO,NONE,NOT,NULL,NULLIF,NUMERIC,OCTET_LENGTH,OF,ON,ONLY,OPEN,OPTION,OR,ORDER,OUTER,OUTPUT,OVERLAPS,PAD,PARTIAL,PASCAL,PLI,POSITION,PRECISION,PREPARE,PRESERVE,PRIMARY,PRIOR,PRIVILEGES,PROCEDURE,PUBLIC,READ,REAL,REFERENCES,RELATIVE,RESTRICT,REVOKE,RIGHT,ROLLBACK,ROWSSCHEMA,SCROLL,SECOND,SECTION,SELECT,SESSION,SESSION_USER,SET,SIZE,SMALLINT,SOME,SPACE,SQL,SQLCA,SQLCODE,SQLERROR,SQLSTATE,SQLWARNING,SUBSTRING,SUM,SYSTEM_USER,TABLE,TEMPORARY,THEN,TIME,TIMESTAMP,TIMEZONE_HOUR,TIMEZONE_MINUTE,TO,TRAILING,TRANSACTION,TRANSLATE,TRANSLATION,TRIM,TRUE,UNION,UNIQUE,UNKNOWN,UPDATE,UPPER,USAGE,USER,USING,VALUE,VALUES,VARCHAR,VARYING,VIEW,WHEN,WHENEVER,WHERE,WITH,WORK,WRITE,YEAR,ZONE" ) ;
pub const SQL_ODBC_SAG_CLI_CONFORMANCE: u32 = 12u32;
pub const SQL_ODBC_SQL_CONFORMANCE: u32 = 15u32;
pub const SQL_ODBC_SQL_OPT_IEF: u32 = 73u32;
pub const SQL_ODBC_VER: u32 = 10u32;
pub const SQL_OIC_CORE: u32 = 1u32;
pub const SQL_OIC_LEVEL1: u32 = 2u32;
pub const SQL_OIC_LEVEL2: u32 = 3u32;
pub const SQL_OJ_ALL_COMPARISON_OPS: i32 = 64i32;
pub const SQL_OJ_CAPABILITIES: u32 = 115u32;
pub const SQL_OJ_FULL: i32 = 4i32;
pub const SQL_OJ_INNER: i32 = 32i32;
pub const SQL_OJ_LEFT: i32 = 1i32;
pub const SQL_OJ_NESTED: i32 = 8i32;
pub const SQL_OJ_NOT_ORDERED: i32 = 16i32;
pub const SQL_OJ_RIGHT: i32 = 2i32;
pub const SQL_OPT_TRACE: u32 = 104u32;
pub const SQL_OPT_TRACEFILE: u32 = 105u32;
pub const SQL_OPT_TRACE_DEFAULT: u32 = 0u32;
pub const SQL_OPT_TRACE_FILE_DEFAULT: windows_core::PCSTR = windows_core::s!("\\SQL.LOG");
pub const SQL_OPT_TRACE_OFF: u32 = 0u32;
pub const SQL_OPT_TRACE_ON: u32 = 1u32;
pub const SQL_ORDER_BY_COLUMNS_IN_SELECT: u32 = 90u32;
pub const SQL_OSCC_COMPLIANT: u32 = 1u32;
pub const SQL_OSCC_NOT_COMPLIANT: u32 = 0u32;
pub const SQL_OSC_CORE: u32 = 1u32;
pub const SQL_OSC_EXTENDED: u32 = 2u32;
pub const SQL_OSC_MINIMUM: u32 = 0u32;
pub const SQL_OUTER_JOINS: u32 = 38u32;
pub const SQL_OUTER_JOIN_CAPABILITIES: u32 = 115u32;
pub const SQL_OU_DML_STATEMENTS: i32 = 1i32;
pub const SQL_OU_INDEX_DEFINITION: i32 = 8i32;
pub const SQL_OU_PRIVILEGE_DEFINITION: i32 = 16i32;
pub const SQL_OU_PROCEDURE_INVOCATION: i32 = 2i32;
pub const SQL_OU_TABLE_DEFINITION: i32 = 4i32;
pub const SQL_OV_ODBC2: u32 = 2u32;
pub const SQL_OV_ODBC3: u32 = 3u32;
pub const SQL_OV_ODBC3_80: u32 = 380u32;
pub const SQL_OWNER_TERM: u32 = 39u32;
pub const SQL_OWNER_USAGE: u32 = 91u32;
pub const SQL_PACKET_SIZE: u32 = 112u32;
pub const SQL_PARAM_ARRAY_ROW_COUNTS: u32 = 153u32;
pub const SQL_PARAM_ARRAY_SELECTS: u32 = 154u32;
pub const SQL_PARAM_BIND_BY_COLUMN: u32 = 0u32;
pub const SQL_PARAM_BIND_TYPE_DEFAULT: u32 = 0u32;
pub const SQL_PARAM_DATA_AVAILABLE: u32 = 101u32;
pub const SQL_PARAM_DIAG_UNAVAILABLE: u32 = 1u32;
pub const SQL_PARAM_ERROR: u32 = 5u32;
pub const SQL_PARAM_IGNORE: u32 = 1u32;
pub const SQL_PARAM_INPUT: u32 = 1u32;
pub const SQL_PARAM_INPUT_OUTPUT: u32 = 2u32;
pub const SQL_PARAM_INPUT_OUTPUT_STREAM: u32 = 8u32;
pub const SQL_PARAM_OUTPUT: u32 = 4u32;
pub const SQL_PARAM_OUTPUT_STREAM: u32 = 16u32;
pub const SQL_PARAM_PROCEED: u32 = 0u32;
pub const SQL_PARAM_SUCCESS: u32 = 0u32;
pub const SQL_PARAM_SUCCESS_WITH_INFO: u32 = 6u32;
pub const SQL_PARAM_TYPE_UNKNOWN: u32 = 0u32;
pub const SQL_PARAM_UNUSED: u32 = 7u32;
pub const SQL_PARC_BATCH: u32 = 1u32;
pub const SQL_PARC_NO_BATCH: u32 = 2u32;
pub const SQL_PAS_BATCH: u32 = 1u32;
pub const SQL_PAS_NO_BATCH: u32 = 2u32;
pub const SQL_PAS_NO_SELECT: u32 = 3u32;
pub const SQL_PC_DEFAULT: i32 = 0i32;
pub const SQL_PC_NON_PSEUDO: u32 = 1u32;
pub const SQL_PC_NOT_PSEUDO: u32 = 1u32;
pub const SQL_PC_OFF: i32 = 0i32;
pub const SQL_PC_ON: i32 = 1i32;
pub const SQL_PC_PSEUDO: u32 = 2u32;
pub const SQL_PC_UNKNOWN: u32 = 0u32;
pub const SQL_PERF_START: u32 = 1u32;
pub const SQL_PERF_STOP: u32 = 2u32;
pub const SQL_POSITION: u32 = 0u32;
pub const SQL_POSITIONED_STATEMENTS: u32 = 80u32;
pub const SQL_POS_ADD: i32 = 16i32;
pub const SQL_POS_DELETE: i32 = 8i32;
pub const SQL_POS_OPERATIONS: u32 = 79u32;
pub const SQL_POS_POSITION: i32 = 1i32;
pub const SQL_POS_REFRESH: i32 = 2i32;
pub const SQL_POS_UPDATE: i32 = 4i32;
pub const SQL_PRED_BASIC: u32 = 2u32;
pub const SQL_PRED_CHAR: u32 = 1u32;
pub const SQL_PRED_NONE: u32 = 0u32;
pub const SQL_PRED_SEARCHABLE: u32 = 3u32;
pub const SQL_PRESERVE_CURSORS: u32 = 1204u32;
pub const SQL_PROCEDURES: u32 = 21u32;
pub const SQL_PROCEDURE_TERM: u32 = 40u32;
pub const SQL_PS_POSITIONED_DELETE: i32 = 1i32;
pub const SQL_PS_POSITIONED_UPDATE: i32 = 2i32;
pub const SQL_PS_SELECT_FOR_UPDATE: i32 = 4i32;
pub const SQL_PT_FUNCTION: u32 = 2u32;
pub const SQL_PT_PROCEDURE: u32 = 1u32;
pub const SQL_PT_UNKNOWN: u32 = 0u32;
pub const SQL_QI_DEFAULT: i32 = 1i32;
pub const SQL_QI_OFF: i32 = 0i32;
pub const SQL_QI_ON: i32 = 1i32;
pub const SQL_QL_END: u32 = 2u32;
pub const SQL_QL_START: u32 = 1u32;
pub const SQL_QUALIFIER_LOCATION: u32 = 114u32;
pub const SQL_QUALIFIER_NAME_SEPARATOR: u32 = 41u32;
pub const SQL_QUALIFIER_TERM: u32 = 42u32;
pub const SQL_QUALIFIER_USAGE: u32 = 92u32;
pub const SQL_QUERY_TIMEOUT: u32 = 0u32;
pub const SQL_QUERY_TIMEOUT_DEFAULT: u32 = 0u32;
pub const SQL_QUICK: u32 = 0u32;
pub const SQL_QUIET_MODE: u32 = 111u32;
pub const SQL_QUOTED_IDENTIFIER_CASE: u32 = 93u32;
pub const SQL_QU_DML_STATEMENTS: i32 = 1i32;
pub const SQL_QU_INDEX_DEFINITION: i32 = 8i32;
pub const SQL_QU_PRIVILEGE_DEFINITION: i32 = 16i32;
pub const SQL_QU_PROCEDURE_INVOCATION: i32 = 2i32;
pub const SQL_QU_TABLE_DEFINITION: i32 = 4i32;
pub const SQL_RD_DEFAULT: u32 = 1u32;
pub const SQL_RD_OFF: u32 = 0u32;
pub const SQL_RD_ON: u32 = 1u32;
pub const SQL_REAL: u32 = 7u32;
pub const SQL_REFRESH: u32 = 1u32;
pub const SQL_REMOTE_PWD: u32 = 1201u32;
pub const SQL_RESET_CONNECTION_YES: u32 = 1u32;
pub const SQL_RESET_PARAMS: u32 = 3u32;
pub const SQL_RESET_YES: i32 = 1i32;
pub const SQL_RESTRICT: u32 = 1u32;
pub const SQL_RESULT_COL: u32 = 3u32;
pub const SQL_RETRIEVE_DATA: u32 = 11u32;
pub const SQL_RETURN_VALUE: u32 = 5u32;
pub const SQL_RE_DEFAULT: i32 = 0i32;
pub const SQL_RE_OFF: i32 = 0i32;
pub const SQL_RE_ON: i32 = 1i32;
pub const SQL_ROLLBACK: u32 = 1u32;
pub const SQL_ROWSET_SIZE: u32 = 9u32;
pub const SQL_ROWSET_SIZE_DEFAULT: u32 = 1u32;
pub const SQL_ROWVER: u32 = 2u32;
pub const SQL_ROW_ADDED: u32 = 4u32;
pub const SQL_ROW_DELETED: u32 = 1u32;
pub const SQL_ROW_ERROR: u32 = 5u32;
pub const SQL_ROW_IDENTIFIER: u32 = 1u32;
pub const SQL_ROW_IGNORE: u32 = 1u32;
pub const SQL_ROW_NOROW: u32 = 3u32;
pub const SQL_ROW_NUMBER: u32 = 14u32;
pub const SQL_ROW_NUMBER_UNKNOWN: i32 = -2i32;
pub const SQL_ROW_PROCEED: u32 = 0u32;
pub const SQL_ROW_SUCCESS: u32 = 0u32;
pub const SQL_ROW_SUCCESS_WITH_INFO: u32 = 6u32;
pub const SQL_ROW_UPDATED: u32 = 2u32;
pub const SQL_ROW_UPDATES: u32 = 11u32;
pub const SQL_SCCO_LOCK: i32 = 2i32;
pub const SQL_SCCO_OPT_ROWVER: i32 = 4i32;
pub const SQL_SCCO_OPT_TIMESTAMP: i32 = 4i32;
pub const SQL_SCCO_OPT_VALUES: i32 = 8i32;
pub const SQL_SCCO_READ_ONLY: i32 = 1i32;
pub const SQL_SCC_ISO92_CLI: i32 = 2i32;
pub const SQL_SCC_XOPEN_CLI_VERSION1: i32 = 1i32;
pub const SQL_SCHEMA_TERM: u32 = 39u32;
pub const SQL_SCHEMA_USAGE: u32 = 91u32;
pub const SQL_SCOPE_CURROW: u32 = 0u32;
pub const SQL_SCOPE_SESSION: u32 = 2u32;
pub const SQL_SCOPE_TRANSACTION: u32 = 1u32;
pub const SQL_SCROLLABLE: u32 = 1u32;
pub const SQL_SCROLL_CONCURRENCY: u32 = 43u32;
pub const SQL_SCROLL_DYNAMIC: i32 = -2i32;
pub const SQL_SCROLL_FORWARD_ONLY: i32 = 0i32;
pub const SQL_SCROLL_KEYSET_DRIVEN: i32 = -1i32;
pub const SQL_SCROLL_OPTIONS: u32 = 44u32;
pub const SQL_SCROLL_STATIC: i32 = -3i32;
pub const SQL_SC_FIPS127_2_TRANSITIONAL: i32 = 2i32;
pub const SQL_SC_NON_UNIQUE: u32 = 0u32;
pub const SQL_SC_SQL92_ENTRY: i32 = 1i32;
pub const SQL_SC_SQL92_FULL: i32 = 8i32;
pub const SQL_SC_SQL92_INTERMEDIATE: i32 = 4i32;
pub const SQL_SC_TRY_UNIQUE: u32 = 1u32;
pub const SQL_SC_UNIQUE: u32 = 2u32;
pub const SQL_SDF_CURRENT_DATE: i32 = 1i32;
pub const SQL_SDF_CURRENT_TIME: i32 = 2i32;
pub const SQL_SDF_CURRENT_TIMESTAMP: i32 = 4i32;
pub const SQL_SEARCHABLE: u32 = 3u32;
pub const SQL_SEARCH_PATTERN_ESCAPE: u32 = 14u32;
pub const SQL_SECOND: u32 = 6u32;
pub const SQL_SENSITIVE: u32 = 2u32;
pub const SQL_SERVER_NAME: u32 = 13u32;
pub const SQL_SETPARAM_VALUE_MAX: i32 = -1i32;
pub const SQL_SETPOS_MAX_LOCK_VALUE: u32 = 2u32;
pub const SQL_SETPOS_MAX_OPTION_VALUE: u32 = 4u32;
pub const SQL_SET_DEFAULT: u32 = 4u32;
pub const SQL_SET_NULL: u32 = 2u32;
pub const SQL_SFKD_CASCADE: i32 = 1i32;
pub const SQL_SFKD_NO_ACTION: i32 = 2i32;
pub const SQL_SFKD_SET_DEFAULT: i32 = 4i32;
pub const SQL_SFKD_SET_NULL: i32 = 8i32;
pub const SQL_SFKU_CASCADE: i32 = 1i32;
pub const SQL_SFKU_NO_ACTION: i32 = 2i32;
pub const SQL_SFKU_SET_DEFAULT: i32 = 4i32;
pub const SQL_SFKU_SET_NULL: i32 = 8i32;
pub const SQL_SG_DELETE_TABLE: i32 = 32i32;
pub const SQL_SG_INSERT_COLUMN: i32 = 128i32;
pub const SQL_SG_INSERT_TABLE: i32 = 64i32;
pub const SQL_SG_REFERENCES_COLUMN: i32 = 512i32;
pub const SQL_SG_REFERENCES_TABLE: i32 = 256i32;
pub const SQL_SG_SELECT_TABLE: i32 = 1024i32;
pub const SQL_SG_UPDATE_COLUMN: i32 = 4096i32;
pub const SQL_SG_UPDATE_TABLE: i32 = 2048i32;
pub const SQL_SG_USAGE_ON_CHARACTER_SET: i32 = 2i32;
pub const SQL_SG_USAGE_ON_COLLATION: i32 = 4i32;
pub const SQL_SG_USAGE_ON_DOMAIN: i32 = 1i32;
pub const SQL_SG_USAGE_ON_TRANSLATION: i32 = 8i32;
pub const SQL_SG_WITH_GRANT_OPTION: i32 = 16i32;
pub const SQL_SIGNED_OFFSET: i32 = -20i32;
pub const SQL_SIMULATE_CURSOR: u32 = 10u32;
pub const SQL_SMALLINT: u32 = 5u32;
pub const SQL_SNVF_BIT_LENGTH: i32 = 1i32;
pub const SQL_SNVF_CHARACTER_LENGTH: i32 = 4i32;
pub const SQL_SNVF_CHAR_LENGTH: i32 = 2i32;
pub const SQL_SNVF_EXTRACT: i32 = 8i32;
pub const SQL_SNVF_OCTET_LENGTH: i32 = 16i32;
pub const SQL_SNVF_POSITION: i32 = 32i32;
pub const SQL_SOPT_SS_BASE: u32 = 1225u32;
pub const SQL_SOPT_SS_CURRENT_COMMAND: u32 = 1226u32;
pub const SQL_SOPT_SS_CURSOR_OPTIONS: u32 = 1230u32;
pub const SQL_SOPT_SS_DEFER_PREPARE: u32 = 1232u32;
pub const SQL_SOPT_SS_HIDDEN_COLUMNS: u32 = 1227u32;
pub const SQL_SOPT_SS_MAX_USED: u32 = 1232u32;
pub const SQL_SOPT_SS_NOBROWSETABLE: u32 = 1228u32;
pub const SQL_SOPT_SS_NOCOUNT_STATUS: u32 = 1231u32;
pub const SQL_SOPT_SS_REGIONALIZE: u32 = 1229u32;
pub const SQL_SOPT_SS_TEXTPTR_LOGGING: u32 = 1225u32;
pub const SQL_SO_DYNAMIC: i32 = 4i32;
pub const SQL_SO_FORWARD_ONLY: i32 = 1i32;
pub const SQL_SO_KEYSET_DRIVEN: i32 = 2i32;
pub const SQL_SO_MIXED: i32 = 8i32;
pub const SQL_SO_STATIC: i32 = 16i32;
pub const SQL_SPECIAL_CHARACTERS: u32 = 94u32;
pub const SQL_SPEC_MAJOR: u32 = 3u32;
pub const SQL_SPEC_MINOR: u32 = 80u32;
pub const SQL_SPEC_STRING: windows_core::PCSTR = windows_core::s!("03.80");
pub const SQL_SP_BETWEEN: i32 = 2048i32;
pub const SQL_SP_COMPARISON: i32 = 4096i32;
pub const SQL_SP_EXISTS: i32 = 1i32;
pub const SQL_SP_IN: i32 = 1024i32;
pub const SQL_SP_ISNOTNULL: i32 = 2i32;
pub const SQL_SP_ISNULL: i32 = 4i32;
pub const SQL_SP_LIKE: i32 = 512i32;
pub const SQL_SP_MATCH_FULL: i32 = 8i32;
pub const SQL_SP_MATCH_PARTIAL: i32 = 16i32;
pub const SQL_SP_MATCH_UNIQUE_FULL: i32 = 32i32;
pub const SQL_SP_MATCH_UNIQUE_PARTIAL: i32 = 64i32;
pub const SQL_SP_OVERLAPS: i32 = 128i32;
pub const SQL_SP_QUANTIFIED_COMPARISON: i32 = 8192i32;
pub const SQL_SP_UNIQUE: i32 = 256i32;
pub const SQL_SQL92_DATETIME_FUNCTIONS: u32 = 155u32;
pub const SQL_SQL92_FOREIGN_KEY_DELETE_RULE: u32 = 156u32;
pub const SQL_SQL92_FOREIGN_KEY_UPDATE_RULE: u32 = 157u32;
pub const SQL_SQL92_GRANT: u32 = 158u32;
pub const SQL_SQL92_NUMERIC_VALUE_FUNCTIONS: u32 = 159u32;
pub const SQL_SQL92_PREDICATES: u32 = 160u32;
pub const SQL_SQL92_RELATIONAL_JOIN_OPERATORS: u32 = 161u32;
pub const SQL_SQL92_REVOKE: u32 = 162u32;
pub const SQL_SQL92_ROW_VALUE_CONSTRUCTOR: u32 = 163u32;
pub const SQL_SQL92_STRING_FUNCTIONS: u32 = 164u32;
pub const SQL_SQL92_VALUE_EXPRESSIONS: u32 = 165u32;
pub const SQL_SQLSTATE_SIZE: u32 = 5u32;
pub const SQL_SQLSTATE_SIZEW: u32 = 10u32;
pub const SQL_SQL_CONFORMANCE: u32 = 118u32;
pub const SQL_SQ_COMPARISON: i32 = 1i32;
pub const SQL_SQ_CORRELATED_SUBQUERIES: i32 = 16i32;
pub const SQL_SQ_EXISTS: i32 = 2i32;
pub const SQL_SQ_IN: i32 = 4i32;
pub const SQL_SQ_QUANTIFIED: i32 = 8i32;
pub const SQL_SRJO_CORRESPONDING_CLAUSE: i32 = 1i32;
pub const SQL_SRJO_CROSS_JOIN: i32 = 2i32;
pub const SQL_SRJO_EXCEPT_JOIN: i32 = 4i32;
pub const SQL_SRJO_FULL_OUTER_JOIN: i32 = 8i32;
pub const SQL_SRJO_INNER_JOIN: i32 = 16i32;
pub const SQL_SRJO_INTERSECT_JOIN: i32 = 32i32;
pub const SQL_SRJO_LEFT_OUTER_JOIN: i32 = 64i32;
pub const SQL_SRJO_NATURAL_JOIN: i32 = 128i32;
pub const SQL_SRJO_RIGHT_OUTER_JOIN: i32 = 256i32;
pub const SQL_SRJO_UNION_JOIN: i32 = 512i32;
pub const SQL_SRVC_DEFAULT: i32 = 4i32;
pub const SQL_SRVC_NULL: i32 = 2i32;
pub const SQL_SRVC_ROW_SUBQUERY: i32 = 8i32;
pub const SQL_SRVC_VALUE_EXPRESSION: i32 = 1i32;
pub const SQL_SR_CASCADE: i32 = 32i32;
pub const SQL_SR_DELETE_TABLE: i32 = 128i32;
pub const SQL_SR_GRANT_OPTION_FOR: i32 = 16i32;
pub const SQL_SR_INSERT_COLUMN: i32 = 512i32;
pub const SQL_SR_INSERT_TABLE: i32 = 256i32;
pub const SQL_SR_REFERENCES_COLUMN: i32 = 2048i32;
pub const SQL_SR_REFERENCES_TABLE: i32 = 1024i32;
pub const SQL_SR_RESTRICT: i32 = 64i32;
pub const SQL_SR_SELECT_TABLE: i32 = 4096i32;
pub const SQL_SR_UPDATE_COLUMN: i32 = 16384i32;
pub const SQL_SR_UPDATE_TABLE: i32 = 8192i32;
pub const SQL_SR_USAGE_ON_CHARACTER_SET: i32 = 2i32;
pub const SQL_SR_USAGE_ON_COLLATION: i32 = 4i32;
pub const SQL_SR_USAGE_ON_DOMAIN: i32 = 1i32;
pub const SQL_SR_USAGE_ON_TRANSLATION: i32 = 8i32;
pub const SQL_SSF_CONVERT: i32 = 1i32;
pub const SQL_SSF_LOWER: i32 = 2i32;
pub const SQL_SSF_SUBSTRING: i32 = 8i32;
pub const SQL_SSF_TRANSLATE: i32 = 16i32;
pub const SQL_SSF_TRIM_BOTH: i32 = 32i32;
pub const SQL_SSF_TRIM_LEADING: i32 = 64i32;
pub const SQL_SSF_TRIM_TRAILING: i32 = 128i32;
pub const SQL_SSF_UPPER: i32 = 4i32;
pub const SQL_SS_ADDITIONS: i32 = 1i32;
pub const SQL_SS_DELETIONS: i32 = 2i32;
pub const SQL_SS_DL_DEFAULT: windows_core::PCWSTR = windows_core::w!("STATS.LOG");
pub const SQL_SS_QI_DEFAULT: u32 = 30000u32;
pub const SQL_SS_QL_DEFAULT: windows_core::PCWSTR = windows_core::w!("QUERY.LOG");
pub const SQL_SS_UPDATES: i32 = 4i32;
pub const SQL_SS_VARIANT: i32 = -150i32;
pub const SQL_STANDARD_CLI_CONFORMANCE: u32 = 166u32;
pub const SQL_STATIC_CURSOR_ATTRIBUTES1: u32 = 167u32;
pub const SQL_STATIC_CURSOR_ATTRIBUTES2: u32 = 168u32;
pub const SQL_STATIC_SENSITIVITY: u32 = 83u32;
pub const SQL_STILL_EXECUTING: u32 = 2u32;
pub const SQL_STMT_OPT_MAX: u32 = 14u32;
pub const SQL_STMT_OPT_MIN: u32 = 0u32;
pub const SQL_STRING_FUNCTIONS: u32 = 50u32;
pub const SQL_SUBQUERIES: u32 = 95u32;
pub const SQL_SUCCESS: u32 = 0u32;
pub const SQL_SUCCESS_WITH_INFO: u32 = 1u32;
pub const SQL_SU_DML_STATEMENTS: i32 = 1i32;
pub const SQL_SU_INDEX_DEFINITION: i32 = 8i32;
pub const SQL_SU_PRIVILEGE_DEFINITION: i32 = 16i32;
pub const SQL_SU_PROCEDURE_INVOCATION: i32 = 2i32;
pub const SQL_SU_TABLE_DEFINITION: i32 = 4i32;
pub const SQL_SVE_CASE: i32 = 1i32;
pub const SQL_SVE_CAST: i32 = 2i32;
pub const SQL_SVE_COALESCE: i32 = 4i32;
pub const SQL_SVE_NULLIF: i32 = 8i32;
pub const SQL_SYSTEM_FUNCTIONS: u32 = 51u32;
pub const SQL_TABLE_STAT: u32 = 0u32;
pub const SQL_TABLE_TERM: u32 = 45u32;
pub const SQL_TC_ALL: u32 = 2u32;
pub const SQL_TC_DDL_COMMIT: u32 = 3u32;
pub const SQL_TC_DDL_IGNORE: u32 = 4u32;
pub const SQL_TC_DML: u32 = 1u32;
pub const SQL_TC_NONE: u32 = 0u32;
pub const SQL_TEXTPTR_LOGGING: u32 = 1225u32;
pub const SQL_TIME: u32 = 10u32;
pub const SQL_TIMEDATE_ADD_INTERVALS: u32 = 109u32;
pub const SQL_TIMEDATE_DIFF_INTERVALS: u32 = 110u32;
pub const SQL_TIMEDATE_FUNCTIONS: u32 = 52u32;
pub const SQL_TIMESTAMP: u32 = 11u32;
pub const SQL_TIMESTAMP_LEN: u32 = 19u32;
pub const SQL_TIME_LEN: u32 = 8u32;
pub const SQL_TINYINT: i32 = -6i32;
pub const SQL_TL_DEFAULT: i32 = 1i32;
pub const SQL_TL_OFF: i32 = 0i32;
pub const SQL_TL_ON: i32 = 1i32;
pub const SQL_TRANSACTION_CAPABLE: u32 = 46u32;
pub const SQL_TRANSACTION_ISOLATION_OPTION: u32 = 72u32;
pub const SQL_TRANSACTION_READ_COMMITTED: i32 = 2i32;
pub const SQL_TRANSACTION_READ_UNCOMMITTED: i32 = 1i32;
pub const SQL_TRANSACTION_REPEATABLE_READ: i32 = 4i32;
pub const SQL_TRANSACTION_SERIALIZABLE: i32 = 8i32;
pub const SQL_TRANSLATE_DLL: u32 = 106u32;
pub const SQL_TRANSLATE_OPTION: u32 = 107u32;
pub const SQL_TRUE: u32 = 1u32;
pub const SQL_TXN_CAPABLE: u32 = 46u32;
pub const SQL_TXN_ISOLATION: u32 = 108u32;
pub const SQL_TXN_ISOLATION_OPTION: u32 = 72u32;
pub const SQL_TXN_READ_COMMITTED: i32 = 2i32;
pub const SQL_TXN_READ_UNCOMMITTED: i32 = 1i32;
pub const SQL_TXN_REPEATABLE_READ: i32 = 4i32;
pub const SQL_TXN_SERIALIZABLE: i32 = 8i32;
pub const SQL_TXN_VERSIONING: i32 = 16i32;
pub const SQL_TYPE_DATE: u32 = 91u32;
pub const SQL_TYPE_DRIVER_END: i32 = -97i32;
pub const SQL_TYPE_DRIVER_START: i32 = -80i32;
pub const SQL_TYPE_MAX: u32 = 12u32;
pub const SQL_TYPE_MIN: i32 = -7i32;
pub const SQL_TYPE_NULL: u32 = 0u32;
pub const SQL_TYPE_TIME: u32 = 92u32;
pub const SQL_TYPE_TIMESTAMP: u32 = 93u32;
pub const SQL_UB_DEFAULT: u32 = 0u32;
pub const SQL_UB_FIXED: u32 = 1u32;
pub const SQL_UB_OFF: u32 = 0u32;
pub const SQL_UB_ON: u32 = 1u32;
pub const SQL_UB_VARIABLE: u32 = 2u32;
pub const SQL_UNBIND: u32 = 2u32;
pub const SQL_UNICODE: i32 = -95i32;
pub const SQL_UNICODE_CHAR: i32 = -95i32;
pub const SQL_UNICODE_LONGVARCHAR: i32 = -97i32;
pub const SQL_UNICODE_VARCHAR: i32 = -96i32;
pub const SQL_UNION: u32 = 96u32;
pub const SQL_UNION_STATEMENT: u32 = 96u32;
pub const SQL_UNKNOWN_TYPE: u32 = 0u32;
pub const SQL_UNNAMED: u32 = 1u32;
pub const SQL_UNSEARCHABLE: u32 = 0u32;
pub const SQL_UNSIGNED_OFFSET: i32 = -22i32;
pub const SQL_UNSPECIFIED: u32 = 0u32;
pub const SQL_UPDATE: u32 = 2u32;
pub const SQL_UPDATE_BY_BOOKMARK: u32 = 5u32;
pub const SQL_UP_DEFAULT: i32 = 1i32;
pub const SQL_UP_OFF: i32 = 0i32;
pub const SQL_UP_ON: i32 = 1i32;
pub const SQL_UP_ON_DROP: i32 = 2i32;
pub const SQL_USER_NAME: u32 = 47u32;
pub const SQL_USE_BOOKMARKS: u32 = 12u32;
pub const SQL_USE_PROCEDURE_FOR_PREPARE: u32 = 1202u32;
pub const SQL_US_UNION: i32 = 1i32;
pub const SQL_US_UNION_ALL: i32 = 2i32;
pub const SQL_U_UNION: i32 = 1i32;
pub const SQL_U_UNION_ALL: i32 = 2i32;
pub const SQL_VARBINARY: i32 = -3i32;
pub const SQL_VARCHAR: u32 = 12u32;
pub const SQL_VARLEN_DATA: i32 = -10i32;
pub const SQL_WARN_NO: i32 = 0i32;
pub const SQL_WARN_YES: i32 = 1i32;
pub const SQL_WCHAR: i32 = -8i32;
pub const SQL_WLONGVARCHAR: i32 = -10i32;
pub const SQL_WVARCHAR: i32 = -9i32;
pub const SQL_XL_DEFAULT: i32 = 1i32;
pub const SQL_XL_OFF: i32 = 0i32;
pub const SQL_XL_ON: i32 = 1i32;
pub const SQL_XOPEN_CLI_YEAR: u32 = 10000u32;
pub const SQL_YEAR: u32 = 1u32;
pub const SQL_YEAR_TO_MONTH: u32 = 7u32;
pub const SQLudtBINARY: u32 = 3u32;
pub const SQLudtBIT: u32 = 16u32;
pub const SQLudtBITN: u32 = 0u32;
pub const SQLudtCHAR: u32 = 1u32;
pub const SQLudtDATETIM4: u32 = 22u32;
pub const SQLudtDATETIME: u32 = 12u32;
pub const SQLudtDATETIMN: u32 = 15u32;
pub const SQLudtDECML: u32 = 24u32;
pub const SQLudtDECMLN: u32 = 26u32;
pub const SQLudtFLT4: u32 = 23u32;
pub const SQLudtFLT8: u32 = 8u32;
pub const SQLudtFLTN: u32 = 14u32;
pub const SQLudtIMAGE: u32 = 20u32;
pub const SQLudtINT1: u32 = 5u32;
pub const SQLudtINT2: u32 = 6u32;
pub const SQLudtINT4: u32 = 7u32;
pub const SQLudtINTN: u32 = 13u32;
pub const SQLudtMONEY: u32 = 11u32;
pub const SQLudtMONEY4: u32 = 21u32;
pub const SQLudtMONEYN: u32 = 17u32;
pub const SQLudtNUM: u32 = 10u32;
pub const SQLudtNUMN: u32 = 25u32;
pub const SQLudtSYSNAME: u32 = 18u32;
pub const SQLudtTEXT: u32 = 19u32;
pub const SQLudtTIMESTAMP: u32 = 80u32;
pub const SQLudtUNIQUEIDENTIFIER: u32 = 0u32;
pub const SQLudtVARBINARY: u32 = 4u32;
pub const SQLudtVARCHAR: u32 = 2u32;
pub const SQMO_DEFAULT_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = STRUCTURED_QUERY_MULTIOPTION(1i32);
pub const SQMO_GENERATOR_FOR_TYPE: STRUCTURED_QUERY_MULTIOPTION = STRUCTURED_QUERY_MULTIOPTION(2i32);
pub const SQMO_MAP_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = STRUCTURED_QUERY_MULTIOPTION(3i32);
pub const SQMO_VIRTUAL_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = STRUCTURED_QUERY_MULTIOPTION(0i32);
pub const SQPE_EXTRA_CLOSING_PARENTHESIS: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(2i32);
pub const SQPE_EXTRA_OPENING_PARENTHESIS: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(1i32);
pub const SQPE_IGNORED_CONNECTOR: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(4i32);
pub const SQPE_IGNORED_KEYWORD: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(5i32);
pub const SQPE_IGNORED_MODIFIER: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(3i32);
pub const SQPE_NONE: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(0i32);
pub const SQPE_UNHANDLED: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(6i32);
pub const SQRO_ADD_ROBUST_ITEM_NAME: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(512i32);
pub const SQRO_ADD_VALUE_TYPE_FOR_PLAIN_VALUES: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(256i32);
pub const SQRO_ALWAYS_ONE_INTERVAL: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(2i32);
pub const SQRO_DEFAULT: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(0i32);
pub const SQRO_DONT_MAP_RELATIONS: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(8i32);
pub const SQRO_DONT_REMOVE_UNRESTRICTED_KEYWORDS: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(32i32);
pub const SQRO_DONT_RESOLVE_DATETIME: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(1i32);
pub const SQRO_DONT_RESOLVE_RANGES: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(16i32);
pub const SQRO_DONT_SIMPLIFY_CONDITION_TREES: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(4i32);
pub const SQRO_DONT_SPLIT_WORDS: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(64i32);
pub const SQRO_IGNORE_PHRASE_ORDER: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(128i32);
pub const SQSO_AUTOMATIC_WILDCARD: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(4i32);
pub const SQSO_CONNECTOR_CASE: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(10i32);
pub const SQSO_IMPLICIT_CONNECTOR: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(9i32);
pub const SQSO_LANGUAGE_KEYWORDS: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(6i32);
pub const SQSO_LOCALE_WORD_BREAKING: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(1i32);
pub const SQSO_NATURAL_SYNTAX: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(3i32);
pub const SQSO_SCHEMA: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(0i32);
pub const SQSO_SYNTAX: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(7i32);
pub const SQSO_TIME_ZONE: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(8i32);
pub const SQSO_TRACE_LEVEL: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(5i32);
pub const SQSO_WORD_BREAKER: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(2i32);
pub const SQS_ADVANCED_QUERY_SYNTAX: STRUCTURED_QUERY_SYNTAX = STRUCTURED_QUERY_SYNTAX(1i32);
pub const SQS_NATURAL_QUERY_SYNTAX: STRUCTURED_QUERY_SYNTAX = STRUCTURED_QUERY_SYNTAX(2i32);
pub const SQS_NO_SYNTAX: STRUCTURED_QUERY_SYNTAX = STRUCTURED_QUERY_SYNTAX(0i32);
pub const SRCH_SCHEMA_CACHE_E_UNEXPECTED: i32 = -2147208447i32;
pub const SSPROPVAL_COMMANDTYPE_BULKLOAD: u32 = 22u32;
pub const SSPROPVAL_COMMANDTYPE_REGULAR: u32 = 21u32;
pub const SSPROPVAL_USEPROCFORPREP_OFF: u32 = 0u32;
pub const SSPROPVAL_USEPROCFORPREP_ON: u32 = 1u32;
pub const SSPROPVAL_USEPROCFORPREP_ON_DROP: u32 = 2u32;
pub const SSPROP_ALLOWNATIVEVARIANT: u32 = 3u32;
pub const SSPROP_AUTH_REPL_SERVER_NAME: u32 = 14u32;
pub const SSPROP_CHARACTERSET: u32 = 5u32;
pub const SSPROP_COLUMNLEVELCOLLATION: u32 = 4u32;
pub const SSPROP_COL_COLLATIONNAME: u32 = 14u32;
pub const SSPROP_CURRENTCOLLATION: u32 = 7u32;
pub const SSPROP_CURSORAUTOFETCH: u32 = 12u32;
pub const SSPROP_DEFERPREPARE: u32 = 13u32;
pub const SSPROP_ENABLEFASTLOAD: u32 = 2u32;
pub const SSPROP_FASTLOADKEEPIDENTITY: u32 = 11u32;
pub const SSPROP_FASTLOADKEEPNULLS: u32 = 10u32;
pub const SSPROP_FASTLOADOPTIONS: u32 = 9u32;
pub const SSPROP_INIT_APPNAME: u32 = 10u32;
pub const SSPROP_INIT_AUTOTRANSLATE: u32 = 8u32;
pub const SSPROP_INIT_CURRENTLANGUAGE: u32 = 4u32;
pub const SSPROP_INIT_ENCRYPT: u32 = 13u32;
pub const SSPROP_INIT_FILENAME: u32 = 12u32;
pub const SSPROP_INIT_NETWORKADDRESS: u32 = 5u32;
pub const SSPROP_INIT_NETWORKLIBRARY: u32 = 6u32;
pub const SSPROP_INIT_PACKETSIZE: u32 = 9u32;
pub const SSPROP_INIT_TAGCOLUMNCOLLATION: u32 = 15u32;
pub const SSPROP_INIT_USEPROCFORPREP: u32 = 7u32;
pub const SSPROP_INIT_WSID: u32 = 11u32;
pub const SSPROP_IRowsetFastLoad: u32 = 14u32;
pub const SSPROP_MAXBLOBLENGTH: u32 = 8u32;
pub const SSPROP_QUOTEDCATALOGNAMES: u32 = 2u32;
pub const SSPROP_SORTORDER: u32 = 6u32;
pub const SSPROP_SQLXMLXPROGID: u32 = 4u32;
pub const SSPROP_STREAM_BASEPATH: u32 = 17u32;
pub const SSPROP_STREAM_COMMANDTYPE: u32 = 18u32;
pub const SSPROP_STREAM_CONTENTTYPE: u32 = 23u32;
pub const SSPROP_STREAM_FLAGS: u32 = 20u32;
pub const SSPROP_STREAM_MAPPINGSCHEMA: u32 = 15u32;
pub const SSPROP_STREAM_XMLROOT: u32 = 19u32;
pub const SSPROP_STREAM_XSL: u32 = 16u32;
pub const SSPROP_UNICODECOMPARISONSTYLE: u32 = 3u32;
pub const SSPROP_UNICODELCID: u32 = 2u32;
pub const STD_BOOKMARKLENGTH: u32 = 1u32;
pub const STGM_COLLECTION: i32 = 8192i32;
pub const STGM_OPEN: i32 = -2147483648i32;
pub const STGM_OUTPUT: i32 = 32768i32;
pub const STGM_RECURSIVE: i32 = 16777216i32;
pub const STGM_STRICTOPEN: i32 = 1073741824i32;
pub const STREAM_FLAGS_DISALLOW_ABSOLUTE_PATH: u32 = 2u32;
pub const STREAM_FLAGS_DISALLOW_QUERY: u32 = 4u32;
pub const STREAM_FLAGS_DISALLOW_UPDATEGRAMS: u32 = 64u32;
pub const STREAM_FLAGS_DISALLOW_URL: u32 = 1u32;
pub const STREAM_FLAGS_DONTCACHEMAPPINGSCHEMA: u32 = 8u32;
pub const STREAM_FLAGS_DONTCACHETEMPLATE: u32 = 16u32;
pub const STREAM_FLAGS_DONTCACHEXSL: u32 = 32u32;
pub const STREAM_FLAGS_RESERVED: u32 = 4294901760u32;
pub const STS_ABORTXMLPARSE: i32 = -2147211756i32;
pub const STS_WS_ERROR: i32 = -2147211754i32;
pub const SUBSINFO_ALLFLAGS: u32 = 61311u32;
pub const SUBSINFO_CHANGESONLY: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(1024i32);
pub const SUBSINFO_CHANNELFLAGS: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(2048i32);
pub const SUBSINFO_FRIENDLYNAME: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(8192i32);
pub const SUBSINFO_GLEAM: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(512i32);
pub const SUBSINFO_MAILNOT: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(8i32);
pub const SUBSINFO_MAXSIZEKB: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(16i32);
pub const SUBSINFO_NEEDPASSWORD: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(16384i32);
pub const SUBSINFO_PASSWORD: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(64i32);
pub const SUBSINFO_RECURSE: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(2i32);
pub const SUBSINFO_SCHEDULE: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(1i32);
pub const SUBSINFO_TASKFLAGS: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(256i32);
pub const SUBSINFO_TYPE: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(32768i32);
pub const SUBSINFO_USER: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(32i32);
pub const SUBSINFO_WEBCRAWL: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(4i32);
pub const SUBSMGRENUM_MASK: u32 = 1u32;
pub const SUBSMGRENUM_TEMP: u32 = 1u32;
pub const SUBSMGRUPDATE_MASK: u32 = 1u32;
pub const SUBSMGRUPDATE_MINIMIZE: u32 = 1u32;
pub const SUBSSCHED_AUTO: SUBSCRIPTIONSCHEDULE = SUBSCRIPTIONSCHEDULE(0i32);
pub const SUBSSCHED_CUSTOM: SUBSCRIPTIONSCHEDULE = SUBSCRIPTIONSCHEDULE(3i32);
pub const SUBSSCHED_DAILY: SUBSCRIPTIONSCHEDULE = SUBSCRIPTIONSCHEDULE(1i32);
pub const SUBSSCHED_MANUAL: SUBSCRIPTIONSCHEDULE = SUBSCRIPTIONSCHEDULE(4i32);
pub const SUBSSCHED_WEEKLY: SUBSCRIPTIONSCHEDULE = SUBSCRIPTIONSCHEDULE(2i32);
pub const SUBSTYPE_CHANNEL: SUBSCRIPTIONTYPE = SUBSCRIPTIONTYPE(1i32);
pub const SUBSTYPE_DESKTOPCHANNEL: SUBSCRIPTIONTYPE = SUBSCRIPTIONTYPE(4i32);
pub const SUBSTYPE_DESKTOPURL: SUBSCRIPTIONTYPE = SUBSCRIPTIONTYPE(2i32);
pub const SUBSTYPE_EXTERNAL: SUBSCRIPTIONTYPE = SUBSCRIPTIONTYPE(3i32);
pub const SUBSTYPE_URL: SUBSCRIPTIONTYPE = SUBSCRIPTIONTYPE(0i32);
pub const SUCCEED: u32 = 1u32;
pub const SUCCEED_ABORT: u32 = 2u32;
pub const SUCCEED_ASYNC: u32 = 3u32;
pub const TRACE_ON: i32 = 1i32;
pub const TRACE_VERSION: u32 = 1000u32;
pub const TRACE_VS_EVENT_ON: i32 = 2i32;
pub const VT_SS_BINARY: SQLVARENUM = SQLVARENUM(207i32);
pub const VT_SS_BIT: SQLVARENUM = SQLVARENUM(11i32);
pub const VT_SS_DATETIME: SQLVARENUM = SQLVARENUM(135i32);
pub const VT_SS_DECIMAL: SQLVARENUM = SQLVARENUM(205i32);
pub const VT_SS_EMPTY: SQLVARENUM = SQLVARENUM(0i32);
pub const VT_SS_GUID: SQLVARENUM = SQLVARENUM(72i32);
pub const VT_SS_I2: SQLVARENUM = SQLVARENUM(2i32);
pub const VT_SS_I4: SQLVARENUM = SQLVARENUM(3i32);
pub const VT_SS_I8: SQLVARENUM = SQLVARENUM(20i32);
pub const VT_SS_MONEY: SQLVARENUM = SQLVARENUM(6i32);
pub const VT_SS_NULL: SQLVARENUM = SQLVARENUM(1i32);
pub const VT_SS_NUMERIC: SQLVARENUM = SQLVARENUM(131i32);
pub const VT_SS_R4: SQLVARENUM = SQLVARENUM(4i32);
pub const VT_SS_R8: SQLVARENUM = SQLVARENUM(5i32);
pub const VT_SS_SMALLDATETIME: SQLVARENUM = SQLVARENUM(206i32);
pub const VT_SS_SMALLMONEY: SQLVARENUM = SQLVARENUM(200i32);
pub const VT_SS_STRING: SQLVARENUM = SQLVARENUM(203i32);
pub const VT_SS_UI1: SQLVARENUM = SQLVARENUM(17i32);
pub const VT_SS_UNKNOWN: SQLVARENUM = SQLVARENUM(209i32);
pub const VT_SS_VARBINARY: SQLVARENUM = SQLVARENUM(208i32);
pub const VT_SS_VARSTRING: SQLVARENUM = SQLVARENUM(204i32);
pub const VT_SS_WSTRING: SQLVARENUM = SQLVARENUM(201i32);
pub const VT_SS_WVARSTRING: SQLVARENUM = SQLVARENUM(202i32);
pub const WEBCRAWL_DONT_MAKE_STICKY: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(1i32);
pub const WEBCRAWL_GET_BGSOUNDS: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(8i32);
pub const WEBCRAWL_GET_CONTROLS: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(16i32);
pub const WEBCRAWL_GET_IMAGES: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(2i32);
pub const WEBCRAWL_GET_VIDEOS: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(4i32);
pub const WEBCRAWL_IGNORE_ROBOTSTXT: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(128i32);
pub const WEBCRAWL_LINKS_ELSEWHERE: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(32i32);
pub const WEBCRAWL_ONLY_LINKS_TO_HTML: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(256i32);
pub const XML_E_BADSXQL: i32 = -2147212799i32;
pub const XML_E_NODEFAULTNS: i32 = -2147212800i32;
pub const _MAPI_E_ACCOUNT_DISABLED: i32 = -2147221212i32;
pub const _MAPI_E_BAD_CHARWIDTH: i32 = -2147221245i32;
pub const _MAPI_E_BAD_COLUMN: i32 = -2147221224i32;
pub const _MAPI_E_BUSY: i32 = -2147221237i32;
pub const _MAPI_E_COMPUTED: i32 = -2147221222i32;
pub const _MAPI_E_CORRUPT_DATA: i32 = -2147221221i32;
pub const _MAPI_E_DISK_ERROR: i32 = -2147221226i32;
pub const _MAPI_E_END_OF_SESSION: i32 = -2147220992i32;
pub const _MAPI_E_EXTENDED_ERROR: i32 = -2147221223i32;
pub const _MAPI_E_FAILONEPROVIDER: i32 = -2147221219i32;
pub const _MAPI_E_INVALID_ACCESS_TIME: i32 = -2147221213i32;
pub const _MAPI_E_INVALID_ENTRYID: i32 = -2147221241i32;
pub const _MAPI_E_INVALID_OBJECT: i32 = -2147221240i32;
pub const _MAPI_E_INVALID_WORKSTATION_ACCOUNT: i32 = -2147221214i32;
pub const _MAPI_E_LOGON_FAILED: i32 = -2147221231i32;
pub const _MAPI_E_MISSING_REQUIRED_COLUMN: i32 = -2147220990i32;
pub const _MAPI_E_NETWORK_ERROR: i32 = -2147221227i32;
pub const _MAPI_E_NOT_ENOUGH_DISK: i32 = -2147221235i32;
pub const _MAPI_E_NOT_ENOUGH_RESOURCES: i32 = -2147221234i32;
pub const _MAPI_E_NOT_FOUND: i32 = -2147221233i32;
pub const _MAPI_E_NO_SUPPORT: i32 = -2147221246i32;
pub const _MAPI_E_OBJECT_CHANGED: i32 = -2147221239i32;
pub const _MAPI_E_OBJECT_DELETED: i32 = -2147221238i32;
pub const _MAPI_E_PASSWORD_CHANGE_REQUIRED: i32 = -2147221216i32;
pub const _MAPI_E_PASSWORD_EXPIRED: i32 = -2147221215i32;
pub const _MAPI_E_SESSION_LIMIT: i32 = -2147221230i32;
pub const _MAPI_E_STRING_TOO_LONG: i32 = -2147221243i32;
pub const _MAPI_E_TOO_COMPLEX: i32 = -2147221225i32;
pub const _MAPI_E_UNABLE_TO_ABORT: i32 = -2147221228i32;
pub const _MAPI_E_UNCONFIGURED: i32 = -2147221220i32;
pub const _MAPI_E_UNKNOWN_CPID: i32 = -2147221218i32;
pub const _MAPI_E_UNKNOWN_ENTRYID: i32 = -2147220991i32;
pub const _MAPI_E_UNKNOWN_FLAGS: i32 = -2147221242i32;
pub const _MAPI_E_UNKNOWN_LCID: i32 = -2147221217i32;
pub const _MAPI_E_USER_CANCEL: i32 = -2147221229i32;
pub const _MAPI_E_VERSION: i32 = -2147221232i32;
pub const _MAPI_W_NO_SERVICE: i32 = 262659i32;
pub const eAUTH_TYPE_ANONYMOUS: AUTH_TYPE = AUTH_TYPE(0i32);
pub const eAUTH_TYPE_BASIC: AUTH_TYPE = AUTH_TYPE(2i32);
pub const eAUTH_TYPE_NTLM: AUTH_TYPE = AUTH_TYPE(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACCESS_MASKENUM(pub i32);
impl windows_core::TypeKind for ACCESS_MASKENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACCESS_MASKENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACCESS_MASKENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTH_TYPE(pub i32);
impl windows_core::TypeKind for AUTH_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTH_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTH_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CASE_REQUIREMENT(pub i32);
impl windows_core::TypeKind for CASE_REQUIREMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CASE_REQUIREMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CASE_REQUIREMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CHANNEL_AGENT_FLAGS(pub i32);
impl windows_core::TypeKind for CHANNEL_AGENT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CHANNEL_AGENT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CHANNEL_AGENT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSION_REASON(pub i32);
impl windows_core::TypeKind for CLUSION_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSION_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSION_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CONDITION_CREATION_OPTIONS(pub i32);
impl windows_core::TypeKind for CONDITION_CREATION_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CONDITION_CREATION_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CONDITION_CREATION_OPTIONS").field(&self.0).finish()
    }
}
impl CONDITION_CREATION_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CONDITION_CREATION_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CONDITION_CREATION_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CONDITION_CREATION_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CONDITION_CREATION_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CONDITION_CREATION_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CREATESUBSCRIPTIONFLAGS(pub i32);
impl windows_core::TypeKind for CREATESUBSCRIPTIONFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CREATESUBSCRIPTIONFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CREATESUBSCRIPTIONFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CatalogPausedReason(pub i32);
impl windows_core::TypeKind for CatalogPausedReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CatalogPausedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CatalogPausedReason").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CatalogStatus(pub i32);
impl windows_core::TypeKind for CatalogStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CatalogStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CatalogStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBACCESSORFLAGSENUM(pub i32);
impl windows_core::TypeKind for DBACCESSORFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBACCESSORFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBACCESSORFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBASYNCHOPENUM(pub i32);
impl windows_core::TypeKind for DBASYNCHOPENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBASYNCHOPENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBASYNCHOPENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBASYNCHPHASEENUM(pub i32);
impl windows_core::TypeKind for DBASYNCHPHASEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBASYNCHPHASEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBASYNCHPHASEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBBINDFLAGENUM(pub i32);
impl windows_core::TypeKind for DBBINDFLAGENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBBINDFLAGENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBBINDFLAGENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBBINDSTATUSENUM(pub i32);
impl windows_core::TypeKind for DBBINDSTATUSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBBINDSTATUSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBBINDSTATUSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBBINDURLFLAGENUM(pub i32);
impl windows_core::TypeKind for DBBINDURLFLAGENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBBINDURLFLAGENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBBINDURLFLAGENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBBINDURLSTATUSENUM(pub i32);
impl windows_core::TypeKind for DBBINDURLSTATUSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBBINDURLSTATUSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBBINDURLSTATUSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBBOOKMARK(pub i32);
impl windows_core::TypeKind for DBBOOKMARK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBBOOKMARK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBBOOKMARK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOLUMNDESCFLAGSENUM(pub i32);
impl windows_core::TypeKind for DBCOLUMNDESCFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOLUMNDESCFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOLUMNDESCFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOLUMNFLAGS15ENUM(pub i32);
impl windows_core::TypeKind for DBCOLUMNFLAGS15ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOLUMNFLAGS15ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGS15ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOLUMNFLAGSDEPRECATED(pub i32);
impl windows_core::TypeKind for DBCOLUMNFLAGSDEPRECATED {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOLUMNFLAGSDEPRECATED {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGSDEPRECATED").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOLUMNFLAGSENUM(pub i32);
impl windows_core::TypeKind for DBCOLUMNFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOLUMNFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOLUMNFLAGSENUM20(pub i32);
impl windows_core::TypeKind for DBCOLUMNFLAGSENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOLUMNFLAGSENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGSENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOLUMNFLAGSENUM21(pub i32);
impl windows_core::TypeKind for DBCOLUMNFLAGSENUM21 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOLUMNFLAGSENUM21 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGSENUM21").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOLUMNFLAGSENUM26(pub i32);
impl windows_core::TypeKind for DBCOLUMNFLAGSENUM26 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOLUMNFLAGSENUM26 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGSENUM26").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOMMANDPERSISTFLAGENUM(pub i32);
impl windows_core::TypeKind for DBCOMMANDPERSISTFLAGENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOMMANDPERSISTFLAGENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOMMANDPERSISTFLAGENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOMMANDPERSISTFLAGENUM21(pub i32);
impl windows_core::TypeKind for DBCOMMANDPERSISTFLAGENUM21 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOMMANDPERSISTFLAGENUM21 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOMMANDPERSISTFLAGENUM21").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOMPAREENUM(pub i32);
impl windows_core::TypeKind for DBCOMPAREENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOMPAREENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOMPAREENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOMPAREOPSENUM(pub i32);
impl windows_core::TypeKind for DBCOMPAREOPSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOMPAREOPSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOMPAREOPSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOMPAREOPSENUM20(pub i32);
impl windows_core::TypeKind for DBCOMPAREOPSENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOMPAREOPSENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOMPAREOPSENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCONSTRAINTTYPEENUM(pub i32);
impl windows_core::TypeKind for DBCONSTRAINTTYPEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCONSTRAINTTYPEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCONSTRAINTTYPEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCONVERTFLAGSENUM(pub i32);
impl windows_core::TypeKind for DBCONVERTFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCONVERTFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCONVERTFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCONVERTFLAGSENUM20(pub i32);
impl windows_core::TypeKind for DBCONVERTFLAGSENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCONVERTFLAGSENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCONVERTFLAGSENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOPYFLAGSENUM(pub i32);
impl windows_core::TypeKind for DBCOPYFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOPYFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOPYFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBCOSTUNITENUM(pub i32);
impl windows_core::TypeKind for DBCOSTUNITENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBCOSTUNITENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBCOSTUNITENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBDATACONVERTENUM(pub i32);
impl windows_core::TypeKind for DBDATACONVERTENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBDATACONVERTENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBDATACONVERTENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBDEFERRABILITYENUM(pub i32);
impl windows_core::TypeKind for DBDEFERRABILITYENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBDEFERRABILITYENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBDEFERRABILITYENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBDELETEFLAGSENUM(pub i32);
impl windows_core::TypeKind for DBDELETEFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBDELETEFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBDELETEFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBEVENTPHASEENUM(pub i32);
impl windows_core::TypeKind for DBEVENTPHASEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBEVENTPHASEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBEVENTPHASEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBEXECLIMITSENUM(pub i32);
impl windows_core::TypeKind for DBEXECLIMITSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBEXECLIMITSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBEXECLIMITSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBINDEX_COL_ORDERENUM(pub i32);
impl windows_core::TypeKind for DBINDEX_COL_ORDERENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBINDEX_COL_ORDERENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBINDEX_COL_ORDERENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBLITERALENUM(pub i32);
impl windows_core::TypeKind for DBLITERALENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBLITERALENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBLITERALENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBLITERALENUM20(pub i32);
impl windows_core::TypeKind for DBLITERALENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBLITERALENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBLITERALENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBLITERALENUM21(pub i32);
impl windows_core::TypeKind for DBLITERALENUM21 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBLITERALENUM21 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBLITERALENUM21").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBMATCHTYPEENUM(pub i32);
impl windows_core::TypeKind for DBMATCHTYPEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBMATCHTYPEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBMATCHTYPEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBMEMOWNERENUM(pub i32);
impl windows_core::TypeKind for DBMEMOWNERENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBMEMOWNERENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBMEMOWNERENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBMOVEFLAGSENUM(pub i32);
impl windows_core::TypeKind for DBMOVEFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBMOVEFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBMOVEFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPARAMFLAGSENUM(pub i32);
impl windows_core::TypeKind for DBPARAMFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPARAMFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPARAMFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPARAMFLAGSENUM20(pub i32);
impl windows_core::TypeKind for DBPARAMFLAGSENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPARAMFLAGSENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPARAMFLAGSENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPARAMIOENUM(pub i32);
impl windows_core::TypeKind for DBPARAMIOENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPARAMIOENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPARAMIOENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPARTENUM(pub i32);
impl windows_core::TypeKind for DBPARTENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPARTENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPARTENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPENDINGSTATUSENUM(pub i32);
impl windows_core::TypeKind for DBPENDINGSTATUSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPENDINGSTATUSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPENDINGSTATUSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPOSITIONFLAGSENUM(pub i32);
impl windows_core::TypeKind for DBPOSITIONFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPOSITIONFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPOSITIONFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROMPTOPTIONSENUM(pub i32);
impl windows_core::TypeKind for DBPROMPTOPTIONSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROMPTOPTIONSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROMPTOPTIONSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPENUM(pub i32);
impl windows_core::TypeKind for DBPROPENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPENUM15(pub i32);
impl windows_core::TypeKind for DBPROPENUM15 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPENUM15 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPENUM15").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPENUM20(pub i32);
impl windows_core::TypeKind for DBPROPENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPENUM21(pub i32);
impl windows_core::TypeKind for DBPROPENUM21 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPENUM21 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPENUM21").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPENUM25(pub i32);
impl windows_core::TypeKind for DBPROPENUM25 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPENUM25 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPENUM25").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPENUM25_DEPRECATED(pub i32);
impl windows_core::TypeKind for DBPROPENUM25_DEPRECATED {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPENUM25_DEPRECATED {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPENUM25_DEPRECATED").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPENUM26(pub i32);
impl windows_core::TypeKind for DBPROPENUM26 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPENUM26 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPENUM26").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPENUMDEPRECATED(pub i32);
impl windows_core::TypeKind for DBPROPENUMDEPRECATED {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPENUMDEPRECATED {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPENUMDEPRECATED").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPFLAGSENUM(pub i32);
impl windows_core::TypeKind for DBPROPFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPFLAGSENUM21(pub i32);
impl windows_core::TypeKind for DBPROPFLAGSENUM21 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPFLAGSENUM21 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPFLAGSENUM21").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPFLAGSENUM25(pub i32);
impl windows_core::TypeKind for DBPROPFLAGSENUM25 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPFLAGSENUM25 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPFLAGSENUM25").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPFLAGSENUM26(pub i32);
impl windows_core::TypeKind for DBPROPFLAGSENUM26 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPFLAGSENUM26 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPFLAGSENUM26").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPOPTIONSENUM(pub i32);
impl windows_core::TypeKind for DBPROPOPTIONSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPOPTIONSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPOPTIONSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPSTATUSENUM(pub i32);
impl windows_core::TypeKind for DBPROPSTATUSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPSTATUSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPSTATUSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBPROPSTATUSENUM21(pub i32);
impl windows_core::TypeKind for DBPROPSTATUSENUM21 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBPROPSTATUSENUM21 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBPROPSTATUSENUM21").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBRANGEENUM(pub i32);
impl windows_core::TypeKind for DBRANGEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBRANGEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBRANGEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBRANGEENUM20(pub i32);
impl windows_core::TypeKind for DBRANGEENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBRANGEENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBRANGEENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBREASONENUM(pub i32);
impl windows_core::TypeKind for DBREASONENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBREASONENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBREASONENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBREASONENUM15(pub i32);
impl windows_core::TypeKind for DBREASONENUM15 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBREASONENUM15 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBREASONENUM15").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBREASONENUM25(pub i32);
impl windows_core::TypeKind for DBREASONENUM25 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBREASONENUM25 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBREASONENUM25").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBRESOURCEKINDENUM(pub i32);
impl windows_core::TypeKind for DBRESOURCEKINDENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBRESOURCEKINDENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBRESOURCEKINDENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBRESULTFLAGENUM(pub i32);
impl windows_core::TypeKind for DBRESULTFLAGENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBRESULTFLAGENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBRESULTFLAGENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBROWCHANGEKINDENUM(pub i32);
impl windows_core::TypeKind for DBROWCHANGEKINDENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBROWCHANGEKINDENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBROWCHANGEKINDENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBROWSTATUSENUM(pub i32);
impl windows_core::TypeKind for DBROWSTATUSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBROWSTATUSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBROWSTATUSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBROWSTATUSENUM20(pub i32);
impl windows_core::TypeKind for DBROWSTATUSENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBROWSTATUSENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBROWSTATUSENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBSEEKENUM(pub i32);
impl windows_core::TypeKind for DBSEEKENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBSEEKENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBSEEKENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBSORTENUM(pub i32);
impl windows_core::TypeKind for DBSORTENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBSORTENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBSORTENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBSOURCETYPEENUM(pub i32);
impl windows_core::TypeKind for DBSOURCETYPEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBSOURCETYPEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBSOURCETYPEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBSOURCETYPEENUM20(pub i32);
impl windows_core::TypeKind for DBSOURCETYPEENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBSOURCETYPEENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBSOURCETYPEENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBSOURCETYPEENUM25(pub i32);
impl windows_core::TypeKind for DBSOURCETYPEENUM25 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBSOURCETYPEENUM25 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBSOURCETYPEENUM25").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBSTATUSENUM(pub i32);
impl windows_core::TypeKind for DBSTATUSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBSTATUSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBSTATUSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBSTATUSENUM20(pub i32);
impl windows_core::TypeKind for DBSTATUSENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBSTATUSENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBSTATUSENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBSTATUSENUM21(pub i32);
impl windows_core::TypeKind for DBSTATUSENUM21 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBSTATUSENUM21 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBSTATUSENUM21").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBSTATUSENUM25(pub i32);
impl windows_core::TypeKind for DBSTATUSENUM25 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBSTATUSENUM25 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBSTATUSENUM25").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBSTATUSENUM26(pub i32);
impl windows_core::TypeKind for DBSTATUSENUM26 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBSTATUSENUM26 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBSTATUSENUM26").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBTABLESTATISTICSTYPE26(pub i32);
impl windows_core::TypeKind for DBTABLESTATISTICSTYPE26 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBTABLESTATISTICSTYPE26 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBTABLESTATISTICSTYPE26").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBTYPEENUM(pub i32);
impl windows_core::TypeKind for DBTYPEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBTYPEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBTYPEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBTYPEENUM15(pub i32);
impl windows_core::TypeKind for DBTYPEENUM15 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBTYPEENUM15 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBTYPEENUM15").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBTYPEENUM20(pub i32);
impl windows_core::TypeKind for DBTYPEENUM20 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBTYPEENUM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBTYPEENUM20").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBUPDELRULEENUM(pub i32);
impl windows_core::TypeKind for DBUPDELRULEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBUPDELRULEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBUPDELRULEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBWATCHMODEENUM(pub i32);
impl windows_core::TypeKind for DBWATCHMODEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBWATCHMODEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBWATCHMODEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DBWATCHNOTIFYENUM(pub i32);
impl windows_core::TypeKind for DBWATCHNOTIFYENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DBWATCHNOTIFYENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DBWATCHNOTIFYENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DCINFOTYPEENUM(pub i32);
impl windows_core::TypeKind for DCINFOTYPEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DCINFOTYPEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DCINFOTYPEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DELIVERY_AGENT_FLAGS(pub i32);
impl windows_core::TypeKind for DELIVERY_AGENT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DELIVERY_AGENT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DELIVERY_AGENT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EBindInfoOptions(pub i32);
impl windows_core::TypeKind for EBindInfoOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EBindInfoOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EBindInfoOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FOLLOW_FLAGS(pub i32);
impl windows_core::TypeKind for FOLLOW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FOLLOW_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FOLLOW_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INTERVAL_LIMIT_KIND(pub i32);
impl windows_core::TypeKind for INTERVAL_LIMIT_KIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INTERVAL_LIMIT_KIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INTERVAL_LIMIT_KIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KAGREQDIAGFLAGSENUM(pub i32);
impl windows_core::TypeKind for KAGREQDIAGFLAGSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KAGREQDIAGFLAGSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KAGREQDIAGFLAGSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOCKMODEENUM(pub i32);
impl windows_core::TypeKind for LOCKMODEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOCKMODEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOCKMODEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSDSDBINITPROPENUM(pub i32);
impl windows_core::TypeKind for MSDSDBINITPROPENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSDSDBINITPROPENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSDSDBINITPROPENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSDSSESSIONPROPENUM(pub i32);
impl windows_core::TypeKind for MSDSSESSIONPROPENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSDSSESSIONPROPENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSDSSESSIONPROPENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NAMED_ENTITY_CERTAINTY(pub i32);
impl windows_core::TypeKind for NAMED_ENTITY_CERTAINTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NAMED_ENTITY_CERTAINTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NAMED_ENTITY_CERTAINTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OSPCOMP(pub i32);
impl windows_core::TypeKind for OSPCOMP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OSPCOMP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OSPCOMP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OSPFIND(pub i32);
impl windows_core::TypeKind for OSPFIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OSPFIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OSPFIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OSPFORMAT(pub i32);
impl windows_core::TypeKind for OSPFORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OSPFORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OSPFORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OSPRW(pub i32);
impl windows_core::TypeKind for OSPRW {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OSPRW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OSPRW").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OSPXFER(pub i32);
impl windows_core::TypeKind for OSPXFER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OSPXFER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OSPXFER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRIORITIZE_FLAGS(pub i32);
impl windows_core::TypeKind for PRIORITIZE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRIORITIZE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRIORITIZE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRIORITY_LEVEL(pub i32);
impl windows_core::TypeKind for PRIORITY_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRIORITY_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRIORITY_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROXY_ACCESS(pub i32);
impl windows_core::TypeKind for PROXY_ACCESS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROXY_ACCESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROXY_ACCESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QUERY_PARSER_MANAGER_OPTION(pub i32);
impl windows_core::TypeKind for QUERY_PARSER_MANAGER_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QUERY_PARSER_MANAGER_OPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QUERY_PARSER_MANAGER_OPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ROWSETEVENT_ITEMSTATE(pub i32);
impl windows_core::TypeKind for ROWSETEVENT_ITEMSTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ROWSETEVENT_ITEMSTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ROWSETEVENT_ITEMSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ROWSETEVENT_TYPE(pub i32);
impl windows_core::TypeKind for ROWSETEVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ROWSETEVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ROWSETEVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SEARCH_INDEXING_PHASE(pub i32);
impl windows_core::TypeKind for SEARCH_INDEXING_PHASE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SEARCH_INDEXING_PHASE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SEARCH_INDEXING_PHASE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SEARCH_KIND_OF_CHANGE(pub i32);
impl windows_core::TypeKind for SEARCH_KIND_OF_CHANGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SEARCH_KIND_OF_CHANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SEARCH_KIND_OF_CHANGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SEARCH_NOTIFICATION_PRIORITY(pub i32);
impl windows_core::TypeKind for SEARCH_NOTIFICATION_PRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SEARCH_NOTIFICATION_PRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SEARCH_NOTIFICATION_PRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SEARCH_QUERY_SYNTAX(pub i32);
impl windows_core::TypeKind for SEARCH_QUERY_SYNTAX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SEARCH_QUERY_SYNTAX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SEARCH_QUERY_SYNTAX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SEARCH_TERM_EXPANSION(pub i32);
impl windows_core::TypeKind for SEARCH_TERM_EXPANSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SEARCH_TERM_EXPANSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SEARCH_TERM_EXPANSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SQLINTERVAL(pub i32);
impl windows_core::TypeKind for SQLINTERVAL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SQLINTERVAL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SQLINTERVAL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SQLVARENUM(pub i32);
impl windows_core::TypeKind for SQLVARENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SQLVARENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SQLVARENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STRUCTURED_QUERY_MULTIOPTION(pub i32);
impl windows_core::TypeKind for STRUCTURED_QUERY_MULTIOPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STRUCTURED_QUERY_MULTIOPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STRUCTURED_QUERY_MULTIOPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STRUCTURED_QUERY_PARSE_ERROR(pub i32);
impl windows_core::TypeKind for STRUCTURED_QUERY_PARSE_ERROR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STRUCTURED_QUERY_PARSE_ERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STRUCTURED_QUERY_PARSE_ERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STRUCTURED_QUERY_RESOLVE_OPTION(pub i32);
impl windows_core::TypeKind for STRUCTURED_QUERY_RESOLVE_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STRUCTURED_QUERY_RESOLVE_OPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STRUCTURED_QUERY_RESOLVE_OPTION").field(&self.0).finish()
    }
}
impl STRUCTURED_QUERY_RESOLVE_OPTION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for STRUCTURED_QUERY_RESOLVE_OPTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for STRUCTURED_QUERY_RESOLVE_OPTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for STRUCTURED_QUERY_RESOLVE_OPTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for STRUCTURED_QUERY_RESOLVE_OPTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for STRUCTURED_QUERY_RESOLVE_OPTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STRUCTURED_QUERY_SINGLE_OPTION(pub i32);
impl windows_core::TypeKind for STRUCTURED_QUERY_SINGLE_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STRUCTURED_QUERY_SINGLE_OPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STRUCTURED_QUERY_SINGLE_OPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STRUCTURED_QUERY_SYNTAX(pub i32);
impl windows_core::TypeKind for STRUCTURED_QUERY_SYNTAX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STRUCTURED_QUERY_SYNTAX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STRUCTURED_QUERY_SYNTAX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SUBSCRIPTIONINFOFLAGS(pub i32);
impl windows_core::TypeKind for SUBSCRIPTIONINFOFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SUBSCRIPTIONINFOFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SUBSCRIPTIONINFOFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SUBSCRIPTIONSCHEDULE(pub i32);
impl windows_core::TypeKind for SUBSCRIPTIONSCHEDULE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SUBSCRIPTIONSCHEDULE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SUBSCRIPTIONSCHEDULE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SUBSCRIPTIONTYPE(pub i32);
impl windows_core::TypeKind for SUBSCRIPTIONTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SUBSCRIPTIONTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SUBSCRIPTIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WEBCRAWL_RECURSEFLAGS(pub i32);
impl windows_core::TypeKind for WEBCRAWL_RECURSEFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WEBCRAWL_RECURSEFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WEBCRAWL_RECURSEFLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTHENTICATION_INFO {
    pub dwSize: u32,
    pub atAuthenticationType: AUTH_TYPE,
    pub pcwszUser: windows_core::PCWSTR,
    pub pcwszPassword: windows_core::PCWSTR,
}
impl windows_core::TypeKind for AUTHENTICATION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BUCKETCATEGORIZE {
    pub cBuckets: u32,
    pub Distribution: u32,
}
impl windows_core::TypeKind for BUCKETCATEGORIZE {
    type TypeKind = windows_core::CopyType;
}
impl Default for BUCKETCATEGORIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy)]
pub struct CATEGORIZATION {
    pub ulCatType: u32,
    pub Anonymous: CATEGORIZATION_0,
    pub csColumns: COLUMNSET,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for CATEGORIZATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for CATEGORIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy)]
pub union CATEGORIZATION_0 {
    pub cClusters: u32,
    pub bucket: BUCKETCATEGORIZE,
    pub range: RANGECATEGORIZE,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for CATEGORIZATION_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for CATEGORIZATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CATEGORIZATIONSET {
    pub cCat: u32,
    pub aCat: *mut CATEGORIZATION,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for CATEGORIZATIONSET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for CATEGORIZATIONSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COLUMNSET {
    pub cCol: u32,
    pub aCol: *mut super::super::Storage::IndexServer::FULLPROPSPEC,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for COLUMNSET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for COLUMNSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy)]
pub struct CONTENTRESTRICTION {
    pub prop: super::super::Storage::IndexServer::FULLPROPSPEC,
    pub pwcsPhrase: windows_core::PWSTR,
    pub lcid: u32,
    pub ulGenerateMethod: u32,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for CONTENTRESTRICTION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for CONTENTRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CSearchLanguageSupport: windows_core::GUID = windows_core::GUID::from_u128(0x6a68cc80_4337_4dbc_bd27_fbfb1053820b);
pub const CSearchManager: windows_core::GUID = windows_core::GUID::from_u128(0x7d096c5f_ac08_4f1f_beb7_5c22c517ce39);
pub const CSearchRoot: windows_core::GUID = windows_core::GUID::from_u128(0x30766bd2_ea1c_4f28_bf27_0b44e2f68db7);
pub const CSearchScopeRule: windows_core::GUID = windows_core::GUID::from_u128(0xe63de750_3bd7_4be5_9c84_6b4281988c44);
pub const CompoundCondition: windows_core::GUID = windows_core::GUID::from_u128(0x116f8d13_101e_4fa5_84d4_ff8279381935);
pub const ConditionFactory: windows_core::GUID = windows_core::GUID::from_u128(0xe03e85b0_7be3_4000_ba98_6c13de9fa486);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DATE_STRUCT {
    pub year: i16,
    pub month: u16,
    pub day: u16,
}
impl windows_core::TypeKind for DATE_STRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DATE_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBBINDEXT {
    pub pExtension: *mut u8,
    pub ulExtension: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBBINDEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBBINDEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBBINDEXT {
    pub pExtension: *mut u8,
    pub ulExtension: usize,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBBINDEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBBINDEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
pub struct DBBINDING {
    pub iOrdinal: usize,
    pub obValue: usize,
    pub obLength: usize,
    pub obStatus: usize,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::Com::ITypeInfo>>,
    pub pObject: *mut DBOBJECT,
    pub pBindExt: *mut DBBINDEXT,
    pub dwPart: u32,
    pub dwMemOwner: u32,
    pub eParamIO: u32,
    pub cbMaxLen: usize,
    pub dwFlags: u32,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl Clone for DBBINDING {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for DBBINDING {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl Default for DBBINDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
pub struct DBBINDING {
    pub iOrdinal: usize,
    pub obValue: usize,
    pub obLength: usize,
    pub obStatus: usize,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::Com::ITypeInfo>>,
    pub pObject: *mut DBOBJECT,
    pub pBindExt: *mut DBBINDEXT,
    pub dwPart: u32,
    pub dwMemOwner: u32,
    pub eParamIO: u32,
    pub cbMaxLen: usize,
    pub dwFlags: u32,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for DBBINDING {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl Default for DBBINDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct DBCOLUMNACCESS {
    pub pData: *mut core::ffi::c_void,
    pub columnid: super::super::Storage::IndexServer::DBID,
    pub cbDataLen: usize,
    pub dwStatus: u32,
    pub cbMaxLen: usize,
    pub dwReserved: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for DBCOLUMNACCESS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for DBCOLUMNACCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct DBCOLUMNACCESS {
    pub pData: *mut core::ffi::c_void,
    pub columnid: super::super::Storage::IndexServer::DBID,
    pub cbDataLen: usize,
    pub dwStatus: u32,
    pub cbMaxLen: usize,
    pub dwReserved: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for DBCOLUMNACCESS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for DBCOLUMNACCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub struct DBCOLUMNDESC {
    pub pwszTypeName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::Com::ITypeInfo>>,
    pub rgPropertySets: *mut DBPROPSET,
    pub pclsid: *mut windows_core::GUID,
    pub cPropertySets: u32,
    pub ulColumnSize: usize,
    pub dbcid: super::super::Storage::IndexServer::DBID,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl Clone for DBCOLUMNDESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::TypeKind for DBCOLUMNDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl Default for DBCOLUMNDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub struct DBCOLUMNDESC {
    pub pwszTypeName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::Com::ITypeInfo>>,
    pub rgPropertySets: *mut DBPROPSET,
    pub pclsid: *mut windows_core::GUID,
    pub cPropertySets: u32,
    pub ulColumnSize: usize,
    pub dbcid: super::super::Storage::IndexServer::DBID,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::TypeKind for DBCOLUMNDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl Default for DBCOLUMNDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub struct DBCOLUMNINFO {
    pub pwszName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::Com::ITypeInfo>>,
    pub iOrdinal: usize,
    pub dwFlags: u32,
    pub ulColumnSize: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
    pub columnid: super::super::Storage::IndexServer::DBID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl Clone for DBCOLUMNINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::TypeKind for DBCOLUMNINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl Default for DBCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub struct DBCOLUMNINFO {
    pub pwszName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::Com::ITypeInfo>>,
    pub iOrdinal: usize,
    pub dwFlags: u32,
    pub ulColumnSize: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
    pub columnid: super::super::Storage::IndexServer::DBID,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::TypeKind for DBCOLUMNINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl Default for DBCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct DBCONSTRAINTDESC {
    pub pConstraintID: *mut super::super::Storage::IndexServer::DBID,
    pub ConstraintType: u32,
    pub cColumns: usize,
    pub rgColumnList: *mut super::super::Storage::IndexServer::DBID,
    pub pReferencedTableID: *mut super::super::Storage::IndexServer::DBID,
    pub cForeignKeyColumns: usize,
    pub rgForeignKeyColumnList: *mut super::super::Storage::IndexServer::DBID,
    pub pwszConstraintText: windows_core::PWSTR,
    pub UpdateRule: u32,
    pub DeleteRule: u32,
    pub MatchType: u32,
    pub Deferrability: u32,
    pub cReserved: usize,
    pub rgReserved: *mut DBPROPSET,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for DBCONSTRAINTDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for DBCONSTRAINTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct DBCONSTRAINTDESC {
    pub pConstraintID: *mut super::super::Storage::IndexServer::DBID,
    pub ConstraintType: u32,
    pub cColumns: usize,
    pub rgColumnList: *mut super::super::Storage::IndexServer::DBID,
    pub pReferencedTableID: *mut super::super::Storage::IndexServer::DBID,
    pub cForeignKeyColumns: usize,
    pub rgForeignKeyColumnList: *mut super::super::Storage::IndexServer::DBID,
    pub pwszConstraintText: windows_core::PWSTR,
    pub UpdateRule: u32,
    pub DeleteRule: u32,
    pub MatchType: u32,
    pub Deferrability: u32,
    pub cReserved: usize,
    pub rgReserved: *mut DBPROPSET,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for DBCONSTRAINTDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for DBCONSTRAINTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBCOST {
    pub eKind: u32,
    pub dwUnits: u32,
    pub lValue: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBCOST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBCOST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBCOST {
    pub eKind: u32,
    pub dwUnits: u32,
    pub lValue: i32,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBCOST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBCOST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBDATE {
    pub year: i16,
    pub month: u16,
    pub day: u16,
}
impl windows_core::TypeKind for DBDATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DBDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBDATETIM4 {
    pub numdays: u16,
    pub nummins: u16,
}
impl windows_core::TypeKind for DBDATETIM4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DBDATETIM4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBDATETIME {
    pub dtdays: i32,
    pub dttime: u32,
}
impl windows_core::TypeKind for DBDATETIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for DBDATETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBFAILUREINFO {
    pub hRow: usize,
    pub iColumn: usize,
    pub failure: windows_core::HRESULT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBFAILUREINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBFAILUREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBFAILUREINFO {
    pub hRow: usize,
    pub iColumn: usize,
    pub failure: windows_core::HRESULT,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBFAILUREINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBFAILUREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub struct DBIMPLICITSESSION {
    pub pUnkOuter: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub piid: *mut windows_core::GUID,
    pub pSession: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Clone for DBIMPLICITSESSION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBIMPLICITSESSION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBIMPLICITSESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
pub struct DBIMPLICITSESSION {
    pub pUnkOuter: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub piid: *mut windows_core::GUID,
    pub pSession: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBIMPLICITSESSION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBIMPLICITSESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct DBINDEXCOLUMNDESC {
    pub pColumnID: *mut super::super::Storage::IndexServer::DBID,
    pub eIndexColOrder: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for DBINDEXCOLUMNDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for DBINDEXCOLUMNDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct DBINDEXCOLUMNDESC {
    pub pColumnID: *mut super::super::Storage::IndexServer::DBID,
    pub eIndexColOrder: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for DBINDEXCOLUMNDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for DBINDEXCOLUMNDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBLITERALINFO {
    pub pwszLiteralValue: windows_core::PWSTR,
    pub pwszInvalidChars: windows_core::PWSTR,
    pub pwszInvalidStartingChars: windows_core::PWSTR,
    pub lt: u32,
    pub fSupported: super::super::Foundation::BOOL,
    pub cchMaxLen: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBLITERALINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBLITERALINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBLITERALINFO {
    pub pwszLiteralValue: windows_core::PWSTR,
    pub pwszInvalidChars: windows_core::PWSTR,
    pub pwszInvalidStartingChars: windows_core::PWSTR,
    pub lt: u32,
    pub fSupported: super::super::Foundation::BOOL,
    pub cchMaxLen: u32,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBLITERALINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBLITERALINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBMONEY {
    pub mnyhigh: i32,
    pub mnylow: u32,
}
impl windows_core::TypeKind for DBMONEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DBMONEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBOBJECT {
    pub dwFlags: u32,
    pub iid: windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBOBJECT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBOBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBOBJECT {
    pub dwFlags: u32,
    pub iid: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBOBJECT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBOBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBPARAMBINDINFO {
    pub pwszDataSourceType: windows_core::PWSTR,
    pub pwszName: windows_core::PWSTR,
    pub ulParamSize: usize,
    pub dwFlags: u32,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBPARAMBINDINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBPARAMBINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBPARAMBINDINFO {
    pub pwszDataSourceType: windows_core::PWSTR,
    pub pwszName: windows_core::PWSTR,
    pub ulParamSize: usize,
    pub dwFlags: u32,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBPARAMBINDINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBPARAMBINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
pub struct DBPARAMINFO {
    pub dwFlags: u32,
    pub iOrdinal: usize,
    pub pwszName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::Com::ITypeInfo>>,
    pub ulParamSize: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl Clone for DBPARAMINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for DBPARAMINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl Default for DBPARAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
pub struct DBPARAMINFO {
    pub dwFlags: u32,
    pub iOrdinal: usize,
    pub pwszName: windows_core::PWSTR,
    pub pTypeInfo: core::mem::ManuallyDrop<Option<super::Com::ITypeInfo>>,
    pub ulParamSize: usize,
    pub wType: u16,
    pub bPrecision: u8,
    pub bScale: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for DBPARAMINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl Default for DBPARAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBPARAMS {
    pub pData: *mut core::ffi::c_void,
    pub cParamSets: usize,
    pub hAccessor: HACCESSOR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBPARAMS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBPARAMS {
    pub pData: *mut core::ffi::c_void,
    pub cParamSets: usize,
    pub hAccessor: HACCESSOR,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBPARAMS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub struct DBPROP {
    pub dwPropertyID: u32,
    pub dwOptions: u32,
    pub dwStatus: u32,
    pub colid: super::super::Storage::IndexServer::DBID,
    pub vValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Clone for DBPROP {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for DBPROP {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for DBPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub struct DBPROP {
    pub dwPropertyID: u32,
    pub dwOptions: u32,
    pub dwStatus: u32,
    pub colid: super::super::Storage::IndexServer::DBID,
    pub vValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for DBPROP {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for DBPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBPROPIDSET {
    pub rgPropertyIDs: *mut u32,
    pub cPropertyIDs: u32,
    pub guidPropertySet: windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBPROPIDSET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBPROPIDSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBPROPIDSET {
    pub rgPropertyIDs: *mut u32,
    pub cPropertyIDs: u32,
    pub guidPropertySet: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBPROPIDSET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBPROPIDSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Variant")]
pub struct DBPROPINFO {
    pub pwszDescription: windows_core::PWSTR,
    pub dwPropertyID: u32,
    pub dwFlags: u32,
    pub vtType: super::Variant::VARENUM,
    pub vValues: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Variant")]
impl Clone for DBPROPINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::TypeKind for DBPROPINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Variant")]
impl Default for DBPROPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Variant")]
pub struct DBPROPINFO {
    pub pwszDescription: windows_core::PWSTR,
    pub dwPropertyID: u32,
    pub dwFlags: u32,
    pub vtType: super::Variant::VARENUM,
    pub vValues: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::TypeKind for DBPROPINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Variant")]
impl Default for DBPROPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy)]
pub struct DBPROPINFOSET {
    pub rgPropertyInfos: *mut DBPROPINFO,
    pub cPropertyInfos: u32,
    pub guidPropertySet: windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::TypeKind for DBPROPINFOSET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Variant")]
impl Default for DBPROPINFOSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy)]
pub struct DBPROPINFOSET {
    pub rgPropertyInfos: *mut DBPROPINFO,
    pub cPropertyInfos: u32,
    pub guidPropertySet: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::TypeKind for DBPROPINFOSET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Variant")]
impl Default for DBPROPINFOSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct DBPROPSET {
    pub rgProperties: *mut DBPROP,
    pub cProperties: u32,
    pub guidPropertySet: windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for DBPROPSET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for DBPROPSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct DBPROPSET {
    pub rgProperties: *mut DBPROP,
    pub cProperties: u32,
    pub guidPropertySet: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for DBPROPSET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for DBPROPSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBROWWATCHCHANGE {
    pub hRegion: usize,
    pub eChangeKind: u32,
    pub hRow: usize,
    pub iRow: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBROWWATCHCHANGE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBROWWATCHCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBROWWATCHCHANGE {
    pub hRegion: usize,
    pub eChangeKind: u32,
    pub hRow: usize,
    pub iRow: usize,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBROWWATCHCHANGE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBROWWATCHCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBTIME {
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
}
impl windows_core::TypeKind for DBTIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for DBTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBTIMESTAMP {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub fraction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBTIMESTAMP {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBTIMESTAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBTIMESTAMP {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub fraction: u32,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBTIMESTAMP {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBTIMESTAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBVARYBIN {
    pub len: i16,
    pub array: [u8; 8001],
}
impl windows_core::TypeKind for DBVARYBIN {
    type TypeKind = windows_core::CopyType;
}
impl Default for DBVARYBIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DBVARYCHAR {
    pub len: i16,
    pub str: [i8; 8001],
}
impl windows_core::TypeKind for DBVARYCHAR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DBVARYCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBVECTOR {
    pub size: usize,
    pub ptr: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DBVECTOR {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBVECTOR {
    pub size: usize,
    pub ptr: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for DBVECTOR {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for DBVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DB_NUMERIC {
    pub precision: u8,
    pub scale: u8,
    pub sign: u8,
    pub val: [u8; 16],
}
impl windows_core::TypeKind for DB_NUMERIC {
    type TypeKind = windows_core::CopyType;
}
impl Default for DB_NUMERIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DB_VARNUMERIC {
    pub precision: u8,
    pub scale: i8,
    pub sign: u8,
    pub val: [u8; 1],
}
impl windows_core::TypeKind for DB_VARNUMERIC {
    type TypeKind = windows_core::CopyType;
}
impl Default for DB_VARNUMERIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct DCINFO {
    pub eInfoType: u32,
    pub vData: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
impl Clone for DCINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for DCINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DCINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DataLinks: windows_core::GUID = windows_core::GUID::from_u128(0x2206cdb2_19c1_11d1_89e0_00c04fd7a829);
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct ERRORINFO {
    pub hrError: windows_core::HRESULT,
    pub dwMinor: u32,
    pub clsid: windows_core::GUID,
    pub iid: windows_core::GUID,
    pub dispid: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for ERRORINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for ERRORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct ERRORINFO {
    pub hrError: windows_core::HRESULT,
    pub dwMinor: u32,
    pub clsid: windows_core::GUID,
    pub iid: windows_core::GUID,
    pub dispid: i32,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for ERRORINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for ERRORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTERED_DATA_SOURCES {
    pub pwcsExtension: windows_core::PCWSTR,
    pub pwcsMime: windows_core::PCWSTR,
    pub pClsid: *const windows_core::GUID,
    pub pwcsOverride: windows_core::PCWSTR,
}
impl windows_core::TypeKind for FILTERED_DATA_SOURCES {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTERED_DATA_SOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FilterRegistration: windows_core::GUID = windows_core::GUID::from_u128(0x9e175b8d_f52a_11d8_b9a5_505054503030);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACCESSOR(pub usize);
impl HACCESSOR {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for HACCESSOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HACCESSOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HITRANGE {
    pub iPosition: u32,
    pub cLength: u32,
}
impl windows_core::TypeKind for HITRANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for HITRANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INCREMENTAL_ACCESS_INFO {
    pub dwSize: u32,
    pub ftLastModifiedTime: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for INCREMENTAL_ACCESS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for INCREMENTAL_ACCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct ITEMPROP {
    pub variantValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
    pub pwszName: windows_core::PWSTR,
}
impl Clone for ITEMPROP {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for ITEMPROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for ITEMPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ITEM_INFO {
    pub dwSize: u32,
    pub pcwszFromEMail: windows_core::PCWSTR,
    pub pcwszApplicationName: windows_core::PCWSTR,
    pub pcwszCatalogName: windows_core::PCWSTR,
    pub pcwszContentClass: windows_core::PCWSTR,
}
impl windows_core::TypeKind for ITEM_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for ITEM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Interval: windows_core::GUID = windows_core::GUID::from_u128(0xd957171f_4bf9_4de2_bcd5_c70a7ca55836);
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct KAGGETDIAG {
    pub ulSize: u32,
    pub vDiagInfo: core::mem::ManuallyDrop<windows_core::VARIANT>,
    pub sDiagField: i16,
}
impl Clone for KAGGETDIAG {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for KAGGETDIAG {
    type TypeKind = windows_core::CopyType;
}
impl Default for KAGGETDIAG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KAGREQDIAG {
    pub ulDiagFlags: u32,
    pub vt: super::Variant::VARENUM,
    pub sDiagField: i16,
}
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::TypeKind for KAGREQDIAG {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for KAGREQDIAG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LeafCondition: windows_core::GUID = windows_core::GUID::from_u128(0x52f15c89_5a17_48e1_bbcd_46a3f89c7cc2);
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct MDAXISINFO {
    pub cbSize: usize,
    pub iAxis: usize,
    pub cDimensions: usize,
    pub cCoordinates: usize,
    pub rgcColumns: *mut usize,
    pub rgpwszDimensionNames: *mut windows_core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for MDAXISINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MDAXISINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct MDAXISINFO {
    pub cbSize: usize,
    pub iAxis: usize,
    pub cDimensions: usize,
    pub cCoordinates: usize,
    pub rgcColumns: *mut usize,
    pub rgpwszDimensionNames: *mut windows_core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for MDAXISINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for MDAXISINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSDAINITIALIZE: windows_core::GUID = windows_core::GUID::from_u128(0x2206cdb0_19c1_11d1_89e0_00c04fd7a829);
pub const MSDAORA: windows_core::GUID = windows_core::GUID::from_u128(0xe8cc4cbe_fdff_11d0_b865_00a0c9081c1d);
pub const MSDAORA8: windows_core::GUID = windows_core::GUID::from_u128(0x7f06a373_dd6a_43db_b4e0_1fc121e5e62b);
pub const MSDAORA8_ERROR: windows_core::GUID = windows_core::GUID::from_u128(0x7f06a374_dd6a_43db_b4e0_1fc121e5e62b);
pub const MSDAORA_ERROR: windows_core::GUID = windows_core::GUID::from_u128(0xe8cc4cbf_fdff_11d0_b865_00a0c9081c1d);
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy)]
pub struct NATLANGUAGERESTRICTION {
    pub prop: super::super::Storage::IndexServer::FULLPROPSPEC,
    pub pwcsPhrase: windows_core::PWSTR,
    pub lcid: u32,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for NATLANGUAGERESTRICTION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for NATLANGUAGERESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NODERESTRICTION {
    pub cRes: u32,
    pub paRes: *mut *mut RESTRICTION,
    pub reserved: u32,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for NODERESTRICTION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for NODERESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NOTRESTRICTION {
    pub pRes: *mut RESTRICTION,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for NOTRESTRICTION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for NOTRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NegationCondition: windows_core::GUID = windows_core::GUID::from_u128(0x8de9c74c_605a_4acd_bee3_2b222aa2d23d);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ODBC_VS_ARGS {
    pub pguidEvent: *const windows_core::GUID,
    pub dwFlags: u32,
    pub Anonymous1: ODBC_VS_ARGS_0,
    pub Anonymous2: ODBC_VS_ARGS_1,
    pub RetCode: i16,
}
impl windows_core::TypeKind for ODBC_VS_ARGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ODBC_VS_ARGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ODBC_VS_ARGS_0 {
    pub wszArg: windows_core::PWSTR,
    pub szArg: windows_core::PSTR,
}
impl windows_core::TypeKind for ODBC_VS_ARGS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ODBC_VS_ARGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ODBC_VS_ARGS_1 {
    pub wszCorrelation: windows_core::PWSTR,
    pub szCorrelation: windows_core::PSTR,
}
impl windows_core::TypeKind for ODBC_VS_ARGS_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ODBC_VS_ARGS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDPO: windows_core::GUID = windows_core::GUID::from_u128(0xccb4ec60_b9dc_11d1_ac80_00a0c9034873);
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct PROPERTYRESTRICTION {
    pub rel: u32,
    pub prop: super::super::Storage::IndexServer::FULLPROPSPEC,
    pub prval: core::mem::ManuallyDrop<windows_core::PROPVARIANT>,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Clone for PROPERTYRESTRICTION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for PROPERTYRESTRICTION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for PROPERTYRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROXY_INFO {
    pub dwSize: u32,
    pub pcwszUserAgent: windows_core::PCWSTR,
    pub paUseProxy: PROXY_ACCESS,
    pub fLocalBypass: super::super::Foundation::BOOL,
    pub dwPortNumber: u32,
    pub pcwszProxyName: windows_core::PCWSTR,
    pub pcwszBypassList: windows_core::PCWSTR,
}
impl windows_core::TypeKind for PROXY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const QueryParser: windows_core::GUID = windows_core::GUID::from_u128(0xb72f8fd8_0fab_4dd9_bdbf_245a6ce1485b);
pub const QueryParserManager: windows_core::GUID = windows_core::GUID::from_u128(0x5088b39a_29b4_4d9d_8245_4ee289222f66);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RANGECATEGORIZE {
    pub cRange: u32,
    pub aRangeBegin: *mut windows_core::PROPVARIANT,
}
impl windows_core::TypeKind for RANGECATEGORIZE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RANGECATEGORIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub struct RESTRICTION {
    pub rt: u32,
    pub weight: u32,
    pub res: RESTRICTION_0,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Clone for RESTRICTION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for RESTRICTION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for RESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub union RESTRICTION_0 {
    pub ar: NODERESTRICTION,
    pub orRestriction: NODERESTRICTION,
    pub pxr: NODERESTRICTION,
    pub vr: VECTORRESTRICTION,
    pub nr: NOTRESTRICTION,
    pub cr: CONTENTRESTRICTION,
    pub nlr: NATLANGUAGERESTRICTION,
    pub pr: core::mem::ManuallyDrop<PROPERTYRESTRICTION>,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Clone for RESTRICTION_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for RESTRICTION_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for RESTRICTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
pub struct RMTPACK {
    pub pISeqStream: core::mem::ManuallyDrop<Option<super::Com::ISequentialStream>>,
    pub cbData: u32,
    pub cBSTR: u32,
    pub rgBSTR: *mut windows_core::BSTR,
    pub cVARIANT: u32,
    pub rgVARIANT: *mut windows_core::VARIANT,
    pub cIDISPATCH: u32,
    pub rgIDISPATCH: *mut Option<super::Com::IDispatch>,
    pub cIUNKNOWN: u32,
    pub rgIUNKNOWN: *mut Option<windows_core::IUnknown>,
    pub cPROPVARIANT: u32,
    pub rgPROPVARIANT: *mut windows_core::PROPVARIANT,
    pub cArray: u32,
    pub rgArray: *mut windows_core::VARIANT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl Clone for RMTPACK {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RMTPACK {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl Default for RMTPACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
pub struct RMTPACK {
    pub pISeqStream: core::mem::ManuallyDrop<Option<super::Com::ISequentialStream>>,
    pub cbData: u32,
    pub cBSTR: u32,
    pub rgBSTR: *mut windows_core::BSTR,
    pub cVARIANT: u32,
    pub rgVARIANT: *mut windows_core::VARIANT,
    pub cIDISPATCH: u32,
    pub rgIDISPATCH: *mut Option<super::Com::IDispatch>,
    pub cIUNKNOWN: u32,
    pub rgIUNKNOWN: *mut Option<windows_core::IUnknown>,
    pub cPROPVARIANT: u32,
    pub rgPROPVARIANT: *mut windows_core::PROPVARIANT,
    pub cArray: u32,
    pub rgArray: *mut windows_core::VARIANT,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RMTPACK {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl Default for RMTPACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RootBinder: windows_core::GUID = windows_core::GUID::from_u128(0xff151822_b0bf_11d1_a80d_000000000000);
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct SEARCH_COLUMN_PROPERTIES {
    pub Value: core::mem::ManuallyDrop<windows_core::PROPVARIANT>,
    pub lcid: u32,
}
impl Clone for SEARCH_COLUMN_PROPERTIES {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for SEARCH_COLUMN_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for SEARCH_COLUMN_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEARCH_ITEM_CHANGE {
    pub Change: SEARCH_KIND_OF_CHANGE,
    pub Priority: SEARCH_NOTIFICATION_PRIORITY,
    pub pUserData: *mut super::Com::BLOB,
    pub lpwszURL: windows_core::PWSTR,
    pub lpwszOldURL: windows_core::PWSTR,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SEARCH_ITEM_CHANGE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SEARCH_ITEM_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEARCH_ITEM_INDEXING_STATUS {
    pub dwDocID: u32,
    pub hrIndexingStatus: windows_core::HRESULT,
}
impl windows_core::TypeKind for SEARCH_ITEM_INDEXING_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SEARCH_ITEM_INDEXING_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEARCH_ITEM_PERSISTENT_CHANGE {
    pub Change: SEARCH_KIND_OF_CHANGE,
    pub URL: windows_core::PWSTR,
    pub OldURL: windows_core::PWSTR,
    pub Priority: SEARCH_NOTIFICATION_PRIORITY,
}
impl windows_core::TypeKind for SEARCH_ITEM_PERSISTENT_CHANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SEARCH_ITEM_PERSISTENT_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct SEC_OBJECT {
    pub cObjects: u32,
    pub prgObjects: *mut SEC_OBJECT_ELEMENT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for SEC_OBJECT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for SEC_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct SEC_OBJECT {
    pub cObjects: u32,
    pub prgObjects: *mut SEC_OBJECT_ELEMENT,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for SEC_OBJECT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for SEC_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct SEC_OBJECT_ELEMENT {
    pub guidObjectType: windows_core::GUID,
    pub ObjectID: super::super::Storage::IndexServer::DBID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for SEC_OBJECT_ELEMENT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for SEC_OBJECT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
#[derive(Clone, Copy)]
pub struct SEC_OBJECT_ELEMENT {
    pub guidObjectType: windows_core::GUID,
    pub ObjectID: super::super::Storage::IndexServer::DBID,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::TypeKind for SEC_OBJECT_ELEMENT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl Default for SEC_OBJECT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy)]
pub struct SORTKEY {
    pub propColumn: super::super::Storage::IndexServer::FULLPROPSPEC,
    pub dwOrder: u32,
    pub locale: u32,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for SORTKEY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for SORTKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SORTSET {
    pub cCol: u32,
    pub aCol: *mut SORTKEY,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for SORTSET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for SORTSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SQLPERF {
    pub TimerResolution: u32,
    pub SQLidu: u32,
    pub SQLiduRows: u32,
    pub SQLSelects: u32,
    pub SQLSelectRows: u32,
    pub Transactions: u32,
    pub SQLPrepares: u32,
    pub ExecDirects: u32,
    pub SQLExecutes: u32,
    pub CursorOpens: u32,
    pub CursorSize: u32,
    pub CursorUsed: u32,
    pub PercentCursorUsed: f64,
    pub AvgFetchTime: f64,
    pub AvgCursorSize: f64,
    pub AvgCursorUsed: f64,
    pub SQLFetchTime: u32,
    pub SQLFetchCount: u32,
    pub CurrentStmtCount: u32,
    pub MaxOpenStmt: u32,
    pub SumOpenStmt: u32,
    pub CurrentConnectionCount: u32,
    pub MaxConnectionsOpened: u32,
    pub SumConnectionsOpened: u32,
    pub SumConnectiontime: u32,
    pub AvgTimeOpened: f64,
    pub ServerRndTrips: u32,
    pub BuffersSent: u32,
    pub BuffersRec: u32,
    pub BytesSent: u32,
    pub BytesRec: u32,
    pub msExecutionTime: u32,
    pub msNetWorkServerTime: u32,
}
impl windows_core::TypeKind for SQLPERF {
    type TypeKind = windows_core::CopyType;
}
impl Default for SQLPERF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SQL_DAY_SECOND_STRUCT {
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub fraction: u32,
}
impl windows_core::TypeKind for SQL_DAY_SECOND_STRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SQL_DAY_SECOND_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SQL_INTERVAL_STRUCT {
    pub interval_type: SQLINTERVAL,
    pub interval_sign: i16,
    pub intval: SQL_INTERVAL_STRUCT_0,
}
impl windows_core::TypeKind for SQL_INTERVAL_STRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SQL_INTERVAL_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SQL_INTERVAL_STRUCT_0 {
    pub year_month: SQL_YEAR_MONTH_STRUCT,
    pub day_second: SQL_DAY_SECOND_STRUCT,
}
impl windows_core::TypeKind for SQL_INTERVAL_STRUCT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SQL_INTERVAL_STRUCT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SQL_NUMERIC_STRUCT {
    pub precision: u8,
    pub scale: i8,
    pub sign: u8,
    pub val: [u8; 16],
}
impl windows_core::TypeKind for SQL_NUMERIC_STRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SQL_NUMERIC_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SQL_YEAR_MONTH_STRUCT {
    pub year: u32,
    pub month: u32,
}
impl windows_core::TypeKind for SQL_YEAR_MONTH_STRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SQL_YEAR_MONTH_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSERRORINFO {
    pub pwszMessage: windows_core::PWSTR,
    pub pwszServer: windows_core::PWSTR,
    pub pwszProcedure: windows_core::PWSTR,
    pub lNative: i32,
    pub bState: u8,
    pub bClass: u8,
    pub wLineNumber: u16,
}
impl windows_core::TypeKind for SSERRORINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SSERRORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct SSVARIANT {
    pub vt: u16,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub Anonymous: SSVARIANT_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for SSVARIANT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SSVARIANT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SSVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union SSVARIANT_0 {
    pub bTinyIntVal: u8,
    pub sShortIntVal: i16,
    pub lIntVal: i32,
    pub llBigIntVal: i64,
    pub fltRealVal: f32,
    pub dblFloatVal: f64,
    pub cyMoneyVal: super::Com::CY,
    pub NCharVal: SSVARIANT_0_3,
    pub CharVal: SSVARIANT_0_2,
    pub fBitVal: super::super::Foundation::VARIANT_BOOL,
    pub rgbGuidVal: [u8; 16],
    pub numNumericVal: DB_NUMERIC,
    pub BinaryVal: SSVARIANT_0_1,
    pub tsDateTimeVal: DBTIMESTAMP,
    pub UnknownType: SSVARIANT_0_4,
    pub BLOBType: core::mem::ManuallyDrop<SSVARIANT_0_0>,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for SSVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SSVARIANT_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SSVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct SSVARIANT_0_0 {
    pub dbobj: DBOBJECT,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for SSVARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SSVARIANT_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SSVARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSVARIANT_0_1 {
    pub sActualLength: i16,
    pub sMaxLength: i16,
    pub prgbBinaryVal: *mut u8,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SSVARIANT_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SSVARIANT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSVARIANT_0_2 {
    pub sActualLength: i16,
    pub sMaxLength: i16,
    pub pchCharVal: windows_core::PSTR,
    pub rgbReserved: [u8; 5],
    pub dwReserved: u32,
    pub pwchReserved: windows_core::PWSTR,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SSVARIANT_0_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SSVARIANT_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSVARIANT_0_3 {
    pub sActualLength: i16,
    pub sMaxLength: i16,
    pub pwchNCharVal: windows_core::PWSTR,
    pub rgbReserved: [u8; 5],
    pub dwReserved: u32,
    pub pwchReserved: windows_core::PWSTR,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SSVARIANT_0_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SSVARIANT_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSVARIANT_0_4 {
    pub dwActualLength: u32,
    pub rgMetadata: [u8; 16],
    pub pUnknownData: *mut u8,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SSVARIANT_0_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SSVARIANT_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct SUBSCRIPTIONINFO {
    pub cbSize: u32,
    pub fUpdateFlags: u32,
    pub schedule: SUBSCRIPTIONSCHEDULE,
    pub customGroupCookie: windows_core::GUID,
    pub pTrigger: *mut core::ffi::c_void,
    pub dwRecurseLevels: u32,
    pub fWebcrawlerFlags: u32,
    pub bMailNotification: super::super::Foundation::BOOL,
    pub bGleam: super::super::Foundation::BOOL,
    pub bChangesOnly: super::super::Foundation::BOOL,
    pub bNeedPassword: super::super::Foundation::BOOL,
    pub fChannelFlags: u32,
    pub bstrUserName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrPassword: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrFriendlyName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub dwMaxSizeKB: u32,
    pub subType: SUBSCRIPTIONTYPE,
    pub fTaskFlags: u32,
    pub dwReserved: u32,
}
impl Clone for SUBSCRIPTIONINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for SUBSCRIPTIONINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SUBSCRIPTIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SUBSCRIPTIONITEMINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwPriority: u32,
    pub ScheduleGroup: windows_core::GUID,
    pub clsidAgent: windows_core::GUID,
}
impl windows_core::TypeKind for SUBSCRIPTIONITEMINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SUBSCRIPTIONITEMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SubscriptionMgr: windows_core::GUID = windows_core::GUID::from_u128(0xabbe31d0_6dae_11d0_beca_00c04fd940be);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TEXT_SOURCE {
    pub pfnFillTextBuffer: PFNFILLTEXTBUFFER,
    pub awcBuffer: windows_core::PCWSTR,
    pub iEnd: u32,
    pub iCur: u32,
}
impl windows_core::TypeKind for TEXT_SOURCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for TEXT_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TIMEOUT_INFO {
    pub dwSize: u32,
    pub dwConnectTimeout: u32,
    pub dwDataTimeout: u32,
}
impl windows_core::TypeKind for TIMEOUT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for TIMEOUT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TIMESTAMP_STRUCT {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub fraction: u32,
}
impl windows_core::TypeKind for TIMESTAMP_STRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for TIMESTAMP_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TIME_STRUCT {
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
}
impl windows_core::TypeKind for TIME_STRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for TIME_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VECTORRESTRICTION {
    pub Node: NODERESTRICTION,
    pub RankMethod: u32,
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for VECTORRESTRICTION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for VECTORRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFNFILLTEXTBUFFER = Option<unsafe extern "system" fn(ptextsource: *mut TEXT_SOURCE) -> windows_core::HRESULT>;
pub type SQL_ASYNC_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(pcontext: *const core::ffi::c_void, flast: super::super::Foundation::BOOL) -> i16>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
