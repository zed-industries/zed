windows_targets::link!("dhcpcsvc.dll" "system" fn McastApiCleanup());
windows_targets::link!("dhcpcsvc.dll" "system" fn McastApiStartup(version : *mut u32) -> u32);
windows_targets::link!("dhcpcsvc.dll" "system" fn McastEnumerateScopes(addrfamily : u16, requery : windows_sys::core::BOOL, pscopelist : *mut MCAST_SCOPE_ENTRY, pscopelen : *mut u32, pscopecount : *mut u32) -> u32);
windows_targets::link!("dhcpcsvc.dll" "system" fn McastGenUID(prequestid : *mut MCAST_CLIENT_UID) -> u32);
windows_targets::link!("dhcpcsvc.dll" "system" fn McastReleaseAddress(addrfamily : u16, prequestid : *mut MCAST_CLIENT_UID, preleaserequest : *mut MCAST_LEASE_REQUEST) -> u32);
windows_targets::link!("dhcpcsvc.dll" "system" fn McastRenewAddress(addrfamily : u16, prequestid : *mut MCAST_CLIENT_UID, prenewrequest : *mut MCAST_LEASE_REQUEST, prenewresponse : *mut MCAST_LEASE_RESPONSE) -> u32);
windows_targets::link!("dhcpcsvc.dll" "system" fn McastRequestAddress(addrfamily : u16, prequestid : *mut MCAST_CLIENT_UID, pscopectx : *mut MCAST_SCOPE_CTX, paddrrequest : *mut MCAST_LEASE_REQUEST, paddrresponse : *mut MCAST_LEASE_RESPONSE) -> u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union IPNG_ADDRESS {
    pub IpAddrV4: u32,
    pub IpAddrV6: [u8; 16],
}
impl Default for IPNG_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCAST_API_CURRENT_VERSION: i32 = 1i32;
pub const MCAST_API_VERSION_0: i32 = 0i32;
pub const MCAST_API_VERSION_1: i32 = 1i32;
pub const MCAST_CLIENT_ID_LEN: u32 = 17u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MCAST_CLIENT_UID {
    pub ClientUID: *mut u8,
    pub ClientUIDLength: u32,
}
impl Default for MCAST_CLIENT_UID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MCAST_LEASE_REQUEST {
    pub LeaseStartTime: i32,
    pub MaxLeaseStartTime: i32,
    pub LeaseDuration: u32,
    pub MinLeaseDuration: u32,
    pub ServerAddress: IPNG_ADDRESS,
    pub MinAddrCount: u16,
    pub AddrCount: u16,
    pub pAddrBuf: *mut u8,
}
impl Default for MCAST_LEASE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MCAST_LEASE_RESPONSE {
    pub LeaseStartTime: i32,
    pub LeaseEndTime: i32,
    pub ServerAddress: IPNG_ADDRESS,
    pub AddrCount: u16,
    pub pAddrBuf: *mut u8,
}
impl Default for MCAST_LEASE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MCAST_SCOPE_CTX {
    pub ScopeID: IPNG_ADDRESS,
    pub Interface: IPNG_ADDRESS,
    pub ServerID: IPNG_ADDRESS,
}
impl Default for MCAST_SCOPE_CTX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MCAST_SCOPE_ENTRY {
    pub ScopeCtx: MCAST_SCOPE_CTX,
    pub LastAddr: IPNG_ADDRESS,
    pub TTL: u32,
    pub ScopeDesc: super::super::Foundation::UNICODE_STRING,
}
impl Default for MCAST_SCOPE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
