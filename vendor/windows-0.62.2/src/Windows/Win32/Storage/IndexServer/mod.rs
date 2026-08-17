#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn BindIFilterFromStorage<P0, P1>(pstg: P0, punkouter: P1, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::StructuredStorage::IStorage>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("query.dll" "system" fn BindIFilterFromStorage(pstg : * mut core::ffi::c_void, punkouter : * mut core::ffi::c_void, ppiunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { BindIFilterFromStorage(pstg.param().abi(), punkouter.param().abi(), ppiunk as _).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn BindIFilterFromStream<P0, P1>(pstm: P0, punkouter: P1, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("query.dll" "system" fn BindIFilterFromStream(pstm : * mut core::ffi::c_void, punkouter : * mut core::ffi::c_void, ppiunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { BindIFilterFromStream(pstm.param().abi(), punkouter.param().abi(), ppiunk as _).ok() }
}
#[inline]
pub unsafe fn LoadIFilter<P0, P1>(pwcspath: P0, punkouter: P1, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("query.dll" "system" fn LoadIFilter(pwcspath : windows_core::PCWSTR, punkouter : * mut core::ffi::c_void, ppiunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { LoadIFilter(pwcspath.param().abi(), punkouter.param().abi(), ppiunk as _).ok() }
}
#[inline]
pub unsafe fn LoadIFilterEx<P0>(pwcspath: P0, dwflags: u32, riid: *const windows_core::GUID, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("query.dll" "system" fn LoadIFilterEx(pwcspath : windows_core::PCWSTR, dwflags : u32, riid : *const windows_core::GUID, ppiunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { LoadIFilterEx(pwcspath.param().abi(), dwflags, riid, ppiunk as _).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHUNKSTATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHUNK_BREAKTYPE(pub i32);
pub const CHUNK_EOC: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(4i32);
pub const CHUNK_EOP: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(3i32);
pub const CHUNK_EOS: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(2i32);
pub const CHUNK_EOW: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(1i32);
pub const CHUNK_FILTER_OWNED_VALUE: CHUNKSTATE = CHUNKSTATE(4i32);
pub const CHUNK_NO_BREAK: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(0i32);
pub const CHUNK_TEXT: CHUNKSTATE = CHUNKSTATE(1i32);
pub const CHUNK_VALUE: CHUNKSTATE = CHUNKSTATE(2i32);
pub const CIADMIN: windows_core::PCWSTR = windows_core::w!("::_nodocstore_::");
pub const CICAT_ALL_OPENED: u32 = 32u32;
pub const CICAT_GET_STATE: u32 = 16u32;
pub const CICAT_NO_QUERY: u32 = 8u32;
pub const CICAT_READONLY: u32 = 2u32;
pub const CICAT_STOPPED: u32 = 1u32;
pub const CICAT_WRITABLE: u32 = 4u32;
pub const CINULLCATALOG: windows_core::PCWSTR = windows_core::w!("::_noindex_::");
pub const CI_PROVIDER_ALL: u32 = 4294967295u32;
pub const CI_PROVIDER_INDEXING_SERVICE: u32 = 2u32;
pub const CI_PROVIDER_MSSEARCH: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CI_STATE {
    pub cbStruct: u32,
    pub cWordList: u32,
    pub cPersistentIndex: u32,
    pub cQueries: u32,
    pub cDocuments: u32,
    pub cFreshTest: u32,
    pub dwMergeProgress: u32,
    pub eState: u32,
    pub cFilteredDocuments: u32,
    pub cTotalDocuments: u32,
    pub cPendingScans: u32,
    pub dwIndexSize: u32,
    pub cUniqueKeys: u32,
    pub cSecQDocuments: u32,
    pub dwPropCacheSize: u32,
}
pub const CI_STATE_ANNEALING_MERGE: u32 = 8u32;
pub const CI_STATE_BATTERY_POLICY: u32 = 262144u32;
pub const CI_STATE_BATTERY_POWER: u32 = 2048u32;
pub const CI_STATE_CONTENT_SCAN_REQUIRED: u32 = 4u32;
pub const CI_STATE_DELETION_MERGE: u32 = 32768u32;
pub const CI_STATE_HIGH_CPU: u32 = 131072u32;
pub const CI_STATE_HIGH_IO: u32 = 256u32;
pub const CI_STATE_INDEX_MIGRATION_MERGE: u32 = 64u32;
pub const CI_STATE_LOW_DISK: u32 = 65536u32;
pub const CI_STATE_LOW_MEMORY: u32 = 128u32;
pub const CI_STATE_MASTER_MERGE: u32 = 2u32;
pub const CI_STATE_MASTER_MERGE_PAUSED: u32 = 512u32;
pub const CI_STATE_READING_USNS: u32 = 16384u32;
pub const CI_STATE_READ_ONLY: u32 = 1024u32;
pub const CI_STATE_RECOVERING: u32 = 32u32;
pub const CI_STATE_SCANNING: u32 = 16u32;
pub const CI_STATE_SHADOW_MERGE: u32 = 1u32;
pub const CI_STATE_STARTING: u32 = 8192u32;
pub const CI_STATE_USER_ACTIVE: u32 = 4096u32;
pub const CI_VERSION_WDS30: u32 = 258u32;
pub const CI_VERSION_WDS40: u32 = 265u32;
pub const CI_VERSION_WIN70: u32 = 1792u32;
pub const CLSID_INDEX_SERVER_DSO: windows_core::GUID = windows_core::GUID::from_u128(0xf9ae8980_7e52_11d0_8964_00c04fd611d7);
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DBID {
    pub uGuid: DBID_0,
    pub eKind: u32,
    pub uName: DBID_1,
}
#[cfg(target_arch = "x86")]
impl Default for DBID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union DBID_0 {
    pub guid: windows_core::GUID,
    pub pguid: *mut windows_core::GUID,
}
#[cfg(target_arch = "x86")]
impl Default for DBID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union DBID_1 {
    pub pwszName: windows_core::PWSTR,
    pub ulPropid: u32,
}
#[cfg(target_arch = "x86")]
impl Default for DBID_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DBID {
    pub uGuid: DBID_0,
    pub eKind: u32,
    pub uName: DBID_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union DBID_0 {
    pub guid: windows_core::GUID,
    pub pguid: *mut windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union DBID_1 {
    pub pwszName: windows_core::PWSTR,
    pub ulPropid: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DBID_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DBKINDENUM(pub i32);
pub const DBKIND_GUID: DBKINDENUM = DBKINDENUM(6i32);
pub const DBKIND_GUID_NAME: DBKINDENUM = DBKINDENUM(0i32);
pub const DBKIND_GUID_PROPID: DBKINDENUM = DBKINDENUM(1i32);
pub const DBKIND_NAME: DBKINDENUM = DBKINDENUM(2i32);
pub const DBKIND_PGUID_NAME: DBKINDENUM = DBKINDENUM(3i32);
pub const DBKIND_PGUID_PROPID: DBKINDENUM = DBKINDENUM(4i32);
pub const DBKIND_PROPID: DBKINDENUM = DBKINDENUM(5i32);
pub const DBPROPSET_CIFRMWRKCORE_EXT: windows_core::GUID = windows_core::GUID::from_u128(0xafafaca5_b5d1_11d0_8c62_00c04fc2db8d);
pub const DBPROPSET_FSCIFRMWRK_EXT: windows_core::GUID = windows_core::GUID::from_u128(0xa9bd1526_6a80_11d0_8c9d_0020af1d740e);
pub const DBPROPSET_MSIDXS_ROWSETEXT: windows_core::GUID = windows_core::GUID::from_u128(0xaa6ee6b0_e828_11d0_b23e_00aa0047fc01);
pub const DBPROPSET_QUERYEXT: windows_core::GUID = windows_core::GUID::from_u128(0xa7ac77ed_f8d7_11ce_a798_0020f8008025);
pub const DBPROPSET_SESS_QUERYEXT: windows_core::GUID = windows_core::GUID::from_u128(0x63623309_2d8b_4d17_b152_6e2956c26a70);
pub const DBPROP_APPLICATION_NAME: u32 = 11u32;
pub const DBPROP_CATALOGLISTID: u32 = 9u32;
pub const DBPROP_CI_CATALOG_NAME: u32 = 2u32;
pub const DBPROP_CI_DEPTHS: u32 = 4u32;
pub const DBPROP_CI_EXCLUDE_SCOPES: u32 = 5u32;
pub const DBPROP_CI_INCLUDE_SCOPES: u32 = 3u32;
pub const DBPROP_CI_PROVIDER: u32 = 8u32;
pub const DBPROP_CI_QUERY_TYPE: u32 = 7u32;
pub const DBPROP_CI_SCOPE_FLAGS: u32 = 4u32;
pub const DBPROP_CI_SECURITY_ID: u32 = 6u32;
pub const DBPROP_CLIENT_CLSID: u32 = 3u32;
pub const DBPROP_DEFAULT_EQUALS_BEHAVIOR: u32 = 2u32;
pub const DBPROP_DEFERCATALOGVERIFICATION: u32 = 8u32;
pub const DBPROP_DEFERNONINDEXEDTRIMMING: u32 = 3u32;
pub const DBPROP_DONOTCOMPUTEEXPENSIVEPROPS: u32 = 15u32;
pub const DBPROP_ENABLEROWSETEVENTS: u32 = 16u32;
pub const DBPROP_FIRSTROWS: u32 = 7u32;
pub const DBPROP_FREETEXTANYTERM: u32 = 12u32;
pub const DBPROP_FREETEXTUSESTEMMING: u32 = 13u32;
pub const DBPROP_GENERATEPARSETREE: u32 = 10u32;
pub const DBPROP_GENERICOPTIONS_STRING: u32 = 6u32;
pub const DBPROP_IGNORENOISEONLYCLAUSES: u32 = 5u32;
pub const DBPROP_IGNORESBRI: u32 = 14u32;
pub const DBPROP_MACHINE: u32 = 2u32;
pub const DBPROP_USECONTENTINDEX: u32 = 2u32;
pub const DBPROP_USEEXTENDEDDBTYPES: u32 = 4u32;
pub const DBSETFUNC_ALL: u32 = 1u32;
pub const DBSETFUNC_DISTINCT: u32 = 2u32;
pub const DBSETFUNC_NONE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILTERREGION {
    pub idChunk: u32,
    pub cwcStart: u32,
    pub cwcExtent: u32,
}
pub const FILTER_E_ACCESS: windows_core::HRESULT = windows_core::HRESULT(0x80041703_u32 as _);
pub const FILTER_E_EMBEDDING_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80041707_u32 as _);
pub const FILTER_E_END_OF_CHUNKS: windows_core::HRESULT = windows_core::HRESULT(0x80041700_u32 as _);
pub const FILTER_E_LINK_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80041708_u32 as _);
pub const FILTER_E_NO_MORE_TEXT: windows_core::HRESULT = windows_core::HRESULT(0x80041701_u32 as _);
pub const FILTER_E_NO_MORE_VALUES: windows_core::HRESULT = windows_core::HRESULT(0x80041702_u32 as _);
pub const FILTER_E_NO_TEXT: windows_core::HRESULT = windows_core::HRESULT(0x80041705_u32 as _);
pub const FILTER_E_NO_VALUES: windows_core::HRESULT = windows_core::HRESULT(0x80041706_u32 as _);
pub const FILTER_E_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x8004170B_u32 as _);
pub const FILTER_E_UNKNOWNFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8004170C_u32 as _);
pub const FILTER_S_LAST_TEXT: windows_core::HRESULT = windows_core::HRESULT(0x41709_u32 as _);
pub const FILTER_S_LAST_VALUES: windows_core::HRESULT = windows_core::HRESULT(0x4170A_u32 as _);
pub const FILTER_W_MONIKER_CLIPPED: windows_core::HRESULT = windows_core::HRESULT(0x41704_u32 as _);
#[repr(C)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[derive(Clone, Copy)]
pub struct FULLPROPSPEC {
    pub guidPropSet: windows_core::GUID,
    pub psProperty: super::super::System::Com::StructuredStorage::PROPSPEC,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Default for FULLPROPSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GENERATE_METHOD_EXACT: u32 = 0u32;
pub const GENERATE_METHOD_INFLECT: u32 = 2u32;
pub const GENERATE_METHOD_PREFIX: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IFILTER_FLAGS(pub i32);
pub const IFILTER_FLAGS_OLE_PROPERTIES: IFILTER_FLAGS = IFILTER_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IFILTER_INIT(pub i32);
pub const IFILTER_INIT_APPLY_CRAWL_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(256i32);
pub const IFILTER_INIT_APPLY_INDEX_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(16i32);
pub const IFILTER_INIT_APPLY_OTHER_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(32i32);
pub const IFILTER_INIT_CANON_HYPHENS: IFILTER_INIT = IFILTER_INIT(4i32);
pub const IFILTER_INIT_CANON_PARAGRAPHS: IFILTER_INIT = IFILTER_INIT(1i32);
pub const IFILTER_INIT_CANON_SPACES: IFILTER_INIT = IFILTER_INIT(8i32);
pub const IFILTER_INIT_DISABLE_EMBEDDED: IFILTER_INIT = IFILTER_INIT(2048i32);
pub const IFILTER_INIT_EMIT_FORMATTING: IFILTER_INIT = IFILTER_INIT(4096i32);
pub const IFILTER_INIT_FILTER_AGGRESSIVE_BREAK: IFILTER_INIT = IFILTER_INIT(1024i32);
pub const IFILTER_INIT_FILTER_OWNED_VALUE_OK: IFILTER_INIT = IFILTER_INIT(512i32);
pub const IFILTER_INIT_HARD_LINE_BREAKS: IFILTER_INIT = IFILTER_INIT(2i32);
pub const IFILTER_INIT_INDEXING_ONLY: IFILTER_INIT = IFILTER_INIT(64i32);
pub const IFILTER_INIT_SEARCH_LINKS: IFILTER_INIT = IFILTER_INIT(128i32);
windows_core::imp::define_interface!(IFilter, IFilter_Vtbl, 0x89bcb740_6119_101a_bcb7_00dd010655af);
windows_core::imp::interface_hierarchy!(IFilter, windows_core::IUnknown);
impl IFilter {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Init(&self, grfflags: u32, aattributes: &[FULLPROPSPEC], pflags: *mut u32) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), grfflags, aattributes.len().try_into().unwrap(), core::mem::transmute(aattributes.as_ptr()), pflags as _) }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).GetChunk)(windows_core::Interface::as_raw(self), pstat as _) }
    }
    pub unsafe fn GetText(&self, pcwcbuffer: *mut u32, awcbuffer: windows_core::PWSTR) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), pcwcbuffer as _, core::mem::transmute(awcbuffer)) }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pppropvalue as _) }
    }
    pub unsafe fn BindRegion(&self, origpos: FILTERREGION, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).BindRegion)(windows_core::Interface::as_raw(self), core::mem::transmute(origpos), riid, ppunk as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const FULLPROPSPEC, *mut u32) -> i32,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Init: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetChunk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STAT_CHUNK) -> i32,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetChunk: usize,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PWSTR) -> i32,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetValue: usize,
    pub BindRegion: unsafe extern "system" fn(*mut core::ffi::c_void, FILTERREGION, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> i32,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IFilter_Impl: windows_core::IUnknownImpl {
    fn Init(&self, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32;
    fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> i32;
    fn GetText(&self, pcwcbuffer: *mut u32, awcbuffer: windows_core::PWSTR) -> i32;
    fn GetValue(&self, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32;
    fn BindRegion(&self, origpos: &FILTERREGION, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> i32;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IFilter_Vtbl {
    pub const fn new<Identity: IFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::Init(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&cattributes), core::mem::transmute_copy(&aattributes), core::mem::transmute_copy(&pflags))
            }
        }
        unsafe extern "system" fn GetChunk<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstat: *mut STAT_CHUNK) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::GetChunk(this, core::mem::transmute_copy(&pstat))
            }
        }
        unsafe extern "system" fn GetText<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwcbuffer: *mut u32, awcbuffer: windows_core::PWSTR) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::GetText(this, core::mem::transmute_copy(&pcwcbuffer), core::mem::transmute_copy(&awcbuffer))
            }
        }
        unsafe extern "system" fn GetValue<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::GetValue(this, core::mem::transmute_copy(&pppropvalue))
            }
        }
        unsafe extern "system" fn BindRegion<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origpos: FILTERREGION, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::BindRegion(this, core::mem::transmute(&origpos), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetChunk: GetChunk::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            BindRegion: BindRegion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFilter {}
windows_core::imp::define_interface!(IPhraseSink, IPhraseSink_Vtbl, 0xcc906ff0_c058_101a_b554_08002b33b0e6);
windows_core::imp::interface_hierarchy!(IPhraseSink, windows_core::IUnknown);
impl IPhraseSink {
    pub unsafe fn PutSmallPhrase<P0, P2>(&self, pwcnoun: P0, cwcnoun: u32, pwcmodifier: P2, cwcmodifier: u32, ulattachmenttype: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutSmallPhrase)(windows_core::Interface::as_raw(self), pwcnoun.param().abi(), cwcnoun, pwcmodifier.param().abi(), cwcmodifier, ulattachmenttype).ok() }
    }
    pub unsafe fn PutPhrase<P0>(&self, pwcphrase: P0, cwcphrase: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutPhrase)(windows_core::Interface::as_raw(self), pwcphrase.param().abi(), cwcphrase).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhraseSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutSmallPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub PutPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
pub trait IPhraseSink_Impl: windows_core::IUnknownImpl {
    fn PutSmallPhrase(&self, pwcnoun: &windows_core::PCWSTR, cwcnoun: u32, pwcmodifier: &windows_core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> windows_core::Result<()>;
    fn PutPhrase(&self, pwcphrase: &windows_core::PCWSTR, cwcphrase: u32) -> windows_core::Result<()>;
}
impl IPhraseSink_Vtbl {
    pub const fn new<Identity: IPhraseSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PutSmallPhrase<Identity: IPhraseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcnoun: windows_core::PCWSTR, cwcnoun: u32, pwcmodifier: windows_core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhraseSink_Impl::PutSmallPhrase(this, core::mem::transmute(&pwcnoun), core::mem::transmute_copy(&cwcnoun), core::mem::transmute(&pwcmodifier), core::mem::transmute_copy(&cwcmodifier), core::mem::transmute_copy(&ulattachmenttype)).into()
            }
        }
        unsafe extern "system" fn PutPhrase<Identity: IPhraseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcphrase: windows_core::PCWSTR, cwcphrase: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPhraseSink_Impl::PutPhrase(this, core::mem::transmute(&pwcphrase), core::mem::transmute_copy(&cwcphrase)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutSmallPhrase: PutSmallPhrase::<Identity, OFFSET>,
            PutPhrase: PutPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhraseSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPhraseSink {}
pub const LIFF_FORCE_TEXT_FILTER_FALLBACK: u32 = 3u32;
pub const LIFF_IMPLEMENT_TEXT_FILTER_FALLBACK_POLICY: u32 = 2u32;
pub const LIFF_LOAD_DEFINED_FILTER: u32 = 1u32;
pub const MSIDXSPROP_COMMAND_LOCALE_STRING: u32 = 3u32;
pub const MSIDXSPROP_MAX_RANK: u32 = 6u32;
pub const MSIDXSPROP_PARSE_TREE: u32 = 5u32;
pub const MSIDXSPROP_QUERY_RESTRICTION: u32 = 4u32;
pub const MSIDXSPROP_RESULTS_FOUND: u32 = 7u32;
pub const MSIDXSPROP_ROWSETQUERYSTATUS: u32 = 2u32;
pub const MSIDXSPROP_SAME_SORTORDER_USED: u32 = 14u32;
pub const MSIDXSPROP_SERVER_NLSVERSION: u32 = 12u32;
pub const MSIDXSPROP_SERVER_NLSVER_DEFINED: u32 = 13u32;
pub const MSIDXSPROP_SERVER_VERSION: u32 = 9u32;
pub const MSIDXSPROP_SERVER_WINVER_MAJOR: u32 = 10u32;
pub const MSIDXSPROP_SERVER_WINVER_MINOR: u32 = 11u32;
pub const MSIDXSPROP_WHEREID: u32 = 8u32;
pub const NOT_AN_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80000_u32 as _);
pub const PID_FILENAME: u32 = 100u32;
pub const PROPID_QUERY_ALL: u32 = 6u32;
pub const PROPID_QUERY_HITCOUNT: u32 = 4u32;
pub const PROPID_QUERY_LASTSEENTIME: u32 = 10u32;
pub const PROPID_QUERY_RANK: u32 = 3u32;
pub const PROPID_QUERY_RANKVECTOR: u32 = 2u32;
pub const PROPID_QUERY_UNFILTERED: u32 = 7u32;
pub const PROPID_QUERY_VIRTUALPATH: u32 = 9u32;
pub const PROPID_QUERY_WORKID: u32 = 5u32;
pub const PROPID_STG_CONTENTS: u32 = 19u32;
pub const PROXIMITY_UNIT_CHAPTER: u32 = 3u32;
pub const PROXIMITY_UNIT_PARAGRAPH: u32 = 2u32;
pub const PROXIMITY_UNIT_SENTENCE: u32 = 1u32;
pub const PROXIMITY_UNIT_WORD: u32 = 0u32;
pub const PSGUID_FILENAME: windows_core::GUID = windows_core::GUID::from_u128(0x41cf5ae0_f75a_4806_bd87_59c7d9248eb9);
pub const QUERY_DEEP: u32 = 1u32;
pub const QUERY_PHYSICAL_PATH: u32 = 0u32;
pub const QUERY_SHALLOW: u32 = 0u32;
pub const QUERY_VIRTUAL_PATH: u32 = 2u32;
pub const SCOPE_FLAG_DEEP: u32 = 2u32;
pub const SCOPE_FLAG_INCLUDE: u32 = 1u32;
pub const SCOPE_FLAG_MASK: u32 = 255u32;
pub const SCOPE_TYPE_MASK: u32 = 4294967040u32;
pub const SCOPE_TYPE_VPATH: u32 = 512u32;
pub const SCOPE_TYPE_WINPATH: u32 = 256u32;
pub const STAT_BUSY: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[derive(Clone, Copy)]
pub struct STAT_CHUNK {
    pub idChunk: u32,
    pub breakType: CHUNK_BREAKTYPE,
    pub flags: CHUNKSTATE,
    pub locale: u32,
    pub attribute: FULLPROPSPEC,
    pub idChunkSource: u32,
    pub cwcStartSource: u32,
    pub cwcLenSource: u32,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Default for STAT_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STAT_COALESCE_COMP_ALL_NOISE: u32 = 8192u32;
pub const STAT_CONTENT_OUT_OF_DATE: u32 = 32u32;
pub const STAT_CONTENT_QUERY_INCOMPLETE: u32 = 128u32;
pub const STAT_DONE: u32 = 2u32;
pub const STAT_ERROR: u32 = 1u32;
pub const STAT_MISSING_PROP_IN_RELDOC: u32 = 2048u32;
pub const STAT_MISSING_RELDOC: u32 = 1024u32;
pub const STAT_NOISE_WORDS: u32 = 16u32;
pub const STAT_PARTIAL_SCOPE: u32 = 8u32;
pub const STAT_REFRESH: u32 = 3u32;
pub const STAT_REFRESH_INCOMPLETE: u32 = 64u32;
pub const STAT_RELDOC_ACCESS_DENIED: u32 = 4096u32;
pub const STAT_SHARING_VIOLATION: u32 = 512u32;
pub const STAT_TIME_LIMIT_EXCEEDED: u32 = 256u32;
pub const VECTOR_RANK_DICE: u32 = 3u32;
pub const VECTOR_RANK_INNER: u32 = 2u32;
pub const VECTOR_RANK_JACCARD: u32 = 4u32;
pub const VECTOR_RANK_MAX: u32 = 1u32;
pub const VECTOR_RANK_MIN: u32 = 0u32;
pub const WORDREP_BREAK_EOC: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(3i32);
pub const WORDREP_BREAK_EOP: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(2i32);
pub const WORDREP_BREAK_EOS: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(1i32);
pub const WORDREP_BREAK_EOW: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WORDREP_BREAK_TYPE(pub i32);
