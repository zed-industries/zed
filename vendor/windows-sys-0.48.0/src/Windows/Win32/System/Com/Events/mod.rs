pub type IDontSupportEventSubscription = *mut ::core::ffi::c_void;
pub type IEnumEventObject = *mut ::core::ffi::c_void;
pub type IEventClass = *mut ::core::ffi::c_void;
pub type IEventClass2 = *mut ::core::ffi::c_void;
pub type IEventControl = *mut ::core::ffi::c_void;
pub type IEventObjectChange = *mut ::core::ffi::c_void;
pub type IEventObjectChange2 = *mut ::core::ffi::c_void;
pub type IEventObjectCollection = *mut ::core::ffi::c_void;
pub type IEventProperty = *mut ::core::ffi::c_void;
pub type IEventPublisher = *mut ::core::ffi::c_void;
pub type IEventSubscription = *mut ::core::ffi::c_void;
pub type IEventSystem = *mut ::core::ffi::c_void;
pub type IFiringControl = *mut ::core::ffi::c_void;
pub type IMultiInterfaceEventControl = *mut ::core::ffi::c_void;
pub type IMultiInterfacePublisherFilter = *mut ::core::ffi::c_void;
pub type IPublisherFilter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventClass: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcdbec9c0_7a68_11d1_88f9_0080c7d771bf);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventPublisher: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xab944620_79c6_11d1_88f9_0080c7d771bf);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventSubscription: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7542e960_79c7_11d1_88f9_0080c7d771bf);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const CEventSystem: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4e14fba2_2e22_11d1_9964_00c04fbbb345);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EventObjectChange: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd0565000_9df4_11d1_a281_00c04fca0aa7);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EventObjectChange2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbb07bacd_cd56_4e63_a8ff_cbf0355fb9f4);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub type EOC_ChangeType = i32;
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_NewObject: EOC_ChangeType = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_ModifiedObject: EOC_ChangeType = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_DeletedObject: EOC_ChangeType = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub struct COMEVENTSYSCHANGEINFO {
    pub cbSize: u32,
    pub changeType: EOC_ChangeType,
    pub objectId: ::windows_sys::core::BSTR,
    pub partitionId: ::windows_sys::core::BSTR,
    pub applicationId: ::windows_sys::core::BSTR,
    pub reserved: [::windows_sys::core::GUID; 10],
}
impl ::core::marker::Copy for COMEVENTSYSCHANGEINFO {}
impl ::core::clone::Clone for COMEVENTSYSCHANGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
