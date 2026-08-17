// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::{CLSID, FMTID, GUID, REFCLSID, REFFMTID};
use shared::minwindef::{
    BYTE, DWORD, FILETIME, FLOAT, HIBYTE, HIWORD, INT, LOBYTE, LOWORD, UINT, WORD
};
use shared::ntdef::{
    BOOLEAN, CHAR, HRESULT, LARGE_INTEGER, LONG, LPSTR, LPWSTR, PVOID, SHORT,
    UCHAR, ULARGE_INTEGER, ULONG, USHORT
};
use shared::wtypes::{
    BSTR, BSTRBLOB, CLIPDATA, CY, DATE, DECIMAL, PROPID, VARIANT_BOOL, VARTYPE
};
use shared::wtypesbase::{BLOB, DOUBLE, LPOLESTR, SCODE};
use um::oaidl::{IDispatch, LPSAFEARRAY};
use um::objidlbase::IStream;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
pub const PROPSETFLAG_DEFAULT: DWORD = 0;
pub const PROPSETFLAG_NONSIMPLE: DWORD = 1;
pub const PROPSETFLAG_ANSI: DWORD = 2;
pub const PROPSETFLAG_UNBUFFERED: DWORD = 4;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: DWORD = 1;
STRUCT!{struct VERSIONEDSTREAM {
    guidVersion: GUID,
    pStream: *mut IStream,
}}
pub type LPVERSIONEDSTREAM = *mut VERSIONEDSTREAM;
macro_rules! TYPEDEF_CA {
    ($type_:ty, $name:ident) => { STRUCT!{struct $name {
        cElems: $crate::shared::ntdef::ULONG,
        pElems: *mut $type_,
    }}}
}
TYPEDEF_CA!(CHAR, CAC);
TYPEDEF_CA!(UCHAR, CAUB);
TYPEDEF_CA!(SHORT, CAI);
TYPEDEF_CA!(USHORT, CAUI);
TYPEDEF_CA!(LONG, CAL);
TYPEDEF_CA!(ULONG, CAUL);
TYPEDEF_CA!(FLOAT, CAFLT);
TYPEDEF_CA!(DOUBLE, CADBL);
TYPEDEF_CA!(CY, CACY);
TYPEDEF_CA!(DATE, CADATE);
TYPEDEF_CA!(BSTR, CABSTR);
TYPEDEF_CA!(BSTRBLOB, CABSTRBLOB);
TYPEDEF_CA!(VARIANT_BOOL, CABOOL);
TYPEDEF_CA!(SCODE, CASCODE);
TYPEDEF_CA!(PROPVARIANT, CAPROPVARIANT);
TYPEDEF_CA!(LARGE_INTEGER, CAH);
TYPEDEF_CA!(ULARGE_INTEGER, CAUH);
TYPEDEF_CA!(LPSTR, CALPSTR);
TYPEDEF_CA!(LPWSTR, CALPWSTR);
TYPEDEF_CA!(FILETIME, CAFILETIME);
TYPEDEF_CA!(CLIPDATA, CACLIPDATA);
TYPEDEF_CA!(CLSID, CACLSID);
UNION!{union PROPVARIANT_data {
    [u64; 1] [u64; 2],
    cVal cVal_mut: CHAR,
    bVal bVal_mut: UCHAR,
    iVal iVal_mut: SHORT,
    uiVal uiVal_mut: USHORT,
    lVal lVal_mut: LONG,
    ulVal ulVal_mut: ULONG,
    intVal intVal_mut: INT,
    uintVal uintVal_mut: UINT,
    hVal hVal_mut: LARGE_INTEGER,
    uhVal uhVal_mut: ULARGE_INTEGER,
    fltVal fltVal_mut: FLOAT,
    dblVal dblVal_mut: DOUBLE,
    boolVal boolVal_mut: VARIANT_BOOL,
    scode scode_mut: SCODE,
    cyVal cyVal_mut: CY,
    date date_mut: DATE,
    filetime filetime_mut: FILETIME,
    puuid puuid_mut: *mut CLSID,
    pclipdata pclipdata_mut: *mut CLIPDATA,
    bstrVal bstrVal_mut: BSTR,
    bstrblobVal bstrblobVal_mut: BSTRBLOB,
    blob blob_mut: BLOB,
    pszVal pszVal_mut: LPSTR,
    pwszVal pwszVal_mut: LPWSTR,
    punkVal punkVal_mut: *mut IUnknown,
    pdispVal pdisp_mut: *mut IDispatch,
    pStream pStream_mut: *mut IStream,
    // pStorage pStorage_mut: *mut IStorage,
    pVersionedStream pVersionedStream_mut: LPVERSIONEDSTREAM,
    parray parray_mut: LPSAFEARRAY,
    cac cac_mut: CAC,
    caub caub_mut: CAUB,
    cai cai_mut: CAI,
    caui caui_mut: CAUI,
    cal cal_mut: CAL,
    caul caul_mut: CAUL,
    cah cah_mut: CAH,
    cauh cauh_mut: CAUH,
    caflt caflt_mut: CAFLT,
    cadbl cadbl_mut: CADBL,
    cabool cabool_mut: CABOOL,
    cascode cascode_mut: CASCODE,
    cacy cacy_mut: CACY,
    cadate cadate_mut: CADATE,
    cafiletime cafiletime_mut: CAFILETIME,
    cauuid cauuid_mut: CACLSID,
    caclipdata caclipdata_mut: CACLIPDATA,
    cabstr cabstr_mut: CABSTR,
    cabstrblob cabstrblob_mut: CABSTRBLOB,
    calpstr calpstr_mut: CALPSTR,
    calpwstr calpwstr_mut: CALPWSTR,
    capropvar capropvar_mut: CAPROPVARIANT,
    pcVal pcVal_mut: *mut CHAR,
    pbVal pbVal_mut: *mut UCHAR,
    piVal piVal_mut: *mut SHORT,
    puiVal puiVal_mut: *mut USHORT,
    plVal plVal_mut: *mut LONG,
    pulVal pulVal_mut: *mut ULONG,
    pintVal pintVal_mut: *mut INT,
    puintVal puintVal_mut: *mut UINT,
    pfltVal pfltVal_mut: *mut FLOAT,
    pdblVal pdblVal_mut: *mut DOUBLE,
    pboolVal pboolVal_mut: *mut VARIANT_BOOL,
    pdecVal pdecVal_mut: *mut DECIMAL,
    pscode pscode_mut: *mut SCODE,
    pcyVal pcyVal_mut: *mut CY,
    pdate pdate_mut: *mut DATE,
    ppunkVal ppunkVal_mut: *mut *mut IUnknown,
    ppdispVal ppdispVal_mut: *mut *mut IDispatch,
    ppStream ppStream_mut: *mut *mut IStream,
    // ppStorage ppStorage_mut: *mut *mut IStorage,
}}
// This is actually defined as a union between this struct
// and DECIMAL. I don't this we need to do that.
STRUCT!{struct PROPVARIANT {
    vt: VARTYPE,
    wReserved1: WORD,
    wReserved2: WORD,
    wReserved3: WORD,
    data: PROPVARIANT_data,
}}
pub type LPPROPVARIANT = *mut PROPVARIANT;
pub type REFPROPVARIANT = *const PROPVARIANT;
pub const PID_DICTIONARY: DWORD = 0;
pub const PID_CODEPAGE: DWORD = 0x1;
pub const PID_FIRST_USABLE: DWORD = 0x2;
pub const PID_FIRST_NAME_DEFAULT: DWORD = 0xfff;
pub const PID_LOCALE: DWORD = 0x80000000;
pub const PID_MODIFY_TIME: DWORD = 0x80000001;
pub const PID_SECURITY: DWORD = 0x80000002;
pub const PID_BEHAVIOR: DWORD = 0x80000003;
pub const PID_ILLEGAL: DWORD = 0xffffffff;
pub const PID_MIN_READONLY: DWORD = 0x80000000;
pub const PID_MAX_READONLY: DWORD = 0xbfffffff;
pub const PRSPEC_INVALID: ULONG = 0xffffffff;
pub const PRSPEC_LPWSTR: ULONG = 0;
pub const PRSPEC_PROPID: ULONG = 1;
UNION!{union PROPSPEC_u {
    [u32; 1] [u64; 1],
    propid propid_mut: PROPID,
    lpwstr lpwstr_mut: LPOLESTR,
}}
STRUCT!{struct PROPSPEC {
    ulKind: ULONG,
    u: PROPSPEC_u,
}}
STRUCT!{struct STATPROPSTG {
    lpwstrName: LPOLESTR,
    propid: PROPID,
    vt: VARTYPE,
}}
#[inline]
pub fn PROPSETHDR_OSVER_KIND(dwOSVer: DWORD) -> WORD {
    HIWORD(dwOSVer)
}
#[inline]
pub fn PROPSETHDR_OSVER_MAJOR(dwOSVer: DWORD) -> BYTE {
    LOBYTE(LOWORD(dwOSVer))
}
#[inline]
pub fn PROPSETHDR_OSVER_MINOR(dwOSVer: DWORD) -> BYTE {
    HIBYTE(LOWORD(dwOSVer))
}
pub const PROPSETHDR_OSVERSION_UNKNOWN: DWORD = 0xFFFFFFFF;
STRUCT!{struct STATPROPSETSTG {
    fmtid: FMTID,
    clsid: CLSID,
    grfFlags: DWORD,
    mtime: FILETIME,
    ctime: FILETIME,
    atime: FILETIME,
    dwOSVersion: DWORD,
}}
RIDL!{#[uuid(0x00000138, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPropertyStorage(IPropertyStorageVtbl): IUnknown(IUnknownVtbl) {
    fn ReadMultiple(
        cpspec: ULONG,
        rgpspec: *const PROPSPEC,
        rgpropvar: *mut PROPVARIANT,
    ) -> HRESULT,
    fn WriteMultiple(
        cpspec: ULONG,
        rgpspec: *const PROPSPEC,
        rgpropvar: *const PROPVARIANT,
    ) -> HRESULT,
    fn DeleteMultiple(
        cpspec: ULONG,
        rgpspec: *const PROPSPEC,
    ) -> HRESULT,
    fn ReadPropertyNames(
        cppropid: ULONG,
        rgpropid: *const PROPID,
        rglpwstrName: *mut LPOLESTR,
    ) -> HRESULT,
    fn WritePropertyNames(
        cppropid: ULONG,
        rgpropid: *const PROPID,
        rglpwstrName: *const LPOLESTR,
    ) -> HRESULT,
    fn DeletePropertyNames(
        cppropid: ULONG,
        rgpropid: *const PROPID,
    ) -> HRESULT,
    fn Commit(
        grfCommitFlags: DWORD,
    ) -> HRESULT,
    fn Revert() -> HRESULT,
    fn Enum(
        ppenum: *mut *mut IEnumSTATPROPSTG,
    ) -> HRESULT,
    fn SetTimes(
        pctime: *const FILETIME,
        patime: *const FILETIME,
        pmtime: *const FILETIME,
    ) -> HRESULT,
    fn SetClass(
        clsid: REFCLSID,
    ) -> HRESULT,
    fn Stat(
        pstatpsstg: *mut STATPROPSETSTG,
    ) -> HRESULT,
}}
pub type LPPROPERTYSETSTORAGE = *mut IPropertySetStorage;
RIDL!{#[uuid(0x0000013A, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPropertySetStorage(IPropertySetStorageVtbl): IUnknown(IUnknownVtbl) {
    fn Create(
        rfmtid: REFFMTID,
        pclsid: *const CLSID,
        grfFlags: DWORD,
        grfMode: DWORD,
        ppprstg: *mut *mut IPropertyStorage,
    ) -> HRESULT,
    fn Open(
        rfmtid: REFFMTID,
        grfMode: DWORD,
        ppprstg: *mut *mut IPropertyStorage,
    ) -> HRESULT,
    fn Delete(
        rfmtid: REFFMTID,
    ) -> HRESULT,
    fn Enum(
        ppenum: *mut *mut IEnumSTATPROPSTG,
    ) -> HRESULT,
}}
pub type LPENUMSTATPROPSTG = *mut IEnumSTATPROPSTG;
RIDL!{#[uuid(0x00000139, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumSTATPROPSTG(IEnumSTATPROPSTGVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut STATPROPSTG,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Revert() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumSTATPROPSTG,
    ) -> HRESULT,
}}
pub type LPENUMSTATPROPSETSTG = *mut IEnumSTATPROPSETSTG;
RIDL!{#[uuid(0x0000013B, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumSTATPROPSETSTG(IEnumSTATPROPSETSTGVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut STATPROPSETSTG,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Revert() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumSTATPROPSETSTG,
    ) -> HRESULT,
}}
pub type LPPROPERTYSTORAGE = *mut IPropertyStorage;
pub const PIDDI_THUMBNAIL: DWORD = 0x00000002;
pub const PIDSI_TITLE: DWORD = 0x00000002;
pub const PIDSI_SUBJECT: DWORD = 0x00000003;
pub const PIDSI_AUTHOR: DWORD = 0x00000004;
pub const PIDSI_KEYWORDS: DWORD = 0x00000005;
pub const PIDSI_COMMENTS: DWORD = 0x00000006;
pub const PIDSI_TEMPLATE: DWORD = 0x00000007;
pub const PIDSI_LASTAUTHOR: DWORD = 0x00000008;
pub const PIDSI_REVNUMBER: DWORD = 0x00000009;
pub const PIDSI_EDITTIME: DWORD = 0x0000000a;
pub const PIDSI_LASTPRINTED: DWORD = 0x0000000b;
pub const PIDSI_CREATE_DTM: DWORD = 0x0000000c;
pub const PIDSI_LASTSAVE_DTM: DWORD = 0x0000000d;
pub const PIDSI_PAGECOUNT: DWORD = 0x0000000e;
pub const PIDSI_WORDCOUNT: DWORD = 0x0000000f;
pub const PIDSI_CHARCOUNT: DWORD = 0x00000010;
pub const PIDSI_THUMBNAIL: DWORD = 0x00000011;
pub const PIDSI_APPNAME: DWORD = 0x00000012;
pub const PIDSI_DOC_SECURITY: DWORD = 0x00000013;
pub const PIDDSI_CATEGORY: DWORD = 0x00000002;
pub const PIDDSI_PRESFORMAT: DWORD = 0x00000003;
pub const PIDDSI_BYTECOUNT: DWORD = 0x00000004;
pub const PIDDSI_LINECOUNT: DWORD = 0x00000005;
pub const PIDDSI_PARCOUNT: DWORD = 0x00000006;
pub const PIDDSI_SLIDECOUNT: DWORD = 0x00000007;
pub const PIDDSI_NOTECOUNT: DWORD = 0x00000008;
pub const PIDDSI_HIDDENCOUNT: DWORD = 0x00000009;
pub const PIDDSI_MMCLIPCOUNT: DWORD = 0x0000000A;
pub const PIDDSI_SCALE: DWORD = 0x0000000B;
pub const PIDDSI_HEADINGPAIR: DWORD = 0x0000000C;
pub const PIDDSI_DOCPARTS: DWORD = 0x0000000D;
pub const PIDDSI_MANAGER: DWORD = 0x0000000E;
pub const PIDDSI_COMPANY: DWORD = 0x0000000F;
pub const PIDDSI_LINKSDIRTY: DWORD = 0x00000010;
pub const PIDMSI_EDITOR: DWORD = 0x00000002;
pub const PIDMSI_SUPPLIER: DWORD = 0x00000003;
pub const PIDMSI_SOURCE: DWORD = 0x00000004;
pub const PIDMSI_SEQUENCE_NO: DWORD = 0x00000005;
pub const PIDMSI_PROJECT: DWORD = 0x00000006;
pub const PIDMSI_STATUS: DWORD = 0x00000007;
pub const PIDMSI_OWNER: DWORD = 0x00000008;
pub const PIDMSI_RATING: DWORD = 0x00000009;
pub const PIDMSI_PRODUCTION: DWORD = 0x0000000A;
pub const PIDMSI_COPYRIGHT: DWORD = 0x0000000B;
ENUM!{enum PIDMSI_STATUS_VALUE {
    PIDMSI_STATUS_NORMAL = 0,
    PIDMSI_STATUS_NEW,
    PIDMSI_STATUS_PRELIM,
    PIDMSI_STATUS_DRAFT,
    PIDMSI_STATUS_INPROGRESS,
    PIDMSI_STATUS_EDIT,
    PIDMSI_STATUS_REVIEW,
    PIDMSI_STATUS_PROOF,
    PIDMSI_STATUS_FINAL,
    PIDMSI_STATUS_OTHER = 0x7fff,
}}
extern "system" {
    pub fn PropVariantCopy(
        pvarDest: *mut PROPVARIANT,
        pvarSrc: *const PROPVARIANT,
    ) -> HRESULT;
    pub fn PropVariantClear(
        pvar: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn FreePropVariantArray(
        cVariants: ULONG,
        rgvars: *mut PROPVARIANT,
    ) -> HRESULT;
}
// #[inline]
// pub fn PropVariantInit(pvar: *mut PROPVARIANT) {
//     memset(pvar, 0, sizeof(PROPVARIANT))
// }
STRUCT!{struct SERIALIZEDPROPERTYVALUE {
    dwType: DWORD,
    rgb: *mut BYTE,
}}
pub type PMemoryAllocator = PVOID;
extern "system" {
    pub fn StgConvertVariantToProperty(
        pvar: *const PROPVARIANT,
        CodePage: USHORT,
        pprop: *mut SERIALIZEDPROPERTYVALUE,
        pcb: *mut ULONG,
        pid: PROPID,
        fReserved: BOOLEAN,
        pcIndirect: *mut ULONG,
    ) -> *mut SERIALIZEDPROPERTYVALUE;
    pub fn StgConvertPropertyToVariant(
        pprop: *const SERIALIZEDPROPERTYVALUE,
        CodePage: USHORT,
        pvar: *mut PROPVARIANT,
        pma: *mut PMemoryAllocator
    ) -> BOOLEAN;
}
