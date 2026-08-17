#[inline]
pub unsafe fn PrjAllocateAlignedBuffer<P0>(namespacevirtualizationcontext: P0, size: usize) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjAllocateAlignedBuffer(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, size : usize) -> *mut core::ffi::c_void);
    PrjAllocateAlignedBuffer(namespacevirtualizationcontext.param().abi(), size)
}
#[inline]
pub unsafe fn PrjClearNegativePathCache<P0>(namespacevirtualizationcontext: P0, totalentrynumber: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjClearNegativePathCache(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, totalentrynumber : *mut u32) -> windows_core::HRESULT);
    PrjClearNegativePathCache(namespacevirtualizationcontext.param().abi(), core::mem::transmute(totalentrynumber.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn PrjCompleteCommand<P0>(namespacevirtualizationcontext: P0, commandid: i32, completionresult: windows_core::HRESULT, extendedparameters: Option<*const PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjCompleteCommand(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, commandid : i32, completionresult : windows_core::HRESULT, extendedparameters : *const PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS) -> windows_core::HRESULT);
    PrjCompleteCommand(namespacevirtualizationcontext.param().abi(), commandid, completionresult, core::mem::transmute(extendedparameters.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn PrjDeleteFile<P0, P1>(namespacevirtualizationcontext: P0, destinationfilename: P1, updateflags: PRJ_UPDATE_TYPES, failurereason: Option<*mut PRJ_UPDATE_FAILURE_CAUSES>) -> windows_core::Result<()>
where
    P0: windows_core::Param<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjDeleteFile(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename : windows_core::PCWSTR, updateflags : PRJ_UPDATE_TYPES, failurereason : *mut PRJ_UPDATE_FAILURE_CAUSES) -> windows_core::HRESULT);
    PrjDeleteFile(namespacevirtualizationcontext.param().abi(), destinationfilename.param().abi(), updateflags, core::mem::transmute(failurereason.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn PrjDoesNameContainWildCards<P0>(filename: P0) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjDoesNameContainWildCards(filename : windows_core::PCWSTR) -> super::super::Foundation:: BOOLEAN);
    PrjDoesNameContainWildCards(filename.param().abi())
}
#[inline]
pub unsafe fn PrjFileNameCompare<P0, P1>(filename1: P0, filename2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjFileNameCompare(filename1 : windows_core::PCWSTR, filename2 : windows_core::PCWSTR) -> i32);
    PrjFileNameCompare(filename1.param().abi(), filename2.param().abi())
}
#[inline]
pub unsafe fn PrjFileNameMatch<P0, P1>(filenametocheck: P0, pattern: P1) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjFileNameMatch(filenametocheck : windows_core::PCWSTR, pattern : windows_core::PCWSTR) -> super::super::Foundation:: BOOLEAN);
    PrjFileNameMatch(filenametocheck.param().abi(), pattern.param().abi())
}
#[inline]
pub unsafe fn PrjFillDirEntryBuffer<P0, P1>(filename: P0, filebasicinfo: Option<*const PRJ_FILE_BASIC_INFO>, direntrybufferhandle: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<PRJ_DIR_ENTRY_BUFFER_HANDLE>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjFillDirEntryBuffer(filename : windows_core::PCWSTR, filebasicinfo : *const PRJ_FILE_BASIC_INFO, direntrybufferhandle : PRJ_DIR_ENTRY_BUFFER_HANDLE) -> windows_core::HRESULT);
    PrjFillDirEntryBuffer(filename.param().abi(), core::mem::transmute(filebasicinfo.unwrap_or(std::ptr::null())), direntrybufferhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn PrjFillDirEntryBuffer2<P0, P1>(direntrybufferhandle: P0, filename: P1, filebasicinfo: Option<*const PRJ_FILE_BASIC_INFO>, extendedinfo: Option<*const PRJ_EXTENDED_INFO>) -> windows_core::Result<()>
where
    P0: windows_core::Param<PRJ_DIR_ENTRY_BUFFER_HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjFillDirEntryBuffer2(direntrybufferhandle : PRJ_DIR_ENTRY_BUFFER_HANDLE, filename : windows_core::PCWSTR, filebasicinfo : *const PRJ_FILE_BASIC_INFO, extendedinfo : *const PRJ_EXTENDED_INFO) -> windows_core::HRESULT);
    PrjFillDirEntryBuffer2(direntrybufferhandle.param().abi(), filename.param().abi(), core::mem::transmute(filebasicinfo.unwrap_or(std::ptr::null())), core::mem::transmute(extendedinfo.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn PrjFreeAlignedBuffer(buffer: *const core::ffi::c_void) {
    windows_targets::link!("projectedfslib.dll" "system" fn PrjFreeAlignedBuffer(buffer : *const core::ffi::c_void));
    PrjFreeAlignedBuffer(buffer)
}
#[inline]
pub unsafe fn PrjGetOnDiskFileState<P0>(destinationfilename: P0) -> windows_core::Result<PRJ_FILE_STATE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjGetOnDiskFileState(destinationfilename : windows_core::PCWSTR, filestate : *mut PRJ_FILE_STATE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PrjGetOnDiskFileState(destinationfilename.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PrjGetVirtualizationInstanceInfo<P0>(namespacevirtualizationcontext: P0, virtualizationinstanceinfo: *mut PRJ_VIRTUALIZATION_INSTANCE_INFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjGetVirtualizationInstanceInfo(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, virtualizationinstanceinfo : *mut PRJ_VIRTUALIZATION_INSTANCE_INFO) -> windows_core::HRESULT);
    PrjGetVirtualizationInstanceInfo(namespacevirtualizationcontext.param().abi(), virtualizationinstanceinfo).ok()
}
#[inline]
pub unsafe fn PrjMarkDirectoryAsPlaceholder<P0, P1>(rootpathname: P0, targetpathname: P1, versioninfo: Option<*const PRJ_PLACEHOLDER_VERSION_INFO>, virtualizationinstanceid: *const windows_core::GUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjMarkDirectoryAsPlaceholder(rootpathname : windows_core::PCWSTR, targetpathname : windows_core::PCWSTR, versioninfo : *const PRJ_PLACEHOLDER_VERSION_INFO, virtualizationinstanceid : *const windows_core::GUID) -> windows_core::HRESULT);
    PrjMarkDirectoryAsPlaceholder(rootpathname.param().abi(), targetpathname.param().abi(), core::mem::transmute(versioninfo.unwrap_or(std::ptr::null())), virtualizationinstanceid).ok()
}
#[inline]
pub unsafe fn PrjStartVirtualizing<P0>(virtualizationrootpath: P0, callbacks: *const PRJ_CALLBACKS, instancecontext: Option<*const core::ffi::c_void>, options: Option<*const PRJ_STARTVIRTUALIZING_OPTIONS>) -> windows_core::Result<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjStartVirtualizing(virtualizationrootpath : windows_core::PCWSTR, callbacks : *const PRJ_CALLBACKS, instancecontext : *const core::ffi::c_void, options : *const PRJ_STARTVIRTUALIZING_OPTIONS, namespacevirtualizationcontext : *mut PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PrjStartVirtualizing(virtualizationrootpath.param().abi(), callbacks, core::mem::transmute(instancecontext.unwrap_or(std::ptr::null())), core::mem::transmute(options.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PrjStopVirtualizing<P0>(namespacevirtualizationcontext: P0)
where
    P0: windows_core::Param<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjStopVirtualizing(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT));
    PrjStopVirtualizing(namespacevirtualizationcontext.param().abi())
}
#[inline]
pub unsafe fn PrjUpdateFileIfNeeded<P0, P1>(namespacevirtualizationcontext: P0, destinationfilename: P1, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32, updateflags: PRJ_UPDATE_TYPES, failurereason: Option<*mut PRJ_UPDATE_FAILURE_CAUSES>) -> windows_core::Result<()>
where
    P0: windows_core::Param<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjUpdateFileIfNeeded(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename : windows_core::PCWSTR, placeholderinfo : *const PRJ_PLACEHOLDER_INFO, placeholderinfosize : u32, updateflags : PRJ_UPDATE_TYPES, failurereason : *mut PRJ_UPDATE_FAILURE_CAUSES) -> windows_core::HRESULT);
    PrjUpdateFileIfNeeded(namespacevirtualizationcontext.param().abi(), destinationfilename.param().abi(), placeholderinfo, placeholderinfosize, updateflags, core::mem::transmute(failurereason.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn PrjWriteFileData<P0>(namespacevirtualizationcontext: P0, datastreamid: *const windows_core::GUID, buffer: *const core::ffi::c_void, byteoffset: u64, length: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjWriteFileData(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, datastreamid : *const windows_core::GUID, buffer : *const core::ffi::c_void, byteoffset : u64, length : u32) -> windows_core::HRESULT);
    PrjWriteFileData(namespacevirtualizationcontext.param().abi(), datastreamid, buffer, byteoffset, length).ok()
}
#[inline]
pub unsafe fn PrjWritePlaceholderInfo<P0, P1>(namespacevirtualizationcontext: P0, destinationfilename: P1, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjWritePlaceholderInfo(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename : windows_core::PCWSTR, placeholderinfo : *const PRJ_PLACEHOLDER_INFO, placeholderinfosize : u32) -> windows_core::HRESULT);
    PrjWritePlaceholderInfo(namespacevirtualizationcontext.param().abi(), destinationfilename.param().abi(), placeholderinfo, placeholderinfosize).ok()
}
#[inline]
pub unsafe fn PrjWritePlaceholderInfo2<P0, P1>(namespacevirtualizationcontext: P0, destinationfilename: P1, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32, extendedinfo: Option<*const PRJ_EXTENDED_INFO>) -> windows_core::Result<()>
where
    P0: windows_core::Param<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("projectedfslib.dll" "system" fn PrjWritePlaceholderInfo2(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename : windows_core::PCWSTR, placeholderinfo : *const PRJ_PLACEHOLDER_INFO, placeholderinfosize : u32, extendedinfo : *const PRJ_EXTENDED_INFO) -> windows_core::HRESULT);
    PrjWritePlaceholderInfo2(namespacevirtualizationcontext.param().abi(), destinationfilename.param().abi(), placeholderinfo, placeholderinfosize, core::mem::transmute(extendedinfo.unwrap_or(std::ptr::null()))).ok()
}
pub const PRJ_CB_DATA_FLAG_ENUM_RESTART_SCAN: PRJ_CALLBACK_DATA_FLAGS = PRJ_CALLBACK_DATA_FLAGS(1i32);
pub const PRJ_CB_DATA_FLAG_ENUM_RETURN_SINGLE_ENTRY: PRJ_CALLBACK_DATA_FLAGS = PRJ_CALLBACK_DATA_FLAGS(2i32);
pub const PRJ_COMPLETE_COMMAND_TYPE_ENUMERATION: PRJ_COMPLETE_COMMAND_TYPE = PRJ_COMPLETE_COMMAND_TYPE(2i32);
pub const PRJ_COMPLETE_COMMAND_TYPE_NOTIFICATION: PRJ_COMPLETE_COMMAND_TYPE = PRJ_COMPLETE_COMMAND_TYPE(1i32);
pub const PRJ_EXT_INFO_TYPE_SYMLINK: PRJ_EXT_INFO_TYPE = PRJ_EXT_INFO_TYPE(1i32);
pub const PRJ_FILE_STATE_DIRTY_PLACEHOLDER: PRJ_FILE_STATE = PRJ_FILE_STATE(4i32);
pub const PRJ_FILE_STATE_FULL: PRJ_FILE_STATE = PRJ_FILE_STATE(8i32);
pub const PRJ_FILE_STATE_HYDRATED_PLACEHOLDER: PRJ_FILE_STATE = PRJ_FILE_STATE(2i32);
pub const PRJ_FILE_STATE_PLACEHOLDER: PRJ_FILE_STATE = PRJ_FILE_STATE(1i32);
pub const PRJ_FILE_STATE_TOMBSTONE: PRJ_FILE_STATE = PRJ_FILE_STATE(16i32);
pub const PRJ_FLAG_NONE: PRJ_STARTVIRTUALIZING_FLAGS = PRJ_STARTVIRTUALIZING_FLAGS(0i32);
pub const PRJ_FLAG_USE_NEGATIVE_PATH_CACHE: PRJ_STARTVIRTUALIZING_FLAGS = PRJ_STARTVIRTUALIZING_FLAGS(1i32);
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_DELETED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(2048i32);
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_MODIFIED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(1024i32);
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_NO_MODIFICATION: PRJ_NOTIFICATION = PRJ_NOTIFICATION(512i32);
pub const PRJ_NOTIFICATION_FILE_OPENED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(2i32);
pub const PRJ_NOTIFICATION_FILE_OVERWRITTEN: PRJ_NOTIFICATION = PRJ_NOTIFICATION(8i32);
pub const PRJ_NOTIFICATION_FILE_PRE_CONVERT_TO_FULL: PRJ_NOTIFICATION = PRJ_NOTIFICATION(4096i32);
pub const PRJ_NOTIFICATION_FILE_RENAMED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(128i32);
pub const PRJ_NOTIFICATION_HARDLINK_CREATED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(256i32);
pub const PRJ_NOTIFICATION_NEW_FILE_CREATED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(4i32);
pub const PRJ_NOTIFICATION_PRE_DELETE: PRJ_NOTIFICATION = PRJ_NOTIFICATION(16i32);
pub const PRJ_NOTIFICATION_PRE_RENAME: PRJ_NOTIFICATION = PRJ_NOTIFICATION(32i32);
pub const PRJ_NOTIFICATION_PRE_SET_HARDLINK: PRJ_NOTIFICATION = PRJ_NOTIFICATION(64i32);
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_DELETED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(2048u32);
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_MODIFIED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(1024u32);
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_NO_MODIFICATION: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(512u32);
pub const PRJ_NOTIFY_FILE_OPENED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(2u32);
pub const PRJ_NOTIFY_FILE_OVERWRITTEN: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(8u32);
pub const PRJ_NOTIFY_FILE_PRE_CONVERT_TO_FULL: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(4096u32);
pub const PRJ_NOTIFY_FILE_RENAMED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(128u32);
pub const PRJ_NOTIFY_HARDLINK_CREATED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(256u32);
pub const PRJ_NOTIFY_NEW_FILE_CREATED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(4u32);
pub const PRJ_NOTIFY_NONE: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(0u32);
pub const PRJ_NOTIFY_PRE_DELETE: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(16u32);
pub const PRJ_NOTIFY_PRE_RENAME: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(32u32);
pub const PRJ_NOTIFY_PRE_SET_HARDLINK: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(64u32);
pub const PRJ_NOTIFY_SUPPRESS_NOTIFICATIONS: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(1u32);
pub const PRJ_NOTIFY_USE_EXISTING_MASK: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(4294967295u32);
pub const PRJ_PLACEHOLDER_ID_LENGTH: PRJ_PLACEHOLDER_ID = PRJ_PLACEHOLDER_ID(128i32);
pub const PRJ_UPDATE_ALLOW_DIRTY_DATA: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(2i32);
pub const PRJ_UPDATE_ALLOW_DIRTY_METADATA: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(1i32);
pub const PRJ_UPDATE_ALLOW_READ_ONLY: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(32i32);
pub const PRJ_UPDATE_ALLOW_TOMBSTONE: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(4i32);
pub const PRJ_UPDATE_FAILURE_CAUSE_DIRTY_DATA: PRJ_UPDATE_FAILURE_CAUSES = PRJ_UPDATE_FAILURE_CAUSES(2i32);
pub const PRJ_UPDATE_FAILURE_CAUSE_DIRTY_METADATA: PRJ_UPDATE_FAILURE_CAUSES = PRJ_UPDATE_FAILURE_CAUSES(1i32);
pub const PRJ_UPDATE_FAILURE_CAUSE_NONE: PRJ_UPDATE_FAILURE_CAUSES = PRJ_UPDATE_FAILURE_CAUSES(0i32);
pub const PRJ_UPDATE_FAILURE_CAUSE_READ_ONLY: PRJ_UPDATE_FAILURE_CAUSES = PRJ_UPDATE_FAILURE_CAUSES(8i32);
pub const PRJ_UPDATE_FAILURE_CAUSE_TOMBSTONE: PRJ_UPDATE_FAILURE_CAUSES = PRJ_UPDATE_FAILURE_CAUSES(4i32);
pub const PRJ_UPDATE_MAX_VAL: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(64i32);
pub const PRJ_UPDATE_NONE: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(0i32);
pub const PRJ_UPDATE_RESERVED1: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(8i32);
pub const PRJ_UPDATE_RESERVED2: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(16i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRJ_CALLBACK_DATA_FLAGS(pub i32);
impl windows_core::TypeKind for PRJ_CALLBACK_DATA_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRJ_CALLBACK_DATA_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRJ_CALLBACK_DATA_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRJ_COMPLETE_COMMAND_TYPE(pub i32);
impl windows_core::TypeKind for PRJ_COMPLETE_COMMAND_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRJ_COMPLETE_COMMAND_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRJ_COMPLETE_COMMAND_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRJ_EXT_INFO_TYPE(pub i32);
impl windows_core::TypeKind for PRJ_EXT_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRJ_EXT_INFO_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRJ_EXT_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRJ_FILE_STATE(pub i32);
impl windows_core::TypeKind for PRJ_FILE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRJ_FILE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRJ_FILE_STATE").field(&self.0).finish()
    }
}
impl PRJ_FILE_STATE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PRJ_FILE_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PRJ_FILE_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PRJ_FILE_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PRJ_FILE_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PRJ_FILE_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRJ_NOTIFICATION(pub i32);
impl windows_core::TypeKind for PRJ_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRJ_NOTIFICATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRJ_NOTIFICATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRJ_NOTIFY_TYPES(pub u32);
impl windows_core::TypeKind for PRJ_NOTIFY_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRJ_NOTIFY_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRJ_NOTIFY_TYPES").field(&self.0).finish()
    }
}
impl PRJ_NOTIFY_TYPES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PRJ_NOTIFY_TYPES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PRJ_NOTIFY_TYPES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PRJ_NOTIFY_TYPES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PRJ_NOTIFY_TYPES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PRJ_NOTIFY_TYPES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRJ_PLACEHOLDER_ID(pub i32);
impl windows_core::TypeKind for PRJ_PLACEHOLDER_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRJ_PLACEHOLDER_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRJ_PLACEHOLDER_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRJ_STARTVIRTUALIZING_FLAGS(pub i32);
impl windows_core::TypeKind for PRJ_STARTVIRTUALIZING_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRJ_STARTVIRTUALIZING_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRJ_STARTVIRTUALIZING_FLAGS").field(&self.0).finish()
    }
}
impl PRJ_STARTVIRTUALIZING_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PRJ_STARTVIRTUALIZING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PRJ_STARTVIRTUALIZING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PRJ_STARTVIRTUALIZING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PRJ_STARTVIRTUALIZING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PRJ_STARTVIRTUALIZING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRJ_UPDATE_FAILURE_CAUSES(pub i32);
impl windows_core::TypeKind for PRJ_UPDATE_FAILURE_CAUSES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRJ_UPDATE_FAILURE_CAUSES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRJ_UPDATE_FAILURE_CAUSES").field(&self.0).finish()
    }
}
impl PRJ_UPDATE_FAILURE_CAUSES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PRJ_UPDATE_FAILURE_CAUSES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PRJ_UPDATE_FAILURE_CAUSES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PRJ_UPDATE_FAILURE_CAUSES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PRJ_UPDATE_FAILURE_CAUSES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PRJ_UPDATE_FAILURE_CAUSES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRJ_UPDATE_TYPES(pub i32);
impl windows_core::TypeKind for PRJ_UPDATE_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRJ_UPDATE_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRJ_UPDATE_TYPES").field(&self.0).finish()
    }
}
impl PRJ_UPDATE_TYPES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PRJ_UPDATE_TYPES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PRJ_UPDATE_TYPES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PRJ_UPDATE_TYPES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PRJ_UPDATE_TYPES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PRJ_UPDATE_TYPES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PRJ_CALLBACKS {
    pub StartDirectoryEnumerationCallback: PRJ_START_DIRECTORY_ENUMERATION_CB,
    pub EndDirectoryEnumerationCallback: PRJ_END_DIRECTORY_ENUMERATION_CB,
    pub GetDirectoryEnumerationCallback: PRJ_GET_DIRECTORY_ENUMERATION_CB,
    pub GetPlaceholderInfoCallback: PRJ_GET_PLACEHOLDER_INFO_CB,
    pub GetFileDataCallback: PRJ_GET_FILE_DATA_CB,
    pub QueryFileNameCallback: PRJ_QUERY_FILE_NAME_CB,
    pub NotificationCallback: PRJ_NOTIFICATION_CB,
    pub CancelCommandCallback: PRJ_CANCEL_COMMAND_CB,
}
impl windows_core::TypeKind for PRJ_CALLBACKS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_CALLBACKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_CALLBACK_DATA {
    pub Size: u32,
    pub Flags: PRJ_CALLBACK_DATA_FLAGS,
    pub NamespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
    pub CommandId: i32,
    pub FileId: windows_core::GUID,
    pub DataStreamId: windows_core::GUID,
    pub FilePathName: windows_core::PCWSTR,
    pub VersionInfo: *mut PRJ_PLACEHOLDER_VERSION_INFO,
    pub TriggeringProcessId: u32,
    pub TriggeringProcessImageFileName: windows_core::PCWSTR,
    pub InstanceContext: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for PRJ_CALLBACK_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    pub CommandType: PRJ_COMPLETE_COMMAND_TYPE,
    pub Anonymous: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0,
}
impl windows_core::TypeKind for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    pub Notification: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1,
    pub Enumeration: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0,
}
impl windows_core::TypeKind for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    pub DirEntryBufferHandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
}
impl windows_core::TypeKind for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
impl windows_core::TypeKind for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRJ_DIR_ENTRY_BUFFER_HANDLE(pub isize);
impl PRJ_DIR_ENTRY_BUFFER_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for PRJ_DIR_ENTRY_BUFFER_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PRJ_DIR_ENTRY_BUFFER_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_EXTENDED_INFO {
    pub InfoType: PRJ_EXT_INFO_TYPE,
    pub NextInfoOffset: u32,
    pub Anonymous: PRJ_EXTENDED_INFO_0,
}
impl windows_core::TypeKind for PRJ_EXTENDED_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_EXTENDED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PRJ_EXTENDED_INFO_0 {
    pub Symlink: PRJ_EXTENDED_INFO_0_0,
}
impl windows_core::TypeKind for PRJ_EXTENDED_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_EXTENDED_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_EXTENDED_INFO_0_0 {
    pub TargetName: windows_core::PCWSTR,
}
impl windows_core::TypeKind for PRJ_EXTENDED_INFO_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_EXTENDED_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_FILE_BASIC_INFO {
    pub IsDirectory: super::super::Foundation::BOOLEAN,
    pub FileSize: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
impl windows_core::TypeKind for PRJ_FILE_BASIC_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_FILE_BASIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT(pub isize);
impl PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_NOTIFICATION_MAPPING {
    pub NotificationBitMask: PRJ_NOTIFY_TYPES,
    pub NotificationRoot: windows_core::PCWSTR,
}
impl windows_core::TypeKind for PRJ_NOTIFICATION_MAPPING {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_NOTIFICATION_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PRJ_NOTIFICATION_PARAMETERS {
    pub PostCreate: PRJ_NOTIFICATION_PARAMETERS_2,
    pub FileRenamed: PRJ_NOTIFICATION_PARAMETERS_1,
    pub FileDeletedOnHandleClose: PRJ_NOTIFICATION_PARAMETERS_0,
}
impl windows_core::TypeKind for PRJ_NOTIFICATION_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_NOTIFICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_NOTIFICATION_PARAMETERS_0 {
    pub IsFileModified: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for PRJ_NOTIFICATION_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_NOTIFICATION_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_NOTIFICATION_PARAMETERS_1 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
impl windows_core::TypeKind for PRJ_NOTIFICATION_PARAMETERS_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_NOTIFICATION_PARAMETERS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_NOTIFICATION_PARAMETERS_2 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
impl windows_core::TypeKind for PRJ_NOTIFICATION_PARAMETERS_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_NOTIFICATION_PARAMETERS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_PLACEHOLDER_INFO {
    pub FileBasicInfo: PRJ_FILE_BASIC_INFO,
    pub EaInformation: PRJ_PLACEHOLDER_INFO_0,
    pub SecurityInformation: PRJ_PLACEHOLDER_INFO_1,
    pub StreamsInformation: PRJ_PLACEHOLDER_INFO_2,
    pub VersionInfo: PRJ_PLACEHOLDER_VERSION_INFO,
    pub VariableData: [u8; 1],
}
impl windows_core::TypeKind for PRJ_PLACEHOLDER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_PLACEHOLDER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_PLACEHOLDER_INFO_0 {
    pub EaBufferSize: u32,
    pub OffsetToFirstEa: u32,
}
impl windows_core::TypeKind for PRJ_PLACEHOLDER_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_PLACEHOLDER_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_PLACEHOLDER_INFO_1 {
    pub SecurityBufferSize: u32,
    pub OffsetToSecurityDescriptor: u32,
}
impl windows_core::TypeKind for PRJ_PLACEHOLDER_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_PLACEHOLDER_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_PLACEHOLDER_INFO_2 {
    pub StreamsInfoBufferSize: u32,
    pub OffsetToFirstStreamInfo: u32,
}
impl windows_core::TypeKind for PRJ_PLACEHOLDER_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_PLACEHOLDER_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_PLACEHOLDER_VERSION_INFO {
    pub ProviderID: [u8; 128],
    pub ContentID: [u8; 128],
}
impl windows_core::TypeKind for PRJ_PLACEHOLDER_VERSION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_PLACEHOLDER_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_STARTVIRTUALIZING_OPTIONS {
    pub Flags: PRJ_STARTVIRTUALIZING_FLAGS,
    pub PoolThreadCount: u32,
    pub ConcurrentThreadCount: u32,
    pub NotificationMappings: *mut PRJ_NOTIFICATION_MAPPING,
    pub NotificationMappingsCount: u32,
}
impl windows_core::TypeKind for PRJ_STARTVIRTUALIZING_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_STARTVIRTUALIZING_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRJ_VIRTUALIZATION_INSTANCE_INFO {
    pub InstanceID: windows_core::GUID,
    pub WriteAlignment: u32,
}
impl windows_core::TypeKind for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRJ_CANCEL_COMMAND_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA)>;
pub type PRJ_END_DIRECTORY_ENUMERATION_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const windows_core::GUID) -> windows_core::HRESULT>;
pub type PRJ_GET_DIRECTORY_ENUMERATION_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const windows_core::GUID, searchexpression: windows_core::PCWSTR, direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE) -> windows_core::HRESULT>;
pub type PRJ_GET_FILE_DATA_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, byteoffset: u64, length: u32) -> windows_core::HRESULT>;
pub type PRJ_GET_PLACEHOLDER_INFO_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA) -> windows_core::HRESULT>;
pub type PRJ_NOTIFICATION_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, isdirectory: super::super::Foundation::BOOLEAN, notification: PRJ_NOTIFICATION, destinationfilename: windows_core::PCWSTR, operationparameters: *mut PRJ_NOTIFICATION_PARAMETERS) -> windows_core::HRESULT>;
pub type PRJ_QUERY_FILE_NAME_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA) -> windows_core::HRESULT>;
pub type PRJ_START_DIRECTORY_ENUMERATION_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const windows_core::GUID) -> windows_core::HRESULT>;
