windows_targets::link!("wnvapi.dll" "system" fn WnvOpen() -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("wnvapi.dll" "system" fn WnvRequestNotification(wnvhandle : super::super::Foundation:: HANDLE, notificationparam : *mut WNV_NOTIFICATION_PARAM, overlapped : *mut super::super::System::IO:: OVERLAPPED, bytestransferred : *mut u32) -> u32);
pub const WNV_API_MAJOR_VERSION_1: u32 = 1u32;
pub const WNV_API_MINOR_VERSION_0: u32 = 0u32;
pub type WNV_CA_NOTIFICATION_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    pub MACAddress: super::super::Networking::WinSock::DL_EUI48,
    pub CAFamily: super::super::Networking::WinSock::ADDRESS_FAMILY,
    pub CA: WNV_IP_ADDRESS,
    pub VirtualSubnetId: u32,
    pub PAFamily: super::super::Networking::WinSock::ADDRESS_FAMILY,
    pub PA: WNV_IP_ADDRESS,
    pub NotificationReason: WNV_CA_NOTIFICATION_TYPE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct WNV_IP_ADDRESS {
    pub IP: WNV_IP_ADDRESS_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for WNV_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union WNV_IP_ADDRESS_0 {
    pub v4: super::super::Networking::WinSock::IN_ADDR,
    pub v6: super::super::Networking::WinSock::IN6_ADDR,
    pub Addr: [u8; 16],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for WNV_IP_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNV_NOTIFICATION_PARAM {
    pub Header: WNV_OBJECT_HEADER,
    pub NotificationType: WNV_NOTIFICATION_TYPE,
    pub PendingNotifications: u32,
    pub Buffer: *mut u8,
}
impl Default for WNV_NOTIFICATION_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WNV_NOTIFICATION_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct WNV_OBJECT_CHANGE_PARAM {
    pub ObjectType: WNV_OBJECT_TYPE,
    pub ObjectParam: WNV_OBJECT_CHANGE_PARAM_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for WNV_OBJECT_CHANGE_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union WNV_OBJECT_CHANGE_PARAM_0 {
    pub ProviderAddressChange: WNV_PROVIDER_ADDRESS_CHANGE_PARAM,
    pub CustomerAddressChange: WNV_CUSTOMER_ADDRESS_CHANGE_PARAM,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for WNV_OBJECT_CHANGE_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WNV_OBJECT_HEADER {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub Size: u32,
}
pub type WNV_OBJECT_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct WNV_POLICY_MISMATCH_PARAM {
    pub CAFamily: super::super::Networking::WinSock::ADDRESS_FAMILY,
    pub PAFamily: super::super::Networking::WinSock::ADDRESS_FAMILY,
    pub VirtualSubnetId: u32,
    pub CA: WNV_IP_ADDRESS,
    pub PA: WNV_IP_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for WNV_POLICY_MISMATCH_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    pub PAFamily: super::super::Networking::WinSock::ADDRESS_FAMILY,
    pub PA: WNV_IP_ADDRESS,
    pub AddressState: super::super::Networking::WinSock::NL_DAD_STATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct WNV_REDIRECT_PARAM {
    pub CAFamily: super::super::Networking::WinSock::ADDRESS_FAMILY,
    pub PAFamily: super::super::Networking::WinSock::ADDRESS_FAMILY,
    pub NewPAFamily: super::super::Networking::WinSock::ADDRESS_FAMILY,
    pub VirtualSubnetId: u32,
    pub CA: WNV_IP_ADDRESS,
    pub PA: WNV_IP_ADDRESS,
    pub NewPA: WNV_IP_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for WNV_REDIRECT_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WnvCustomerAddressAdded: WNV_CA_NOTIFICATION_TYPE = 0i32;
pub const WnvCustomerAddressDeleted: WNV_CA_NOTIFICATION_TYPE = 1i32;
pub const WnvCustomerAddressMax: WNV_CA_NOTIFICATION_TYPE = 3i32;
pub const WnvCustomerAddressMoved: WNV_CA_NOTIFICATION_TYPE = 2i32;
pub const WnvCustomerAddressType: WNV_OBJECT_TYPE = 1i32;
pub const WnvNotificationTypeMax: WNV_NOTIFICATION_TYPE = 3i32;
pub const WnvObjectChangeType: WNV_NOTIFICATION_TYPE = 2i32;
pub const WnvObjectTypeMax: WNV_OBJECT_TYPE = 2i32;
pub const WnvPolicyMismatchType: WNV_NOTIFICATION_TYPE = 0i32;
pub const WnvProviderAddressType: WNV_OBJECT_TYPE = 0i32;
pub const WnvRedirectType: WNV_NOTIFICATION_TYPE = 1i32;
