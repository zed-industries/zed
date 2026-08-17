pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ExtendedIsolationState(pub i32);
impl windows_core::TypeKind for ExtendedIsolationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ExtendedIsolationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ExtendedIsolationState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FailureCategory(pub i32);
impl windows_core::TypeKind for FailureCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FailureCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FailureCategory").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FixupState(pub i32);
impl windows_core::TypeKind for FixupState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FixupState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FixupState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolationState(pub i32);
impl windows_core::TypeKind for IsolationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IsolationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolationState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NapNotifyType(pub i32);
impl windows_core::TypeKind for NapNotifyType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NapNotifyType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NapNotifyType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NapTracingLevel(pub i32);
impl windows_core::TypeKind for NapTracingLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NapTracingLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NapTracingLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteConfigurationType(pub i32);
impl windows_core::TypeKind for RemoteConfigurationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteConfigurationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteConfigurationType").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct CorrelationId {
    pub connId: windows_core::GUID,
    pub timeStamp: super::super::Foundation::FILETIME,
}
impl Copy for CorrelationId {}
impl Clone for CorrelationId {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for CorrelationId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CorrelationId").field("connId", &self.connId).field("timeStamp", &self.timeStamp).finish()
    }
}
impl windows_core::TypeKind for CorrelationId {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for CorrelationId {
    fn eq(&self, other: &Self) -> bool {
        self.connId == other.connId && self.timeStamp == other.timeStamp
    }
}
impl Eq for CorrelationId {}
impl Default for CorrelationId {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CountedString {
    pub length: u16,
    pub string: windows_core::PWSTR,
}
impl Copy for CountedString {}
impl Clone for CountedString {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for CountedString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CountedString").field("length", &self.length).field("string", &self.string).finish()
    }
}
impl windows_core::TypeKind for CountedString {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for CountedString {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.string == other.string
    }
}
impl Eq for CountedString {}
impl Default for CountedString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FailureCategoryMapping {
    pub mappingCompliance: [super::super::Foundation::BOOL; 5],
}
impl Copy for FailureCategoryMapping {}
impl Clone for FailureCategoryMapping {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for FailureCategoryMapping {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FailureCategoryMapping").field("mappingCompliance", &self.mappingCompliance).finish()
    }
}
impl windows_core::TypeKind for FailureCategoryMapping {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for FailureCategoryMapping {
    fn eq(&self, other: &Self) -> bool {
        self.mappingCompliance == other.mappingCompliance
    }
}
impl Eq for FailureCategoryMapping {}
impl Default for FailureCategoryMapping {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FixupInfo {
    pub state: FixupState,
    pub percentage: u8,
    pub resultCodes: ResultCodes,
    pub fixupMsgId: u32,
}
impl Copy for FixupInfo {}
impl Clone for FixupInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for FixupInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FixupInfo").field("state", &self.state).field("percentage", &self.percentage).field("resultCodes", &self.resultCodes).field("fixupMsgId", &self.fixupMsgId).finish()
    }
}
impl windows_core::TypeKind for FixupInfo {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for FixupInfo {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.percentage == other.percentage && self.resultCodes == other.resultCodes && self.fixupMsgId == other.fixupMsgId
    }
}
impl Eq for FixupInfo {}
impl Default for FixupInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Ipv4Address {
    pub addr: [u8; 4],
}
impl Copy for Ipv4Address {}
impl Clone for Ipv4Address {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for Ipv4Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Ipv4Address").field("addr", &self.addr).finish()
    }
}
impl windows_core::TypeKind for Ipv4Address {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for Ipv4Address {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl Eq for Ipv4Address {}
impl Default for Ipv4Address {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Ipv6Address {
    pub addr: [u8; 16],
}
impl Copy for Ipv6Address {}
impl Clone for Ipv6Address {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for Ipv6Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Ipv6Address").field("addr", &self.addr).finish()
    }
}
impl windows_core::TypeKind for Ipv6Address {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for Ipv6Address {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl Eq for Ipv6Address {}
impl Default for Ipv6Address {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IsolationInfo {
    pub isolationState: IsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
impl Copy for IsolationInfo {}
impl Clone for IsolationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for IsolationInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IsolationInfo").field("isolationState", &self.isolationState).field("probEndTime", &self.probEndTime).field("failureUrl", &self.failureUrl).finish()
    }
}
impl windows_core::TypeKind for IsolationInfo {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for IsolationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.isolationState == other.isolationState && self.probEndTime == other.probEndTime && self.failureUrl == other.failureUrl
    }
}
impl Eq for IsolationInfo {}
impl Default for IsolationInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IsolationInfoEx {
    pub isolationState: IsolationState,
    pub extendedIsolationState: ExtendedIsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
impl Copy for IsolationInfoEx {}
impl Clone for IsolationInfoEx {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for IsolationInfoEx {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IsolationInfoEx").field("isolationState", &self.isolationState).field("extendedIsolationState", &self.extendedIsolationState).field("probEndTime", &self.probEndTime).field("failureUrl", &self.failureUrl).finish()
    }
}
impl windows_core::TypeKind for IsolationInfoEx {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for IsolationInfoEx {
    fn eq(&self, other: &Self) -> bool {
        self.isolationState == other.isolationState && self.extendedIsolationState == other.extendedIsolationState && self.probEndTime == other.probEndTime && self.failureUrl == other.failureUrl
    }
}
impl Eq for IsolationInfoEx {}
impl Default for IsolationInfoEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl Copy for NapComponentRegistrationInfo {}
impl Clone for NapComponentRegistrationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for NapComponentRegistrationInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NapComponentRegistrationInfo").field("id", &self.id).field("friendlyName", &self.friendlyName).field("description", &self.description).field("version", &self.version).field("vendorName", &self.vendorName).field("infoClsid", &self.infoClsid).field("configClsid", &self.configClsid).field("registrationDate", &self.registrationDate).field("componentType", &self.componentType).finish()
    }
}
impl windows_core::TypeKind for NapComponentRegistrationInfo {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for NapComponentRegistrationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.friendlyName == other.friendlyName && self.description == other.description && self.version == other.version && self.vendorName == other.vendorName && self.infoClsid == other.infoClsid && self.configClsid == other.configClsid && self.registrationDate == other.registrationDate && self.componentType == other.componentType
    }
}
impl Eq for NapComponentRegistrationInfo {}
impl Default for NapComponentRegistrationInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NetworkSoH {
    pub size: u16,
    pub data: *mut u8,
}
impl Copy for NetworkSoH {}
impl Clone for NetworkSoH {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for NetworkSoH {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NetworkSoH").field("size", &self.size).field("data", &self.data).finish()
    }
}
impl windows_core::TypeKind for NetworkSoH {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for NetworkSoH {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl Eq for NetworkSoH {}
impl Default for NetworkSoH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PrivateData {
    pub size: u16,
    pub data: *mut u8,
}
impl Copy for PrivateData {}
impl Clone for PrivateData {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PrivateData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PrivateData").field("size", &self.size).field("data", &self.data).finish()
    }
}
impl windows_core::TypeKind for PrivateData {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PrivateData {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl Eq for PrivateData {}
impl Default for PrivateData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ResultCodes {
    pub count: u16,
    pub results: *mut windows_core::HRESULT,
}
impl Copy for ResultCodes {}
impl Clone for ResultCodes {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for ResultCodes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ResultCodes").field("count", &self.count).field("results", &self.results).finish()
    }
}
impl windows_core::TypeKind for ResultCodes {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for ResultCodes {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.results == other.results
    }
}
impl Eq for ResultCodes {}
impl Default for ResultCodes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SoH {
    pub count: u16,
    pub attributes: *mut SoHAttribute,
}
impl Copy for SoH {}
impl Clone for SoH {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for SoH {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SoH").field("count", &self.count).field("attributes", &self.attributes).finish()
    }
}
impl windows_core::TypeKind for SoH {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for SoH {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.attributes == other.attributes
    }
}
impl Eq for SoH {}
impl Default for SoH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SoHAttribute {
    pub r#type: u16,
    pub size: u16,
    pub value: *mut u8,
}
impl Copy for SoHAttribute {}
impl Clone for SoHAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for SoHAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SoHAttribute").field("type", &self.r#type).field("size", &self.size).field("value", &self.value).finish()
    }
}
impl windows_core::TypeKind for SoHAttribute {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for SoHAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.size == other.size && self.value == other.value
    }
}
impl Eq for SoHAttribute {}
impl Default for SoHAttribute {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SystemHealthAgentState {
    pub id: u32,
    pub shaResultCodes: ResultCodes,
    pub failureCategory: FailureCategory,
    pub fixupInfo: FixupInfo,
}
impl Copy for SystemHealthAgentState {}
impl Clone for SystemHealthAgentState {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for SystemHealthAgentState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SystemHealthAgentState").field("id", &self.id).field("shaResultCodes", &self.shaResultCodes).field("failureCategory", &self.failureCategory).field("fixupInfo", &self.fixupInfo).finish()
    }
}
impl windows_core::TypeKind for SystemHealthAgentState {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for SystemHealthAgentState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.shaResultCodes == other.shaResultCodes && self.failureCategory == other.failureCategory && self.fixupInfo == other.fixupInfo
    }
}
impl Eq for SystemHealthAgentState {}
impl Default for SystemHealthAgentState {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
