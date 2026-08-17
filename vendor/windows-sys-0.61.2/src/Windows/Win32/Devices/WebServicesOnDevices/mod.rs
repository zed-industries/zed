windows_link::link!("wsdapi.dll" "system" fn WSDAllocateLinkedMemory(pparent : *mut core::ffi::c_void, cbsize : usize) -> *mut core::ffi::c_void);
windows_link::link!("wsdapi.dll" "system" fn WSDAttachLinkedMemory(pparent : *mut core::ffi::c_void, pchild : *mut core::ffi::c_void));
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost(pszlocalid : windows_sys::core::PCWSTR, pcontext : * mut core::ffi::c_void, ppdevicehost : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost2(pszlocalid : windows_sys::core::PCWSTR, pcontext : * mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppdevicehost : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHostAdvanced(pszlocalid : windows_sys::core::PCWSTR, pcontext : * mut core::ffi::c_void, pphostaddresses : *const * mut core::ffi::c_void, dwhostaddresscount : u32, ppdevicehost : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy(pszdeviceid : windows_sys::core::PCWSTR, pszlocalid : windows_sys::core::PCWSTR, pcontext : * mut core::ffi::c_void, ppdeviceproxy : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy2(pszdeviceid : windows_sys::core::PCWSTR, pszlocalid : windows_sys::core::PCWSTR, pcontext : * mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppdeviceproxy : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxyAdvanced(pszdeviceid : windows_sys::core::PCWSTR, pdeviceaddress : * mut core::ffi::c_void, pszlocalid : windows_sys::core::PCWSTR, pcontext : * mut core::ffi::c_void, ppdeviceproxy : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider(pcontext : * mut core::ffi::c_void, ppprovider : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider2(pcontext : * mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppprovider : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher(pcontext : * mut core::ffi::c_void, pppublisher : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher2(pcontext : * mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, pppublisher : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateHttpAddress(ppaddress : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateHttpMessageParameters(pptxparams : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateOutboundAttachment(ppattachment : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateUdpAddress(ppaddress : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateUdpMessageParameters(pptxparams : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDDetachLinkedMemory(pvoid : *mut core::ffi::c_void));
windows_link::link!("wsdapi.dll" "system" fn WSDFreeLinkedMemory(pvoid : *mut core::ffi::c_void));
windows_link::link!("wsdapi.dll" "system" fn WSDGenerateFault(pszcode : windows_sys::core::PCWSTR, pszsubcode : windows_sys::core::PCWSTR, pszreason : windows_sys::core::PCWSTR, pszdetail : windows_sys::core::PCWSTR, pcontext : * mut core::ffi::c_void, ppfault : *mut *mut WSD_SOAP_FAULT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDGenerateFaultEx(pcode : *const WSDXML_NAME, psubcode : *const WSDXML_NAME, preasons : *const WSD_LOCALIZED_STRING_LIST, pszdetail : windows_sys::core::PCWSTR, ppfault : *mut *mut WSD_SOAP_FAULT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDGetConfigurationOption(dwoption : u32, pvoid : *mut core::ffi::c_void, cboutbuffer : u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDSetConfigurationOption(dwoption : u32, pvoid : *const core::ffi::c_void, cbinbuffer : u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDUriDecode(source : windows_sys::core::PCWSTR, cchsource : u32, destout : *mut windows_sys::core::PWSTR, cchdestout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDUriEncode(source : windows_sys::core::PCWSTR, cchsource : u32, destout : *mut windows_sys::core::PWSTR, cchdestout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLAddChild(pparent : *mut WSDXML_ELEMENT, pchild : *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLAddSibling(pfirst : *mut WSDXML_ELEMENT, psecond : *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLBuildAnyForSingleElement(pelementname : *mut WSDXML_NAME, psztext : windows_sys::core::PCWSTR, ppany : *mut *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLCleanupElement(pany : *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLCreateContext(ppcontext : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLGetNameFromBuiltinNamespace(psznamespace : windows_sys::core::PCWSTR, pszname : windows_sys::core::PCWSTR, ppname : *mut *mut WSDXML_NAME) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLGetValueFromAny(psznamespace : windows_sys::core::PCWSTR, pszname : windows_sys::core::PCWSTR, pany : *mut WSDXML_ELEMENT, ppszvalue : *mut windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
pub type DeviceDiscoveryMechanism = i32;
pub const DirectedDiscovery: DeviceDiscoveryMechanism = 1i32;
pub const MulticastDiscovery: DeviceDiscoveryMechanism = 0i32;
pub const ONE_WAY: WSDUdpMessageType = 0i32;
pub const OpAnyElement: WSDXML_OP = 6i32;
pub const OpAnyElements: WSDXML_OP = 7i32;
pub const OpAnyNumber: WSDXML_OP = 17i32;
pub const OpAnyText: WSDXML_OP = 8i32;
pub const OpAnything: WSDXML_OP = 16i32;
pub const OpAttribute_: WSDXML_OP = 9i32;
pub const OpBeginAll: WSDXML_OP = 14i32;
pub const OpBeginAnyElement: WSDXML_OP = 3i32;
pub const OpBeginChoice: WSDXML_OP = 10i32;
pub const OpBeginElement_: WSDXML_OP = 2i32;
pub const OpBeginSequence: WSDXML_OP = 12i32;
pub const OpElement_: WSDXML_OP = 5i32;
pub const OpEndAll: WSDXML_OP = 15i32;
pub const OpEndChoice: WSDXML_OP = 11i32;
pub const OpEndElement: WSDXML_OP = 4i32;
pub const OpEndOfTable: WSDXML_OP = 1i32;
pub const OpEndSequence: WSDXML_OP = 13i32;
pub const OpFormatBool_: WSDXML_OP = 20i32;
pub const OpFormatDateTime_: WSDXML_OP = 40i32;
pub const OpFormatDom_: WSDXML_OP = 30i32;
pub const OpFormatDouble_: WSDXML_OP = 42i32;
pub const OpFormatDuration_: WSDXML_OP = 39i32;
pub const OpFormatDynamicType_: WSDXML_OP = 37i32;
pub const OpFormatFloat_: WSDXML_OP = 41i32;
pub const OpFormatInt16_: WSDXML_OP = 22i32;
pub const OpFormatInt32_: WSDXML_OP = 23i32;
pub const OpFormatInt64_: WSDXML_OP = 24i32;
pub const OpFormatInt8_: WSDXML_OP = 21i32;
pub const OpFormatListInsertTail_: WSDXML_OP = 35i32;
pub const OpFormatLookupType_: WSDXML_OP = 38i32;
pub const OpFormatMax: WSDXML_OP = 46i32;
pub const OpFormatName_: WSDXML_OP = 34i32;
pub const OpFormatStruct_: WSDXML_OP = 31i32;
pub const OpFormatType_: WSDXML_OP = 36i32;
pub const OpFormatUInt16_: WSDXML_OP = 26i32;
pub const OpFormatUInt32_: WSDXML_OP = 27i32;
pub const OpFormatUInt64_: WSDXML_OP = 28i32;
pub const OpFormatUInt8_: WSDXML_OP = 25i32;
pub const OpFormatUnicodeString_: WSDXML_OP = 29i32;
pub const OpFormatUri_: WSDXML_OP = 32i32;
pub const OpFormatUuidUri_: WSDXML_OP = 33i32;
pub const OpFormatXMLDeclaration_: WSDXML_OP = 45i32;
pub const OpNone: WSDXML_OP = 0i32;
pub const OpOneOrMore: WSDXML_OP = 18i32;
pub const OpOptional: WSDXML_OP = 19i32;
pub const OpProcess_: WSDXML_OP = 43i32;
pub const OpQualifiedAttribute_: WSDXML_OP = 44i32;
pub type PWSD_SOAP_MESSAGE_HANDLER = Option<unsafe extern "system" fn(thisunknown: *mut core::ffi::c_void, event: *mut WSD_EVENT) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for REQUESTBODY_GetStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for REQUESTBODY_Renew {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for REQUESTBODY_Subscribe {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
impl Default for REQUESTBODY_Unsubscribe {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
impl Default for RESPONSEBODY_GetMetadata {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl Default for RESPONSEBODY_GetStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl Default for RESPONSEBODY_Renew {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl Default for RESPONSEBODY_Subscribe {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: windows_sys::core::PCWSTR,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for RESPONSEBODY_SubscriptionEnd {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = 2i32;
pub const TWO_WAY: WSDUdpMessageType = 1i32;
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1u32;
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2u32;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1u32;
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1u32;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2u32;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3u32;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0u32;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2u32;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16u32;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1u32;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8u32;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4u32;
pub const WSDET_INCOMING_FAULT: WSDEventType = 2i32;
pub const WSDET_INCOMING_MESSAGE: WSDEventType = 1i32;
pub const WSDET_NONE: WSDEventType = 0i32;
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = 4i32;
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = 3i32;
pub type WSDEventType = i32;
pub type WSDUdpMessageType = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_ATTRIBUTE,
    pub Name: *mut WSDXML_NAME,
    pub Value: windows_sys::core::PWSTR,
}
impl Default for WSDXML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
impl Default for WSDXML_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_ELEMENT_LIST {
    pub Next: *mut WSDXML_ELEMENT_LIST,
    pub Element: *mut WSDXML_ELEMENT,
}
impl Default for WSDXML_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: windows_sys::core::PWSTR,
}
impl Default for WSDXML_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_NAMESPACE {
    pub Uri: windows_sys::core::PCWSTR,
    pub PreferredPrefix: windows_sys::core::PCWSTR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: u16,
    pub Encoding: u16,
}
impl Default for WSDXML_NAMESPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_NODE,
}
impl WSDXML_NODE {
    pub const ElementType: i32 = 0i32;
    pub const TextType: i32 = 1i32;
}
impl Default for WSDXML_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSDXML_OP = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_PREFIX_MAPPING {
    pub Refs: u32,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: windows_sys::core::PWSTR,
}
impl Default for WSDXML_PREFIX_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: windows_sys::core::PWSTR,
}
impl Default for WSDXML_TEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_TYPE {
    pub Uri: windows_sys::core::PCWSTR,
    pub Table: *const u8,
}
impl Default for WSDXML_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_APP_SEQUENCE {
    pub InstanceId: u64,
    pub SequenceId: windows_sys::core::PCWSTR,
    pub MessageNumber: u64,
}
impl Default for WSD_APP_SEQUENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_BYE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut *mut core::ffi::c_void,
    pub dwAddressCount: u32,
}
impl Default for WSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 10i32;
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 9i32;
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 1i32;
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut core::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl Default for WSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSD_CONFIG_PARAM_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSD_DATETIME {
    pub isPositive: windows_sys::core::BOOL,
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
    pub TZIsLocal: windows_sys::core::BOOL,
    pub TZIsPositive: windows_sys::core::BOOL,
    pub TZHour: u8,
    pub TZMinute: u8,
}
pub const WSD_DEFAULT_EVENTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("http://*:5357/");
pub const WSD_DEFAULT_HOSTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("http://*:5357/");
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("https://*:5358/");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSD_DURATION {
    pub isPositive: windows_sys::core::BOOL,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_ENDPOINT_REFERENCE {
    pub Address: windows_sys::core::PCWSTR,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_ENDPOINT_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
impl Default for WSD_ENDPOINT_REFERENCE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENT {
    pub Hr: windows_sys::core::HRESULT,
    pub EventType: u32,
    pub DispatchTag: windows_sys::core::PWSTR,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: *mut core::ffi::c_void,
}
impl Default for WSD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_DELIVERY_MODE {
    pub Mode: windows_sys::core::PCWSTR,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut core::ffi::c_void,
}
impl Default for WSD_EVENTING_DELIVERY_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
impl Default for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
impl Default for WSD_EVENTING_EXPIRES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_FILTER {
    pub Dialect: windows_sys::core::PCWSTR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut core::ffi::c_void,
}
impl Default for WSD_EVENTING_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
impl Default for WSD_EVENTING_FILTER_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_HANDLER_CONTEXT {
    pub Handler: PWSD_SOAP_MESSAGE_HANDLER,
    pub PVoid: *mut core::ffi::c_void,
    pub Unknown: *mut core::ffi::c_void,
}
impl Default for WSD_HANDLER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: windows_sys::core::PCWSTR,
}
impl Default for WSD_HEADER_RELATESTO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_HELLO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
impl Default for WSD_HOST_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_LOCALIZED_STRING {
    pub lang: windows_sys::core::PCWSTR,
    pub String: windows_sys::core::PCWSTR,
}
impl Default for WSD_LOCALIZED_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
impl Default for WSD_LOCALIZED_STRING_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_METADATA_SECTION {
    pub Dialect: windows_sys::core::PCWSTR,
    pub Identifier: windows_sys::core::PCWSTR,
    pub Data: *mut core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: windows_sys::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_METADATA_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_METADATA_SECTION_LIST {
    pub Next: *mut WSD_METADATA_SECTION_LIST,
    pub Element: *mut WSD_METADATA_SECTION,
}
impl Default for WSD_METADATA_SECTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
impl Default for WSD_NAME_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: WSD_STUB_FUNCTION,
}
impl Default for WSD_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_PORT_TYPE {
    pub EncodedName: u32,
    pub OperationCount: u32,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
impl Default for WSD_PORT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_PROBE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_PROBE_MATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_PROBE_MATCHES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_PROBE_MATCH_LIST {
    pub Next: *mut WSD_PROBE_MATCH_LIST,
    pub Element: *mut WSD_PROBE_MATCH,
}
impl Default for WSD_PROBE_MATCH_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSD_PROTOCOL_TYPE = i32;
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = 255i32;
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = 2i32;
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = 4i32;
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = 0i32;
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_REFERENCE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_REFERENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_RELATIONSHIP_METADATA {
    pub Type: windows_sys::core::PCWSTR,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_RELATIONSHIP_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_RESOLVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_RESOLVE_MATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_RESOLVE_MATCHES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SCOPES {
    pub MatchBy: windows_sys::core::PCWSTR,
    pub Scopes: *mut WSD_URI_LIST,
}
impl Default for WSD_SCOPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy)]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::super::Security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: windows_sys::core::PCWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for WSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy)]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::super::Security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = 7i32;
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = 8i32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1u32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2u32;
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 12i32;
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 11i32;
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy)]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 3i32;
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 5i32;
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = 6i32;
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 4i32;
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 13i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: windows_sys::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SERVICE_METADATA_LIST {
    pub Next: *mut WSD_SERVICE_METADATA_LIST,
    pub Element: *mut WSD_SERVICE_METADATA,
}
impl Default for WSD_SERVICE_METADATA_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: windows_sys::core::PCWSTR,
    pub Role: windows_sys::core::PCWSTR,
    pub Detail: *mut WSDXML_ELEMENT,
}
impl Default for WSD_SOAP_FAULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl Default for WSD_SOAP_FAULT_CODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
impl Default for WSD_SOAP_FAULT_REASON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl Default for WSD_SOAP_FAULT_SUBCODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_HEADER {
    pub To: windows_sys::core::PCWSTR,
    pub Action: windows_sys::core::PCWSTR,
    pub MessageID: windows_sys::core::PCWSTR,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
impl Default for WSD_SOAP_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut core::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
impl Default for WSD_SOAP_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSD_STUB_FUNCTION = Option<unsafe extern "system" fn(server: *mut core::ffi::c_void, session: *mut core::ffi::c_void, event: *mut WSD_EVENT) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: windows_sys::core::HRESULT,
    pub eventHandle: super::super::Foundation::HANDLE,
    pub messageParameters: *mut core::ffi::c_void,
    pub results: *mut core::ffi::c_void,
}
impl Default for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: windows_sys::core::PCWSTR,
    pub SerialNumber: windows_sys::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_THIS_DEVICE_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: windows_sys::core::PCWSTR,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: windows_sys::core::PCWSTR,
    pub ModelUrl: windows_sys::core::PCWSTR,
    pub PresentationUrl: windows_sys::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_THIS_MODEL_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_UNKNOWN_LOOKUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: windows_sys::core::PCWSTR,
}
impl Default for WSD_URI_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
