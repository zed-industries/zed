#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcClose ( object : isize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcDeleteSubscription ( subscriptionname : ::windows_sys::core::PCWSTR , flags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcEnumNextSubscription ( subscriptionenum : isize , subscriptionnamebuffersize : u32 , subscriptionnamebuffer : ::windows_sys::core::PWSTR , subscriptionnamebufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcGetObjectArrayProperty ( objectarray : isize , propertyid : EC_SUBSCRIPTION_PROPERTY_ID , arrayindex : u32 , flags : u32 , propertyvaluebuffersize : u32 , propertyvaluebuffer : *mut EC_VARIANT , propertyvaluebufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcGetObjectArraySize ( objectarray : isize , objectarraysize : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcGetSubscriptionProperty ( subscription : isize , propertyid : EC_SUBSCRIPTION_PROPERTY_ID , flags : u32 , propertyvaluebuffersize : u32 , propertyvaluebuffer : *mut EC_VARIANT , propertyvaluebufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcGetSubscriptionRunTimeStatus ( subscriptionname : ::windows_sys::core::PCWSTR , statusinfoid : EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID , eventsourcename : ::windows_sys::core::PCWSTR , flags : u32 , statusvaluebuffersize : u32 , statusvaluebuffer : *mut EC_VARIANT , statusvaluebufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcInsertObjectArrayElement ( objectarray : isize , arrayindex : u32 ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`*"] fn EcOpenSubscription ( subscriptionname : ::windows_sys::core::PCWSTR , accessmask : u32 , flags : u32 ) -> isize );
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`*"] fn EcOpenSubscriptionEnum ( flags : u32 ) -> isize );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcRemoveObjectArrayElement ( objectarray : isize , arrayindex : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcRetrySubscription ( subscriptionname : ::windows_sys::core::PCWSTR , eventsourcename : ::windows_sys::core::PCWSTR , flags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcSaveSubscription ( subscription : isize , flags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcSetObjectArrayProperty ( objectarray : isize , propertyid : EC_SUBSCRIPTION_PROPERTY_ID , arrayindex : u32 , flags : u32 , propertyvalue : *mut EC_VARIANT ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "wecapi.dll""system" #[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"] fn EcSetSubscriptionProperty ( subscription : isize , propertyid : EC_SUBSCRIPTION_PROPERTY_ID , flags : u32 , propertyvalue : *mut EC_VARIANT ) -> super::super::Foundation:: BOOL );
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EC_CREATE_NEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EC_OPEN_ALWAYS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EC_OPEN_EXISTING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EC_READ_ACCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EC_VARIANT_TYPE_ARRAY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EC_VARIANT_TYPE_MASK: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EC_WRITE_ACCESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub type EC_SUBSCRIPTION_CONFIGURATION_MODE = i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcConfigurationModeNormal: EC_SUBSCRIPTION_CONFIGURATION_MODE = 0i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcConfigurationModeCustom: EC_SUBSCRIPTION_CONFIGURATION_MODE = 1i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcConfigurationModeMinLatency: EC_SUBSCRIPTION_CONFIGURATION_MODE = 2i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcConfigurationModeMinBandwidth: EC_SUBSCRIPTION_CONFIGURATION_MODE = 3i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub type EC_SUBSCRIPTION_CONTENT_FORMAT = i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcContentFormatEvents: EC_SUBSCRIPTION_CONTENT_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcContentFormatRenderedText: EC_SUBSCRIPTION_CONTENT_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub type EC_SUBSCRIPTION_CREDENTIALS_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionCredDefault: EC_SUBSCRIPTION_CREDENTIALS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionCredNegotiate: EC_SUBSCRIPTION_CREDENTIALS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionCredDigest: EC_SUBSCRIPTION_CREDENTIALS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionCredBasic: EC_SUBSCRIPTION_CREDENTIALS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionCredLocalMachine: EC_SUBSCRIPTION_CREDENTIALS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub type EC_SUBSCRIPTION_DELIVERY_MODE = i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcDeliveryModePull: EC_SUBSCRIPTION_DELIVERY_MODE = 1i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcDeliveryModePush: EC_SUBSCRIPTION_DELIVERY_MODE = 2i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub type EC_SUBSCRIPTION_PROPERTY_ID = i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionEnabled: EC_SUBSCRIPTION_PROPERTY_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionEventSources: EC_SUBSCRIPTION_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionEventSourceAddress: EC_SUBSCRIPTION_PROPERTY_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionEventSourceEnabled: EC_SUBSCRIPTION_PROPERTY_ID = 3i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionEventSourceUserName: EC_SUBSCRIPTION_PROPERTY_ID = 4i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionEventSourcePassword: EC_SUBSCRIPTION_PROPERTY_ID = 5i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionDescription: EC_SUBSCRIPTION_PROPERTY_ID = 6i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionURI: EC_SUBSCRIPTION_PROPERTY_ID = 7i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionConfigurationMode: EC_SUBSCRIPTION_PROPERTY_ID = 8i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionExpires: EC_SUBSCRIPTION_PROPERTY_ID = 9i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionQuery: EC_SUBSCRIPTION_PROPERTY_ID = 10i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionTransportName: EC_SUBSCRIPTION_PROPERTY_ID = 11i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionTransportPort: EC_SUBSCRIPTION_PROPERTY_ID = 12i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionDeliveryMode: EC_SUBSCRIPTION_PROPERTY_ID = 13i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionDeliveryMaxItems: EC_SUBSCRIPTION_PROPERTY_ID = 14i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionDeliveryMaxLatencyTime: EC_SUBSCRIPTION_PROPERTY_ID = 15i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionHeartbeatInterval: EC_SUBSCRIPTION_PROPERTY_ID = 16i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionLocale: EC_SUBSCRIPTION_PROPERTY_ID = 17i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionContentFormat: EC_SUBSCRIPTION_PROPERTY_ID = 18i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionLogFile: EC_SUBSCRIPTION_PROPERTY_ID = 19i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionPublisherName: EC_SUBSCRIPTION_PROPERTY_ID = 20i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionCredentialsType: EC_SUBSCRIPTION_PROPERTY_ID = 21i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionCommonUserName: EC_SUBSCRIPTION_PROPERTY_ID = 22i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionCommonPassword: EC_SUBSCRIPTION_PROPERTY_ID = 23i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionHostName: EC_SUBSCRIPTION_PROPERTY_ID = 24i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionReadExistingEvents: EC_SUBSCRIPTION_PROPERTY_ID = 25i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionDialect: EC_SUBSCRIPTION_PROPERTY_ID = 26i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionType: EC_SUBSCRIPTION_PROPERTY_ID = 27i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionAllowedIssuerCAs: EC_SUBSCRIPTION_PROPERTY_ID = 28i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionAllowedSubjects: EC_SUBSCRIPTION_PROPERTY_ID = 29i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionDeniedSubjects: EC_SUBSCRIPTION_PROPERTY_ID = 30i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionAllowedSourceDomainComputers: EC_SUBSCRIPTION_PROPERTY_ID = 31i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionPropertyIdEND: EC_SUBSCRIPTION_PROPERTY_ID = 32i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub type EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcRuntimeStatusActiveStatusDisabled: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcRuntimeStatusActiveStatusActive: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcRuntimeStatusActiveStatusInactive: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcRuntimeStatusActiveStatusTrying: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub type EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionRunTimeStatusActive: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionRunTimeStatusLastError: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionRunTimeStatusLastErrorMessage: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionRunTimeStatusLastErrorTime: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = 3i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionRunTimeStatusNextRetryTime: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = 4i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionRunTimeStatusEventSources: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = 5i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionRunTimeStatusLastHeartbeatTime: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = 6i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionRunTimeStatusInfoIdEND: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = 7i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub type EC_SUBSCRIPTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionTypeSourceInitiated: EC_SUBSCRIPTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcSubscriptionTypeCollectorInitiated: EC_SUBSCRIPTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub type EC_VARIANT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcVarTypeNull: EC_VARIANT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcVarTypeBoolean: EC_VARIANT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcVarTypeUInt32: EC_VARIANT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcVarTypeDateTime: EC_VARIANT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcVarTypeString: EC_VARIANT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_EventCollector\"`*"]
pub const EcVarObjectArrayPropertyHandle: EC_VARIANT_TYPE = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EC_VARIANT {
    pub Anonymous: EC_VARIANT_0,
    pub Count: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EC_VARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EC_VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventCollector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EC_VARIANT_0 {
    pub BooleanVal: super::super::Foundation::BOOL,
    pub UInt32Val: u32,
    pub DateTimeVal: u64,
    pub StringVal: ::windows_sys::core::PCWSTR,
    pub BinaryVal: *mut u8,
    pub BooleanArr: *mut super::super::Foundation::BOOL,
    pub Int32Arr: *mut i32,
    pub StringArr: *mut ::windows_sys::core::PWSTR,
    pub PropertyHandleVal: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EC_VARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EC_VARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
