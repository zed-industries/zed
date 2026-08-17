#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtDeviceIoControlFile<P0, P1>(filehandle: P0, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, iocontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtDeviceIoControlFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, iocontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtDeviceIoControlFile(filehandle.param().abi(), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, iocontrolcode, core::mem::transmute(inputbuffer.unwrap_or(std::ptr::null())), inputbufferlength, core::mem::transmute(outputbuffer.unwrap_or(std::ptr::null_mut())), outputbufferlength)
}
