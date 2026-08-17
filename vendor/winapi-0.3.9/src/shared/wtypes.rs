// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{__int64, c_double, c_short, c_ushort, c_void, wchar_t};
use shared::guiddef::{CLSID, GUID};
use shared::minwindef::{BYTE, DWORD, ULONG, USHORT, WORD};
use shared::ntdef::{LCID, LONG, LONGLONG, ULONGLONG};
use shared::rpcndr::byte;
use shared::wtypesbase::{
    BYTE_BLOB, DWORD_BLOB, FLAGGED_BYTE_BLOB, FLAGGED_WORD_BLOB, LPOLESTR, OLECHAR
};
use um::wingdi::LOGPALETTE;
// extern RPC_IF_HANDLE __MIDL_itf_wtypes_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wtypes_0000_0000_v0_0_s_ifspec;
STRUCT!{struct RemHGLOBAL {
    fNullHGlobal: LONG,
    cbData: ULONG,
    data: [byte; 1],
}}
STRUCT!{struct RemHMETAFILEPICT {
    mm: LONG,
    xExt: LONG,
    yExt: LONG,
    cbData: ULONG,
    data: [byte; 1],
}}
STRUCT!{struct RemHENHMETAFILE {
    cbData: ULONG,
    data: [byte; 1],
}}
STRUCT!{struct RemHBITMAP {
    cbData: ULONG,
    data: [byte; 1],
}}
STRUCT!{struct RemHPALETTE {
    cbData: ULONG,
    data: [byte; 1],
}}
STRUCT!{struct RemHBRUSH {
    cbData: ULONG,
    data: [byte; 1],
}}
pub const ROTFLAGS_REGISTRATIONKEEPSALIVE: DWORD = 0x1;
pub const ROTFLAGS_ALLOWANYCLIENT: DWORD = 0x2;
pub const ROT_COMPARE_MAX: DWORD = 2048;
ENUM!{enum DVASPECT {
    DVASPECT_CONTENT = 1,
    DVASPECT_THUMBNAIL = 2,
    DVASPECT_ICON = 4,
    DVASPECT_DOCPRINT = 8,
}}
ENUM!{enum STGC {
    STGC_DEFAULT = 0,
    STGC_OVERWRITE = 1,
    STGC_ONLYIFCURRENT = 2,
    STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE = 4,
    STGC_CONSOLIDATE = 8,
}}
ENUM!{enum STGMOVE {
    STGMOVE_MOVE = 0,
    STGMOVE_COPY = 1,
    STGMOVE_SHALLOWCOPY = 2,
}}
ENUM!{enum STATFLAG {
    STATFLAG_DEFAULT = 0,
    STATFLAG_NONAME = 1,
    STATFLAG_NOOPEN = 2,
}}
pub type HCONTEXT = *mut c_void;
pub const WDT_INPROC_CALL: ULONG = 0x48746457;
pub const WDT_REMOTE_CALL: ULONG = 0x52746457;
pub const WDT_INPROC64_CALL: ULONG = 0x50746457;
UNION!{union userCLIPFORMAT_u {
    [usize; 1],
    dwValue dwValue_mut: DWORD,
    pwszName pwszName_mut: *mut wchar_t,
}}
STRUCT!{struct userCLIPFORMAT {
    fContext: LONG,
    u: userCLIPFORMAT_u,
}}
pub type wireCLIPFORMAT = *mut userCLIPFORMAT;
pub type CLIPFORMAT = WORD;
UNION!{union GDI_NONREMOTE_u {
    [usize; 1],
    hInproc hInproc_mut: LONG,
    hRemote hRemote_mut: *mut DWORD_BLOB,
}}
STRUCT!{struct GDI_NONREMOTE {
    fContext: LONG,
    u: GDI_NONREMOTE_u,
}}
UNION!{union userHGLOBAL_u {
    [u64; 1],
    hInproc hInproc_mut: LONG,
    hRemote hRemote_mut: *mut FLAGGED_BYTE_BLOB,
    hInproc64 hInproc64_mut: __int64,
}}
STRUCT!{struct userHGLOBAL {
    fContext: LONG,
    u: userHGLOBAL_u,
}}
pub type wireHGLOBAL = *mut userHGLOBAL;
UNION!{union userHMETAFILE_u {
    [u64; 1],
    hInproc hInproc_mut: LONG,
    hRemote hRemote_mut: *mut BYTE_BLOB,
    hInproc64 hInproc64_mut: __int64,
}}
STRUCT!{struct userHMETAFILE {
    fContext: LONG,
    u: userHMETAFILE_u,
}}
STRUCT!{struct remoteMETAFILEPICT {
    mm: LONG,
    xExt: LONG,
    yExt: LONG,
    hMF: *mut userHMETAFILE,
}}
UNION!{union userHMETAFILEPICT_u {
    [u64; 1],
    hInproc hInproc_mut: LONG,
    hRemote hRemote_mut: *mut remoteMETAFILEPICT,
    hInproc64 hInproc64_mut: __int64,
}}
STRUCT!{struct userHMETAFILEPICT {
    fContext: LONG,
    u: userHMETAFILEPICT_u,
}}
UNION!{union userHENHMETAFILE_u {
    [u64; 1],
    hInproc hInproc_mut: LONG,
    hRemote hRemote_mut: *mut BYTE_BLOB,
    hInproc64 hInproc64_mut: __int64,
}}
STRUCT!{struct userHENHMETAFILE {
    fContext: LONG,
    u: userHENHMETAFILE_u,
}}
STRUCT!{struct userBITMAP {
    bmType: LONG,
    bmWidth: LONG,
    bmHeight: LONG,
    bmWidthBytes: LONG,
    bmPlanes: WORD,
    bmBitsPixel: WORD,
    cbSize: ULONG,
    pBuffer: [byte; 1],
}}
UNION!{union userHBITMAP_u {
    [u64; 1],
    hInproc hInproc_mut: LONG,
    hRemote hRemote_mut: *mut userBITMAP,
    hInproc64 hInproc64_mut: __int64,
}}
STRUCT!{struct userHBITMAP {
    fContext: LONG,
    u: userHBITMAP_u,
}}
UNION!{union userHPALETTE_u {
    [u64; 1],
    hInproc hInproc_mut: LONG,
    hRemote hRemote_mut: *mut LOGPALETTE,
    hInproc64 hInproc64_mut: __int64,
}}
STRUCT!{struct userHPALETTE {
    fContext: LONG,
    u: userHPALETTE_u,
}}
UNION!{union RemotableHandle_u {
    [u32; 1],
    hInproc hInproc_mut: LONG,
    hRemote hRemote_mut: LONG,
}}
STRUCT!{struct RemotableHandle {
    fContext: LONG,
    u: RemotableHandle_u,
}}
pub type wireHWND = *mut RemotableHandle;
pub type wireHMENU = *mut RemotableHandle;
pub type wireHACCEL = *mut RemotableHandle;
pub type wireHBRUSH = *mut RemotableHandle;
pub type wireHFONT = *mut RemotableHandle;
pub type wireHDC = *mut RemotableHandle;
pub type wireHICON = *mut RemotableHandle;
pub type wireHRGN = *mut RemotableHandle;
pub type wireHMONITOR = *mut RemotableHandle;
pub type wireHBITMAP = *mut userHBITMAP;
pub type wireHPALETTE = *mut userHPALETTE;
pub type wireHENHMETAFILE = *mut userHENHMETAFILE;
pub type wireHMETAFILE = *mut userHMETAFILE;
pub type wireHMETAFILEPICT = *mut userHMETAFILEPICT;
pub type HMETAFILEPICT = *mut c_void;
// extern RPC_IF_HANDLE IWinTypes_v0_1_c_ifspec;
// extern RPC_IF_HANDLE IWinTypes_v0_1_s_ifspec;
pub type DATE = c_double;
STRUCT!{struct CY {
    int64: LONGLONG,
}}
pub type LPCY = *mut CY;
STRUCT!{struct DECIMAL {
    wReserved: USHORT,
    scale: BYTE,
    sign: BYTE,
    Hi32: ULONG,
    Lo64: ULONGLONG,
}}
pub const DECIMAL_NEG: BYTE = 0x80;
#[inline]
pub fn DECIMAL_SETZERO(dec: &mut DECIMAL) {
    dec.Lo64 = 0;
    dec.Hi32 = 0;
    dec.scale = 0;
    dec.sign = 0;
}
pub type LPDECIMAL = *mut DECIMAL;
pub type wireBSTR = *mut FLAGGED_WORD_BLOB;
pub type BSTR = *mut OLECHAR;
pub type LPBSTR = *mut BSTR;
pub type VARIANT_BOOL = c_short;
STRUCT!{struct BSTRBLOB {
    cbSize: ULONG,
    pData: *mut BYTE,
}}
pub type LPBSTRBLOB = *mut BSTRBLOB;
pub const VARIANT_TRUE: VARIANT_BOOL = -1;
pub const VARIANT_FALSE: VARIANT_BOOL = 0;
STRUCT!{struct CLIPDATA {
    cbSize: ULONG,
    ulClipFmt: LONG,
    pClipData: *mut BYTE,
}}
#[inline]
pub fn CBPCLIPDATA(clipdata: CLIPDATA) -> ULONG {
    clipdata.cbSize - 4
}
pub type VARTYPE = c_ushort;
ENUM!{enum VARENUM {
    VT_EMPTY = 0,
    VT_NULL = 1,
    VT_I2 = 2,
    VT_I4 = 3,
    VT_R4 = 4,
    VT_R8 = 5,
    VT_CY = 6,
    VT_DATE = 7,
    VT_BSTR = 8,
    VT_DISPATCH = 9,
    VT_ERROR = 10,
    VT_BOOL = 11,
    VT_VARIANT = 12,
    VT_UNKNOWN = 13,
    VT_DECIMAL = 14,
    VT_I1 = 16,
    VT_UI1 = 17,
    VT_UI2 = 18,
    VT_UI4 = 19,
    VT_I8 = 20,
    VT_UI8 = 21,
    VT_INT = 22,
    VT_UINT = 23,
    VT_VOID = 24,
    VT_HRESULT = 25,
    VT_PTR = 26,
    VT_SAFEARRAY = 27,
    VT_CARRAY = 28,
    VT_USERDEFINED = 29,
    VT_LPSTR = 30,
    VT_LPWSTR = 31,
    VT_RECORD = 36,
    VT_INT_PTR = 37,
    VT_UINT_PTR = 38,
    VT_FILETIME = 64,
    VT_BLOB = 65,
    VT_STREAM = 66,
    VT_STORAGE = 67,
    VT_STREAMED_OBJECT = 68,
    VT_STORED_OBJECT = 69,
    VT_BLOB_OBJECT = 70,
    VT_CF = 71,
    VT_CLSID = 72,
    VT_VERSIONED_STREAM = 73,
    VT_BSTR_BLOB = 0xfff,
    VT_VECTOR = 0x1000,
    VT_ARRAY = 0x2000,
    VT_BYREF = 0x4000,
    VT_RESERVED = 0x8000,
    VT_ILLEGAL = 0xffff,
    VT_ILLEGALMASKED = 0xfff,
    VT_TYPEMASK = 0xfff,
}}
pub type PROPID = ULONG;
STRUCT!{struct PROPERTYKEY {
    fmtid: GUID,
    pid: DWORD,
}}
STRUCT!{struct CSPLATFORM {
    dwPlatformId: DWORD,
    dwVersionHi: DWORD,
    dwVersionLo: DWORD,
    dwProcessorArch: DWORD,
}}
STRUCT!{struct QUERYCONTEXT {
    dwContext: DWORD,
    Platform: CSPLATFORM,
    Locale: LCID,
    dwVersionHi: DWORD,
    dwVersionLo: DWORD,
}}
ENUM!{enum TYSPEC {
    TYSPEC_CLSID,
    TYSPEC_FILEEXT,
    TYSPEC_MIMETYPE,
    TYSPEC_FILENAME,
    TYSPEC_PROGID,
    TYSPEC_PACKAGENAME,
    TYSPEC_OBJECTID,
}}
STRUCT!{struct uCLSSPEC_ByName {
    pPackageName: LPOLESTR,
    PolicyId: GUID,
}}
STRUCT!{struct uCLSSPEC_ByObjectId {
    ObjectId: GUID,
    PolicyId: GUID,
}}
UNION!{union uCLSSPEC_u {
    [u32; 8] [u64; 4],
    clsid clsid_mut: CLSID,
    pFileExt pFileExt_mut: LPOLESTR,
    pMimeType pMimeType_mut: LPOLESTR,
    pProgId pProgId_mut: LPOLESTR,
    pFileName pFileName_mut: LPOLESTR,
    ByName ByName_mut: uCLSSPEC_ByName,
    ByObjectId ByObjectId_mut: uCLSSPEC_ByObjectId,
}}
STRUCT!{struct uCLSSPEC {
    tyspec: DWORD,
    u: uCLSSPEC_u,
}}
// extern RPC_IF_HANDLE __MIDL_itf_wtypes_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wtypes_0000_0001_v0_0_s_ifspec;
