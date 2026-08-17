#[inline]
pub unsafe fn RtlExtendCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlExtendCorrelationVector(correlationvector : *mut CORRELATION_VECTOR) -> u32);
    unsafe { RtlExtendCorrelationVector(correlationvector as _) }
}
#[inline]
pub unsafe fn RtlIncrementCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlIncrementCorrelationVector(correlationvector : *mut CORRELATION_VECTOR) -> u32);
    unsafe { RtlIncrementCorrelationVector(correlationvector as _) }
}
#[inline]
pub unsafe fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: Option<*const windows_core::GUID>) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlInitializeCorrelationVector(correlationvector : *mut CORRELATION_VECTOR, version : i32, guid : *const windows_core::GUID) -> u32);
    unsafe { RtlInitializeCorrelationVector(correlationvector as _, version, guid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlValidateCorrelationVector(vector : *const CORRELATION_VECTOR) -> u32);
    unsafe { RtlValidateCorrelationVector(vector) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CORRELATION_VECTOR {
    pub Version: i8,
    pub Vector: [i8; 129],
}
impl Default for CORRELATION_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTL_CORRELATION_VECTOR_STRING_LENGTH: u32 = 129u32;
pub const RTL_CORRELATION_VECTOR_V1_LENGTH: u32 = 64u32;
pub const RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH: u32 = 16u32;
pub const RTL_CORRELATION_VECTOR_V2_LENGTH: u32 = 128u32;
pub const RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH: u32 = 22u32;
