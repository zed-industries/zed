//! Shell Protocol
//!
//! Provides shell services to UEFI applications.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x6302d008,
    0x7f9b,
    0x4f30,
    0x87,
    0xac,
    &[0x60, 0xc9, 0xfe, 0xf5, 0xda, 0x4e],
);

pub const MAJOR_VERSION: u32 = 0x00000002;
pub const MINOR_VERSION: u32 = 0x00000002;

pub type FileHandle = *mut core::ffi::c_void;

pub type DeviceNameFlags = u32;
pub const DEVICE_NAME_USE_COMPONENT_NAME: DeviceNameFlags = 0x00000001;
pub const DEVICE_NAME_USE_DEVICE_PATH: DeviceNameFlags = 0x00000002;

#[repr(C)]
pub struct ListEntry {
    pub flink: *mut ListEntry,
    pub blink: *mut ListEntry,
}

#[repr(C)]
pub struct FileInfo {
    pub link: ListEntry,
    pub status: crate::base::Status,
    pub full_name: *mut crate::base::Char16,
    pub file_name: *mut crate::base::Char16,
    pub handle: FileHandle,
    pub info: *mut crate::protocols::file::Info,
}

pub type Execute = eficall! {fn(
    *mut crate::base::Handle,
    *mut crate::base::Char16,
    *mut *mut crate::base::Char16,
    *mut crate::base::Status,
) -> crate::base::Status};

pub type GetEnv = eficall! {fn(
    *mut crate::base::Char16,
) -> *mut crate::base::Char16};

pub type SetEnv = eficall! {fn(
    *mut crate::base::Char16,
    *mut crate::base::Char16,
    crate::base::Boolean,
) -> crate::base::Status};

pub type GetAlias = eficall! {fn(
    *mut crate::base::Char16,
    *mut crate::base::Boolean,
) -> *mut crate::base::Char16};

pub type SetAlias = eficall! {fn(
    *mut crate::base::Char16,
    *mut crate::base::Char16,
    crate::base::Boolean,
    crate::base::Boolean,
) -> crate::base::Status};

pub type GetHelpText = eficall! {fn(
    *mut crate::base::Char16,
    *mut crate::base::Char16,
    *mut *mut crate::base::Char16,
) -> crate::base::Status};

pub type GetDevicePathFromMap = eficall! {fn(
    *mut crate::base::Char16,
) -> *mut crate::protocols::device_path::Protocol};

pub type GetMapFromDevicePath = eficall! {fn(
    *mut *mut crate::protocols::device_path::Protocol,
) -> *mut crate::base::Char16};

pub type GetDevicePathFromFilePath = eficall! {fn(
    *mut crate::base::Char16,
) -> *mut crate::protocols::device_path::Protocol};

pub type GetFilePathFromDevicePath = eficall! {fn(
    *mut crate::protocols::device_path::Protocol,
) -> *mut crate::base::Char16};

pub type SetMap = eficall! {fn(
    *mut crate::protocols::device_path::Protocol,
    *mut crate::base::Char16,
) -> crate::base::Status};

pub type GetCurDir = eficall! {fn(
    *mut crate::base::Char16,
) -> *mut crate::base::Char16};

pub type SetCurDir = eficall! {fn(
    *mut crate::base::Char16,
    *mut crate::base::Char16,
) -> crate::base::Status};

pub type OpenFileList = eficall! {fn(
    *mut crate::base::Char16,
    u64,
    *mut *mut FileInfo,
) -> crate::base::Status};

pub type FreeFileList = eficall! {fn(
    *mut *mut FileInfo,
) -> crate::base::Status};

pub type RemoveDupInFileList = eficall! {fn(
    *mut *mut FileInfo,
) -> crate::base::Status};

pub type BatchIsActive = eficall! {fn() -> crate::base::Boolean};

pub type IsRootShell = eficall! {fn() -> crate::base::Boolean};

pub type EnablePageBreak = eficall! {fn()};

pub type DisablePageBreak = eficall! {fn()};

pub type GetPageBreak = eficall! {fn() -> crate::base::Boolean};

pub type GetDeviceName = eficall! {fn(
    crate::base::Handle,
    DeviceNameFlags,
    *mut crate::base::Char8,
    *mut *mut crate::base::Char16,
) -> crate::base::Status};

pub type GetFileInfo = eficall! {fn(
    FileHandle,
) -> *mut crate::protocols::file::Info};

pub type SetFileInfo = eficall! {fn(
    FileHandle,
    *mut crate::protocols::file::Info
) -> crate::base::Status};

pub type OpenFileByName = eficall! {fn(
    *mut crate::base::Char16,
    *mut FileHandle,
    u64,
) -> crate::base::Status};

pub type CloseFile = eficall! {fn(
    FileHandle,
) -> crate::base::Status};

pub type CreateFile = eficall! {fn(
    *mut crate::base::Char16,
    u64,
    *mut FileHandle,
) -> crate::base::Status};

pub type ReadFile = eficall! {fn(
    FileHandle,
    *mut usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type WriteFile = eficall! {fn(
    FileHandle,
    *mut usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type DeleteFile = eficall! {fn(
    FileHandle,
) -> crate::base::Status};

pub type DeleteFileByName = eficall! {fn(
    *mut crate::base::Char16,
) -> crate::base::Status};

pub type GetFilePosition = eficall! {fn(
    FileHandle,
    *mut u64,
) -> crate::base::Status};

pub type SetFilePosition = eficall! {fn(
    FileHandle,
    u64,
) -> crate::base::Status};

pub type FlushFile = eficall! {fn(
    FileHandle,
) -> crate::base::Status};

pub type FindFiles = eficall! {fn(
    *mut crate::base::Char16,
    *mut *mut FileInfo,
) -> crate::base::Status};

pub type FindFilesInDir = eficall! {fn(
    FileHandle,
    *mut *mut FileInfo,
) -> crate::base::Status};

pub type GetFileSize = eficall! {fn(
    FileHandle,
    *mut u64,
) -> crate::base::Status};

pub type OpenRoot = eficall! {fn(
    *mut crate::protocols::device_path::Protocol,
    *mut FileHandle,
) -> crate::base::Status};

pub type OpenRootByHandle = eficall! {fn(
    crate::base::Handle,
    *mut FileHandle,
) -> crate::base::Status};

pub type RegisterGuidName = eficall! {fn(
    *mut crate::base::Guid,
    *mut crate::base::Char16,
) -> crate::base::Status};

pub type GetGuidName = eficall! {fn(
    *mut crate::base::Guid,
    *mut *mut crate::base::Char16,
) -> crate::base::Status};

pub type GetGuidFromName = eficall! {fn(
    *mut crate::base::Char16,
    *mut crate::base::Guid,
) -> crate::base::Status};

pub type GetEnvEx = eficall! {fn(
    *mut crate::base::Char16,
    *mut u32,
) -> *mut crate::base::Char16};

#[repr(C)]
pub struct Protocol {
    pub execute: Execute,
    pub get_env: GetEnv,
    pub set_env: SetEnv,
    pub get_alias: GetAlias,
    pub set_alias: SetAlias,
    pub get_help_text: GetHelpText,
    pub get_device_path_from_map: GetDevicePathFromMap,
    pub get_map_from_device_path: GetMapFromDevicePath,
    pub get_device_path_from_file_path: GetDevicePathFromFilePath,
    pub get_file_path_from_device_path: GetFilePathFromDevicePath,
    pub set_map: SetMap,

    pub get_cur_dir: GetCurDir,
    pub set_cur_dir: SetCurDir,
    pub open_file_list: OpenFileList,
    pub free_file_list: FreeFileList,
    pub remove_dup_in_file_list: RemoveDupInFileList,

    pub batch_is_active: BatchIsActive,
    pub is_root_shell: IsRootShell,
    pub enable_page_break: EnablePageBreak,
    pub disable_page_break: DisablePageBreak,
    pub get_page_break: GetPageBreak,
    pub get_device_name: GetDeviceName,

    pub get_file_info: GetFileInfo,
    pub set_file_info: SetFileInfo,
    pub open_file_by_name: OpenFileByName,
    pub close_file: CloseFile,
    pub create_file: CreateFile,
    pub read_file: ReadFile,
    pub write_file: WriteFile,
    pub delete_file: DeleteFile,
    pub delete_file_by_name: DeleteFileByName,
    pub get_file_position: GetFilePosition,
    pub set_file_position: SetFilePosition,
    pub flush_file: FlushFile,
    pub find_files: FindFiles,
    pub find_files_in_dir: FindFilesInDir,
    pub get_file_size: GetFileSize,

    pub open_root: OpenRoot,
    pub open_root_by_handle: OpenRootByHandle,

    pub execution_break: crate::base::Event,

    pub major_version: u32,
    pub minor_version: u32,
    pub register_guid_name: RegisterGuidName,
    pub get_guid_name: GetGuidName,
    pub get_guid_from_name: GetGuidFromName,

    // Shell 2.1
    pub get_env_ex: GetEnvEx,
}
