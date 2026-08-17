// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of OleAuto.h
use ctypes::{c_double, c_float, c_int, c_uint, c_void};
use shared::basetsd::{LONG64, ULONG64};
use shared::minwindef::{BYTE, DWORD, FLOAT, UINT, ULONG, USHORT, WORD};
use shared::wtypes::{BSTR, DATE, DECIMAL, LPBSTR, LPDECIMAL, VARTYPE};
use shared::wtypesbase::{DOUBLE, LPCOLESTR, LPOLESTR, OLECHAR};
use um::minwinbase::LPSYSTEMTIME;
use um::oaidl::{
    DISPID_UNKNOWN, ICreateErrorInfo, IErrorInfo, ITypeLib, SAFEARRAY, VARIANT, VARIANTARG
};
use um::winnt::{CHAR, HRESULT, INT, LCID, LONG, LPCSTR, SHORT};
extern "system" {
    pub fn SysAllocString(
        psz: *const OLECHAR,
    ) -> BSTR;
    pub fn SysReAllocString(
        pbstr: *mut BSTR,
        psz: *const OLECHAR,
    ) -> INT;
    pub fn SysAllocStringLen(
        strIn: *const OLECHAR,
        ui: UINT,
    ) -> BSTR;
    pub fn SysReAllocStringLen(
        pbstr: *mut BSTR,
        psz: *const OLECHAR,
        len: c_uint,
    ) -> INT;
    pub fn SysFreeString(
        bstrString: BSTR,
    );
    pub fn SysStringLen(
        pbstr: BSTR,
    ) -> UINT;
    pub fn SysStringByteLen(
        bstr: BSTR,
    ) -> UINT;
    pub fn SysAllocStringByteLen(
        psz: LPCSTR,
        len: UINT,
    ) -> BSTR;
    pub fn DosDateTimeToVariantTime(
        wDosDate: USHORT,
        wDosTime: USHORT,
        pvtime: *mut DOUBLE,
    ) -> INT;
    pub fn VariantTimeToDosDateTime(
        vtime: DOUBLE,
        pwDosDate: *mut USHORT,
        pwDosTime: *mut USHORT,
    ) -> INT;
    pub fn SystemTimeToVariantTime(
        lpSystemTime: LPSYSTEMTIME,
        pvtime: *mut DOUBLE,
    ) -> INT;
    pub fn VariantTimeToSystemTime(
        vtime: DOUBLE,
        lpSystemTime: LPSYSTEMTIME,
    ) -> INT;
    pub fn SafeArrayAccessData(
        psa: *mut SAFEARRAY,
        ppvData: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SafeArrayUnaccessData(
        psa: *mut SAFEARRAY,
    ) -> HRESULT;
    pub fn SafeArrayCreateVector(
        vt: VARTYPE,
        lLbound: LONG,
        cElements: ULONG,
    ) -> *mut SAFEARRAY;
    pub fn SafeArrayGetLBound(
        psa: *mut SAFEARRAY,
        nDim: UINT,
        plLbound: *mut LONG
    ) -> HRESULT;
    pub fn SafeArrayGetUBound(
        psa: *mut SAFEARRAY,
        nDim: UINT,
        plUbound: *mut LONG
    ) -> HRESULT;
    pub fn SafeArrayDestroy(
        psa: *mut SAFEARRAY
    ) -> HRESULT;
    pub fn VariantInit(
        pvarg: *mut VARIANTARG,
    );
    pub fn VariantClear(
        pvarg: *mut VARIANTARG,
    ) -> HRESULT;
    pub fn VariantCopy(
        pvargDest: *mut VARIANTARG,
        pvargSrc: *const VARIANTARG,
    ) -> HRESULT;
    pub fn VariantCopyInd(
        pvarDest: *mut VARIANT,
        pvargSrc: *const VARIANTARG,
    ) -> HRESULT;
    pub fn VariantChangeType(
        pvargDest: *mut VARIANTARG,
        pvarSrc: *const VARIANTARG,
        wFlags: USHORT,
        vt: VARTYPE,
    ) -> HRESULT;
    pub fn VariantChangeTypeEx(
        pvargDest: *mut VARIANTARG,
        pvarSrc: *const VARIANTARG,
        lcid: LCID,
        wFlags: USHORT,
        vt: VARTYPE,
    ) -> HRESULT;
    pub fn VarUI1FromI2(
        sIn: SHORT,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromI4(
        lIn: LONG,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromI8(
        i64In: LONG64,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromR4(
        fltIn: FLOAT,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromR8(
        dblIn: DOUBLE,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromDate(
        dateIn: DATE,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromI1(
        cIn: CHAR,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromUI2(
        uiIn: USHORT,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromUI4(
        ulIn: ULONG,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromUI8(
        ui64In: ULONG64,
        pbOut: *mut BYTE,
    );
    pub fn VarUI1FromDec(
        pdecIn: *const DECIMAL,
        pbOut: *mut BYTE,
    );
    pub fn VarI2FromUI1(
        bIn: BYTE,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromI4(
        lIn: LONG,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromI8(
        i64In: LONG64,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromR4(
        fltIn: FLOAT,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromR8(
        dblIn: DOUBLE,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromDate(
        dateIn: DATE,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromI1(
        cIn: CHAR,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromUI2(
        uiIn: USHORT,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromUI4(
        ulIn: ULONG,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromUI8(
        ui64In: ULONG64,
        psOut: *mut SHORT,
    );
    pub fn VarI2FromDec(
        pdecIn: *const DECIMAL,
        psOut: *mut SHORT,
    );
    pub fn VarI4FromUI1(
        bIn: BYTE,
        plOut: *mut LONG,
    );
    pub fn VarI4FromI2(
        sIn: SHORT,
        plOut: *mut LONG,
    );
    pub fn VarI4FromI8(
        i64In: LONG64,
        plOut: *mut LONG,
    );
    pub fn VarI4FromR4(
        fltIn: FLOAT,
        plOut: *mut LONG,
    );
    pub fn VarI4FromR8(
        dblIn: DOUBLE,
        plOut: *mut LONG,
    );
    pub fn VarI4FromDate(
        dateIn: DATE,
        plOut: *mut LONG,
    );
    pub fn VarI4FromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        plOut: *mut LONG,
    );
    pub fn VarI4FromI1(
        cIn: CHAR,
        plOut: *mut LONG,
    );
    pub fn VarI4FromUI2(
        uiIn: USHORT,
        plOut: *mut LONG,
    );
    pub fn VarI4FromUI4(
        ulIn: ULONG,
        plOut: *mut LONG,
    );
    pub fn VarI4FromUI8(
        ui64In: ULONG64,
        plOut: *mut LONG,
    );
    pub fn VarI4FromDec(
        pdecIn: *const DECIMAL,
        plOut: *mut LONG,
    );
    pub fn VarI8FromUI1(
        bIn: BYTE,
        pi64Out: *mut LONG64,
    );
    pub fn VarI8FromI2(
        sIn: SHORT,
        pi64Out: *mut LONG64,
    );
    pub fn VarI8FromR4(
        fltIn: FLOAT,
        pi64Out: *mut LONG64,
    );
    pub fn VarI8FromR8(
        dblIn: DOUBLE,
        pi64Out: *mut LONG64,
    );
    pub fn VarI8FromDate(
        dateIn: DATE,
        pi64Out: *mut LONG64,
    );
    pub fn VarI8FromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        pi64Out: *mut LONG64,
    );
    pub fn VarI8FromI1(
        cIn: CHAR,
        pi64Out: *mut LONG64,
    );
    pub fn VarI8FromUI2(
        uiIn: USHORT,
        pi64Out: *mut LONG64,
    );
    pub fn VarI8FromUI4(
        ulIn: ULONG,
        pi64Out: *mut LONG64,
    );
    pub fn VarI8FromUI8(
        ui64In: ULONG64,
        pi64Out: *mut LONG64,
    );
    pub fn VarI8FromDec(
        pdecIn: *const DECIMAL,
        pi64Out: *mut LONG64,
    );
    pub fn VarR4FromUI1(
        bIn: BYTE,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromI2(
        sIn: SHORT,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromI4(
        lIn: LONG,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromI8(
        i64In: LONG64,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromR8(
        dblIn: DOUBLE,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromDate(
        dateIn: DATE,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromI1(
        cIn: CHAR,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromUI2(
        uiIn: USHORT,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromUI4(
        ulIn: ULONG,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromUI8(
        ui64In: ULONG64,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR4FromDec(
        pdecIn: *const DECIMAL,
        pfltOut: *mut FLOAT,
    );
    pub fn VarR8FromUI1(
        bIn: BYTE,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromI2(
        sIn: SHORT,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromI4(
        lIn: LONG,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromI8(
        i64In: LONG64,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromR4(
        fltIn: FLOAT,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromDate(
        dateIn: DATE,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromI1(
        cIn: CHAR,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromUI2(
        uiIn: USHORT,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromUI4(
        ulIn: ULONG,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromUI8(
        ui64In: ULONG64,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarR8FromDec(
        pdecIn: *const DECIMAL,
        pdblOut: *mut DOUBLE,
    );
    pub fn VarDateFromUI1(
        bIn: BYTE,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromI2(
        sIn: SHORT,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromI4(
        lIn: LONG,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromI8(
        i64In: LONG64,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromR4(
        fltIn: FLOAT,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromR8(
        dblIn: DOUBLE,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromI1(
        cIn: CHAR,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromUI2(
        uiIn: USHORT,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromUI4(
        ulIn: ULONG,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromUI8(
        ui64In: ULONG64,
        pdateOut: *mut DATE,
    );
    pub fn VarDateFromDec(
        pdecIn: *const DECIMAL,
        pdateOut: *mut DATE,
    );
    pub fn VarBstrFromUI1(
        bVal: BYTE,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromI2(
        iVal: SHORT,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromI4(
        lIn: LONG,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromI8(
        i64In: LONG64,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromR4(
        fltIn: FLOAT,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromR8(
        dblIn: DOUBLE,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromDate(
        dateIn: DATE,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromI1(
        cIn: CHAR,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromUI2(
        uiIn: USHORT,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromUI4(
        ulIn: ULONG,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromUI8(
        ui64In: ULONG64,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarBstrFromDec(
        pdecIn: *const DECIMAL,
        lcid: LCID,
        dwFlags: ULONG,
        pbstrOut: *mut BSTR,
    );
    pub fn VarUI2FromUI1(
        bIn: BYTE,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromI2(
        uiIn: SHORT,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromI4(
        lIn: LONG,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromI8(
        i64In: LONG64,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromR4(
        fltIn: FLOAT,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromR8(
        dblIn: DOUBLE,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromDate(
        dateIn: DATE,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromI1(
        cIn: CHAR,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromUI4(
        ulIn: ULONG,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromUI8(
        i64In: ULONG64,
        puiOut: *mut USHORT,
    );
    pub fn VarUI2FromDec(
        pdecIn: *const DECIMAL,
        puiOut: *mut USHORT,
    );
    pub fn VarUI4FromUI1(
        bIn: BYTE,
        pulOut: *mut ULONG,
    );
    pub fn VarUI4FromI2(
        uiIn: SHORT,
        pulOut: *mut ULONG,
    );
    pub fn VarUI4FromI4(
        lIn: LONG,
        pulOut: *mut ULONG,
    );
    pub fn VarUI4FromI8(
        i64In: LONG64,
        plOut: *mut ULONG,
    );
    pub fn VarUI4FromR4(
        fltIn: FLOAT,
        pulOut: *mut ULONG,
    );
    pub fn VarUI4FromR8(
        dblIn: DOUBLE,
        pulOut: *mut ULONG,
    );
    pub fn VarUI4FromDate(
        dateIn: DATE,
        pulOut: *mut ULONG,
    );
    pub fn VarUI4FromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        pulOut: *mut ULONG,
    );
    pub fn VarUI4FromI1(
        cIn: CHAR,
        pulOut: *mut ULONG,
    );
    pub fn VarUI4FromUI2(
        uiIn: USHORT,
        pulOut: *mut ULONG,
    );
    pub fn VarUI4FromUI8(
        ui64In: ULONG64,
        plOut: *mut ULONG,
    );
    pub fn VarUI4FromDec(
        pdecIn: *const DECIMAL,
        pulOut: *mut ULONG,
    );
    pub fn VarUI8FromUI1(
        bIn: BYTE,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromI2(
        sIn: SHORT,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromI4(
        lIn: LONG,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromI8(
        ui64In: LONG64,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromR4(
        fltIn: FLOAT,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromR8(
        dblIn: DOUBLE,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromDate(
        dateIn: DATE,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromI1(
        cIn: CHAR,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromUI2(
        uiIn: USHORT,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromUI4(
        ulIn: ULONG,
        pi64Out: *mut ULONG64,
    );
    pub fn VarUI8FromDec(
        pdecIn: *const DECIMAL,
        pi64Out: *mut ULONG64,
    );
    pub fn VarDecFromUI1(
        bIn: BYTE,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromI2(
        uiIn: SHORT,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromI4(
        lIn: LONG,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromI8(
        i64In: LONG64,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromR4(
        fltIn: FLOAT,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromR8(
        dblIn: DOUBLE,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromDate(
        dateIn: DATE,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromStr(
        strIn: LPCOLESTR,
        lcid: LCID,
        dwFlags: ULONG,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromI1(
        cIn: CHAR,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromUI2(
        uiIn: USHORT,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromUI4(
        ulIn: ULONG,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecFromUI8(
        ui64In: ULONG64,
        pdecOut: *mut DECIMAL,
    );
    pub fn VarDecAdd(
        pdecLeft: LPDECIMAL,
        pdecRight: LPDECIMAL,
        pdecResult: LPDECIMAL,
    );
    pub fn VarDecDiv(
        pdecLeft: LPDECIMAL,
        pdecRight: LPDECIMAL,
        pdecResult: LPDECIMAL,
    );
    pub fn VarDecMul(
        pdecLeft: LPDECIMAL,
        pdecRight: LPDECIMAL,
        pdecResult: LPDECIMAL,
    );
    pub fn VarDecSub(
        pdecLeft: LPDECIMAL,
        pdecRight: LPDECIMAL,
        pdecResult: LPDECIMAL,
    );
    pub fn VarDecAbs(
        pdecIn: LPDECIMAL,
        pdecResult: LPDECIMAL,
    );
    pub fn VarDecFix(
        pdecIn: LPDECIMAL,
        pdecResult: LPDECIMAL,
    );
    pub fn VarDecInt(
        pdecIn: LPDECIMAL,
        pdecResult: LPDECIMAL,
    );
    pub fn VarDecNeg(
        pdecIn: LPDECIMAL,
        pdecResult: LPDECIMAL,
    );
    pub fn VarDecRound(
        pdecIn: LPDECIMAL,
        cDecimals: c_int,
        pdecResult: LPDECIMAL,
    );
    pub fn VarDecCmp(
        pdecLeft: LPDECIMAL,
        pdecRight: LPDECIMAL,
    );
    pub fn VarDecCmpR8(
        pdecLeft: LPDECIMAL,
        dblRight: c_double,
    );
    pub fn VarBstrCat(
        bstrLeft: BSTR,
        bstrRight: BSTR,
        pbstrResult: LPBSTR,
    );
    pub fn VarBstrCmp(
        bstrLeft: BSTR,
        bstrRight: BSTR,
        lcid: LCID,
        dwFlags: ULONG,
    );
    pub fn VarR8Pow(
        dblLeft: c_double,
        dblRight: c_double,
        pdblResult: *mut c_double,
    );
    pub fn VarR4CmpR8(
        fltLeft: c_float,
        dblRight: c_double,
    );
    pub fn VarR8Round(
        dblIn: c_double,
        cDecimals: c_int,
        pdblResult: *mut c_double,
    );
    pub fn GetAltMonthNames(
        lcid: LCID,
        prgp: *mut LPOLESTR,
    );
}
pub type DISPID = LONG;
pub type MEMBERID = DISPID;
pub const MEMBERID_NIL: MEMBERID = DISPID_UNKNOWN;
pub const DISPATCH_METHOD: WORD = 0x1;
pub const DISPATCH_PROPERTYGET: WORD = 0x2;
pub const DISPATCH_PROPERTYPUT: WORD = 0x4;
pub const DISPATCH_PROPERTYPUTREF: WORD = 0x8;
ENUM!{enum REGKIND {
    REGKIND_DEFAULT = 0,
    REGKIND_REGISTER,
    REGKIND_NONE,
}}
extern "system" {
    pub fn LoadTypeLibEx(
        szFile: LPCOLESTR,
        regkind: REGKIND,
        pptlib: *mut *mut ITypeLib,
    ) -> HRESULT;
    pub fn RevokeActiveObject(
        dwRegister: DWORD,
        pvReserved: *mut c_void,
    );
    pub fn SetErrorInfo(
        dwReserved: ULONG,
        perrinfo: *mut IErrorInfo,
    ) -> HRESULT;
    pub fn GetErrorInfo(
        dwReserved: ULONG,
        pperrinfo: *mut *mut IErrorInfo,
    ) -> HRESULT;
    pub fn CreateErrorInfo(
        pperrinfo: *mut *mut ICreateErrorInfo,
    ) -> HRESULT;
    pub fn OaBuildVersion() -> ULONG;
    pub fn OaEnablePerUserTLibRegistration();
}
