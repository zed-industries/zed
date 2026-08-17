#[inline]
pub unsafe fn FCIAddFile<P1, P2>(hfci: *const core::ffi::c_void, pszsourcefile: P1, pszfilename: P2, fexecute: bool, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS, pfnfcigoi: PFNFCIGETOPENINFO, typecompress: u16) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cabinet.dll" "C" fn FCIAddFile(hfci : *const core::ffi::c_void, pszsourcefile : windows_core::PCSTR, pszfilename : windows_core::PCSTR, fexecute : windows_core::BOOL, pfnfcignc : PFNFCIGETNEXTCABINET, pfnfcis : PFNFCISTATUS, pfnfcigoi : PFNFCIGETOPENINFO, typecompress : u16) -> windows_core::BOOL);
    unsafe { FCIAddFile(hfci, pszsourcefile.param().abi(), pszfilename.param().abi(), fexecute.into(), pfnfcignc, pfnfcis, pfnfcigoi, typecompress) }
}
#[inline]
pub unsafe fn FCICreate(perf: *const ERF, pfnfcifp: PFNFCIFILEPLACED, pfna: PFNFCIALLOC, pfnf: PFNFCIFREE, pfnopen: PFNFCIOPEN, pfnread: PFNFCIREAD, pfnwrite: PFNFCIWRITE, pfnclose: PFNFCICLOSE, pfnseek: PFNFCISEEK, pfndelete: PFNFCIDELETE, pfnfcigtf: PFNFCIGETTEMPFILE, pccab: *const CCAB, pv: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_core::link!("cabinet.dll" "C" fn FCICreate(perf : *const ERF, pfnfcifp : PFNFCIFILEPLACED, pfna : PFNFCIALLOC, pfnf : PFNFCIFREE, pfnopen : PFNFCIOPEN, pfnread : PFNFCIREAD, pfnwrite : PFNFCIWRITE, pfnclose : PFNFCICLOSE, pfnseek : PFNFCISEEK, pfndelete : PFNFCIDELETE, pfnfcigtf : PFNFCIGETTEMPFILE, pccab : *const CCAB, pv : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { FCICreate(perf, pfnfcifp, pfna, pfnf, pfnopen, pfnread, pfnwrite, pfnclose, pfnseek, pfndelete, pfnfcigtf, pccab, pv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FCIDestroy(hfci: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "C" fn FCIDestroy(hfci : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FCIDestroy(hfci) }
}
#[inline]
pub unsafe fn FCIFlushCabinet(hfci: *const core::ffi::c_void, fgetnextcab: bool, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "C" fn FCIFlushCabinet(hfci : *const core::ffi::c_void, fgetnextcab : windows_core::BOOL, pfnfcignc : PFNFCIGETNEXTCABINET, pfnfcis : PFNFCISTATUS) -> windows_core::BOOL);
    unsafe { FCIFlushCabinet(hfci, fgetnextcab.into(), pfnfcignc, pfnfcis) }
}
#[inline]
pub unsafe fn FCIFlushFolder(hfci: *const core::ffi::c_void, pfnfcignc: PFNFCIGETNEXTCABINET, pfnfcis: PFNFCISTATUS) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "C" fn FCIFlushFolder(hfci : *const core::ffi::c_void, pfnfcignc : PFNFCIGETNEXTCABINET, pfnfcis : PFNFCISTATUS) -> windows_core::BOOL);
    unsafe { FCIFlushFolder(hfci, pfnfcignc, pfnfcis) }
}
#[inline]
pub unsafe fn FDICopy<P1, P2>(hfdi: *const core::ffi::c_void, pszcabinet: P1, pszcabpath: P2, flags: i32, pfnfdin: PFNFDINOTIFY, pfnfdid: PFNFDIDECRYPT, pvuser: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cabinet.dll" "C" fn FDICopy(hfdi : *const core::ffi::c_void, pszcabinet : windows_core::PCSTR, pszcabpath : windows_core::PCSTR, flags : i32, pfnfdin : PFNFDINOTIFY, pfnfdid : PFNFDIDECRYPT, pvuser : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FDICopy(hfdi, pszcabinet.param().abi(), pszcabpath.param().abi(), flags, pfnfdin, pfnfdid, pvuser.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FDICreate(pfnalloc: PFNALLOC, pfnfree: PFNFREE, pfnopen: PFNOPEN, pfnread: PFNREAD, pfnwrite: PFNWRITE, pfnclose: PFNCLOSE, pfnseek: PFNSEEK, cputype: FDICREATE_CPU_TYPE, perf: *mut ERF) -> *mut core::ffi::c_void {
    windows_core::link!("cabinet.dll" "C" fn FDICreate(pfnalloc : PFNALLOC, pfnfree : PFNFREE, pfnopen : PFNOPEN, pfnread : PFNREAD, pfnwrite : PFNWRITE, pfnclose : PFNCLOSE, pfnseek : PFNSEEK, cputype : FDICREATE_CPU_TYPE, perf : *mut ERF) -> *mut core::ffi::c_void);
    unsafe { FDICreate(pfnalloc, pfnfree, pfnopen, pfnread, pfnwrite, pfnclose, pfnseek, cputype, perf as _) }
}
#[inline]
pub unsafe fn FDIDestroy(hfdi: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "C" fn FDIDestroy(hfdi : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FDIDestroy(hfdi) }
}
#[inline]
pub unsafe fn FDIIsCabinet(hfdi: *const core::ffi::c_void, hf: isize, pfdici: Option<*mut FDICABINETINFO>) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "C" fn FDIIsCabinet(hfdi : *const core::ffi::c_void, hf : isize, pfdici : *mut FDICABINETINFO) -> windows_core::BOOL);
    unsafe { FDIIsCabinet(hfdi, hf, pfdici.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FDITruncateCabinet<P1>(hfdi: *const core::ffi::c_void, pszcabinetname: P1, ifoldertodelete: u16) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cabinet.dll" "C" fn FDITruncateCabinet(hfdi : *const core::ffi::c_void, pszcabinetname : windows_core::PCSTR, ifoldertodelete : u16) -> windows_core::BOOL);
    unsafe { FDITruncateCabinet(hfdi, pszcabinetname.param().abi(), ifoldertodelete) }
}
pub const CB_MAX_CABINET_NAME: u32 = 256u32;
pub const CB_MAX_CAB_PATH: u32 = 256u32;
pub const CB_MAX_DISK: i32 = 2147483647i32;
pub const CB_MAX_DISK_NAME: u32 = 256u32;
pub const CB_MAX_FILENAME: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CCAB {
    pub cb: u32,
    pub cbFolderThresh: u32,
    pub cbReserveCFHeader: u32,
    pub cbReserveCFFolder: u32,
    pub cbReserveCFData: u32,
    pub iCab: i32,
    pub iDisk: i32,
    pub fFailOnIncompressible: i32,
    pub setID: u16,
    pub szDisk: [i8; 256],
    pub szCab: [i8; 256],
    pub szCabPath: [i8; 256],
}
impl Default for CCAB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ERF {
    pub erfOper: i32,
    pub erfType: i32,
    pub fError: windows_core::BOOL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FCIERROR(pub i32);
pub const FCIERR_ALLOC_FAIL: FCIERROR = FCIERROR(3i32);
pub const FCIERR_BAD_COMPR_TYPE: FCIERROR = FCIERROR(5i32);
pub const FCIERR_CAB_FILE: FCIERROR = FCIERROR(6i32);
pub const FCIERR_CAB_FORMAT_LIMIT: FCIERROR = FCIERROR(9i32);
pub const FCIERR_MCI_FAIL: FCIERROR = FCIERROR(8i32);
pub const FCIERR_NONE: FCIERROR = FCIERROR(0i32);
pub const FCIERR_OPEN_SRC: FCIERROR = FCIERROR(1i32);
pub const FCIERR_READ_SRC: FCIERROR = FCIERROR(2i32);
pub const FCIERR_TEMP_FILE: FCIERROR = FCIERROR(4i32);
pub const FCIERR_USER_ABORT: FCIERROR = FCIERROR(7i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FDICABINETINFO {
    pub cbCabinet: i32,
    pub cFolders: u16,
    pub cFiles: u16,
    pub setID: u16,
    pub iCabinet: u16,
    pub fReserve: windows_core::BOOL,
    pub hasprev: windows_core::BOOL,
    pub hasnext: windows_core::BOOL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FDICREATE_CPU_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FDIDECRYPT {
    pub fdidt: FDIDECRYPTTYPE,
    pub pvUser: *mut core::ffi::c_void,
    pub Anonymous: FDIDECRYPT_0,
}
impl Default for FDIDECRYPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FDIDECRYPT_0 {
    pub cabinet: FDIDECRYPT_0_0,
    pub folder: FDIDECRYPT_0_1,
    pub decrypt: FDIDECRYPT_0_2,
}
impl Default for FDIDECRYPT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDIDECRYPT_0_0 {
    pub pHeaderReserve: *mut core::ffi::c_void,
    pub cbHeaderReserve: u16,
    pub setID: u16,
    pub iCabinet: i32,
}
impl Default for FDIDECRYPT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDIDECRYPT_0_2 {
    pub pDataReserve: *mut core::ffi::c_void,
    pub cbDataReserve: u16,
    pub pbData: *mut core::ffi::c_void,
    pub cbData: u16,
    pub fSplit: windows_core::BOOL,
    pub cbPartial: u16,
}
impl Default for FDIDECRYPT_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDIDECRYPT_0_1 {
    pub pFolderReserve: *mut core::ffi::c_void,
    pub cbFolderReserve: u16,
    pub iFolder: u16,
}
impl Default for FDIDECRYPT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FDIDECRYPTTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FDIERROR(pub i32);
pub const FDIERROR_ALLOC_FAIL: FDIERROR = FDIERROR(5i32);
pub const FDIERROR_BAD_COMPR_TYPE: FDIERROR = FDIERROR(6i32);
pub const FDIERROR_CABINET_NOT_FOUND: FDIERROR = FDIERROR(1i32);
pub const FDIERROR_CORRUPT_CABINET: FDIERROR = FDIERROR(4i32);
pub const FDIERROR_EOF: FDIERROR = FDIERROR(12i32);
pub const FDIERROR_MDI_FAIL: FDIERROR = FDIERROR(7i32);
pub const FDIERROR_NONE: FDIERROR = FDIERROR(0i32);
pub const FDIERROR_NOT_A_CABINET: FDIERROR = FDIERROR(2i32);
pub const FDIERROR_RESERVE_MISMATCH: FDIERROR = FDIERROR(9i32);
pub const FDIERROR_TARGET_FILE: FDIERROR = FDIERROR(8i32);
pub const FDIERROR_UNKNOWN_CABINET_VERSION: FDIERROR = FDIERROR(3i32);
pub const FDIERROR_USER_ABORT: FDIERROR = FDIERROR(11i32);
pub const FDIERROR_WRONG_CABINET: FDIERROR = FDIERROR(10i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDINOTIFICATION {
    pub cb: i32,
    pub psz1: windows_core::PSTR,
    pub psz2: windows_core::PSTR,
    pub psz3: windows_core::PSTR,
    pub pv: *mut core::ffi::c_void,
    pub hf: isize,
    pub date: u16,
    pub time: u16,
    pub attribs: u16,
    pub setID: u16,
    pub iCabinet: u16,
    pub iFolder: u16,
    pub fdie: FDIERROR,
}
impl Default for FDINOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FDINOTIFICATIONTYPE(pub i32);
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FDISPILLFILE {
    pub ach: [i8; 2],
    pub cbFile: i32,
}
#[cfg(target_arch = "x86")]
impl Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FDISPILLFILE {
    pub ach: [i8; 2],
    pub cbFile: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INCLUDED_FCI: u32 = 1u32;
pub const INCLUDED_FDI: u32 = 1u32;
pub const INCLUDED_TYPES_FCI_FDI: u32 = 1u32;
pub type PFNALLOC = Option<unsafe extern "system" fn(cb: u32) -> *mut core::ffi::c_void>;
pub type PFNCLOSE = Option<unsafe extern "system" fn(hf: isize) -> i32>;
pub type PFNFCIALLOC = Option<unsafe extern "system" fn(cb: u32) -> *mut core::ffi::c_void>;
pub type PFNFCICLOSE = Option<unsafe extern "system" fn(hf: isize, err: *mut i32, pv: *mut core::ffi::c_void) -> i32>;
pub type PFNFCIDELETE = Option<unsafe extern "system" fn(pszfile: windows_core::PCSTR, err: *mut i32, pv: *mut core::ffi::c_void) -> i32>;
pub type PFNFCIFILEPLACED = Option<unsafe extern "system" fn(pccab: *mut CCAB, pszfile: windows_core::PCSTR, cbfile: i32, fcontinuation: windows_core::BOOL, pv: *mut core::ffi::c_void) -> i32>;
pub type PFNFCIFREE = Option<unsafe extern "system" fn(memory: *mut core::ffi::c_void)>;
pub type PFNFCIGETNEXTCABINET = Option<unsafe extern "system" fn(pccab: *mut CCAB, cbprevcab: u32, pv: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFNFCIGETOPENINFO = Option<unsafe extern "system" fn(pszname: windows_core::PCSTR, pdate: *mut u16, ptime: *mut u16, pattribs: *mut u16, err: *mut i32, pv: *mut core::ffi::c_void) -> isize>;
pub type PFNFCIGETTEMPFILE = Option<unsafe extern "system" fn(psztempname: windows_core::PSTR, cbtempname: i32, pv: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFNFCIOPEN = Option<unsafe extern "system" fn(pszfile: windows_core::PCSTR, oflag: i32, pmode: i32, err: *mut i32, pv: *mut core::ffi::c_void) -> isize>;
pub type PFNFCIREAD = Option<unsafe extern "system" fn(hf: isize, memory: *mut core::ffi::c_void, cb: u32, err: *mut i32, pv: *mut core::ffi::c_void) -> u32>;
pub type PFNFCISEEK = Option<unsafe extern "system" fn(hf: isize, dist: i32, seektype: i32, err: *mut i32, pv: *mut core::ffi::c_void) -> i32>;
pub type PFNFCISTATUS = Option<unsafe extern "system" fn(typestatus: u32, cb1: u32, cb2: u32, pv: *mut core::ffi::c_void) -> i32>;
pub type PFNFCIWRITE = Option<unsafe extern "system" fn(hf: isize, memory: *mut core::ffi::c_void, cb: u32, err: *mut i32, pv: *mut core::ffi::c_void) -> u32>;
pub type PFNFDIDECRYPT = Option<unsafe extern "system" fn(pfdid: *mut FDIDECRYPT) -> i32>;
pub type PFNFDINOTIFY = Option<unsafe extern "system" fn(fdint: FDINOTIFICATIONTYPE, pfdin: *mut FDINOTIFICATION) -> isize>;
pub type PFNFREE = Option<unsafe extern "system" fn(pv: *const core::ffi::c_void)>;
pub type PFNOPEN = Option<unsafe extern "system" fn(pszfile: windows_core::PCSTR, oflag: i32, pmode: i32) -> isize>;
pub type PFNREAD = Option<unsafe extern "system" fn(hf: isize, pv: *mut core::ffi::c_void, cb: u32) -> u32>;
pub type PFNSEEK = Option<unsafe extern "system" fn(hf: isize, dist: i32, seektype: i32) -> i32>;
pub type PFNWRITE = Option<unsafe extern "system" fn(hf: isize, pv: *const core::ffi::c_void, cb: u32) -> u32>;
pub const _A_EXEC: u32 = 64u32;
pub const _A_NAME_IS_UTF: u32 = 128u32;
pub const cpu80286: FDICREATE_CPU_TYPE = FDICREATE_CPU_TYPE(0i32);
pub const cpu80386: FDICREATE_CPU_TYPE = FDICREATE_CPU_TYPE(1i32);
pub const cpuUNKNOWN: FDICREATE_CPU_TYPE = FDICREATE_CPU_TYPE(-1i32);
pub const fdidtDECRYPT: FDIDECRYPTTYPE = FDIDECRYPTTYPE(2i32);
pub const fdidtNEW_CABINET: FDIDECRYPTTYPE = FDIDECRYPTTYPE(0i32);
pub const fdidtNEW_FOLDER: FDIDECRYPTTYPE = FDIDECRYPTTYPE(1i32);
pub const fdintCABINET_INFO: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(0i32);
pub const fdintCLOSE_FILE_INFO: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(3i32);
pub const fdintCOPY_FILE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(2i32);
pub const fdintENUMERATE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(5i32);
pub const fdintNEXT_CABINET: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(4i32);
pub const fdintPARTIAL_FILE: FDINOTIFICATIONTYPE = FDINOTIFICATIONTYPE(1i32);
pub const statusCabinet: u32 = 2u32;
pub const statusFile: u32 = 0u32;
pub const statusFolder: u32 = 1u32;
pub const tcompBAD: u32 = 15u32;
pub const tcompLZX_WINDOW_HI: u32 = 5376u32;
pub const tcompLZX_WINDOW_LO: u32 = 3840u32;
pub const tcompMASK_LZX_WINDOW: u32 = 7936u32;
pub const tcompMASK_QUANTUM_LEVEL: u32 = 240u32;
pub const tcompMASK_QUANTUM_MEM: u32 = 7936u32;
pub const tcompMASK_RESERVED: u32 = 57344u32;
pub const tcompMASK_TYPE: u32 = 15u32;
pub const tcompQUANTUM_LEVEL_HI: u32 = 112u32;
pub const tcompQUANTUM_LEVEL_LO: u32 = 16u32;
pub const tcompQUANTUM_MEM_HI: u32 = 5376u32;
pub const tcompQUANTUM_MEM_LO: u32 = 2560u32;
pub const tcompSHIFT_LZX_WINDOW: u32 = 8u32;
pub const tcompSHIFT_QUANTUM_LEVEL: u32 = 4u32;
pub const tcompSHIFT_QUANTUM_MEM: u32 = 8u32;
pub const tcompTYPE_LZX: u32 = 3u32;
pub const tcompTYPE_MSZIP: u32 = 1u32;
pub const tcompTYPE_NONE: u32 = 0u32;
pub const tcompTYPE_QUANTUM: u32 = 2u32;
