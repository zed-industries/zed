#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryCount: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const freshSoHRequest: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxConnectionCountPerEnforcer: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxEnforcerCount: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxNetworkSoHSize: u32 = 4000u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxPrivateDataSize: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxSoHAttributeCount: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxSoHAttributeSize: u32 = 4000u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxStringLength: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxSystemHealthEntityCount: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const minNetworkSoHSize: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const percentageNotSupported: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const shaFixup: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub type ExtendedIsolationState = i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const extendedIsolationStateNoData: ExtendedIsolationState = 0i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const extendedIsolationStateTransition: ExtendedIsolationState = 1i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const extendedIsolationStateInfected: ExtendedIsolationState = 2i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const extendedIsolationStateUnknown: ExtendedIsolationState = 3i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub type FailureCategory = i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryNone: FailureCategory = 0i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryOther: FailureCategory = 1i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryClientComponent: FailureCategory = 2i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryClientCommunication: FailureCategory = 3i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryServerComponent: FailureCategory = 4i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryServerCommunication: FailureCategory = 5i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub type FixupState = i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const fixupStateSuccess: FixupState = 0i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const fixupStateInProgress: FixupState = 1i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const fixupStateCouldNotUpdate: FixupState = 2i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub type IsolationState = i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const isolationStateNotRestricted: IsolationState = 1i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const isolationStateInProbation: IsolationState = 2i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const isolationStateRestrictedAccess: IsolationState = 3i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub type NapNotifyType = i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const napNotifyTypeUnknown: NapNotifyType = 0i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const napNotifyTypeServiceState: NapNotifyType = 1i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const napNotifyTypeQuarState: NapNotifyType = 2i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub type NapTracingLevel = i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const tracingLevelUndefined: NapTracingLevel = 0i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const tracingLevelBasic: NapTracingLevel = 1i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const tracingLevelAdvanced: NapTracingLevel = 2i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const tracingLevelDebug: NapTracingLevel = 3i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub type RemoteConfigurationType = i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const remoteConfigTypeMachine: RemoteConfigurationType = 1i32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const remoteConfigTypeConfigBlob: RemoteConfigurationType = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CorrelationId {
    pub connId: ::windows_sys::core::GUID,
    pub timeStamp: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CorrelationId {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CorrelationId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct CountedString {
    pub length: u16,
    pub string: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CountedString {}
impl ::core::clone::Clone for CountedString {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FailureCategoryMapping {
    pub mappingCompliance: [super::super::Foundation::BOOL; 5],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FailureCategoryMapping {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FailureCategoryMapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct FixupInfo {
    pub state: FixupState,
    pub percentage: u8,
    pub resultCodes: ResultCodes,
    pub fixupMsgId: u32,
}
impl ::core::marker::Copy for FixupInfo {}
impl ::core::clone::Clone for FixupInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct Ipv4Address {
    pub addr: [u8; 4],
}
impl ::core::marker::Copy for Ipv4Address {}
impl ::core::clone::Clone for Ipv4Address {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct Ipv6Address {
    pub addr: [u8; 16],
}
impl ::core::marker::Copy for Ipv6Address {}
impl ::core::clone::Clone for Ipv6Address {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolationInfo {
    pub isolationState: IsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IsolationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IsolationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolationInfoEx {
    pub isolationState: IsolationState,
    pub extendedIsolationState: ExtendedIsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IsolationInfoEx {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IsolationInfoEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NapComponentRegistrationInfo {
    pub id: u32,
    pub friendlyName: CountedString,
    pub description: CountedString,
    pub version: CountedString,
    pub vendorName: CountedString,
    pub infoClsid: ::windows_sys::core::GUID,
    pub configClsid: ::windows_sys::core::GUID,
    pub registrationDate: super::super::Foundation::FILETIME,
    pub componentType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NapComponentRegistrationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NapComponentRegistrationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct NetworkSoH {
    pub size: u16,
    pub data: *mut u8,
}
impl ::core::marker::Copy for NetworkSoH {}
impl ::core::clone::Clone for NetworkSoH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct PrivateData {
    pub size: u16,
    pub data: *mut u8,
}
impl ::core::marker::Copy for PrivateData {}
impl ::core::clone::Clone for PrivateData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct ResultCodes {
    pub count: u16,
    pub results: *mut ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for ResultCodes {}
impl ::core::clone::Clone for ResultCodes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct SoH {
    pub count: u16,
    pub attributes: *mut SoHAttribute,
}
impl ::core::marker::Copy for SoH {}
impl ::core::clone::Clone for SoH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct SoHAttribute {
    pub r#type: u16,
    pub size: u16,
    pub value: *mut u8,
}
impl ::core::marker::Copy for SoHAttribute {}
impl ::core::clone::Clone for SoHAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct SystemHealthAgentState {
    pub id: u32,
    pub shaResultCodes: ResultCodes,
    pub failureCategory: FailureCategory,
    pub fixupInfo: FixupInfo,
}
impl ::core::marker::Copy for SystemHealthAgentState {}
impl ::core::clone::Clone for SystemHealthAgentState {
    fn clone(&self) -> Self {
        *self
    }
}
