// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of OAIdl.h
use shared::basetsd::ULONG_PTR;
use shared::guiddef::{GUID, IID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, INT, UINT, ULONG, USHORT, WORD};
use shared::rpcndr::byte;
use shared::wtypes::{
    BSTR, CY, DATE, DECIMAL, VARIANT_BOOL, VARTYPE, VT_BSTR, VT_DISPATCH, VT_ERROR,
    VT_I1, VT_I2, VT_I4, VT_I8, VT_RECORD, VT_RESERVED, VT_UNKNOWN, VT_VARIANT,
    wireBSTR
};
use shared::wtypesbase::{
    BYTE_SIZEDARR, DOUBLE, DWORD_SIZEDARR, HYPER_SIZEDARR, LPCOLESTR, LPOLESTR, SCODE,
    WORD_SIZEDARR
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{CHAR, HRESULT, LCID, LONG, LONGLONG, PVOID, SHORT, ULONGLONG};
pub type CURRENCY = CY;
STRUCT!{struct SAFEARRAYBOUND {
    cElements: ULONG,
    lLbound: LONG,
}}
pub type LPSAFEARRAYBOUND = *mut SAFEARRAYBOUND;
pub type wireBRECORD = *mut _wireBRECORD;
pub type wireVARIANT = *mut _wireVARIANT;
STRUCT!{struct SAFEARR_BSTR {
    Size: ULONG,
    aBstr: *mut wireBSTR,
}}
STRUCT!{struct SAFEARR_UNKNOWN {
    Size: ULONG,
    apUnknown: *mut *mut IUnknown,
}}
STRUCT!{struct SAFEARR_DISPATCH {
    Size: ULONG,
    apDispatch: *mut *mut IDispatch,
}}
STRUCT!{struct SAFEARR_VARIANT {
    Size: ULONG,
    aVariant: *mut wireVARIANT,
}}
STRUCT!{struct SAFEARR_BRECORD {
    Size: ULONG,
    aRecord: *mut wireBRECORD,
}}
STRUCT!{struct SAFEARR_HAVEIID {
    Size: ULONG,
    apUnknown: *mut *mut IUnknown,
    iid: IID,
}}
ENUM!{enum SF_TYPE {
    SF_ERROR = VT_ERROR,
    SF_I1 = VT_I1,
    SF_I2 = VT_I2,
    SF_I4 = VT_I4,
    SF_I8 = VT_I8,
    SF_BSTR = VT_BSTR,
    SF_UNKNOWN = VT_UNKNOWN,
    SF_DISPATCH = VT_DISPATCH,
    SF_VARIANT = VT_VARIANT,
    SF_RECORD = VT_RECORD,
    SF_HAVEIID = VT_UNKNOWN | VT_RESERVED,
}}
#[cfg(target_pointer_width = "32")]
UNION!{union __MIDL_IOleAutomationTypes_0001 {
    [u32; 6],
    BstrStr BstrStr_mut: SAFEARR_BSTR,
    UnknownStr UnknownStr_mut: SAFEARR_UNKNOWN,
    DispatchStr DispatchStr_mut: SAFEARR_DISPATCH,
    VariantStr VariantStr_mut: SAFEARR_VARIANT,
    RecordStr RecordStr_mut: SAFEARR_BRECORD,
    HaveIidStr HaveIidStr_mut: SAFEARR_HAVEIID,
    ByteStr ByteStr_mut: BYTE_SIZEDARR,
    WordStr WordStr_mut: WORD_SIZEDARR,
    LongStr LongStr_mut: DWORD_SIZEDARR,
    HyperStr HyperStr_mut: HYPER_SIZEDARR,
}}
#[cfg(target_pointer_width = "64")]
UNION!{union __MIDL_IOleAutomationTypes_0001 {
    [u64; 4],
    BstrStr BstrStr_mut: SAFEARR_BSTR,
    UnknownStr UnknownStr_mut: SAFEARR_UNKNOWN,
    DispatchStr DispatchStr_mut: SAFEARR_DISPATCH,
    VariantStr VariantStr_mut: SAFEARR_VARIANT,
    RecordStr RecordStr_mut: SAFEARR_BRECORD,
    HaveIidStr HaveIidStr_mut: SAFEARR_HAVEIID,
    ByteStr ByteStr_mut: BYTE_SIZEDARR,
    WordStr WordStr_mut: WORD_SIZEDARR,
    LongStr LongStr_mut: DWORD_SIZEDARR,
    HyperStr HyperStr_mut: HYPER_SIZEDARR,
}}
STRUCT!{struct SAFEARRAYUNION {
    sfType: ULONG,
    u: __MIDL_IOleAutomationTypes_0001,
}}
STRUCT!{struct _wireSAFEARRAY {
    cDims: USHORT,
    fFeatures: USHORT,
    cbElements: ULONG,
    cLocks: ULONG,
    uArrayStructs: SAFEARRAYUNION,
    rgsaBound: [SAFEARRAYBOUND; 1],
}}
pub type wireSAFEARRAY = *mut _wireSAFEARRAY;
pub type wirePSAFEARRAY = *mut wireSAFEARRAY;
STRUCT!{struct SAFEARRAY {
    cDims: USHORT,
    fFeatures: USHORT,
    cbElements: ULONG,
    cLocks: ULONG,
    pvData: PVOID,
    rgsabound: [SAFEARRAYBOUND; 1],
}}
pub type LPSAFEARRAY = *mut SAFEARRAY;
pub const FADF_AUTO: DWORD = 0x1;
pub const FADF_STATIC: DWORD = 0x2;
pub const FADF_EMBEDDED: DWORD = 0x4;
pub const FADF_FIXEDSIZE: DWORD = 0x10;
pub const FADF_RECORD: DWORD = 0x20;
pub const FADF_HAVEIID: DWORD = 0x40;
pub const FADF_HAVEVARTYPE: DWORD = 0x80;
pub const FADF_BSTR: DWORD = 0x100;
pub const FADF_UNKNOWN: DWORD = 0x200;
pub const FADF_DISPATCH: DWORD = 0x400;
pub const FADF_VARIANT: DWORD = 0x800;
pub const FADF_RESERVED: DWORD = 0xf008;
STRUCT!{struct __tagBRECORD {
    pvRecord: PVOID,
    pRecInfo: *mut IRecordInfo,
}}
UNION!{union VARIANT_n3 {
    [u64; 1] [u64; 2],
    llVal llVal_mut: LONGLONG,
    lVal lVal_mut: LONG,
    bVal bVal_mut: BYTE,
    iVal iVal_mut: SHORT,
    fltVal fltVal_mut: FLOAT,
    dblVal dblVal_mut: DOUBLE,
    boolVal boolVal_mut: VARIANT_BOOL,
    scode scode_mut: SCODE,
    cyVal cyVal_mut: CY,
    date date_mut: DATE,
    bstrVal bstrVal_mut: BSTR,
    punkVal punkVal_mut: *mut IUnknown,
    pdispVal pdispVal_mut: *mut IDispatch,
    parray parray_mut: *mut SAFEARRAY,
    pbVal pbVal_mut: *mut BYTE,
    piVal piVal_mut: *mut SHORT,
    plVal plVal_mut: *mut LONG,
    pllVal pllVal_mut: *mut LONGLONG,
    pfltVal pfltVal_mut: *mut FLOAT,
    pdblVal pdblVal_mut: *mut DOUBLE,
    pboolVal pboolVal_mut: *mut VARIANT_BOOL,
    pscode pscode_mut: *mut SCODE,
    pcyVal pcyVal_mut: *mut CY,
    pdate pdate_mut: *mut DATE,
    pbstrVal pbstrVal_mut: *mut BSTR,
    ppunkVal ppunkVal_mut: *mut *mut IUnknown,
    ppdispVal ppdispVal_mut: *mut *mut IDispatch,
    pparray pparray_mut: *mut *mut SAFEARRAY,
    pvarVal pvarVal_mut: *mut VARIANT,
    byref byref_mut: PVOID,
    cVal cVal_mut: CHAR,
    uiVal uiVal_mut: USHORT,
    ulVal ulVal_mut: ULONG,
    ullVal ullVal_mut: ULONGLONG,
    intVal intVal_mut: INT,
    uintVal uintVal_mut: UINT,
    pdecVal pdecVal_mut: *mut DECIMAL,
    pcVal pcVal_mut: *mut CHAR,
    puiVal puiVal_mut: *mut USHORT,
    pulVal pulVal_mut: *mut ULONG,
    pullVal pullVal_mut: *mut ULONGLONG,
    pintVal pintVal_mut: *mut INT,
    puintVal puintVal_mut: *mut UINT,
    n4 n4_mut: __tagBRECORD,
}}
STRUCT!{struct __tagVARIANT {
    vt: VARTYPE,
    wReserved1: WORD,
    wReserved2: WORD,
    wReserved3: WORD,
    n3: VARIANT_n3,
}}
UNION!{union VARIANT_n1 {
    [u64; 2] [u64; 3],
    n2 n2_mut: __tagVARIANT,
    decVal decVal_mut: DECIMAL,
}}
STRUCT!{struct VARIANT {
    n1: VARIANT_n1,
}}
pub type LPVARIANT = *mut VARIANT;
pub type VARIANTARG = VARIANT;
pub type LPVARIANTARG = *mut VARIANT;
pub type REFVARIANT = *const VARIANT;
STRUCT!{struct _wireBRECORD {
    fFlags: ULONG,
    clSize: ULONG,
    pRecInfo: *mut IRecordInfo,
    pRecord: *mut byte,
}}
UNION!{union _wireVARIANT_u {
    [u64; 2],
    llVal llVal_mut: LONGLONG,
    lVal lVal_mut: LONG,
    bVal bVal_mut: BYTE,
    iVal iVal_mut: SHORT,
    fltVal fltVal_mut: FLOAT,
    dblVal dblVal_mut: DOUBLE,
    boolVal boolVal_mut: VARIANT_BOOL,
    scode scode_mut: SCODE,
    cyVal cyVal_mut: CY,
    date date_mut: DATE,
    bstrVal bstrVal_mut: wireBSTR,
    punkVal punkVal_mut: *mut IUnknown,
    pdispVal pdispVal_mut: *mut IDispatch,
    parray parray_mut: wirePSAFEARRAY,
    brecVal brecVal_mut: wireBRECORD,
    pbVal pbVal_mut: *mut BYTE,
    piVal piVal_mut: *mut SHORT,
    plVal plVal_mut: *mut LONG,
    pllVal pllVal_mut: *mut LONGLONG,
    pfltVal pfltVal_mut: *mut FLOAT,
    pdblVal pdblVal_mut: *mut DOUBLE,
    pboolVal pboolVal_mut: *mut VARIANT_BOOL,
    pscode pscode_mut: *mut SCODE,
    pcyVal pcyVal_mut: *mut CY,
    pdate pdate_mut: *mut DATE,
    pbstrVal pbstrVal_mut: *mut wireBSTR,
    ppunkVal ppunkVal_mut: *mut *mut IUnknown,
    ppdispVal ppdispVal_mut: *mut *mut IDispatch,
    pparray pparray_mut: *mut wirePSAFEARRAY,
    pvarVal pvarVal_mut: *mut wireVARIANT,
    cVal cVal_mut: CHAR,
    uiVal uiVal_mut: USHORT,
    ulVal ulVal_mut: ULONG,
    ullVal ullVal_mut: ULONGLONG,
    intVal intVal_mut: INT,
    uintVal uintVal_mut: UINT,
    decVal decVal_mut: DECIMAL,
    pdecVal pdecVal_mut: *mut DECIMAL,
    pcVal pcVal_mut: *mut CHAR,
    puiVal puiVal_mut: *mut USHORT,
    pulVal pulVal_mut: *mut ULONG,
    pullVal pullVal_mut: *mut ULONGLONG,
    pintVal pintVal_mut: *mut INT,
    puintVal puintVal_mut: *mut UINT,
}}
STRUCT!{struct _wireVARIANT {
    clSize: DWORD,
    rpcReserved: DWORD,
    vt: USHORT,
    wReserved1: USHORT,
    wReserved2: USHORT,
    wReserved3: USHORT,
    u: _wireVARIANT_u,
}}
pub type DISPID = LONG;
pub type MEMBERID = DISPID;
pub type HREFTYPE = DWORD;
ENUM!{enum TYPEKIND {
    TKIND_ENUM = 0,
    TKIND_RECORD,
    TKIND_MODULE,
    TKIND_INTERFACE,
    TKIND_DISPATCH,
    TKIND_COCLASS,
    TKIND_ALIAS,
    TKIND_UNION,
    TKIND_MAX,
}}
UNION!{union TYPEDESC_u {
    [usize; 1],
    lptdesc lptdesc_mut: *mut TYPEDESC,
    lpadesc lpadesc_mut: *mut ARRAYDESC,
    hreftype hreftype_mut: HREFTYPE,
}}
STRUCT!{struct TYPEDESC {
    u: TYPEDESC_u,
    vt: VARTYPE,
}}
STRUCT!{struct ARRAYDESC {
    tdescElem: TYPEDESC,
    cDims: USHORT,
    rgbounds: [SAFEARRAYBOUND; 1],
}}
STRUCT!{struct PARAMDESCEX {
    cBytes: ULONG,
    varDefaultValue: VARIANTARG,
}}
pub type LPPARAMDESCEX = *mut PARAMDESCEX;
STRUCT!{struct PARAMDESC {
    pparamdescex: LPPARAMDESCEX,
    wParamFlags: USHORT,
}}
pub type LPPARAMDESC = *mut PARAMDESC;
pub const PARAMFLAG_NONE: DWORD = 0;
pub const PARAMFLAG_FIN: DWORD = 0x1;
pub const PARAMFLAG_FOUT: DWORD = 0x2;
pub const PARAMFLAG_FLCID: DWORD = 0x4;
pub const PARAMFLAG_FRETVAL: DWORD = 0x8;
pub const PARAMFLAG_FOPT: DWORD = 0x10;
pub const PARAMFLAG_FHASDEFAULT: DWORD = 0x20;
pub const PARAMFLAG_FHASCUSTDATA: DWORD = 0x40;
STRUCT!{struct IDLDESC {
    dwReserved: ULONG_PTR,
    wIDLFlags: USHORT,
}}
pub type LPIDLDESC = *mut IDLDESC;
pub const IDLFLAG_NONE: DWORD = PARAMFLAG_NONE;
pub const IDLFLAG_FIN: DWORD = PARAMFLAG_FIN;
pub const IDLFLAG_FOUT: DWORD = PARAMFLAG_FOUT;
pub const IDLFLAG_FLCID: DWORD = PARAMFLAG_FLCID;
pub const IDLFLAG_FRETVAL: DWORD = PARAMFLAG_FRETVAL;
UNION!{union ELEMDESC_u {
    [usize; 2],
    idldesc idldesc_mut: IDLDESC,
    paramdesc paramdesc_mut: PARAMDESC,
}}
STRUCT!{struct ELEMDESC {
    tdesc: TYPEDESC,
    u: ELEMDESC_u,
}}
pub type LPELEMDESC = *mut ELEMDESC;
STRUCT!{struct TYPEATTR {
    guid: GUID,
    lcid: LCID,
    dwReserved: DWORD,
    memidConstructor: MEMBERID,
    memidDestructor: MEMBERID,
    lpstrSchema: LPOLESTR,
    cbSizeInstance: ULONG,
    typekind: TYPEKIND,
    cFuncs: WORD,
    cVars: WORD,
    cImplTypes: WORD,
    cbSizeVft: WORD,
    cbAlignment: WORD,
    wTypeFlags: WORD,
    wMajorVerNum: WORD,
    wMinorVerNum: WORD,
    tdescAlias: TYPEDESC,
    idldescType: IDLDESC,
}}
pub type LPTYPEATTR = *mut TYPEATTR;
STRUCT!{struct DISPPARAMS {
    rgvarg: *mut VARIANTARG,
    rgdispidNamedArgs: *mut DISPID,
    cArgs: UINT,
    cNamedArgs: UINT,
}}
STRUCT!{struct EXCEPINFO {
    wCode: WORD,
    wReserved: WORD,
    bstrSource: BSTR,
    bstrDescription: BSTR,
    bstrHelpFile: BSTR,
    dwHelpContext: DWORD,
    pvReserved: PVOID,
    pfnDeferredFillIn: Option<unsafe extern "system" fn(
        einfo: *mut EXCEPINFO,
    ) -> HRESULT>,
    scode: SCODE,
}}
ENUM!{enum CALLCONV {
    CC_FASTCALL = 0,
    CC_CDECL = 1,
    CC_MSCPASCAL,
    CC_PASCAL,
    CC_MACPASCAL,
    CC_STDCALL,
    CC_FPFASTCALL,
    CC_SYSCALL,
    CC_MPWCDECL,
    CC_MPWPASCAL,
    CC_MAX,
}}
ENUM!{enum FUNCKIND {
    FUNC_VIRTUAL = 0,
    FUNC_PUREVIRTUAL,
    FUNC_NONVIRTUAL,
    FUNC_STATIC,
    FUNC_DISPATCH,
}}
ENUM!{enum INVOKEKIND {
    INVOKE_FUNC = 1,
    INVOKE_PROPERTYGET = 2,
    INVOKE_PROPERTYPUT = 4,
    INVOKE_PROPERTYPUTREF = 8,
}}
STRUCT!{struct FUNCDESC {
    memid: MEMBERID,
    lprgscode: *mut SCODE,
    lprgelemdescParam: *mut ELEMDESC,
    funckind: FUNCKIND,
    invkind: INVOKEKIND,
    callconv: CALLCONV,
    cParams: SHORT,
    cParamsOpt: SHORT,
    oVft: SHORT,
    cScodes: SHORT,
    elemdescFunc: ELEMDESC,
    wFuncFlags: WORD,
}}
pub type LPFUNCDESC = *mut FUNCDESC;
ENUM!{enum VARKIND {
    VAR_PERINSTANCE = 0,
    VAR_STATIC,
    VAR_CONST,
    VAR_DISPATCH,
}}
pub const IMPLTYPEFLAG_FDEFAULT: DWORD = 0x1;
pub const IMPLTYPEFLAG_FSOURCE: DWORD = 0x2;
pub const IMPLTYPEFLAG_FRESTRICTED: DWORD = 0x4;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: DWORD = 0x8;
UNION!{union VARDESC_u {
    [usize; 1],
    oInst oInst_mut: ULONG,
    lpvarValue lpvarValue_mut: *mut VARIANT,
}}
STRUCT!{struct VARDESC {
    memid: MEMBERID,
    lpstrSchema: LPOLESTR,
    u: VARDESC_u,
    elemdescVar: ELEMDESC,
    wVarFlags: WORD,
    varkind: VARKIND,
}}
pub type LPVARDESC = *mut VARDESC;
ENUM!{enum TYPEFLAGS {
    TYPEFLAG_FAPPOBJECT = 0x1,
    TYPEFLAG_FCANCREATE = 0x2,
    TYPEFLAG_FLICENSED = 0x4,
    TYPEFLAG_FPREDECLID = 0x8,
    TYPEFLAG_FHIDDEN = 0x10,
    TYPEFLAG_FCONTROL = 0x20,
    TYPEFLAG_FDUAL = 0x40,
    TYPEFLAG_FNONEXTENSIBLE = 0x80,
    TYPEFLAG_FOLEAUTOMATION = 0x100,
    TYPEFLAG_FRESTRICTED = 0x200,
    TYPEFLAG_FAGGREGATABLE = 0x400,
    TYPEFLAG_FREPLACEABLE = 0x800,
    TYPEFLAG_FDISPATCHABLE = 0x1000,
    TYPEFLAG_FREVERSEBIND = 0x2000,
    TYPEFLAG_FPROXY = 0x4000,
}}
ENUM!{enum FUNCFLAGS {
    FUNCFLAG_FRESTRICTED = 0x1,
    FUNCFLAG_FSOURCE = 0x2,
    FUNCFLAG_FBINDABLE = 0x4,
    FUNCFLAG_FREQUESTEDIT = 0x8,
    FUNCFLAG_FDISPLAYBIND = 0x10,
    FUNCFLAG_FDEFAULTBIND = 0x20,
    FUNCFLAG_FHIDDEN = 0x40,
    FUNCFLAG_FUSESGETLASTERROR = 0x80,
    FUNCFLAG_FDEFAULTCOLLELEM = 0x100,
    FUNCFLAG_FUIDEFAULT = 0x200,
    FUNCFLAG_FNONBROWSABLE = 0x400,
    FUNCFLAG_FREPLACEABLE = 0x800,
    FUNCFLAG_FIMMEDIATEBIND = 0x1000,
}}
ENUM!{enum VARFLAGS {
    VARFLAG_FREADONLY = 0x1,
    VARFLAG_FSOURCE = 0x2,
    VARFLAG_FBINDABLE = 0x4,
    VARFLAG_FREQUESTEDIT = 0x8,
    VARFLAG_FDISPLAYBIND = 0x10,
    VARFLAG_FDEFAULTBIND = 0x20,
    VARFLAG_FHIDDEN = 0x40,
    VARFLAG_FRESTRICTED = 0x80,
    VARFLAG_FDEFAULTCOLLELEM = 0x100,
    VARFLAG_FUIDEFAULT = 0x200,
    VARFLAG_FNONBROWSABLE = 0x400,
    VARFLAG_FREPLACEABLE = 0x800,
    VARFLAG_FIMMEDIATEBIND = 0x1000,
}}
STRUCT!{struct CLEANLOCALSTORAGE {
    pInterface: *mut IUnknown,
    pStorage: PVOID,
    flags: DWORD,
}}
STRUCT!{struct CUSTDATAITEM {
    guid: GUID,
    varValue: VARIANTARG,
}}
pub type LPCUSTDATAITEM = *mut CUSTDATAITEM;
STRUCT!{struct CUSTDATA {
    cCustData: DWORD,
    prgCustData: LPCUSTDATAITEM,
}}
pub type LPCUSTDATA = *mut CUSTDATA;
pub type LPCREATETYPEINFO = *mut ICreateTypeInfo;
RIDL!{#[uuid(0x00020405, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ICreateTypeInfo(ICreateTypeInfoVtbl): IUnknown(IUnknownVtbl) {
    fn SetGuid(
        guid: REFGUID,
    ) -> HRESULT,
    fn SetTypeFlags(
        uTypeFlags: UINT,
    ) -> HRESULT,
    fn SetDocString(
        pStrDoc: LPOLESTR,
    ) -> HRESULT,
    fn SetHelpContext(
        dwHelpContext: DWORD,
    ) -> HRESULT,
    fn SetVersion(
        wMajorVerNum: WORD,
        wMinorVerNum: WORD,
    ) -> HRESULT,
    fn AddRefTypeInfo(
        pTInfo: *mut ITypeInfo,
    ) -> HRESULT,
    fn AddFuncDesc(
        index: UINT,
        pFuncDesc: *mut FUNCDESC,
    ) -> HRESULT,
    fn SetImplTypeFlags(
        index: UINT,
        implTypeFlags: INT,
    ) -> HRESULT,
    fn SetAlignment(
        cbAlignment: WORD,
    ) -> HRESULT,
    fn SetSchema(
        pStrSchema: LPOLESTR,
    ) -> HRESULT,
    fn AddVarDesc(
        index: UINT,
        pVarDesc: *mut VARDESC,
    ) -> HRESULT,
    fn SetFuncAndParamNames(
        index: UINT,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
    ) -> HRESULT,
    fn SetVarName(
        index: UINT,
        szName: LPOLESTR,
    ) -> HRESULT,
    fn SetTypeDescAlias(
        pTDescAlias: *mut TYPEDESC,
    ) -> HRESULT,
    fn DefineFuncAsDllEntry(
        index: UINT,
        szDllName: LPOLESTR,
        szProcName: LPOLESTR,
    ) -> HRESULT,
    fn SetFuncDocString(
        index: UINT,
        szDocString: LPOLESTR,
    ) -> HRESULT,
    fn SetVarDocString(
        index: UINT,
        szDocString: LPOLESTR,
    ) -> HRESULT,
    fn SetFuncHelpContext(
        index: UINT,
        dwHelpContext: DWORD,
    ) -> HRESULT,
    fn SetVarHelpContext(
        index: UINT,
        dwHelpContext: DWORD,
    ) -> HRESULT,
    fn SetMops(
        index: UINT,
        bstrMops: BSTR,
    ) -> HRESULT,
    fn SetTypeIdldesc(
        pIdlDesc: *mut IDLDESC,
    ) -> HRESULT,
    fn LayOut() -> HRESULT,
}}
// LPCREATETYPEINFO2
// ICreateTypeInfo2
// LPCREATETYPELIB
// ICreateTypeLib
// LPCREATETYPELIB2
// ICreateTypeLib2
pub type LPDISPATCH = *mut IDispatch;
pub const DISPID_UNKNOWN: INT = -1;
pub const DISPID_VALUE: INT = 0;
pub const DISPID_PROPERTYPUT: INT = -3;
pub const DISPID_NEWENUM: INT = -4;
pub const DISPID_EVALUATE: INT = -5;
pub const DISPID_CONSTRUCTOR: INT = -6;
pub const DISPID_DESTRUCTOR: INT = -7;
pub const DISPID_COLLECT: INT = -8;
RIDL!{#[uuid(0x00020400, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IDispatch(IDispatchVtbl): IUnknown(IUnknownVtbl) {
    fn GetTypeInfoCount(
        pctinfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo(
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames(
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke(
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}}
// IDispatch_RemoteInvoke_Proxy
// IDispatch_RemoteInvoke_Stub
// LPENUMVARIANT
// IEnumVARIANT
// IEnumVARIANT_RemoteNext_Proxy
// IEnumVARIANT_RemoteNext_Stub
RIDL!{#[uuid(0x0000002F, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IRecordInfo(IRecordInfoVtbl): IUnknown(IUnknownVtbl) {
    fn RecordInit(
        pvNew: PVOID,
    ) -> HRESULT,
    fn RecordClear(
        pvExisting: PVOID,
    ) -> HRESULT,
    fn RecordCopy(
        pvExisting: PVOID,
        pvNew: PVOID,
    ) -> HRESULT,
    fn GetGuid(
        pguid: *mut GUID,
    ) -> HRESULT,
    fn GetName(
        pbstrName: *mut BSTR,
    ) -> HRESULT,
    fn GetSize(
        pcbSize: *mut ULONG,
    ) -> HRESULT,
    fn GetTypeInfo(
        ppTypeInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetField(
        pvData: PVOID,
        szFieldName: LPCOLESTR,
        pvarField: *mut VARIANT,
    ) -> HRESULT,
    fn GetFieldNoCopy(
        pvData: PVOID,
        szFieldName: LPCOLESTR,
        pvarField: *mut VARIANT,
        ppvDataCArray: *mut PVOID,
    ) -> HRESULT,
    fn PutField(
        wFlags: ULONG,
        pvData: PVOID,
        szFieldName: LPCOLESTR,
        pvarField: *mut VARIANT,
    ) -> HRESULT,
    fn PutFieldNoCopy(
        wFlags: ULONG,
        pvData: PVOID,
        szFieldName: LPCOLESTR,
        pvarField: *mut VARIANT,
    ) -> HRESULT,
    fn GetFieldNames(
        pcNames: *mut ULONG,
        rgBstrNames: *mut BSTR,
    ) -> HRESULT,
    fn IsMatchingType(
        pRecordInfo: *mut IRecordInfo,
    ) -> BOOL,
    fn RecordCreate() -> PVOID,
    fn RecordCreateCopy(
        pvSource: PVOID,
        ppvDest: *mut PVOID,
    ) -> HRESULT,
    fn RecordDestroy(
        pvRecord: PVOID,
    ) -> HRESULT,
}}
pub type LPTYPECOMP = *mut ITypeComp;
ENUM!{enum DESCKIND {
    DESCKIND_NONE = 0,
    DESCKIND_FUNCDESC = DESCKIND_NONE + 1,
    DESCKIND_VARDESC = DESCKIND_FUNCDESC + 1,
    DESCKIND_TYPECOMP = DESCKIND_VARDESC + 1,
    DESCKIND_IMPLICITAPPOBJ = DESCKIND_TYPECOMP + 1,
    DESCKIND_MAX = DESCKIND_IMPLICITAPPOBJ + 1,
}}
UNION!{union BINDPTR {
    [usize; 1],
    lpfuncdesc lpfuncdesc_mut: *mut FUNCDESC,
    lpvardesc lpvardesc_mut: *mut VARDESC,
    lptcomp lptcomp_mut: *mut ITypeComp,
}}
pub type LPBINDPTR = *mut BINDPTR;
RIDL!{#[uuid(0x00020403, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeComp(ITypeCompVtbl): IUnknown(IUnknownVtbl) {
    fn Bind(
        szName: LPOLESTR,
        lHashVal: ULONG,
        wFlags: WORD,
        ppTInfo: *mut *mut ITypeInfo,
        pDescKind: *mut DESCKIND,
        pBindPtr: *mut BINDPTR,
    ) -> HRESULT,
    fn BindType(
        szName: LPOLESTR,
        lHashVal: ULONG,
        ppTInfo: *mut *mut ITypeInfo,
        ppTComp: *mut *mut ITypeComp,
    ) -> HRESULT,
}}
ENUM!{enum SYSKIND {
    SYS_WIN16 = 0,
    SYS_WIN32,
    SYS_MAC,
    SYS_WIN64,
}}
STRUCT!{struct TLIBATTR {
    guid: GUID,
    lcid: LCID,
    syskind: SYSKIND,
    wMajorVerNum: WORD,
    wMinorVerNum: WORD,
    wLibFlags: WORD,
}}
RIDL!{#[uuid(0x00020402, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeLib(ITypeLibVtbl): IUnknown(IUnknownVtbl) {
    fn GetTypeInfoCount() -> UINT,
    fn GetTypeInfo(
        index: UINT,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetTypeInfoType(
        index: UINT,
        pTKind: *mut TYPEKIND,
    ) -> HRESULT,
    fn GetTypeInfoOfGuid(
        guid: REFGUID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetLibAttr(
        ppTLibAttr: *mut *mut TLIBATTR,
    ) -> HRESULT,
    fn GetTypeComp(
        ppTComp: *mut *mut ITypeComp,
    ) -> HRESULT,
    fn GetDocumentation(
        index: INT,
        pbstrName: *mut BSTR,
        pBstrDocString: *mut BSTR,
        pdwHelpContext: *mut DWORD,
        pBstrHelpFile: *mut BSTR,
    ) -> HRESULT,
    fn IsName(
        szNameBuf: LPOLESTR,
        lHashVal: ULONG,
        pfName: *mut BOOL,
    ) -> HRESULT,
    fn FindName(
        szNameBuf: LPOLESTR,
        lHashVal: ULONG,
        ppTInfo: *mut *mut ITypeInfo,
        rgMemId: *mut MEMBERID,
        pcFound: *mut USHORT,
    ) -> HRESULT,
    fn ReleaseTLibAttr(
        pTLibAttr: *const TLIBATTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00020401, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ITypeInfo(ITypeInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetTypeAttr(
        ppTypeAttr: *mut *mut TYPEATTR,
    ) -> HRESULT,
    fn GetTypeComp(
        ppTComp: *mut *mut ITypeComp,
    ) -> HRESULT,
    fn GetFuncDesc(
        index: UINT,
        ppFunDesc: *mut *mut FUNCDESC,
    ) -> HRESULT,
    fn GetVarDesc(
        index: UINT,
        pPVarDesc: *mut *mut VARDESC,
    ) -> HRESULT,
    fn GetNames(
        memid: MEMBERID,
        rgBstrNames: *mut BSTR,
        cMaxNames: UINT,
        pcNames: *mut UINT,
    ) -> HRESULT,
    fn GetRefTypeOfImplType(
        index: UINT,
        pRefType: *mut HREFTYPE,
    ) -> HRESULT,
    fn GetImplTypeFlags(
        index: UINT,
        pImplTypeFlags: *mut INT,
    ) -> HRESULT,
    fn GetIDsOfNames(
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        pMemId: *mut MEMBERID,
    ) -> HRESULT,
    fn Invoke(
        pvInstance: PVOID,
        memid: MEMBERID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
    fn GetDocumentation(
        memid: MEMBERID,
        pBstrName: *mut BSTR,
        pBstrDocString: *mut BSTR,
        pdwHelpContext: *mut DWORD,
        pBstrHelpFile: *mut BSTR,
    ) -> HRESULT,
    fn GetDllEntry(
        memid: MEMBERID,
        invKind: INVOKEKIND,
        pBstrDllName: *mut BSTR,
        pBstrName: *mut BSTR,
        pwOrdinal: *mut WORD,
    ) -> HRESULT,
    fn GetRefTypeInfo(
        hRefType: HREFTYPE,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn AddressOfMember(
        memid: MEMBERID,
        invKind: INVOKEKIND,
        ppv: *mut PVOID,
    ) -> HRESULT,
    fn CreateInstance(
        pUnkOuter: *mut IUnknown,
        riid: REFIID,
        ppvObj: *mut PVOID,
    ) -> HRESULT,
    fn GetMops(
        memid: MEMBERID,
        pBstrMops: *mut BSTR,
    ) -> HRESULT,
    fn GetContainingTypeLib(
        ppTLib: *mut *mut ITypeLib,
        pIndex: *mut UINT,
    ) -> HRESULT,
    fn ReleaseTypeAttr(
        pTypeAttr: *mut TYPEATTR,
    ) -> (),
    fn ReleaseFuncDesc(
        pFuncDesc: *mut FUNCDESC,
    ) -> (),
    fn ReleaseVarDesc(
        pVarDesc: *mut VARDESC,
    ) -> (),
}}
RIDL!{#[uuid(0x1cf2b120, 0x547d, 0x101b, 0x8e, 0x65, 0x08, 0x00, 0x2b, 0x2b, 0xd1, 0x19)]
interface IErrorInfo(IErrorInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetGUID(
        pGUID: *mut GUID,
    ) -> HRESULT,
    fn GetSource(
        pBstrSource: *mut BSTR,
    ) -> HRESULT,
    fn GetDescription(
        pBstrDescription: *mut BSTR,
    ) -> HRESULT,
    fn GetHelpFile(
        pBstrHelpFile: *mut BSTR,
    ) -> HRESULT,
    fn GetHelpContext(
        pdwHelpContext: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x22f03340, 0x547d, 0x101b, 0x8e, 0x65, 0x08, 0x00, 0x2b, 0x2b, 0xd1, 0x19)]
interface ICreateErrorInfo(ICreateErrorInfoVtbl): IUnknown(IUnknownVtbl) {
    fn SetGUID(
        rguid: REFGUID,
    ) -> HRESULT,
    fn SetSource(
        szSource: LPOLESTR,
    ) -> HRESULT,
    fn SetDescription(
        szDescription: LPOLESTR,
    ) -> HRESULT,
    fn SetHelpFile(
        szHelpFile: LPOLESTR,
    ) -> HRESULT,
    fn SetHelpContext(
        dwHelpContext: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3127ca40, 0x446e, 0x11ce, 0x81, 0x35, 0x00, 0xaa, 0x00, 0x4b, 0xb8, 0x51)]
interface IErrorLog(IErrorLogVtbl): IUnknown(IUnknownVtbl) {
    fn AddError(
        pszPropName: LPCOLESTR,
        pExcepInfo: *const EXCEPINFO,
    ) -> HRESULT,
}}
pub type LPERRORLOG = *mut IErrorLog;
