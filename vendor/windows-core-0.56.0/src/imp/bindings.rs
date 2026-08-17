#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]
windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : * mut core::ffi::c_void, iid : *const GUID, factory : *mut *mut core::ffi::c_void) -> HRESULT);
windows_targets::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> BOOL);
windows_targets::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const SECURITY_ATTRIBUTES, bmanualreset : BOOL, binitialstate : BOOL, lpname : PCWSTR) -> HANDLE);
windows_targets::link!("kernel32.dll" "system" fn EncodePointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : PCSTR) -> FARPROC);
windows_targets::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
windows_targets::link!("kernel32.dll" "system" fn HeapAlloc(hheap : HANDLE, dwflags : HEAP_FLAGS, dwbytes : usize) -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> BOOL);
windows_targets::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : PCSTR, hfile : HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE);
windows_targets::link!("kernel32.dll" "system" fn SetEvent(hevent : HANDLE) -> BOOL);
windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : HANDLE, dwmilliseconds : u32) -> WAIT_EVENT);
windows_targets::link!("ole32.dll" "system" fn CoIncrementMTAUsage(pcookie : *mut CO_MTA_USAGE_COOKIE) -> HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
windows_targets::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
windows_targets::link!("ole32.dll" "system" fn PropVariantClear(pvar : *mut PROPVARIANT) -> HRESULT);
windows_targets::link!("ole32.dll" "system" fn PropVariantCopy(pvardest : *mut PROPVARIANT, pvarsrc : *const PROPVARIANT) -> HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : PCWSTR, ui : u32) -> BSTR);
windows_targets::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : BSTR));
windows_targets::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : BSTR) -> u32);
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
pub struct ARRAYDESC {
    pub tdescElem: TYPEDESC,
    pub cDims: u16,
    pub rgbounds: [SAFEARRAYBOUND; 1],
}
impl Copy for ARRAYDESC {}
impl Clone for ARRAYDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: *mut core::ffi::c_void,
}
impl Copy for BINDPTR {}
impl Clone for BINDPTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl Copy for BLOB {}
impl Clone for BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BOOL = i32;
pub type BSTR = *const u16;
#[repr(C)]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
impl Copy for BSTRBLOB {}
impl Clone for BSTRBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut VARIANT_BOOL,
}
impl Copy for CABOOL {}
impl Clone for CABOOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut BSTR,
}
impl Copy for CABSTR {}
impl Clone for CABSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
impl Copy for CABSTRBLOB {}
impl Clone for CABSTRBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAC {
    pub cElems: u32,
    pub pElems: PSTR,
}
impl Copy for CAC {}
impl Clone for CAC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
impl Copy for CACLIPDATA {}
impl Clone for CACLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut GUID,
}
impl Copy for CACLSID {}
impl Clone for CACLSID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut CY,
}
impl Copy for CACY {}
impl Clone for CACY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl Copy for CADATE {}
impl Clone for CADATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl Copy for CADBL {}
impl Clone for CADBL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut FILETIME,
}
impl Copy for CAFILETIME {}
impl Clone for CAFILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
impl Copy for CAFLT {}
impl Clone for CAFLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
impl Copy for CAH {}
impl Clone for CAH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl Copy for CAI {}
impl Clone for CAI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl Copy for CAL {}
impl Clone for CAL {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CALLCONV = i32;
#[repr(C)]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut PSTR,
}
impl Copy for CALPSTR {}
impl Clone for CALPSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut PWSTR,
}
impl Copy for CALPWSTR {}
impl Clone for CALPWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
impl Copy for CAPROPVARIANT {}
impl Clone for CAPROPVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl Copy for CASCODE {}
impl Clone for CASCODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
impl Copy for CAUB {}
impl Clone for CAUB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
impl Copy for CAUH {}
impl Clone for CAUH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
impl Copy for CAUI {}
impl Clone for CAUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl Copy for CAUL {}
impl Clone for CAUL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
impl Copy for CLIPDATA {}
impl Clone for CLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CO_MTA_USAGE_COOKIE = isize;
#[repr(C)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Copy for CY {}
impl Clone for CY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl Copy for CY_0 {}
impl Clone for CY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous1: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
impl Copy for DECIMAL {}
impl Clone for DECIMAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
impl Copy for DECIMAL_0 {}
impl Clone for DECIMAL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
impl Copy for DECIMAL_0_0 {}
impl Clone for DECIMAL_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
impl Copy for DECIMAL_1 {}
impl Clone for DECIMAL_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
impl Copy for DECIMAL_1_0 {}
impl Clone for DECIMAL_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DESCKIND = i32;
pub type DISPATCH_FLAGS = u16;
#[repr(C)]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
impl Copy for DISPPARAMS {}
impl Clone for DISPPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
impl Copy for ELEMDESC {}
impl Clone for ELEMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: PARAMDESC,
}
impl Copy for ELEMDESC_0 {}
impl Clone for ELEMDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl Copy for EXCEPINFO {}
impl Clone for EXCEPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(C)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
impl Copy for FILETIME {}
impl Clone for FILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl Copy for FUNCDESC {}
impl Clone for FUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FUNCFLAGS = u16;
pub type FUNCKIND = i32;
#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl Copy for GUID {}
impl Clone for GUID {
    fn clone(&self) -> Self {
        *self
    }
}
impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self { data1: (uuid >> 96) as u32, data2: (uuid >> 80 & 0xffff) as u16, data3: (uuid >> 64 & 0xffff) as u16, data4: (uuid as u64).to_be_bytes() }
    }
}
pub type HANDLE = isize;
pub type HEAP_FLAGS = u32;
pub type HMODULE = isize;
pub type HRESULT = i32;
#[repr(C)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: IDLFLAGS,
}
impl Copy for IDLDESC {}
impl Clone for IDLDESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IDLFLAGS = u16;
pub type IMPLTYPEFLAGS = i32;
pub type INVOKEKIND = i32;
pub type LOAD_LIBRARY_FLAGS = u32;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
pub type LPEXCEPFINO_DEFERRED_FILLIN = Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> HRESULT>;
#[repr(C)]
pub struct PARAMDESC {
    pub pparamdescex: *mut PARAMDESCEX,
    pub wParamFlags: PARAMFLAGS,
}
impl Copy for PARAMDESC {}
impl Clone for PARAMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: VARIANT,
}
impl Copy for PARAMDESCEX {}
impl Clone for PARAMDESCEX {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PARAMFLAGS = u16;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
#[repr(C)]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
impl Copy for PROPVARIANT {}
impl Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union PROPVARIANT_0 {
    pub Anonymous: PROPVARIANT_0_0,
    pub decVal: DECIMAL,
}
impl Copy for PROPVARIANT_0 {}
impl Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROPVARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
impl Copy for PROPVARIANT_0_0 {}
impl Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl Copy for PROPVARIANT_0_0_0 {}
impl Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROPVAR_COMPARE_FLAGS = i32;
pub type PROPVAR_COMPARE_UNIT = i32;
pub type PSTR = *mut u8;
pub type PWSTR = *mut u16;
#[repr(C)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Copy for SAFEARRAY {}
impl Clone for SAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl Copy for SAFEARRAYBOUND {}
impl Clone for SAFEARRAYBOUND {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: BOOL,
}
impl Copy for SECURITY_ATTRIBUTES {}
impl Clone for SECURITY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl Copy for STATSTG {}
impl Clone for STATSTG {
    fn clone(&self) -> Self {
        *self
    }
}
pub type STGM = u32;
pub type STREAM_SEEK = u32;
pub type SYSKIND = i32;
#[repr(C)]
pub struct TLIBATTR {
    pub guid: GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl Copy for TLIBATTR {}
impl Clone for TLIBATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl Copy for TYPEATTR {}
impl Clone for TYPEATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: VARENUM,
}
impl Copy for TYPEDESC {}
impl Clone for TYPEDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut ARRAYDESC,
    pub hreftype: u32,
}
impl Copy for TYPEDESC_0 {}
impl Clone for TYPEDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TYPEKIND = i32;
#[repr(C)]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: VARFLAGS,
    pub varkind: VARKIND,
}
impl Copy for VARDESC {}
impl Clone for VARDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut VARIANT,
}
impl Copy for VARDESC_0 {}
impl Clone for VARDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VARENUM = u16;
pub type VARFLAGS = u16;
#[repr(C)]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
impl Copy for VARIANT {}
impl Clone for VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VARIANT_0 {
    pub Anonymous: VARIANT_0_0,
    pub decVal: DECIMAL,
}
impl Copy for VARIANT_0 {}
impl Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
impl Copy for VARIANT_0_0 {}
impl Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl Copy for VARIANT_0_0_0 {}
impl Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut core::ffi::c_void,
    pub pRecInfo: *mut core::ffi::c_void,
}
impl Copy for VARIANT_0_0_0_0 {}
impl Clone for VARIANT_0_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VARIANT_BOOL = i16;
pub type VARKIND = i32;
#[repr(C)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: GUID,
    pub pStream: *mut core::ffi::c_void,
}
impl Copy for VERSIONEDSTREAM {}
impl Clone for VERSIONEDSTREAM {
    fn clone(&self) -> Self {
        *self
    }
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
