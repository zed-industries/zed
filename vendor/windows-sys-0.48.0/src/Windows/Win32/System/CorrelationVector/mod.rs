::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"] fn RtlExtendCorrelationVector ( correlationvector : *mut CORRELATION_VECTOR ) -> u32 );
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"] fn RtlIncrementCorrelationVector ( correlationvector : *mut CORRELATION_VECTOR ) -> u32 );
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"] fn RtlInitializeCorrelationVector ( correlationvector : *mut CORRELATION_VECTOR , version : i32 , guid : *const ::windows_sys::core::GUID ) -> u32 );
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"] fn RtlValidateCorrelationVector ( vector : *const CORRELATION_VECTOR ) -> u32 );
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub const RTL_CORRELATION_VECTOR_STRING_LENGTH: u32 = 129u32;
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub const RTL_CORRELATION_VECTOR_V1_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub const RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub const RTL_CORRELATION_VECTOR_V2_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub const RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH: u32 = 22u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub struct CORRELATION_VECTOR {
    pub Version: u8,
    pub Vector: [u8; 129],
}
impl ::core::marker::Copy for CORRELATION_VECTOR {}
impl ::core::clone::Clone for CORRELATION_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
