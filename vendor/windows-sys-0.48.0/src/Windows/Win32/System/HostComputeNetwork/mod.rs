::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnCloseEndpoint ( endpoint : *const ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnCloseGuestNetworkService ( guestnetworkservice : *const ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnCloseLoadBalancer ( loadbalancer : *const ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnCloseNamespace ( namespace : *const ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnCloseNetwork ( network : *const ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnCreateEndpoint ( network : *const ::core::ffi::c_void , id : *const ::windows_sys::core::GUID , settings : ::windows_sys::core::PCWSTR , endpoint : *mut *mut ::core::ffi::c_void , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnCreateGuestNetworkService ( id : *const ::windows_sys::core::GUID , settings : ::windows_sys::core::PCWSTR , guestnetworkservice : *mut *mut ::core::ffi::c_void , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnCreateLoadBalancer ( id : *const ::windows_sys::core::GUID , settings : ::windows_sys::core::PCWSTR , loadbalancer : *mut *mut ::core::ffi::c_void , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnCreateNamespace ( id : *const ::windows_sys::core::GUID , settings : ::windows_sys::core::PCWSTR , namespace : *mut *mut ::core::ffi::c_void , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnCreateNetwork ( id : *const ::windows_sys::core::GUID , settings : ::windows_sys::core::PCWSTR , network : *mut *mut ::core::ffi::c_void , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnDeleteEndpoint ( id : *const ::windows_sys::core::GUID , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnDeleteGuestNetworkService ( id : *const ::windows_sys::core::GUID , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnDeleteLoadBalancer ( id : *const ::windows_sys::core::GUID , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnDeleteNamespace ( id : *const ::windows_sys::core::GUID , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnDeleteNetwork ( id : *const ::windows_sys::core::GUID , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnEnumerateEndpoints ( query : ::windows_sys::core::PCWSTR , endpoints : *mut ::windows_sys::core::PWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnEnumerateGuestNetworkPortReservations ( returncount : *mut u32 , portentries : *mut *mut HCN_PORT_RANGE_ENTRY ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnEnumerateLoadBalancers ( query : ::windows_sys::core::PCWSTR , loadbalancer : *mut ::windows_sys::core::PWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnEnumerateNamespaces ( query : ::windows_sys::core::PCWSTR , namespaces : *mut ::windows_sys::core::PWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnEnumerateNetworks ( query : ::windows_sys::core::PCWSTR , networks : *mut ::windows_sys::core::PWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnFreeGuestNetworkPortReservations ( portentries : *mut HCN_PORT_RANGE_ENTRY ) -> ( ) );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnModifyEndpoint ( endpoint : *const ::core::ffi::c_void , settings : ::windows_sys::core::PCWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnModifyGuestNetworkService ( guestnetworkservice : *const ::core::ffi::c_void , settings : ::windows_sys::core::PCWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnModifyLoadBalancer ( loadbalancer : *const ::core::ffi::c_void , settings : ::windows_sys::core::PCWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnModifyNamespace ( namespace : *const ::core::ffi::c_void , settings : ::windows_sys::core::PCWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnModifyNetwork ( network : *const ::core::ffi::c_void , settings : ::windows_sys::core::PCWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnOpenEndpoint ( id : *const ::windows_sys::core::GUID , endpoint : *mut *mut ::core::ffi::c_void , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnOpenLoadBalancer ( id : *const ::windows_sys::core::GUID , loadbalancer : *mut *mut ::core::ffi::c_void , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnOpenNamespace ( id : *const ::windows_sys::core::GUID , namespace : *mut *mut ::core::ffi::c_void , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnOpenNetwork ( id : *const ::windows_sys::core::GUID , network : *mut *mut ::core::ffi::c_void , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnQueryEndpointProperties ( endpoint : *const ::core::ffi::c_void , query : ::windows_sys::core::PCWSTR , properties : *mut ::windows_sys::core::PWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnQueryLoadBalancerProperties ( loadbalancer : *const ::core::ffi::c_void , query : ::windows_sys::core::PCWSTR , properties : *mut ::windows_sys::core::PWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnQueryNamespaceProperties ( namespace : *const ::core::ffi::c_void , query : ::windows_sys::core::PCWSTR , properties : *mut ::windows_sys::core::PWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnQueryNetworkProperties ( network : *const ::core::ffi::c_void , query : ::windows_sys::core::PCWSTR , properties : *mut ::windows_sys::core::PWSTR , errorrecord : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnRegisterGuestNetworkServiceCallback ( guestnetworkservice : *const ::core::ffi::c_void , callback : HCN_NOTIFICATION_CALLBACK , context : *const ::core::ffi::c_void , callbackhandle : *mut *mut ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnRegisterServiceCallback ( callback : HCN_NOTIFICATION_CALLBACK , context : *const ::core::ffi::c_void , callbackhandle : *mut *mut ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`, `\"Win32_Foundation\"`*"] fn HcnReleaseGuestNetworkServicePortReservationHandle ( portreservationhandle : super::super::Foundation:: HANDLE ) -> ::windows_sys::core::HRESULT );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`, `\"Win32_Foundation\"`*"] fn HcnReserveGuestNetworkServicePort ( guestnetworkservice : *const ::core::ffi::c_void , protocol : HCN_PORT_PROTOCOL , access : HCN_PORT_ACCESS , port : u16 , portreservationhandle : *mut super::super::Foundation:: HANDLE ) -> ::windows_sys::core::HRESULT );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`, `\"Win32_Foundation\"`*"] fn HcnReserveGuestNetworkServicePortRange ( guestnetworkservice : *const ::core::ffi::c_void , portcount : u16 , portrangereservation : *mut HCN_PORT_RANGE_RESERVATION , portreservationhandle : *mut super::super::Foundation:: HANDLE ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnUnregisterGuestNetworkServiceCallback ( callbackhandle : *const ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "computenetwork.dll""system" #[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"] fn HcnUnregisterServiceCallback ( callbackhandle : *const ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub type HCN_NOTIFICATIONS = i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationInvalid: HCN_NOTIFICATIONS = 0i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkPreCreate: HCN_NOTIFICATIONS = 1i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkCreate: HCN_NOTIFICATIONS = 2i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkPreDelete: HCN_NOTIFICATIONS = 3i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkDelete: HCN_NOTIFICATIONS = 4i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNamespaceCreate: HCN_NOTIFICATIONS = 5i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNamespaceDelete: HCN_NOTIFICATIONS = 6i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationGuestNetworkServiceCreate: HCN_NOTIFICATIONS = 7i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationGuestNetworkServiceDelete: HCN_NOTIFICATIONS = 8i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkEndpointAttached: HCN_NOTIFICATIONS = 9i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkEndpointDetached: HCN_NOTIFICATIONS = 16i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationGuestNetworkServiceStateChanged: HCN_NOTIFICATIONS = 17i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationGuestNetworkServiceInterfaceStateChanged: HCN_NOTIFICATIONS = 18i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationServiceDisconnect: HCN_NOTIFICATIONS = 16777216i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationFlagsReserved: HCN_NOTIFICATIONS = -268435456i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub type HCN_PORT_ACCESS = i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HCN_PORT_ACCESS_EXCLUSIVE: HCN_PORT_ACCESS = 1i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HCN_PORT_ACCESS_SHARED: HCN_PORT_ACCESS = 2i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub type HCN_PORT_PROTOCOL = i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HCN_PORT_PROTOCOL_TCP: HCN_PORT_PROTOCOL = 1i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HCN_PORT_PROTOCOL_UDP: HCN_PORT_PROTOCOL = 2i32;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HCN_PORT_PROTOCOL_BOTH: HCN_PORT_PROTOCOL = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub struct HCN_PORT_RANGE_ENTRY {
    pub OwningPartitionId: ::windows_sys::core::GUID,
    pub TargetPartitionId: ::windows_sys::core::GUID,
    pub Protocol: HCN_PORT_PROTOCOL,
    pub Priority: u64,
    pub ReservationType: u32,
    pub SharingFlags: u32,
    pub DeliveryMode: u32,
    pub StartingPort: u16,
    pub EndingPort: u16,
}
impl ::core::marker::Copy for HCN_PORT_RANGE_ENTRY {}
impl ::core::clone::Clone for HCN_PORT_RANGE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub struct HCN_PORT_RANGE_RESERVATION {
    pub startingPort: u16,
    pub endingPort: u16,
}
impl ::core::marker::Copy for HCN_PORT_RANGE_RESERVATION {}
impl ::core::clone::Clone for HCN_PORT_RANGE_RESERVATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub type HCN_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows_sys::core::HRESULT, notificationdata: ::windows_sys::core::PCWSTR) -> ()>;
