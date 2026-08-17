#[inline]
pub unsafe fn CreateToolhelp32Snapshot(dwflags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, th32processid: u32) -> windows_core::Result<super::super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn CreateToolhelp32Snapshot(dwflags : CREATE_TOOLHELP_SNAPSHOT_FLAGS, th32processid : u32) -> super::super::super::Foundation:: HANDLE);
    let result__ = CreateToolhelp32Snapshot(dwflags, th32processid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn Heap32First(lphe: *mut HEAPENTRY32, th32processid: u32, th32heapid: usize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn Heap32First(lphe : *mut HEAPENTRY32, th32processid : u32, th32heapid : usize) -> super::super::super::Foundation:: BOOL);
    Heap32First(lphe, th32processid, th32heapid).ok()
}
#[inline]
pub unsafe fn Heap32ListFirst<P0>(hsnapshot: P0, lphl: *mut HEAPLIST32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Heap32ListFirst(hsnapshot : super::super::super::Foundation:: HANDLE, lphl : *mut HEAPLIST32) -> super::super::super::Foundation:: BOOL);
    Heap32ListFirst(hsnapshot.param().abi(), lphl).ok()
}
#[inline]
pub unsafe fn Heap32ListNext<P0>(hsnapshot: P0, lphl: *mut HEAPLIST32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Heap32ListNext(hsnapshot : super::super::super::Foundation:: HANDLE, lphl : *mut HEAPLIST32) -> super::super::super::Foundation:: BOOL);
    Heap32ListNext(hsnapshot.param().abi(), lphl).ok()
}
#[inline]
pub unsafe fn Heap32Next(lphe: *mut HEAPENTRY32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn Heap32Next(lphe : *mut HEAPENTRY32) -> super::super::super::Foundation:: BOOL);
    Heap32Next(lphe).ok()
}
#[inline]
pub unsafe fn Module32First<P0>(hsnapshot: P0, lpme: *mut MODULEENTRY32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Module32First(hsnapshot : super::super::super::Foundation:: HANDLE, lpme : *mut MODULEENTRY32) -> super::super::super::Foundation:: BOOL);
    Module32First(hsnapshot.param().abi(), lpme).ok()
}
#[inline]
pub unsafe fn Module32FirstW<P0>(hsnapshot: P0, lpme: *mut MODULEENTRY32W) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Module32FirstW(hsnapshot : super::super::super::Foundation:: HANDLE, lpme : *mut MODULEENTRY32W) -> super::super::super::Foundation:: BOOL);
    Module32FirstW(hsnapshot.param().abi(), lpme).ok()
}
#[inline]
pub unsafe fn Module32Next<P0>(hsnapshot: P0, lpme: *mut MODULEENTRY32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Module32Next(hsnapshot : super::super::super::Foundation:: HANDLE, lpme : *mut MODULEENTRY32) -> super::super::super::Foundation:: BOOL);
    Module32Next(hsnapshot.param().abi(), lpme).ok()
}
#[inline]
pub unsafe fn Module32NextW<P0>(hsnapshot: P0, lpme: *mut MODULEENTRY32W) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Module32NextW(hsnapshot : super::super::super::Foundation:: HANDLE, lpme : *mut MODULEENTRY32W) -> super::super::super::Foundation:: BOOL);
    Module32NextW(hsnapshot.param().abi(), lpme).ok()
}
#[inline]
pub unsafe fn Process32First<P0>(hsnapshot: P0, lppe: *mut PROCESSENTRY32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Process32First(hsnapshot : super::super::super::Foundation:: HANDLE, lppe : *mut PROCESSENTRY32) -> super::super::super::Foundation:: BOOL);
    Process32First(hsnapshot.param().abi(), lppe).ok()
}
#[inline]
pub unsafe fn Process32FirstW<P0>(hsnapshot: P0, lppe: *mut PROCESSENTRY32W) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Process32FirstW(hsnapshot : super::super::super::Foundation:: HANDLE, lppe : *mut PROCESSENTRY32W) -> super::super::super::Foundation:: BOOL);
    Process32FirstW(hsnapshot.param().abi(), lppe).ok()
}
#[inline]
pub unsafe fn Process32Next<P0>(hsnapshot: P0, lppe: *mut PROCESSENTRY32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Process32Next(hsnapshot : super::super::super::Foundation:: HANDLE, lppe : *mut PROCESSENTRY32) -> super::super::super::Foundation:: BOOL);
    Process32Next(hsnapshot.param().abi(), lppe).ok()
}
#[inline]
pub unsafe fn Process32NextW<P0>(hsnapshot: P0, lppe: *mut PROCESSENTRY32W) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Process32NextW(hsnapshot : super::super::super::Foundation:: HANDLE, lppe : *mut PROCESSENTRY32W) -> super::super::super::Foundation:: BOOL);
    Process32NextW(hsnapshot.param().abi(), lppe).ok()
}
#[inline]
pub unsafe fn Thread32First<P0>(hsnapshot: P0, lpte: *mut THREADENTRY32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Thread32First(hsnapshot : super::super::super::Foundation:: HANDLE, lpte : *mut THREADENTRY32) -> super::super::super::Foundation:: BOOL);
    Thread32First(hsnapshot.param().abi(), lpte).ok()
}
#[inline]
pub unsafe fn Thread32Next<P0>(hsnapshot: P0, lpte: *mut THREADENTRY32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Thread32Next(hsnapshot : super::super::super::Foundation:: HANDLE, lpte : *mut THREADENTRY32) -> super::super::super::Foundation:: BOOL);
    Thread32Next(hsnapshot.param().abi(), lpte).ok()
}
#[inline]
pub unsafe fn Toolhelp32ReadProcessMemory(th32processid: u32, lpbaseaddress: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, cbread: usize, lpnumberofbytesread: *mut usize) -> super::super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn Toolhelp32ReadProcessMemory(th32processid : u32, lpbaseaddress : *const core::ffi::c_void, lpbuffer : *mut core::ffi::c_void, cbread : usize, lpnumberofbytesread : *mut usize) -> super::super::super::Foundation:: BOOL);
    Toolhelp32ReadProcessMemory(th32processid, lpbaseaddress, lpbuffer, cbread, lpnumberofbytesread)
}
pub const HF32_DEFAULT: u32 = 1u32;
pub const HF32_SHARED: u32 = 2u32;
pub const LF32_FIXED: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(1u32);
pub const LF32_FREE: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(2u32);
pub const LF32_MOVEABLE: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(4u32);
pub const MAX_MODULE_NAME32: u32 = 255u32;
pub const TH32CS_INHERIT: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2147483648u32);
pub const TH32CS_SNAPALL: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(15u32);
pub const TH32CS_SNAPHEAPLIST: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(1u32);
pub const TH32CS_SNAPMODULE: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(8u32);
pub const TH32CS_SNAPMODULE32: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(16u32);
pub const TH32CS_SNAPPROCESS: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2u32);
pub const TH32CS_SNAPTHREAD: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(4u32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CREATE_TOOLHELP_SNAPSHOT_FLAGS(pub u32);
impl windows_core::TypeKind for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CREATE_TOOLHELP_SNAPSHOT_FLAGS").field(&self.0).finish()
    }
}
impl CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HEAPENTRY32_FLAGS(pub u32);
impl windows_core::TypeKind for HEAPENTRY32_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HEAPENTRY32_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HEAPENTRY32_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct HEAPENTRY32 {
    pub dwSize: usize,
    pub hHandle: super::super::super::Foundation::HANDLE,
    pub dwAddress: usize,
    pub dwBlockSize: usize,
    pub dwFlags: HEAPENTRY32_FLAGS,
    pub dwLockCount: u32,
    pub dwResvd: u32,
    pub th32ProcessID: u32,
    pub th32HeapID: usize,
}
impl Copy for HEAPENTRY32 {}
impl Clone for HEAPENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for HEAPENTRY32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HEAPENTRY32").field("dwSize", &self.dwSize).field("hHandle", &self.hHandle).field("dwAddress", &self.dwAddress).field("dwBlockSize", &self.dwBlockSize).field("dwFlags", &self.dwFlags).field("dwLockCount", &self.dwLockCount).field("dwResvd", &self.dwResvd).field("th32ProcessID", &self.th32ProcessID).field("th32HeapID", &self.th32HeapID).finish()
    }
}
impl windows_core::TypeKind for HEAPENTRY32 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for HEAPENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hHandle == other.hHandle && self.dwAddress == other.dwAddress && self.dwBlockSize == other.dwBlockSize && self.dwFlags == other.dwFlags && self.dwLockCount == other.dwLockCount && self.dwResvd == other.dwResvd && self.th32ProcessID == other.th32ProcessID && self.th32HeapID == other.th32HeapID
    }
}
impl Eq for HEAPENTRY32 {}
impl Default for HEAPENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HEAPLIST32 {
    pub dwSize: usize,
    pub th32ProcessID: u32,
    pub th32HeapID: usize,
    pub dwFlags: u32,
}
impl Copy for HEAPLIST32 {}
impl Clone for HEAPLIST32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for HEAPLIST32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HEAPLIST32").field("dwSize", &self.dwSize).field("th32ProcessID", &self.th32ProcessID).field("th32HeapID", &self.th32HeapID).field("dwFlags", &self.dwFlags).finish()
    }
}
impl windows_core::TypeKind for HEAPLIST32 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for HEAPLIST32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.th32ProcessID == other.th32ProcessID && self.th32HeapID == other.th32HeapID && self.dwFlags == other.dwFlags
    }
}
impl Eq for HEAPLIST32 {}
impl Default for HEAPLIST32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MODULEENTRY32 {
    pub dwSize: u32,
    pub th32ModuleID: u32,
    pub th32ProcessID: u32,
    pub GlblcntUsage: u32,
    pub ProccntUsage: u32,
    pub modBaseAddr: *mut u8,
    pub modBaseSize: u32,
    pub hModule: super::super::super::Foundation::HMODULE,
    pub szModule: [i8; 256],
    pub szExePath: [i8; 260],
}
impl Copy for MODULEENTRY32 {}
impl Clone for MODULEENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for MODULEENTRY32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MODULEENTRY32").field("dwSize", &self.dwSize).field("th32ModuleID", &self.th32ModuleID).field("th32ProcessID", &self.th32ProcessID).field("GlblcntUsage", &self.GlblcntUsage).field("ProccntUsage", &self.ProccntUsage).field("modBaseAddr", &self.modBaseAddr).field("modBaseSize", &self.modBaseSize).field("hModule", &self.hModule).field("szModule", &self.szModule).field("szExePath", &self.szExePath).finish()
    }
}
impl windows_core::TypeKind for MODULEENTRY32 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for MODULEENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.th32ModuleID == other.th32ModuleID && self.th32ProcessID == other.th32ProcessID && self.GlblcntUsage == other.GlblcntUsage && self.ProccntUsage == other.ProccntUsage && self.modBaseAddr == other.modBaseAddr && self.modBaseSize == other.modBaseSize && self.hModule == other.hModule && self.szModule == other.szModule && self.szExePath == other.szExePath
    }
}
impl Eq for MODULEENTRY32 {}
impl Default for MODULEENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MODULEENTRY32W {
    pub dwSize: u32,
    pub th32ModuleID: u32,
    pub th32ProcessID: u32,
    pub GlblcntUsage: u32,
    pub ProccntUsage: u32,
    pub modBaseAddr: *mut u8,
    pub modBaseSize: u32,
    pub hModule: super::super::super::Foundation::HMODULE,
    pub szModule: [u16; 256],
    pub szExePath: [u16; 260],
}
impl Copy for MODULEENTRY32W {}
impl Clone for MODULEENTRY32W {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for MODULEENTRY32W {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MODULEENTRY32W").field("dwSize", &self.dwSize).field("th32ModuleID", &self.th32ModuleID).field("th32ProcessID", &self.th32ProcessID).field("GlblcntUsage", &self.GlblcntUsage).field("ProccntUsage", &self.ProccntUsage).field("modBaseAddr", &self.modBaseAddr).field("modBaseSize", &self.modBaseSize).field("hModule", &self.hModule).field("szModule", &self.szModule).field("szExePath", &self.szExePath).finish()
    }
}
impl windows_core::TypeKind for MODULEENTRY32W {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for MODULEENTRY32W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.th32ModuleID == other.th32ModuleID && self.th32ProcessID == other.th32ProcessID && self.GlblcntUsage == other.GlblcntUsage && self.ProccntUsage == other.ProccntUsage && self.modBaseAddr == other.modBaseAddr && self.modBaseSize == other.modBaseSize && self.hModule == other.hModule && self.szModule == other.szModule && self.szExePath == other.szExePath
    }
}
impl Eq for MODULEENTRY32W {}
impl Default for MODULEENTRY32W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PROCESSENTRY32 {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ProcessID: u32,
    pub th32DefaultHeapID: usize,
    pub th32ModuleID: u32,
    pub cntThreads: u32,
    pub th32ParentProcessID: u32,
    pub pcPriClassBase: i32,
    pub dwFlags: u32,
    pub szExeFile: [i8; 260],
}
impl Copy for PROCESSENTRY32 {}
impl Clone for PROCESSENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PROCESSENTRY32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PROCESSENTRY32").field("dwSize", &self.dwSize).field("cntUsage", &self.cntUsage).field("th32ProcessID", &self.th32ProcessID).field("th32DefaultHeapID", &self.th32DefaultHeapID).field("th32ModuleID", &self.th32ModuleID).field("cntThreads", &self.cntThreads).field("th32ParentProcessID", &self.th32ParentProcessID).field("pcPriClassBase", &self.pcPriClassBase).field("dwFlags", &self.dwFlags).field("szExeFile", &self.szExeFile).finish()
    }
}
impl windows_core::TypeKind for PROCESSENTRY32 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PROCESSENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cntUsage == other.cntUsage && self.th32ProcessID == other.th32ProcessID && self.th32DefaultHeapID == other.th32DefaultHeapID && self.th32ModuleID == other.th32ModuleID && self.cntThreads == other.cntThreads && self.th32ParentProcessID == other.th32ParentProcessID && self.pcPriClassBase == other.pcPriClassBase && self.dwFlags == other.dwFlags && self.szExeFile == other.szExeFile
    }
}
impl Eq for PROCESSENTRY32 {}
impl Default for PROCESSENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PROCESSENTRY32W {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ProcessID: u32,
    pub th32DefaultHeapID: usize,
    pub th32ModuleID: u32,
    pub cntThreads: u32,
    pub th32ParentProcessID: u32,
    pub pcPriClassBase: i32,
    pub dwFlags: u32,
    pub szExeFile: [u16; 260],
}
impl Copy for PROCESSENTRY32W {}
impl Clone for PROCESSENTRY32W {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PROCESSENTRY32W {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PROCESSENTRY32W").field("dwSize", &self.dwSize).field("cntUsage", &self.cntUsage).field("th32ProcessID", &self.th32ProcessID).field("th32DefaultHeapID", &self.th32DefaultHeapID).field("th32ModuleID", &self.th32ModuleID).field("cntThreads", &self.cntThreads).field("th32ParentProcessID", &self.th32ParentProcessID).field("pcPriClassBase", &self.pcPriClassBase).field("dwFlags", &self.dwFlags).field("szExeFile", &self.szExeFile).finish()
    }
}
impl windows_core::TypeKind for PROCESSENTRY32W {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PROCESSENTRY32W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cntUsage == other.cntUsage && self.th32ProcessID == other.th32ProcessID && self.th32DefaultHeapID == other.th32DefaultHeapID && self.th32ModuleID == other.th32ModuleID && self.cntThreads == other.cntThreads && self.th32ParentProcessID == other.th32ParentProcessID && self.pcPriClassBase == other.pcPriClassBase && self.dwFlags == other.dwFlags && self.szExeFile == other.szExeFile
    }
}
impl Eq for PROCESSENTRY32W {}
impl Default for PROCESSENTRY32W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct THREADENTRY32 {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ThreadID: u32,
    pub th32OwnerProcessID: u32,
    pub tpBasePri: i32,
    pub tpDeltaPri: i32,
    pub dwFlags: u32,
}
impl Copy for THREADENTRY32 {}
impl Clone for THREADENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for THREADENTRY32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("THREADENTRY32").field("dwSize", &self.dwSize).field("cntUsage", &self.cntUsage).field("th32ThreadID", &self.th32ThreadID).field("th32OwnerProcessID", &self.th32OwnerProcessID).field("tpBasePri", &self.tpBasePri).field("tpDeltaPri", &self.tpDeltaPri).field("dwFlags", &self.dwFlags).finish()
    }
}
impl windows_core::TypeKind for THREADENTRY32 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for THREADENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cntUsage == other.cntUsage && self.th32ThreadID == other.th32ThreadID && self.th32OwnerProcessID == other.th32OwnerProcessID && self.tpBasePri == other.tpBasePri && self.tpDeltaPri == other.tpDeltaPri && self.dwFlags == other.dwFlags
    }
}
impl Eq for THREADENTRY32 {}
impl Default for THREADENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
