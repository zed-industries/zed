#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : * mut core::ffi::c_void, iid : *const GUID, factory : *mut *mut core::ffi::c_void) -> HRESULT);
windows_targets::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> BOOL);
windows_targets::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const SECURITY_ATTRIBUTES, bmanualreset : BOOL, binitialstate : BOOL, lpname : PCWSTR) -> HANDLE);
windows_targets::link!("kernel32.dll" "system" fn EncodePointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : PCSTR) -> FARPROC);
windows_targets::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : PCSTR, hfile : HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE);
windows_targets::link!("kernel32.dll" "system" fn SetEvent(hevent : HANDLE) -> BOOL);
windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : HANDLE, dwmilliseconds : u32) -> WAIT_EVENT);
windows_targets::link!("ole32.dll" "system" fn CoIncrementMTAUsage(pcookie : *mut CO_MTA_USAGE_COOKIE) -> HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
windows_targets::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
windows_targets::link!("ole32.dll" "system" fn PropVariantClear(pvar : *mut PROPVARIANT) -> HRESULT);
windows_targets::link!("ole32.dll" "system" fn PropVariantCopy(pvardest : *mut PROPVARIANT, pvarsrc : *const PROPVARIANT) -> HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VariantClear(pvarg : *mut VARIANT) -> HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn VariantCopy(pvargdest : *mut VARIANT, pvargsrc : *const VARIANT) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn PropVariantCompareEx(propvar1 : *const PROPVARIANT, propvar2 : *const PROPVARIANT, unit : PROPVAR_COMPARE_UNIT, flags : PROPVAR_COMPARE_FLAGS) -> i32);
windows_targets::link!("propsys.dll" "system" fn PropVariantToBSTR(propvar : *const PROPVARIANT, pbstrout : *mut BSTR) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn PropVariantToBoolean(propvarin : *const PROPVARIANT, pfret : *mut BOOL) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn PropVariantToDouble(propvarin : *const PROPVARIANT, pdblret : *mut f64) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn PropVariantToInt16(propvarin : *const PROPVARIANT, piret : *mut i16) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn PropVariantToInt32(propvarin : *const PROPVARIANT, plret : *mut i32) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn PropVariantToInt64(propvarin : *const PROPVARIANT, pllret : *mut i64) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt16(propvarin : *const PROPVARIANT, puiret : *mut u16) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt32(propvarin : *const PROPVARIANT, pulret : *mut u32) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt64(propvarin : *const PROPVARIANT, pullret : *mut u64) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn PropVariantToVariant(ppropvar : *const PROPVARIANT, pvar : *mut VARIANT) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn VariantToBoolean(varin : *const VARIANT, pfret : *mut BOOL) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn VariantToDouble(varin : *const VARIANT, pdblret : *mut f64) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn VariantToInt16(varin : *const VARIANT, piret : *mut i16) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn VariantToInt32(varin : *const VARIANT, plret : *mut i32) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn VariantToInt64(varin : *const VARIANT, pllret : *mut i64) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn VariantToPropVariant(pvar : *const VARIANT, ppropvar : *mut PROPVARIANT) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn VariantToUInt16(varin : *const VARIANT, puiret : *mut u16) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn VariantToUInt32(varin : *const VARIANT, pulret : *mut u32) -> HRESULT);
windows_targets::link!("propsys.dll" "system" fn VariantToUInt64(varin : *const VARIANT, pullret : *mut u64) -> HRESULT);
pub type ADVANCED_FEATURE_FLAGS = u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ARRAYDESC {
    pub tdescElem: TYPEDESC,
    pub cDims: u16,
    pub rgbounds: [SAFEARRAYBOUND; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
pub type BOOL = i32;
pub type BSTR = *const u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut VARIANT_BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut BSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAC {
    pub cElems: u32,
    pub pElems: PSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut CY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
pub type CALLCONV = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut PSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
pub type CO_MTA_USAGE_COOKIE = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous1: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
pub type DESCKIND = i32;
pub type DISPATCH_FLAGS = u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: PARAMDESC,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: BSTR,
    pub bstrDescription: BSTR,
    pub bstrHelpFile: BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: *mut i32,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: FUNCFLAGS,
}
pub type FUNCFLAGS = u16;
pub type FUNCKIND = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}
pub type HANDLE = *mut core::ffi::c_void;
pub type HMODULE = *mut core::ffi::c_void;
pub type HRESULT = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: IDLFLAGS,
}
pub type IDLFLAGS = u16;
pub type IMPLTYPEFLAGS = i32;
pub type INVOKEKIND = i32;
pub type LOAD_LIBRARY_FLAGS = u32;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
pub type LPEXCEPFINO_DEFERRED_FILLIN =
    Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> HRESULT>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PARAMDESC {
    pub pparamdescex: *mut PARAMDESCEX,
    pub wParamFlags: PARAMFLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: VARIANT,
}
pub type PARAMFLAGS = u16;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROPVARIANT_0 {
    pub Anonymous: PROPVARIANT_0_0,
    pub decVal: DECIMAL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROPVARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub boolVal: VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub filetime: FILETIME,
    pub puuid: *mut GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: BSTR,
    pub bstrblobVal: BSTRBLOB,
    pub blob: BLOB,
    pub pszVal: PSTR,
    pub pwszVal: PWSTR,
    pub punkVal: *mut core::ffi::c_void,
    pub pdispVal: *mut core::ffi::c_void,
    pub pStream: *mut core::ffi::c_void,
    pub pStorage: *mut core::ffi::c_void,
    pub pVersionedStream: *mut VERSIONEDSTREAM,
    pub parray: *mut SAFEARRAY,
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
    pub pcVal: PSTR,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut VARIANT_BOOL,
    pub pdecVal: *mut DECIMAL,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut BSTR,
    pub ppunkVal: *mut *mut core::ffi::c_void,
    pub ppdispVal: *mut *mut core::ffi::c_void,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
pub type PROPVAR_COMPARE_FLAGS = i32;
pub type PROPVAR_COMPARE_UNIT = i32;
pub type PSTR = *mut u8;
pub type PWSTR = *mut u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STATSTG {
    pub pwcsName: PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: FILETIME,
    pub ctime: FILETIME,
    pub atime: FILETIME,
    pub grfMode: STGM,
    pub grfLocksSupported: u32,
    pub clsid: GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
pub type STGM = u32;
pub type STREAM_SEEK = u32;
pub type SYSKIND = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TLIBATTR {
    pub guid: GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TYPEATTR {
    pub guid: GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: VARENUM,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut ARRAYDESC,
    pub hreftype: u32,
}
pub type TYPEKIND = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: VARFLAGS,
    pub varkind: VARKIND,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut VARIANT,
}
pub type VARENUM = u16;
pub type VARFLAGS = u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VARIANT_0 {
    pub Anonymous: VARIANT_0_0,
    pub decVal: DECIMAL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub bstrVal: BSTR,
    pub punkVal: *mut core::ffi::c_void,
    pub pdispVal: *mut core::ffi::c_void,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut VARIANT_BOOL,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut BSTR,
    pub ppunkVal: *mut *mut core::ffi::c_void,
    pub ppdispVal: *mut *mut core::ffi::c_void,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut core::ffi::c_void,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut DECIMAL,
    pub pcVal: PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: VARIANT_0_0_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut core::ffi::c_void,
    pub pRecInfo: *mut core::ffi::c_void,
}
pub type VARIANT_BOOL = i16;
pub type VARKIND = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: GUID,
    pub pStream: *mut core::ffi::c_void,
}
pub const VT_BOOL: VARENUM = 11u16;
pub const VT_BSTR: VARENUM = 8u16;
pub const VT_EMPTY: VARENUM = 0u16;
pub const VT_I1: VARENUM = 16u16;
pub const VT_I2: VARENUM = 2u16;
pub const VT_I4: VARENUM = 3u16;
pub const VT_I8: VARENUM = 20u16;
pub const VT_R4: VARENUM = 4u16;
pub const VT_R8: VARENUM = 5u16;
pub const VT_UI1: VARENUM = 17u16;
pub const VT_UI2: VARENUM = 18u16;
pub const VT_UI4: VARENUM = 19u16;
pub const VT_UI8: VARENUM = 21u16;
pub const VT_UNKNOWN: VARENUM = 13u16;
pub type WAIT_EVENT = u32;
