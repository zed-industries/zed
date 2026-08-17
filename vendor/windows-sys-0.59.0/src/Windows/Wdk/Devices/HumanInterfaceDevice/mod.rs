windows_targets::link!("vhfum.dll" "system" fn VhfAsyncOperationComplete(vhfoperationhandle : *const core::ffi::c_void, completionstatus : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("vhfum.dll" "system" fn VhfCreate(vhfconfig : *const VHF_CONFIG, vhfhandle : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("vhfum.dll" "system" fn VhfDelete(vhfhandle : *const core::ffi::c_void, wait : super::super::super::Win32::Foundation:: BOOLEAN));
windows_targets::link!("vhfum.dll" "system" fn VhfReadReportSubmit(vhfhandle : *const core::ffi::c_void, hidtransferpacket : *const HID_XFER_PACKET) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("vhfum.dll" "system" fn VhfStart(vhfhandle : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HID_XFER_PACKET {
    pub reportBuffer: *mut u8,
    pub reportBufferLen: u32,
    pub reportId: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VHF_CONFIG {
    pub Size: u32,
    pub VhfClientContext: *mut core::ffi::c_void,
    pub OperationContextSize: u32,
    pub FileHandle: super::super::super::Win32::Foundation::HANDLE,
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
    pub ContainerID: windows_sys::core::GUID,
    pub InstanceIDLength: u16,
    pub InstanceID: windows_sys::core::PWSTR,
    pub ReportDescriptorLength: u16,
    pub ReportDescriptor: *mut u8,
    pub EvtVhfReadyForNextReadReport: EVT_VHF_READY_FOR_NEXT_READ_REPORT,
    pub EvtVhfAsyncOperationGetFeature: EVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationSetFeature: EVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationWriteReport: EVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationGetInputReport: EVT_VHF_ASYNC_OPERATION,
    pub EvtVhfCleanup: EVT_VHF_CLEANUP,
    pub HardwareIDsLength: u16,
    pub HardwareIDs: windows_sys::core::PWSTR,
}
pub type EVT_VHF_ASYNC_OPERATION = Option<unsafe extern "system" fn(vhfclientcontext: *const core::ffi::c_void, vhfoperationhandle: *const core::ffi::c_void, vhfoperationcontext: *const core::ffi::c_void, hidtransferpacket: *const HID_XFER_PACKET)>;
pub type EVT_VHF_CLEANUP = Option<unsafe extern "system" fn(vhfclientcontext: *const core::ffi::c_void)>;
pub type EVT_VHF_READY_FOR_NEXT_READ_REPORT = Option<unsafe extern "system" fn(vhfclientcontext: *const core::ffi::c_void)>;
