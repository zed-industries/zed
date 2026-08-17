#[inline]
pub unsafe fn CfCloseHandle(filehandle: super::super::Foundation::HANDLE) {
    windows_core::link!("cldapi.dll" "system" fn CfCloseHandle(filehandle : super::super::Foundation:: HANDLE));
    unsafe { CfCloseHandle(filehandle) }
}
#[cfg(feature = "Win32_System_CorrelationVector")]
#[inline]
pub unsafe fn CfConnectSyncRoot<P0>(syncrootpath: P0, callbacktable: *const CF_CALLBACK_REGISTRATION, callbackcontext: Option<*const core::ffi::c_void>, connectflags: CF_CONNECT_FLAGS) -> windows_core::Result<CF_CONNECTION_KEY>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfConnectSyncRoot(syncrootpath : windows_core::PCWSTR, callbacktable : *const CF_CALLBACK_REGISTRATION, callbackcontext : *const core::ffi::c_void, connectflags : CF_CONNECT_FLAGS, connectionkey : *mut CF_CONNECTION_KEY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfConnectSyncRoot(syncrootpath.param().abi(), callbacktable, callbackcontext.unwrap_or(core::mem::zeroed()) as _, connectflags, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn CfConvertToPlaceholder(filehandle: super::super::Foundation::HANDLE, fileidentity: Option<*const core::ffi::c_void>, fileidentitylength: u32, convertflags: CF_CONVERT_FLAGS, convertusn: Option<*mut i64>, overlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfConvertToPlaceholder(filehandle : super::super::Foundation:: HANDLE, fileidentity : *const core::ffi::c_void, fileidentitylength : u32, convertflags : CF_CONVERT_FLAGS, convertusn : *mut i64, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfConvertToPlaceholder(filehandle, fileidentity.unwrap_or(core::mem::zeroed()) as _, fileidentitylength, convertflags, convertusn.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn CfCreatePlaceholders<P0>(basedirectorypath: P0, placeholderarray: &mut [CF_PLACEHOLDER_CREATE_INFO], createflags: CF_CREATE_FLAGS, entriesprocessed: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfCreatePlaceholders(basedirectorypath : windows_core::PCWSTR, placeholderarray : *mut CF_PLACEHOLDER_CREATE_INFO, placeholdercount : u32, createflags : CF_CREATE_FLAGS, entriesprocessed : *mut u32) -> windows_core::HRESULT);
    unsafe { CfCreatePlaceholders(basedirectorypath.param().abi(), core::mem::transmute(placeholderarray.as_ptr()), placeholderarray.len().try_into().unwrap(), createflags, entriesprocessed.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn CfDehydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, dehydrateflags: CF_DEHYDRATE_FLAGS, overlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfDehydratePlaceholder(filehandle : super::super::Foundation:: HANDLE, startingoffset : i64, length : i64, dehydrateflags : CF_DEHYDRATE_FLAGS, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfDehydratePlaceholder(filehandle, startingoffset, length, dehydrateflags, overlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CfDisconnectSyncRoot(connectionkey: CF_CONNECTION_KEY) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfDisconnectSyncRoot(connectionkey : CF_CONNECTION_KEY) -> windows_core::HRESULT);
    unsafe { CfDisconnectSyncRoot(connectionkey).ok() }
}
#[cfg(all(feature = "Win32_Storage_FileSystem", feature = "Win32_System_CorrelationVector"))]
#[inline]
pub unsafe fn CfExecute(opinfo: *const CF_OPERATION_INFO, opparams: *mut CF_OPERATION_PARAMETERS) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfExecute(opinfo : *const CF_OPERATION_INFO, opparams : *mut CF_OPERATION_PARAMETERS) -> windows_core::HRESULT);
    unsafe { CfExecute(opinfo, opparams as _).ok() }
}
#[cfg(feature = "Win32_System_CorrelationVector")]
#[inline]
pub unsafe fn CfGetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *mut super::super::System::CorrelationVector::CORRELATION_VECTOR) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfGetCorrelationVector(filehandle : super::super::Foundation:: HANDLE, correlationvector : *mut super::super::System::CorrelationVector:: CORRELATION_VECTOR) -> windows_core::HRESULT);
    unsafe { CfGetCorrelationVector(filehandle, correlationvector as _).ok() }
}
#[inline]
pub unsafe fn CfGetPlaceholderInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_INFO_CLASS, infobuffer: *mut core::ffi::c_void, infobufferlength: u32, returnedlength: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderInfo(filehandle : super::super::Foundation:: HANDLE, infoclass : CF_PLACEHOLDER_INFO_CLASS, infobuffer : *mut core::ffi::c_void, infobufferlength : u32, returnedlength : *mut u32) -> windows_core::HRESULT);
    unsafe { CfGetPlaceholderInfo(filehandle, infoclass, infobuffer as _, infobufferlength, returnedlength.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CfGetPlaceholderRangeInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, length: i64, infobuffer: *mut core::ffi::c_void, infobufferlength: u32, returnedlength: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderRangeInfo(filehandle : super::super::Foundation:: HANDLE, infoclass : CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset : i64, length : i64, infobuffer : *mut core::ffi::c_void, infobufferlength : u32, returnedlength : *mut u32) -> windows_core::HRESULT);
    unsafe { CfGetPlaceholderRangeInfo(filehandle, infoclass, startingoffset, length, infobuffer as _, infobufferlength, returnedlength.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CfGetPlaceholderRangeInfoForHydration(connectionkey: CF_CONNECTION_KEY, transferkey: i64, fileid: i64, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, rangelength: i64, infobuffer: *mut core::ffi::c_void, infobuffersize: u32, infobufferwritten: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderRangeInfoForHydration(connectionkey : CF_CONNECTION_KEY, transferkey : i64, fileid : i64, infoclass : CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset : i64, rangelength : i64, infobuffer : *mut core::ffi::c_void, infobuffersize : u32, infobufferwritten : *mut u32) -> windows_core::HRESULT);
    unsafe { CfGetPlaceholderRangeInfoForHydration(connectionkey, transferkey, fileid, infoclass, startingoffset, rangelength, infobuffer as _, infobuffersize, infobufferwritten.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CfGetPlaceholderStateFromAttributeTag(fileattributes: u32, reparsetag: u32) -> CF_PLACEHOLDER_STATE {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderStateFromAttributeTag(fileattributes : u32, reparsetag : u32) -> CF_PLACEHOLDER_STATE);
    unsafe { CfGetPlaceholderStateFromAttributeTag(fileattributes, reparsetag) }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromFileInfo(infobuffer: *const core::ffi::c_void, infoclass: super::FileSystem::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderStateFromFileInfo(infobuffer : *const core::ffi::c_void, infoclass : super::FileSystem:: FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE);
    unsafe { CfGetPlaceholderStateFromFileInfo(infobuffer, infoclass) }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromFindData(finddata: *const super::FileSystem::WIN32_FIND_DATAA) -> CF_PLACEHOLDER_STATE {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderStateFromFindData(finddata : *const super::FileSystem:: WIN32_FIND_DATAA) -> CF_PLACEHOLDER_STATE);
    unsafe { CfGetPlaceholderStateFromFindData(finddata) }
}
#[inline]
pub unsafe fn CfGetPlatformInfo() -> windows_core::Result<CF_PLATFORM_INFO> {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlatformInfo(platformversion : *mut CF_PLATFORM_INFO) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfGetPlatformInfo(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CfGetSyncRootInfoByHandle(filehandle: super::super::Foundation::HANDLE, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut core::ffi::c_void, infobufferlength: u32, returnedlength: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfGetSyncRootInfoByHandle(filehandle : super::super::Foundation:: HANDLE, infoclass : CF_SYNC_ROOT_INFO_CLASS, infobuffer : *mut core::ffi::c_void, infobufferlength : u32, returnedlength : *mut u32) -> windows_core::HRESULT);
    unsafe { CfGetSyncRootInfoByHandle(filehandle, infoclass, infobuffer as _, infobufferlength, returnedlength.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CfGetSyncRootInfoByPath<P0>(filepath: P0, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut core::ffi::c_void, infobufferlength: u32, returnedlength: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfGetSyncRootInfoByPath(filepath : windows_core::PCWSTR, infoclass : CF_SYNC_ROOT_INFO_CLASS, infobuffer : *mut core::ffi::c_void, infobufferlength : u32, returnedlength : *mut u32) -> windows_core::HRESULT);
    unsafe { CfGetSyncRootInfoByPath(filepath.param().abi(), infoclass, infobuffer as _, infobufferlength, returnedlength.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CfGetTransferKey(filehandle: super::super::Foundation::HANDLE) -> windows_core::Result<i64> {
    windows_core::link!("cldapi.dll" "system" fn CfGetTransferKey(filehandle : super::super::Foundation:: HANDLE, transferkey : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfGetTransferKey(filehandle, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CfGetWin32HandleFromProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE {
    windows_core::link!("cldapi.dll" "system" fn CfGetWin32HandleFromProtectedHandle(protectedhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    unsafe { CfGetWin32HandleFromProtectedHandle(protectedhandle) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn CfHydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, hydrateflags: CF_HYDRATE_FLAGS, overlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfHydratePlaceholder(filehandle : super::super::Foundation:: HANDLE, startingoffset : i64, length : i64, hydrateflags : CF_HYDRATE_FLAGS, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfHydratePlaceholder(filehandle, startingoffset, length, hydrateflags, overlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CfOpenFileWithOplock<P0>(filepath: P0, flags: CF_OPEN_FILE_FLAGS) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfOpenFileWithOplock(filepath : windows_core::PCWSTR, flags : CF_OPEN_FILE_FLAGS, protectedhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfOpenFileWithOplock(filepath.param().abi(), flags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CfQuerySyncProviderStatus(connectionkey: CF_CONNECTION_KEY) -> windows_core::Result<CF_SYNC_PROVIDER_STATUS> {
    windows_core::link!("cldapi.dll" "system" fn CfQuerySyncProviderStatus(connectionkey : CF_CONNECTION_KEY, providerstatus : *mut CF_SYNC_PROVIDER_STATUS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfQuerySyncProviderStatus(connectionkey, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CfReferenceProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> bool {
    windows_core::link!("cldapi.dll" "system" fn CfReferenceProtectedHandle(protectedhandle : super::super::Foundation:: HANDLE) -> bool);
    unsafe { CfReferenceProtectedHandle(protectedhandle) }
}
#[inline]
pub unsafe fn CfRegisterSyncRoot<P0>(syncrootpath: P0, registration: *const CF_SYNC_REGISTRATION, policies: *const CF_SYNC_POLICIES, registerflags: CF_REGISTER_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfRegisterSyncRoot(syncrootpath : windows_core::PCWSTR, registration : *const CF_SYNC_REGISTRATION, policies : *const CF_SYNC_POLICIES, registerflags : CF_REGISTER_FLAGS) -> windows_core::HRESULT);
    unsafe { CfRegisterSyncRoot(syncrootpath.param().abi(), registration, policies, registerflags).ok() }
}
#[inline]
pub unsafe fn CfReleaseProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) {
    windows_core::link!("cldapi.dll" "system" fn CfReleaseProtectedHandle(protectedhandle : super::super::Foundation:: HANDLE));
    unsafe { CfReleaseProtectedHandle(protectedhandle) }
}
#[inline]
pub unsafe fn CfReleaseTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *const i64) {
    windows_core::link!("cldapi.dll" "system" fn CfReleaseTransferKey(filehandle : super::super::Foundation:: HANDLE, transferkey : *const i64));
    unsafe { CfReleaseTransferKey(filehandle, transferkey) }
}
#[inline]
pub unsafe fn CfReportProviderProgress(connectionkey: CF_CONNECTION_KEY, transferkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfReportProviderProgress(connectionkey : CF_CONNECTION_KEY, transferkey : i64, providerprogresstotal : i64, providerprogresscompleted : i64) -> windows_core::HRESULT);
    unsafe { CfReportProviderProgress(connectionkey, transferkey, providerprogresstotal, providerprogresscompleted).ok() }
}
#[inline]
pub unsafe fn CfReportProviderProgress2(connectionkey: CF_CONNECTION_KEY, transferkey: i64, requestkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64, targetsessionid: u32) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfReportProviderProgress2(connectionkey : CF_CONNECTION_KEY, transferkey : i64, requestkey : i64, providerprogresstotal : i64, providerprogresscompleted : i64, targetsessionid : u32) -> windows_core::HRESULT);
    unsafe { CfReportProviderProgress2(connectionkey, transferkey, requestkey, providerprogresstotal, providerprogresscompleted, targetsessionid).ok() }
}
#[inline]
pub unsafe fn CfReportSyncStatus<P0>(syncrootpath: P0, syncstatus: Option<*const CF_SYNC_STATUS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfReportSyncStatus(syncrootpath : windows_core::PCWSTR, syncstatus : *const CF_SYNC_STATUS) -> windows_core::HRESULT);
    unsafe { CfReportSyncStatus(syncrootpath.param().abi(), syncstatus.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn CfRevertPlaceholder(filehandle: super::super::Foundation::HANDLE, revertflags: CF_REVERT_FLAGS, overlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfRevertPlaceholder(filehandle : super::super::Foundation:: HANDLE, revertflags : CF_REVERT_FLAGS, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfRevertPlaceholder(filehandle, revertflags, overlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_CorrelationVector")]
#[inline]
pub unsafe fn CfSetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *const super::super::System::CorrelationVector::CORRELATION_VECTOR) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfSetCorrelationVector(filehandle : super::super::Foundation:: HANDLE, correlationvector : *const super::super::System::CorrelationVector:: CORRELATION_VECTOR) -> windows_core::HRESULT);
    unsafe { CfSetCorrelationVector(filehandle, correlationvector).ok() }
}
#[inline]
pub unsafe fn CfSetInSyncState(filehandle: super::super::Foundation::HANDLE, insyncstate: CF_IN_SYNC_STATE, insyncflags: CF_SET_IN_SYNC_FLAGS, insyncusn: Option<*mut i64>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfSetInSyncState(filehandle : super::super::Foundation:: HANDLE, insyncstate : CF_IN_SYNC_STATE, insyncflags : CF_SET_IN_SYNC_FLAGS, insyncusn : *mut i64) -> windows_core::HRESULT);
    unsafe { CfSetInSyncState(filehandle, insyncstate, insyncflags, insyncusn.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn CfSetPinState(filehandle: super::super::Foundation::HANDLE, pinstate: CF_PIN_STATE, pinflags: CF_SET_PIN_FLAGS, overlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfSetPinState(filehandle : super::super::Foundation:: HANDLE, pinstate : CF_PIN_STATE, pinflags : CF_SET_PIN_FLAGS, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfSetPinState(filehandle, pinstate, pinflags, overlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CfUnregisterSyncRoot<P0>(syncrootpath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfUnregisterSyncRoot(syncrootpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { CfUnregisterSyncRoot(syncrootpath.param().abi()).ok() }
}
#[cfg(all(feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfUpdatePlaceholder(filehandle: super::super::Foundation::HANDLE, fsmetadata: Option<*const CF_FS_METADATA>, fileidentity: Option<*const core::ffi::c_void>, fileidentitylength: u32, dehydraterangearray: Option<&[CF_FILE_RANGE]>, updateflags: CF_UPDATE_FLAGS, updateusn: Option<*mut i64>, overlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfUpdatePlaceholder(filehandle : super::super::Foundation:: HANDLE, fsmetadata : *const CF_FS_METADATA, fileidentity : *const core::ffi::c_void, fileidentitylength : u32, dehydraterangearray : *const CF_FILE_RANGE, dehydraterangecount : u32, updateflags : CF_UPDATE_FLAGS, updateusn : *mut i64, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfUpdatePlaceholder(filehandle, fsmetadata.unwrap_or(core::mem::zeroed()) as _, fileidentity.unwrap_or(core::mem::zeroed()) as _, fileidentitylength, core::mem::transmute(dehydraterangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dehydraterangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), updateflags, updateusn.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CfUpdateSyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: CF_SYNC_PROVIDER_STATUS) -> windows_core::Result<()> {
    windows_core::link!("cldapi.dll" "system" fn CfUpdateSyncProviderStatus(connectionkey : CF_CONNECTION_KEY, providerstatus : CF_SYNC_PROVIDER_STATUS) -> windows_core::HRESULT);
    unsafe { CfUpdateSyncProviderStatus(connectionkey, providerstatus).ok() }
}
#[cfg(feature = "Win32_System_CorrelationVector")]
pub type CF_CALLBACK = Option<unsafe extern "system" fn(callbackinfo: *const CF_CALLBACK_INFO, callbackparameters: *const CF_CALLBACK_PARAMETERS)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_CANCEL_FLAGS(pub i32);
impl CF_CALLBACK_CANCEL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_CANCEL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_CANCEL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_CANCEL_FLAG_IO_ABORTED: CF_CALLBACK_CANCEL_FLAGS = CF_CALLBACK_CANCEL_FLAGS(2i32);
pub const CF_CALLBACK_CANCEL_FLAG_IO_TIMEOUT: CF_CALLBACK_CANCEL_FLAGS = CF_CALLBACK_CANCEL_FLAGS(1i32);
pub const CF_CALLBACK_CANCEL_FLAG_NONE: CF_CALLBACK_CANCEL_FLAGS = CF_CALLBACK_CANCEL_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_CLOSE_COMPLETION_FLAGS(pub i32);
impl CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_DELETED: CF_CALLBACK_CLOSE_COMPLETION_FLAGS = CF_CALLBACK_CLOSE_COMPLETION_FLAGS(1i32);
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_NONE: CF_CALLBACK_CLOSE_COMPLETION_FLAGS = CF_CALLBACK_CLOSE_COMPLETION_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(pub i32);
impl CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_BACKGROUND: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(1i32);
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_DEHYDRATED: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(2i32);
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_NONE: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_DEHYDRATE_FLAGS(pub i32);
impl CF_CALLBACK_DEHYDRATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_DEHYDRATE_FLAG_BACKGROUND: CF_CALLBACK_DEHYDRATE_FLAGS = CF_CALLBACK_DEHYDRATE_FLAGS(1i32);
pub const CF_CALLBACK_DEHYDRATE_FLAG_NONE: CF_CALLBACK_DEHYDRATE_FLAGS = CF_CALLBACK_DEHYDRATE_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_DEHYDRATION_REASON(pub i32);
pub const CF_CALLBACK_DEHYDRATION_REASON_NONE: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(0i32);
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_INACTIVITY: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(3i32);
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_LOW_SPACE: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(2i32);
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_OS_UPGRADE: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(4i32);
pub const CF_CALLBACK_DEHYDRATION_REASON_USER_MANUAL: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_DELETE_COMPLETION_FLAGS(pub i32);
impl CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_DELETE_COMPLETION_FLAG_NONE: CF_CALLBACK_DELETE_COMPLETION_FLAGS = CF_CALLBACK_DELETE_COMPLETION_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_DELETE_FLAGS(pub i32);
impl CF_CALLBACK_DELETE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_DELETE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_DELETE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_DELETE_FLAG_IS_DIRECTORY: CF_CALLBACK_DELETE_FLAGS = CF_CALLBACK_DELETE_FLAGS(1i32);
pub const CF_CALLBACK_DELETE_FLAG_IS_UNDELETE: CF_CALLBACK_DELETE_FLAGS = CF_CALLBACK_DELETE_FLAGS(2i32);
pub const CF_CALLBACK_DELETE_FLAG_NONE: CF_CALLBACK_DELETE_FLAGS = CF_CALLBACK_DELETE_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_FETCH_DATA_FLAGS(pub i32);
impl CF_CALLBACK_FETCH_DATA_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_FETCH_DATA_FLAG_EXPLICIT_HYDRATION: CF_CALLBACK_FETCH_DATA_FLAGS = CF_CALLBACK_FETCH_DATA_FLAGS(2i32);
pub const CF_CALLBACK_FETCH_DATA_FLAG_NONE: CF_CALLBACK_FETCH_DATA_FLAGS = CF_CALLBACK_FETCH_DATA_FLAGS(0i32);
pub const CF_CALLBACK_FETCH_DATA_FLAG_RECOVERY: CF_CALLBACK_FETCH_DATA_FLAGS = CF_CALLBACK_FETCH_DATA_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS(pub i32);
impl CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_FETCH_PLACEHOLDERS_FLAG_NONE: CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS = CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS(0i32);
#[repr(C)]
#[cfg(feature = "Win32_System_CorrelationVector")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_CALLBACK_INFO {
    pub StructSize: u32,
    pub ConnectionKey: CF_CONNECTION_KEY,
    pub CallbackContext: *mut core::ffi::c_void,
    pub VolumeGuidName: windows_core::PCWSTR,
    pub VolumeDosName: windows_core::PCWSTR,
    pub VolumeSerialNumber: u32,
    pub SyncRootFileId: i64,
    pub SyncRootIdentity: *const core::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileId: i64,
    pub FileSize: i64,
    pub FileIdentity: *const core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub NormalizedPath: windows_core::PCWSTR,
    pub TransferKey: i64,
    pub PriorityHint: u8,
    pub CorrelationVector: *mut super::super::System::CorrelationVector::CORRELATION_VECTOR,
    pub ProcessInfo: *mut CF_PROCESS_INFO,
    pub RequestKey: i64,
}
#[cfg(feature = "Win32_System_CorrelationVector")]
impl Default for CF_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_OPEN_COMPLETION_FLAGS(pub i32);
impl CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_NONE: CF_CALLBACK_OPEN_COMPLETION_FLAGS = CF_CALLBACK_OPEN_COMPLETION_FLAGS(0i32);
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNKNOWN: CF_CALLBACK_OPEN_COMPLETION_FLAGS = CF_CALLBACK_OPEN_COMPLETION_FLAGS(1i32);
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNSUPPORTED: CF_CALLBACK_OPEN_COMPLETION_FLAGS = CF_CALLBACK_OPEN_COMPLETION_FLAGS(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CF_CALLBACK_PARAMETERS {
    pub ParamSize: u32,
    pub Anonymous: CF_CALLBACK_PARAMETERS_0,
}
impl Default for CF_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CF_CALLBACK_PARAMETERS_0 {
    pub Cancel: CF_CALLBACK_PARAMETERS_0_0,
    pub FetchData: CF_CALLBACK_PARAMETERS_0_1,
    pub ValidateData: CF_CALLBACK_PARAMETERS_0_2,
    pub FetchPlaceholders: CF_CALLBACK_PARAMETERS_0_3,
    pub OpenCompletion: CF_CALLBACK_PARAMETERS_0_4,
    pub CloseCompletion: CF_CALLBACK_PARAMETERS_0_5,
    pub Dehydrate: CF_CALLBACK_PARAMETERS_0_6,
    pub DehydrateCompletion: CF_CALLBACK_PARAMETERS_0_7,
    pub Delete: CF_CALLBACK_PARAMETERS_0_8,
    pub DeleteCompletion: CF_CALLBACK_PARAMETERS_0_9,
    pub Rename: CF_CALLBACK_PARAMETERS_0_10,
    pub RenameCompletion: CF_CALLBACK_PARAMETERS_0_11,
}
impl Default for CF_CALLBACK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CF_CALLBACK_PARAMETERS_0_0 {
    pub Flags: CF_CALLBACK_CANCEL_FLAGS,
    pub Anonymous: CF_CALLBACK_PARAMETERS_0_0_0,
}
impl Default for CF_CALLBACK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CF_CALLBACK_PARAMETERS_0_0_0 {
    pub FetchData: CF_CALLBACK_PARAMETERS_0_0_0_0,
}
impl Default for CF_CALLBACK_PARAMETERS_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_0_0_0 {
    pub FileOffset: i64,
    pub Length: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_5 {
    pub Flags: CF_CALLBACK_CLOSE_COMPLETION_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_7 {
    pub Flags: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS,
    pub Reason: CF_CALLBACK_DEHYDRATION_REASON,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_6 {
    pub Flags: CF_CALLBACK_DEHYDRATE_FLAGS,
    pub Reason: CF_CALLBACK_DEHYDRATION_REASON,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_9 {
    pub Flags: CF_CALLBACK_DELETE_COMPLETION_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_8 {
    pub Flags: CF_CALLBACK_DELETE_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_1 {
    pub Flags: CF_CALLBACK_FETCH_DATA_FLAGS,
    pub RequiredFileOffset: i64,
    pub RequiredLength: i64,
    pub OptionalFileOffset: i64,
    pub OptionalLength: i64,
    pub LastDehydrationTime: i64,
    pub LastDehydrationReason: CF_CALLBACK_DEHYDRATION_REASON,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_3 {
    pub Flags: CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS,
    pub Pattern: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_4 {
    pub Flags: CF_CALLBACK_OPEN_COMPLETION_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_11 {
    pub Flags: CF_CALLBACK_RENAME_COMPLETION_FLAGS,
    pub SourcePath: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_10 {
    pub Flags: CF_CALLBACK_RENAME_FLAGS,
    pub TargetPath: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_2 {
    pub Flags: CF_CALLBACK_VALIDATE_DATA_FLAGS,
    pub RequiredFileOffset: i64,
    pub RequiredLength: i64,
}
#[repr(C)]
#[cfg(feature = "Win32_System_CorrelationVector")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CF_CALLBACK_REGISTRATION {
    pub Type: CF_CALLBACK_TYPE,
    pub Callback: CF_CALLBACK,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_RENAME_COMPLETION_FLAGS(pub i32);
impl CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_RENAME_COMPLETION_FLAG_NONE: CF_CALLBACK_RENAME_COMPLETION_FLAGS = CF_CALLBACK_RENAME_COMPLETION_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_RENAME_FLAGS(pub i32);
impl CF_CALLBACK_RENAME_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_RENAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_RENAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_RENAME_FLAG_IS_DIRECTORY: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(1i32);
pub const CF_CALLBACK_RENAME_FLAG_NONE: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(0i32);
pub const CF_CALLBACK_RENAME_FLAG_SOURCE_IN_SCOPE: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(2i32);
pub const CF_CALLBACK_RENAME_FLAG_TARGET_IN_SCOPE: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_TYPE(pub i32);
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_DATA: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(2i32);
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_PLACEHOLDERS: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(4i32);
pub const CF_CALLBACK_TYPE_FETCH_DATA: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(0i32);
pub const CF_CALLBACK_TYPE_FETCH_PLACEHOLDERS: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(3i32);
pub const CF_CALLBACK_TYPE_NONE: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(-1i32);
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(7i32);
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(8i32);
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(9i32);
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(10i32);
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_CLOSE_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(6i32);
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_OPEN_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(5i32);
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(11i32);
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(12i32);
pub const CF_CALLBACK_TYPE_VALIDATE_DATA: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_VALIDATE_DATA_FLAGS(pub i32);
impl CF_CALLBACK_VALIDATE_DATA_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_EXPLICIT_HYDRATION: CF_CALLBACK_VALIDATE_DATA_FLAGS = CF_CALLBACK_VALIDATE_DATA_FLAGS(2i32);
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_NONE: CF_CALLBACK_VALIDATE_DATA_FLAGS = CF_CALLBACK_VALIDATE_DATA_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CF_CONNECTION_KEY(pub i64);
impl CF_CONNECTION_KEY {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for CF_CONNECTION_KEY {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("cldapi.dll" "system" fn CfDisconnectSyncRoot(connectionkey : i64) -> i32);
            unsafe {
                CfDisconnectSyncRoot(self.0);
            }
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CONNECT_FLAGS(pub i32);
impl CF_CONNECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CONNECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CONNECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CONNECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CONNECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CONNECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CONNECT_FLAG_BLOCK_SELF_IMPLICIT_HYDRATION: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(8i32);
pub const CF_CONNECT_FLAG_NONE: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(0i32);
pub const CF_CONNECT_FLAG_REQUIRE_FULL_FILE_PATH: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(4i32);
pub const CF_CONNECT_FLAG_REQUIRE_PROCESS_INFO: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CONVERT_FLAGS(pub i32);
impl CF_CONVERT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CONVERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CONVERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CONVERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CONVERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CONVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CONVERT_FLAG_ALWAYS_FULL: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(8i32);
pub const CF_CONVERT_FLAG_DEHYDRATE: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(2i32);
pub const CF_CONVERT_FLAG_ENABLE_ON_DEMAND_POPULATION: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(4i32);
pub const CF_CONVERT_FLAG_FORCE_CONVERT_TO_CLOUD_FILE: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(16i32);
pub const CF_CONVERT_FLAG_MARK_IN_SYNC: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(1i32);
pub const CF_CONVERT_FLAG_NONE: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CREATE_FLAGS(pub i32);
impl CF_CREATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_CREATE_FLAG_NONE: CF_CREATE_FLAGS = CF_CREATE_FLAGS(0i32);
pub const CF_CREATE_FLAG_STOP_ON_ERROR: CF_CREATE_FLAGS = CF_CREATE_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_DEHYDRATE_FLAGS(pub i32);
impl CF_DEHYDRATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_DEHYDRATE_FLAG_BACKGROUND: CF_DEHYDRATE_FLAGS = CF_DEHYDRATE_FLAGS(1i32);
pub const CF_DEHYDRATE_FLAG_NONE: CF_DEHYDRATE_FLAGS = CF_DEHYDRATE_FLAGS(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_FILE_RANGE {
    pub StartingOffset: i64,
    pub Length: i64,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_FS_METADATA {
    pub BasicInfo: super::FileSystem::FILE_BASIC_INFO,
    pub FileSize: i64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_HARDLINK_POLICY(pub i32);
impl CF_HARDLINK_POLICY {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_HARDLINK_POLICY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_HARDLINK_POLICY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_HARDLINK_POLICY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_HARDLINK_POLICY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_HARDLINK_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_HARDLINK_POLICY_ALLOWED: CF_HARDLINK_POLICY = CF_HARDLINK_POLICY(1i32);
pub const CF_HARDLINK_POLICY_NONE: CF_HARDLINK_POLICY = CF_HARDLINK_POLICY(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_HYDRATE_FLAGS(pub i32);
impl CF_HYDRATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_HYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_HYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_HYDRATE_FLAG_NONE: CF_HYDRATE_FLAGS = CF_HYDRATE_FLAGS(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_HYDRATION_POLICY {
    pub Primary: CF_HYDRATION_POLICY_PRIMARY,
    pub Modifier: CF_HYDRATION_POLICY_MODIFIER,
}
pub const CF_HYDRATION_POLICY_ALWAYS_FULL: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(3u16);
pub const CF_HYDRATION_POLICY_FULL: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(2u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_HYDRATION_POLICY_MODIFIER(pub u16);
impl CF_HYDRATION_POLICY_MODIFIER {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_HYDRATION_POLICY_MODIFIER {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_HYDRATION_POLICY_MODIFIER {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_HYDRATION_POLICY_MODIFIER_ALLOW_FULL_RESTART_HYDRATION: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(8u16);
pub const CF_HYDRATION_POLICY_MODIFIER_AUTO_DEHYDRATION_ALLOWED: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(4u16);
pub const CF_HYDRATION_POLICY_MODIFIER_NONE: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(0u16);
pub const CF_HYDRATION_POLICY_MODIFIER_STREAMING_ALLOWED: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(2u16);
pub const CF_HYDRATION_POLICY_MODIFIER_VALIDATION_REQUIRED: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(1u16);
pub const CF_HYDRATION_POLICY_PARTIAL: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(0u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_HYDRATION_POLICY_PRIMARY(pub u16);
pub const CF_HYDRATION_POLICY_PROGRESSIVE: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(1u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_INSYNC_POLICY(pub u32);
impl CF_INSYNC_POLICY {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_INSYNC_POLICY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_INSYNC_POLICY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_INSYNC_POLICY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_INSYNC_POLICY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_INSYNC_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_INSYNC_POLICY_NONE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(0u32);
pub const CF_INSYNC_POLICY_PRESERVE_INSYNC_FOR_SYNC_ENGINE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(2147483648u32);
pub const CF_INSYNC_POLICY_TRACK_ALL: CF_INSYNC_POLICY = CF_INSYNC_POLICY(16777215u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_ALL: CF_INSYNC_POLICY = CF_INSYNC_POLICY(11184880u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_CREATION_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(16u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_HIDDEN_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(64u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_LAST_WRITE_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(512u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_READONLY_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(32u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_SYSTEM_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(128u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_ALL: CF_INSYNC_POLICY = CF_INSYNC_POLICY(5592335u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_CREATION_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(1u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_HIDDEN_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(4u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_LAST_WRITE_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(256u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_READONLY_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(2u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_SYSTEM_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(8u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_IN_SYNC_STATE(pub i32);
pub const CF_IN_SYNC_STATE_IN_SYNC: CF_IN_SYNC_STATE = CF_IN_SYNC_STATE(1i32);
pub const CF_IN_SYNC_STATE_NOT_IN_SYNC: CF_IN_SYNC_STATE = CF_IN_SYNC_STATE(0i32);
pub const CF_MAX_PRIORITY_HINT: u32 = 15u32;
pub const CF_MAX_PROVIDER_NAME_LENGTH: u32 = 255u32;
pub const CF_MAX_PROVIDER_VERSION_LENGTH: u32 = 255u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPEN_FILE_FLAGS(pub i32);
impl CF_OPEN_FILE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_OPEN_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_OPEN_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_OPEN_FILE_FLAG_DELETE_ACCESS: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(4i32);
pub const CF_OPEN_FILE_FLAG_EXCLUSIVE: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(1i32);
pub const CF_OPEN_FILE_FLAG_FOREGROUND: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(8i32);
pub const CF_OPEN_FILE_FLAG_NONE: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(0i32);
pub const CF_OPEN_FILE_FLAG_WRITE_ACCESS: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_ACK_DATA_FLAGS(pub i32);
impl CF_OPERATION_ACK_DATA_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_OPERATION_ACK_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_OPERATION_ACK_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_OPERATION_ACK_DATA_FLAG_NONE: CF_OPERATION_ACK_DATA_FLAGS = CF_OPERATION_ACK_DATA_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_ACK_DEHYDRATE_FLAGS(pub i32);
impl CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_OPERATION_ACK_DEHYDRATE_FLAG_NONE: CF_OPERATION_ACK_DEHYDRATE_FLAGS = CF_OPERATION_ACK_DEHYDRATE_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_ACK_DELETE_FLAGS(pub i32);
impl CF_OPERATION_ACK_DELETE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_OPERATION_ACK_DELETE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_OPERATION_ACK_DELETE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_OPERATION_ACK_DELETE_FLAG_NONE: CF_OPERATION_ACK_DELETE_FLAGS = CF_OPERATION_ACK_DELETE_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_ACK_RENAME_FLAGS(pub i32);
impl CF_OPERATION_ACK_RENAME_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_OPERATION_ACK_RENAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_OPERATION_ACK_RENAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_OPERATION_ACK_RENAME_FLAG_NONE: CF_OPERATION_ACK_RENAME_FLAGS = CF_OPERATION_ACK_RENAME_FLAGS(0i32);
#[repr(C)]
#[cfg(feature = "Win32_System_CorrelationVector")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_OPERATION_INFO {
    pub StructSize: u32,
    pub Type: CF_OPERATION_TYPE,
    pub ConnectionKey: CF_CONNECTION_KEY,
    pub TransferKey: i64,
    pub CorrelationVector: *const super::super::System::CorrelationVector::CORRELATION_VECTOR,
    pub SyncStatus: *const CF_SYNC_STATUS,
    pub RequestKey: i64,
}
#[cfg(feature = "Win32_System_CorrelationVector")]
impl Default for CF_OPERATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy)]
pub struct CF_OPERATION_PARAMETERS {
    pub ParamSize: u32,
    pub Anonymous: CF_OPERATION_PARAMETERS_0,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CF_OPERATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy)]
pub union CF_OPERATION_PARAMETERS_0 {
    pub TransferData: CF_OPERATION_PARAMETERS_0_0,
    pub RetrieveData: CF_OPERATION_PARAMETERS_0_1,
    pub AckData: CF_OPERATION_PARAMETERS_0_2,
    pub RestartHydration: CF_OPERATION_PARAMETERS_0_3,
    pub TransferPlaceholders: CF_OPERATION_PARAMETERS_0_4,
    pub AckDehydrate: CF_OPERATION_PARAMETERS_0_5,
    pub AckRename: CF_OPERATION_PARAMETERS_0_6,
    pub AckDelete: CF_OPERATION_PARAMETERS_0_7,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CF_OPERATION_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_2 {
    pub Flags: CF_OPERATION_ACK_DATA_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub Offset: i64,
    pub Length: i64,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_5 {
    pub Flags: CF_OPERATION_ACK_DEHYDRATE_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub FileIdentity: *const core::ffi::c_void,
    pub FileIdentityLength: u32,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CF_OPERATION_PARAMETERS_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_7 {
    pub Flags: CF_OPERATION_ACK_DELETE_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_6 {
    pub Flags: CF_OPERATION_ACK_RENAME_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_3 {
    pub Flags: CF_OPERATION_RESTART_HYDRATION_FLAGS,
    pub FsMetadata: *const CF_FS_METADATA,
    pub FileIdentity: *const core::ffi::c_void,
    pub FileIdentityLength: u32,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CF_OPERATION_PARAMETERS_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_1 {
    pub Flags: CF_OPERATION_RETRIEVE_DATA_FLAGS,
    pub Buffer: *mut core::ffi::c_void,
    pub Offset: i64,
    pub Length: i64,
    pub ReturnedLength: i64,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CF_OPERATION_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_0 {
    pub Flags: CF_OPERATION_TRANSFER_DATA_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub Buffer: *const core::ffi::c_void,
    pub Offset: i64,
    pub Length: i64,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CF_OPERATION_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_4 {
    pub Flags: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub PlaceholderTotalCount: i64,
    pub PlaceholderArray: *mut CF_PLACEHOLDER_CREATE_INFO,
    pub PlaceholderCount: u32,
    pub EntriesProcessed: u32,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CF_OPERATION_PARAMETERS_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_RESTART_HYDRATION_FLAGS(pub i32);
impl CF_OPERATION_RESTART_HYDRATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_MARK_IN_SYNC: CF_OPERATION_RESTART_HYDRATION_FLAGS = CF_OPERATION_RESTART_HYDRATION_FLAGS(1i32);
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_NONE: CF_OPERATION_RESTART_HYDRATION_FLAGS = CF_OPERATION_RESTART_HYDRATION_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_RETRIEVE_DATA_FLAGS(pub i32);
impl CF_OPERATION_RETRIEVE_DATA_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_OPERATION_RETRIEVE_DATA_FLAG_NONE: CF_OPERATION_RETRIEVE_DATA_FLAGS = CF_OPERATION_RETRIEVE_DATA_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_TRANSFER_DATA_FLAGS(pub i32);
impl CF_OPERATION_TRANSFER_DATA_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_OPERATION_TRANSFER_DATA_FLAG_NONE: CF_OPERATION_TRANSFER_DATA_FLAGS = CF_OPERATION_TRANSFER_DATA_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(pub i32);
impl CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(2i32);
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_NONE: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(0i32);
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_STOP_ON_ERROR: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_TYPE(pub i32);
pub const CF_OPERATION_TYPE_ACK_DATA: CF_OPERATION_TYPE = CF_OPERATION_TYPE(2i32);
pub const CF_OPERATION_TYPE_ACK_DEHYDRATE: CF_OPERATION_TYPE = CF_OPERATION_TYPE(5i32);
pub const CF_OPERATION_TYPE_ACK_DELETE: CF_OPERATION_TYPE = CF_OPERATION_TYPE(6i32);
pub const CF_OPERATION_TYPE_ACK_RENAME: CF_OPERATION_TYPE = CF_OPERATION_TYPE(7i32);
pub const CF_OPERATION_TYPE_RESTART_HYDRATION: CF_OPERATION_TYPE = CF_OPERATION_TYPE(3i32);
pub const CF_OPERATION_TYPE_RETRIEVE_DATA: CF_OPERATION_TYPE = CF_OPERATION_TYPE(1i32);
pub const CF_OPERATION_TYPE_TRANSFER_DATA: CF_OPERATION_TYPE = CF_OPERATION_TYPE(0i32);
pub const CF_OPERATION_TYPE_TRANSFER_PLACEHOLDERS: CF_OPERATION_TYPE = CF_OPERATION_TYPE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_PIN_STATE(pub i32);
pub const CF_PIN_STATE_EXCLUDED: CF_PIN_STATE = CF_PIN_STATE(3i32);
pub const CF_PIN_STATE_INHERIT: CF_PIN_STATE = CF_PIN_STATE(4i32);
pub const CF_PIN_STATE_PINNED: CF_PIN_STATE = CF_PIN_STATE(1i32);
pub const CF_PIN_STATE_UNPINNED: CF_PIN_STATE = CF_PIN_STATE(2i32);
pub const CF_PIN_STATE_UNSPECIFIED: CF_PIN_STATE = CF_PIN_STATE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_PLACEHOLDER_BASIC_INFO {
    pub PinState: CF_PIN_STATE,
    pub InSyncState: CF_IN_SYNC_STATE,
    pub FileId: i64,
    pub SyncRootFileId: i64,
    pub FileIdentityLength: u32,
    pub FileIdentity: [u8; 1],
}
impl Default for CF_PLACEHOLDER_BASIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_PLACEHOLDER_CREATE_FLAGS(pub i32);
impl CF_PLACEHOLDER_CREATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_PLACEHOLDER_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_PLACEHOLDER_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_PLACEHOLDER_CREATE_FLAG_ALWAYS_FULL: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(8i32);
pub const CF_PLACEHOLDER_CREATE_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(1i32);
pub const CF_PLACEHOLDER_CREATE_FLAG_MARK_IN_SYNC: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(2i32);
pub const CF_PLACEHOLDER_CREATE_FLAG_NONE: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(0i32);
pub const CF_PLACEHOLDER_CREATE_FLAG_SUPERSEDE: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(4i32);
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_PLACEHOLDER_CREATE_INFO {
    pub RelativeFileName: windows_core::PCWSTR,
    pub FsMetadata: CF_FS_METADATA,
    pub FileIdentity: *const core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub Flags: CF_PLACEHOLDER_CREATE_FLAGS,
    pub Result: windows_core::HRESULT,
    pub CreateUsn: i64,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl Default for CF_PLACEHOLDER_CREATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CF_PLACEHOLDER_INFO_BASIC: CF_PLACEHOLDER_INFO_CLASS = CF_PLACEHOLDER_INFO_CLASS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_PLACEHOLDER_INFO_CLASS(pub i32);
pub const CF_PLACEHOLDER_INFO_STANDARD: CF_PLACEHOLDER_INFO_CLASS = CF_PLACEHOLDER_INFO_CLASS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_PLACEHOLDER_MANAGEMENT_POLICY(pub i32);
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CONVERT_TO_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(2i32);
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CREATE_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(1i32);
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_DEFAULT: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(0i32);
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_UPDATE_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(4i32);
pub const CF_PLACEHOLDER_MAX_FILE_IDENTITY_LENGTH: u32 = 4096u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_PLACEHOLDER_RANGE_INFO_CLASS(pub i32);
pub const CF_PLACEHOLDER_RANGE_INFO_MODIFIED: CF_PLACEHOLDER_RANGE_INFO_CLASS = CF_PLACEHOLDER_RANGE_INFO_CLASS(3i32);
pub const CF_PLACEHOLDER_RANGE_INFO_ONDISK: CF_PLACEHOLDER_RANGE_INFO_CLASS = CF_PLACEHOLDER_RANGE_INFO_CLASS(1i32);
pub const CF_PLACEHOLDER_RANGE_INFO_VALIDATED: CF_PLACEHOLDER_RANGE_INFO_CLASS = CF_PLACEHOLDER_RANGE_INFO_CLASS(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_PLACEHOLDER_STANDARD_INFO {
    pub OnDiskDataSize: i64,
    pub ValidatedDataSize: i64,
    pub ModifiedDataSize: i64,
    pub PropertiesSize: i64,
    pub PinState: CF_PIN_STATE,
    pub InSyncState: CF_IN_SYNC_STATE,
    pub FileId: i64,
    pub SyncRootFileId: i64,
    pub FileIdentityLength: u32,
    pub FileIdentity: [u8; 1],
}
impl Default for CF_PLACEHOLDER_STANDARD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_PLACEHOLDER_STATE(pub u32);
impl CF_PLACEHOLDER_STATE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_PLACEHOLDER_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_PLACEHOLDER_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_PLACEHOLDER_STATE_ESSENTIAL_PROP_PRESENT: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(4u32);
pub const CF_PLACEHOLDER_STATE_INVALID: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(4294967295u32);
pub const CF_PLACEHOLDER_STATE_IN_SYNC: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(8u32);
pub const CF_PLACEHOLDER_STATE_NO_STATES: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(0u32);
pub const CF_PLACEHOLDER_STATE_PARTIAL: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(16u32);
pub const CF_PLACEHOLDER_STATE_PARTIALLY_ON_DISK: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(32u32);
pub const CF_PLACEHOLDER_STATE_PLACEHOLDER: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(1u32);
pub const CF_PLACEHOLDER_STATE_SYNC_ROOT: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_PLATFORM_INFO {
    pub BuildNumber: u32,
    pub RevisionNumber: u32,
    pub IntegrationNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_POPULATION_POLICY {
    pub Primary: CF_POPULATION_POLICY_PRIMARY,
    pub Modifier: CF_POPULATION_POLICY_MODIFIER,
}
pub const CF_POPULATION_POLICY_ALWAYS_FULL: CF_POPULATION_POLICY_PRIMARY = CF_POPULATION_POLICY_PRIMARY(3u16);
pub const CF_POPULATION_POLICY_FULL: CF_POPULATION_POLICY_PRIMARY = CF_POPULATION_POLICY_PRIMARY(2u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_POPULATION_POLICY_MODIFIER(pub u16);
impl CF_POPULATION_POLICY_MODIFIER {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_POPULATION_POLICY_MODIFIER {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_POPULATION_POLICY_MODIFIER {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_POPULATION_POLICY_MODIFIER_NONE: CF_POPULATION_POLICY_MODIFIER = CF_POPULATION_POLICY_MODIFIER(0u16);
pub const CF_POPULATION_POLICY_PARTIAL: CF_POPULATION_POLICY_PRIMARY = CF_POPULATION_POLICY_PRIMARY(0u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_POPULATION_POLICY_PRIMARY(pub u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_PROCESS_INFO {
    pub StructSize: u32,
    pub ProcessId: u32,
    pub ImagePath: windows_core::PCWSTR,
    pub PackageName: windows_core::PCWSTR,
    pub ApplicationId: windows_core::PCWSTR,
    pub CommandLine: windows_core::PCWSTR,
    pub SessionId: u32,
}
pub const CF_PROVIDER_STATUS_CLEAR_FLAGS: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(2147483648u32);
pub const CF_PROVIDER_STATUS_CONNECTIVITY_LOST: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(64u32);
pub const CF_PROVIDER_STATUS_DISCONNECTED: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(0u32);
pub const CF_PROVIDER_STATUS_ERROR: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(3221225474u32);
pub const CF_PROVIDER_STATUS_IDLE: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(1u32);
pub const CF_PROVIDER_STATUS_POPULATE_CONTENT: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(8u32);
pub const CF_PROVIDER_STATUS_POPULATE_METADATA: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(4u32);
pub const CF_PROVIDER_STATUS_POPULATE_NAMESPACE: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(2u32);
pub const CF_PROVIDER_STATUS_SYNC_FULL: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(32u32);
pub const CF_PROVIDER_STATUS_SYNC_INCREMENTAL: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(16u32);
pub const CF_PROVIDER_STATUS_TERMINATED: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(3221225473u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_REGISTER_FLAGS(pub i32);
impl CF_REGISTER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_REGISTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_REGISTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_REGISTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_REGISTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_REGISTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_REGISTER_FLAG_DISABLE_ON_DEMAND_POPULATION_ON_ROOT: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(2i32);
pub const CF_REGISTER_FLAG_MARK_IN_SYNC_ON_ROOT: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(4i32);
pub const CF_REGISTER_FLAG_NONE: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(0i32);
pub const CF_REGISTER_FLAG_UPDATE: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(1i32);
pub const CF_REQUEST_KEY_DEFAULT: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_REVERT_FLAGS(pub i32);
impl CF_REVERT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_REVERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_REVERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_REVERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_REVERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_REVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_REVERT_FLAG_NONE: CF_REVERT_FLAGS = CF_REVERT_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_SET_IN_SYNC_FLAGS(pub i32);
impl CF_SET_IN_SYNC_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_SET_IN_SYNC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_SET_IN_SYNC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_SET_IN_SYNC_FLAG_NONE: CF_SET_IN_SYNC_FLAGS = CF_SET_IN_SYNC_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_SET_PIN_FLAGS(pub i32);
impl CF_SET_PIN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_SET_PIN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_SET_PIN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_SET_PIN_FLAG_NONE: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(0i32);
pub const CF_SET_PIN_FLAG_RECURSE: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(1i32);
pub const CF_SET_PIN_FLAG_RECURSE_ONLY: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(2i32);
pub const CF_SET_PIN_FLAG_RECURSE_STOP_ON_ERROR: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_SYNC_POLICIES {
    pub StructSize: u32,
    pub Hydration: CF_HYDRATION_POLICY,
    pub Population: CF_POPULATION_POLICY,
    pub InSync: CF_INSYNC_POLICY,
    pub HardLink: CF_HARDLINK_POLICY,
    pub PlaceholderManagement: CF_PLACEHOLDER_MANAGEMENT_POLICY,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_SYNC_PROVIDER_STATUS(pub u32);
impl CF_SYNC_PROVIDER_STATUS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_SYNC_PROVIDER_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_SYNC_PROVIDER_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_SYNC_REGISTRATION {
    pub StructSize: u32,
    pub ProviderName: windows_core::PCWSTR,
    pub ProviderVersion: windows_core::PCWSTR,
    pub SyncRootIdentity: *const core::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileIdentity: *const core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub ProviderId: windows_core::GUID,
}
impl Default for CF_SYNC_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_SYNC_ROOT_BASIC_INFO {
    pub SyncRootFileId: i64,
}
pub const CF_SYNC_ROOT_INFO_BASIC: CF_SYNC_ROOT_INFO_CLASS = CF_SYNC_ROOT_INFO_CLASS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_SYNC_ROOT_INFO_CLASS(pub i32);
pub const CF_SYNC_ROOT_INFO_PROVIDER: CF_SYNC_ROOT_INFO_CLASS = CF_SYNC_ROOT_INFO_CLASS(2i32);
pub const CF_SYNC_ROOT_INFO_STANDARD: CF_SYNC_ROOT_INFO_CLASS = CF_SYNC_ROOT_INFO_CLASS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_SYNC_ROOT_PROVIDER_INFO {
    pub ProviderStatus: CF_SYNC_PROVIDER_STATUS,
    pub ProviderName: [u16; 256],
    pub ProviderVersion: [u16; 256],
}
impl Default for CF_SYNC_ROOT_PROVIDER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CF_SYNC_ROOT_STANDARD_INFO {
    pub SyncRootFileId: i64,
    pub HydrationPolicy: CF_HYDRATION_POLICY,
    pub PopulationPolicy: CF_POPULATION_POLICY,
    pub InSyncPolicy: CF_INSYNC_POLICY,
    pub HardLinkPolicy: CF_HARDLINK_POLICY,
    pub ProviderStatus: CF_SYNC_PROVIDER_STATUS,
    pub ProviderName: [u16; 256],
    pub ProviderVersion: [u16; 256],
    pub SyncRootIdentityLength: u32,
    pub SyncRootIdentity: [u8; 1],
}
impl Default for CF_SYNC_ROOT_STANDARD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CF_SYNC_STATUS {
    pub StructSize: u32,
    pub Code: u32,
    pub DescriptionOffset: u32,
    pub DescriptionLength: u32,
    pub DeviceIdOffset: u32,
    pub DeviceIdLength: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_UPDATE_FLAGS(pub i32);
impl CF_UPDATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CF_UPDATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CF_UPDATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CF_UPDATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CF_UPDATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CF_UPDATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_UPDATE_FLAG_ALLOW_PARTIAL: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(1024i32);
pub const CF_UPDATE_FLAG_ALWAYS_FULL: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(512i32);
pub const CF_UPDATE_FLAG_CLEAR_IN_SYNC: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(64i32);
pub const CF_UPDATE_FLAG_DEHYDRATE: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(4i32);
pub const CF_UPDATE_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(16i32);
pub const CF_UPDATE_FLAG_ENABLE_ON_DEMAND_POPULATION: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(8i32);
pub const CF_UPDATE_FLAG_MARK_IN_SYNC: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(2i32);
pub const CF_UPDATE_FLAG_NONE: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(0i32);
pub const CF_UPDATE_FLAG_PASSTHROUGH_FS_METADATA: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(256i32);
pub const CF_UPDATE_FLAG_REMOVE_FILE_IDENTITY: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(32i32);
pub const CF_UPDATE_FLAG_REMOVE_PROPERTY: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(128i32);
pub const CF_UPDATE_FLAG_VERIFY_IN_SYNC: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(1i32);
