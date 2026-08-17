pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CorrelationId {
    pub connId: windows_core::GUID,
    pub timeStamp: super::super::Foundation::FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CountedString {
    pub length: u16,
    pub string: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExtendedIsolationState(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FailureCategory(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FailureCategoryMapping {
    pub mappingCompliance: [windows_core::BOOL; 5],
}
impl Default for FailureCategoryMapping {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FixupInfo {
    pub state: FixupState,
    pub percentage: u8,
    pub resultCodes: ResultCodes,
    pub fixupMsgId: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FixupState(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ipv4Address {
    pub addr: [u8; 4],
}
impl Default for Ipv4Address {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ipv6Address {
    pub addr: [u8; 16],
}
impl Default for Ipv6Address {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IsolationInfo {
    pub isolationState: IsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IsolationInfoEx {
    pub isolationState: IsolationState,
    pub extendedIsolationState: ExtendedIsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IsolationState(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NapComponentRegistrationInfo {
    pub id: u32,
    pub friendlyName: CountedString,
    pub description: CountedString,
    pub version: CountedString,
    pub vendorName: CountedString,
    pub infoClsid: windows_core::GUID,
    pub configClsid: windows_core::GUID,
    pub registrationDate: super::super::Foundation::FILETIME,
    pub componentType: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NapNotifyType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NapTracingLevel(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NetworkSoH {
    pub size: u16,
    pub data: *mut u8,
}
impl Default for NetworkSoH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrivateData {
    pub size: u16,
    pub data: *mut u8,
}
impl Default for PrivateData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RemoteConfigurationType(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResultCodes {
    pub count: u16,
    pub results: *mut windows_core::HRESULT,
}
impl Default for ResultCodes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SoH {
    pub count: u16,
    pub attributes: *mut SoHAttribute,
}
impl Default for SoH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SoHAttribute {
    pub r#type: u16,
    pub size: u16,
    pub value: *mut u8,
}
impl Default for SoHAttribute {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SystemHealthAgentState {
    pub id: u32,
    pub shaResultCodes: ResultCodes,
    pub failureCategory: FailureCategory,
    pub fixupInfo: FixupInfo,
}
pub const extendedIsolationStateInfected: ExtendedIsolationState = ExtendedIsolationState(2i32);
pub const extendedIsolationStateNoData: ExtendedIsolationState = ExtendedIsolationState(0i32);
pub const extendedIsolationStateTransition: ExtendedIsolationState = ExtendedIsolationState(1i32);
pub const extendedIsolationStateUnknown: ExtendedIsolationState = ExtendedIsolationState(3i32);
pub const failureCategoryClientCommunication: FailureCategory = FailureCategory(3i32);
pub const failureCategoryClientComponent: FailureCategory = FailureCategory(2i32);
pub const failureCategoryCount: u32 = 5u32;
pub const failureCategoryNone: FailureCategory = FailureCategory(0i32);
pub const failureCategoryOther: FailureCategory = FailureCategory(1i32);
pub const failureCategoryServerCommunication: FailureCategory = FailureCategory(5i32);
pub const failureCategoryServerComponent: FailureCategory = FailureCategory(4i32);
pub const fixupStateCouldNotUpdate: FixupState = FixupState(2i32);
pub const fixupStateInProgress: FixupState = FixupState(1i32);
pub const fixupStateSuccess: FixupState = FixupState(0i32);
pub const freshSoHRequest: u32 = 1u32;
pub const isolationStateInProbation: IsolationState = IsolationState(2i32);
pub const isolationStateNotRestricted: IsolationState = IsolationState(1i32);
pub const isolationStateRestrictedAccess: IsolationState = IsolationState(3i32);
pub const maxConnectionCountPerEnforcer: u32 = 20u32;
pub const maxEnforcerCount: u32 = 20u32;
pub const maxNetworkSoHSize: u32 = 4000u32;
pub const maxPrivateDataSize: u32 = 200u32;
pub const maxSoHAttributeCount: u32 = 100u32;
pub const maxSoHAttributeSize: u32 = 4000u32;
pub const maxStringLength: u32 = 1024u32;
pub const maxSystemHealthEntityCount: u32 = 20u32;
pub const minNetworkSoHSize: u32 = 12u32;
pub const napNotifyTypeQuarState: NapNotifyType = NapNotifyType(2i32);
pub const napNotifyTypeServiceState: NapNotifyType = NapNotifyType(1i32);
pub const napNotifyTypeUnknown: NapNotifyType = NapNotifyType(0i32);
pub const percentageNotSupported: u32 = 101u32;
pub const remoteConfigTypeConfigBlob: RemoteConfigurationType = RemoteConfigurationType(2i32);
pub const remoteConfigTypeMachine: RemoteConfigurationType = RemoteConfigurationType(1i32);
pub const shaFixup: u32 = 1u32;
pub const tracingLevelAdvanced: NapTracingLevel = NapTracingLevel(2i32);
pub const tracingLevelBasic: NapTracingLevel = NapTracingLevel(1i32);
pub const tracingLevelDebug: NapTracingLevel = NapTracingLevel(3i32);
pub const tracingLevelUndefined: NapTracingLevel = NapTracingLevel(0i32);
