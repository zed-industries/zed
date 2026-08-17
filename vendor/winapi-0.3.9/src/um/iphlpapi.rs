// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// #include <iprtrmib.h>
// #include <ipexport.h>
// #include <iptypes.h>
// #include <tcpestats.h>
use shared::basetsd::{PULONG64, ULONG64};
use shared::ifdef::NET_LUID;
use shared::ifmib::{PMIB_IFROW, PMIB_IFTABLE};
use shared::ipmib::{
    PMIB_ICMP, PMIB_ICMP_EX, PMIB_IPADDRTABLE, PMIB_IPFORWARDROW, PMIB_IPFORWARDTABLE,
    PMIB_IPNETROW, PMIB_IPNETTABLE, PMIB_IPSTATS
};
use shared::iprtrmib::{TCPIP_OWNER_MODULE_INFO_CLASS, TCP_TABLE_CLASS, UDP_TABLE_CLASS};
use shared::minwindef::{BOOL, BYTE, DWORD, LPDWORD, PDWORD, PUCHAR, PULONG, UINT};
use shared::ntdef::{
    BOOLEAN, HANDLE, LPWSTR, PHANDLE, PVOID, PWSTR, ULONG, ULONGLONG, USHORT, WCHAR,
};
use shared::tcpestats::TCP_ESTATS_TYPE;
use shared::tcpmib::{
    PMIB_TCP6ROW, PMIB_TCP6ROW_OWNER_MODULE, PMIB_TCP6TABLE, PMIB_TCP6TABLE2, PMIB_TCPROW,
    PMIB_TCPROW_OWNER_MODULE, PMIB_TCPSTATS, PMIB_TCPSTATS2, PMIB_TCPTABLE, PMIB_TCPTABLE2
};
use shared::udpmib::{
    PMIB_UDP6ROW_OWNER_MODULE, PMIB_UDP6TABLE, PMIB_UDPROW_OWNER_MODULE, PMIB_UDPSTATS,
    PMIB_UDPSTATS2, PMIB_UDPTABLE
};
use shared::ws2def::{PSOCKADDR, SOCKADDR, SOCKADDR_IN};
use shared::ws2ipdef::SOCKADDR_IN6;
use um::ipexport::{
    IPAddr, IPMask, IP_STATUS, PIP_ADAPTER_INDEX_MAP, PIP_ADAPTER_ORDER_MAP, PIP_INTERFACE_INFO,
    PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS,
};
use um::iptypes::{
    PFIXED_INFO, PIP_ADAPTER_ADDRESSES, PIP_ADAPTER_INFO, PIP_INTERFACE_NAME_INFO,
    PIP_PER_ADAPTER_INFO,
};
use um::minwinbase::{LPOVERLAPPED,OVERLAPPED};
extern "system" {
    pub fn GetNumberOfInterfaces(
        pdwNumIf: PDWORD
    ) -> DWORD;
    pub fn GetIfEntry(
        pIfRow: PMIB_IFROW,
    ) -> DWORD;
    pub fn GetIfTable(
        pIfTable: PMIB_IFTABLE,
        pdwSize: PULONG,
        bOrder: BOOL,
    ) -> DWORD;
    pub fn GetIpAddrTable(
        pIpAddrTable: PMIB_IPADDRTABLE,
        pdwSize: PULONG,
        bOrder: BOOL,
    ) -> DWORD;
    pub fn GetIpNetTable(
        IpNetTable: PMIB_IPNETTABLE,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    pub fn GetIpForwardTable(
        pIpForwardTable: PMIB_IPFORWARDTABLE,
        pdwSize: PULONG,
        bOrder: BOOL,
    ) -> DWORD;
    pub fn GetTcpTable(
        TcpTable: PMIB_TCPTABLE,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    // https://msdn.microsoft.com/en-us/library/windows/desktop/aa365928(v=vs.85).aspx
    pub fn GetExtendedTcpTable(
        pTcpTable: PVOID,
        pdwSize: PDWORD,
        bOrder: BOOL,
        ulAf: ULONG,
        TableClass: TCP_TABLE_CLASS,
        Reserved: ULONG,
    ) -> DWORD;
    pub fn GetOwnerModuleFromTcpEntry(
        pTcpEntry: PMIB_TCPROW_OWNER_MODULE,
        Class: TCPIP_OWNER_MODULE_INFO_CLASS,
        pBuffer: PVOID,
        pdwSize: PDWORD,
    ) -> DWORD;
    pub fn GetUdpTable(
        UdpTable: PMIB_UDPTABLE,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    pub fn GetExtendedUdpTable(
        pUdpTable: PVOID,
        pdwSize: PDWORD,
        bOrder: BOOL,
        ulAf: ULONG,
        TableClass: UDP_TABLE_CLASS,
        Reserved: ULONG,
    ) -> DWORD;
    pub fn GetOwnerModuleFromUdpEntry(
        pUdpEntry: PMIB_UDPROW_OWNER_MODULE,
        Class: TCPIP_OWNER_MODULE_INFO_CLASS,
        pBuffer: PVOID,
        pdwSize: PDWORD,
    ) -> DWORD;
    pub fn GetTcpTable2(
        TcpTable: PMIB_TCPTABLE2,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    // Deprecated APIs, Added for documentation.
    // pub fn AllocateAndGetTcpExTableFromStack() -> DWORD;
    // pub fn AllocateAndGetUdpExTableFromStack() -> DWORD;
    pub fn GetTcp6Table(
        TcpTable: PMIB_TCP6TABLE,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    pub fn GetTcp6Table2(
        TcpTable: PMIB_TCP6TABLE2,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    pub fn GetPerTcpConnectionEStats(
        Row: PMIB_TCPROW,
        EstatsType: TCP_ESTATS_TYPE,
        Rw: PUCHAR,
        RwVersion: ULONG,
        RwSize: ULONG,
        Ros: PUCHAR,
        RosVersion: ULONG,
        RosSize: ULONG,
        Rod: PUCHAR,
        RodVersion: ULONG,
        RodSize: ULONG,
    ) -> ULONG;
    pub fn SetPerTcpConnectionEStats(
        Row: PMIB_TCPROW,
        EstatsType: TCP_ESTATS_TYPE,
        Rw: PUCHAR,
        RwVersion: ULONG,
        RwSize: ULONG,
        Offset: ULONG,
    ) -> ULONG;
    pub fn GetPerTcp6ConnectionEStats(
        Row: PMIB_TCP6ROW,
        EstatsType: TCP_ESTATS_TYPE,
        Rw: PUCHAR,
        RwVersion: ULONG,
        RwSize: ULONG,
        Ros: PUCHAR,
        RosVersion: ULONG,
        RosSize: ULONG,
        Rod: PUCHAR,
        RodVersion: ULONG,
        RodSize: ULONG,
    ) -> ULONG;
    pub fn SetPerTcp6ConnectionEStats(
        Row: PMIB_TCP6ROW,
        EstatsType: TCP_ESTATS_TYPE,
        Rw: PUCHAR,
        RwVersion: ULONG,
        RwSize: ULONG,
        Offset: ULONG,
    ) -> ULONG;
    pub fn GetOwnerModuleFromTcp6Entry(
        pTcpEntry: PMIB_TCP6ROW_OWNER_MODULE,
        Class: TCPIP_OWNER_MODULE_INFO_CLASS,
        pBuffer: PVOID,
        pdwSize: PDWORD,
    ) -> DWORD;
    pub fn GetUdp6Table(
        Udp6Table: PMIB_UDP6TABLE,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    pub fn GetOwnerModuleFromUdp6Entry(
        pUdpEntry: PMIB_UDP6ROW_OWNER_MODULE,
        Class: TCPIP_OWNER_MODULE_INFO_CLASS,
        pBuffer: PVOID,
        pdwSize: PDWORD,
    ) -> DWORD;
    pub fn GetOwnerModuleFromPidAndInfo(
        ulPid: ULONG,
        pInfo: *mut ULONGLONG,
        Class: TCPIP_OWNER_MODULE_INFO_CLASS,
        pBuffer: PVOID,
        pdwSize: PDWORD,
    ) -> DWORD;
    pub fn GetIpStatistics(
        Statistics: PMIB_IPSTATS,
    ) -> ULONG;
    pub fn GetIcmpStatistics(
        Statistics: PMIB_ICMP,
    ) -> ULONG;
    pub fn GetTcpStatistics(
        Statistics: PMIB_TCPSTATS,
    ) -> ULONG;
    pub fn GetUdpStatistics(
        Stats: PMIB_UDPSTATS,
    ) -> ULONG;
    pub fn SetIpStatisticsEx(
        Statistics: PMIB_IPSTATS,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetIpStatisticsEx(
        Statistics: PMIB_IPSTATS,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetIcmpStatisticsEx(
        Statistics: PMIB_ICMP_EX,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetTcpStatisticsEx(
        Statistics: PMIB_TCPSTATS,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetUdpStatisticsEx(
        Statistics: PMIB_UDPSTATS,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetTcpStatisticsEx2(
        Statistics: PMIB_TCPSTATS2,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetUdpStatisticsEx2(
        Statistics: PMIB_UDPSTATS2,
        Family: ULONG,
    ) -> ULONG;
    pub fn SetIfEntry(
        pIfRow: PMIB_IFROW,
    ) -> DWORD;
    pub fn CreateIpForwardEntry(
        pRoute: PMIB_IPFORWARDROW,
    ) -> DWORD;
    pub fn SetIpForwardEntry(
        pRoute: PMIB_IPFORWARDROW,
    ) -> DWORD;
    pub fn DeleteIpForwardEntry(
        pRoute: PMIB_IPFORWARDROW,
    ) -> DWORD;
    pub fn SetIpStatistics(
        pIpStats: PMIB_IPSTATS,
    ) -> DWORD;
    pub fn SetIpTTL(
        nTTL: UINT,
    ) -> DWORD;
    pub fn CreateIpNetEntry(
        pArpEntry: PMIB_IPNETROW,
    ) -> DWORD;
    pub fn SetIpNetEntry(
        pArpEntry: PMIB_IPNETROW,
    ) -> DWORD;
    pub fn DeleteIpNetEntry(
        pArpEntry: PMIB_IPNETROW,
    ) -> DWORD;
    pub fn FlushIpNetTable(
        dwIfIndex: DWORD,
    ) -> DWORD;
    pub fn CreateProxyArpEntry(
        dwAddress: DWORD,
        dwMask: DWORD,
        dwIfIndex: DWORD,
    ) -> DWORD;
    pub fn DeleteProxyArpEntry(
        dwAddress: DWORD,
        dwMask: DWORD,
        dwIfIndex: DWORD,
    ) -> DWORD;
    pub fn SetTcpEntry(
        pTcpRow: PMIB_TCPROW,
    ) -> DWORD;
    pub fn GetInterfaceInfo(
        pIfTable: PIP_INTERFACE_INFO,
        dwOutBufLen: PULONG,
    ) -> DWORD;
    pub fn GetUniDirectionalAdapterInfo(
        pIPIfInfo: PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS,
        dwOutBufLen: PULONG,
    ) -> DWORD;
    pub fn NhpAllocateAndGetInterfaceInfoFromStack(
        ppTable: *mut PIP_INTERFACE_NAME_INFO,
        pdwCount: PDWORD,
        bOrder: BOOL,
        hHeap: HANDLE,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn GetBestInterface(
        dwDestAddr: IPAddr,
        pdwBestIfIndex: PDWORD,
    ) -> DWORD;
    pub fn GetBestInterfaceEx(
        pDestAddr: PSOCKADDR,
        pdwBestIfIndex: PDWORD,
    ) -> DWORD;
    pub fn GetBestRoute(
        dwDestAddr: DWORD,
        dwSourceAddr: DWORD,
        pBestRoute: PMIB_IPFORWARDROW,
    ) -> DWORD;
    pub fn NotifyAddrChange(
        Handle: PHANDLE,
        overlapped: LPOVERLAPPED,
    ) -> DWORD;
    pub fn NotifyRouteChange(
        Handle: PHANDLE,
        overlapped: LPOVERLAPPED,
    ) -> DWORD;
    pub fn CancelIPChangeNotify(
        notifyOverlapped: LPOVERLAPPED
    ) -> BOOL;
    pub fn GetAdapterIndex(
        AdapterName: LPWSTR,
        IfIndex: PULONG,
    ) -> DWORD;
    pub fn AddIPAddress(
        Address: IPAddr,
        IpMask: IPMask,
        IfIndex: DWORD,
        NTEContext: PULONG,
        NTEInstance: PULONG,
    ) -> DWORD;
    pub fn DeleteIPAddress(
        NTEContext: ULONG,
    ) -> DWORD;
    pub fn GetNetworkParams(
        pFixedInfo: PFIXED_INFO,
        pOutBufLen: PULONG,
    ) -> DWORD;
    pub fn GetAdaptersInfo(
        AdapterInfo: PIP_ADAPTER_INFO,
        SizePointer: PULONG,
    ) -> ULONG;
    pub fn GetAdapterOrderMap() -> PIP_ADAPTER_ORDER_MAP;
    pub fn GetAdaptersAddresses(
        Family: ULONG,
        Flags: ULONG,
        Reserved: PVOID,
        AdapterAddresses: PIP_ADAPTER_ADDRESSES,
        SizePointer: PULONG,
    ) -> ULONG;
    pub fn GetPerAdapterInfo(
        IfIndex: ULONG,
        pPerAdapterInfo: PIP_PER_ADAPTER_INFO,
        pOutBufLen: PULONG,
    ) -> DWORD;
}
STRUCT!{struct INTERFACE_TIMESTAMP_CAPABILITY_FLAGS {
    PtpV2OverUdpIPv4EventMsgReceiveHw: BOOLEAN,
    PtpV2OverUdpIPv4AllMsgReceiveHw: BOOLEAN,
    PtpV2OverUdpIPv4EventMsgTransmitHw: BOOLEAN,
    PtpV2OverUdpIPv4AllMsgTransmitHw: BOOLEAN,
    PtpV2OverUdpIPv6EventMsgReceiveHw: BOOLEAN,
    PtpV2OverUdpIPv6AllMsgReceiveHw: BOOLEAN,
    PtpV2OverUdpIPv6EventMsgTransmitHw: BOOLEAN,
    PtpV2OverUdpIPv6AllMsgTransmitHw: BOOLEAN,
    AllReceiveHw: BOOLEAN,
    AllTransmitHw: BOOLEAN,
    TaggedTransmitHw: BOOLEAN,
    AllReceiveSw: BOOLEAN,
    AllTransmitSw: BOOLEAN,
    TaggedTransmitSw: BOOLEAN,
}}
pub type PINTERFACE_TIMESTAMP_CAPABILITY_FLAGS = *mut INTERFACE_TIMESTAMP_CAPABILITY_FLAGS;
STRUCT!{struct INTERFACE_TIMESTAMP_CAPABILITIES {
    Version: ULONG,
    HardwareClockFrequencyHz: ULONG64,
    CrossTimestamp: BOOLEAN,
    Reserved1: ULONG64,
    Reserved2: ULONG64,
    TimestampFlags: INTERFACE_TIMESTAMP_CAPABILITY_FLAGS,
}}
pub type PINTERFACE_TIMESTAMP_CAPABILITIES = *mut INTERFACE_TIMESTAMP_CAPABILITIES;
STRUCT!{struct INTERFACE_HARDWARE_CROSSTIMESTAMP {
    Version: ULONG,
    Flags: ULONG,
    SystemTimestamp1: ULONG64,
    HardwareClockTimestamp: ULONG64,
    SystemTimestamp2: ULONG64,
}}
pub type PINTERFACE_HARDWARE_CROSSTIMESTAMP = *mut INTERFACE_HARDWARE_CROSSTIMESTAMP;
DECLARE_HANDLE!{HIFTIMESTAMPCHANGE, HIFTIMESTAMPCHANGE__}
extern "system" {
    pub fn GetInterfaceCurrentTimestampCapabilities(
        InterfaceLuid: *const NET_LUID,
        TimestampCapabilite: PINTERFACE_TIMESTAMP_CAPABILITIES,
    ) -> DWORD;
    pub fn GetInterfaceHardwareTimestampCapabilities(
        InterfaceLuid: *const NET_LUID,
        TimestampCapabilite: PINTERFACE_TIMESTAMP_CAPABILITIES,
    ) -> DWORD;
    pub fn CaptureInterfaceHardwareCrossTimestamp(
        InterfaceLuid: *const NET_LUID,
        CrossTimestamp: PINTERFACE_HARDWARE_CROSSTIMESTAMP,
    ) -> DWORD;
}
FN!{stdcall INTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK(
    CallerContext: PVOID,
) -> ()}
pub type PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = *mut
    INTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK;
extern "system" {
    pub fn NotifyIfTimestampConfigChange(
        CallerContext: PVOID,
        Callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK,
        NotificationHandle: *mut HIFTIMESTAMPCHANGE,
    ) -> DWORD;
    pub fn CancelIfTimestampConfigChange(
        NotificationHandle: HIFTIMESTAMPCHANGE,
    );
    pub fn IpReleaseAddress(
        AdapterInfo: PIP_ADAPTER_INDEX_MAP,
    ) -> DWORD;
    pub fn IpRenewAddress(
        AdapterInfo: PIP_ADAPTER_INDEX_MAP,
    ) -> DWORD;
    pub fn SendARP(
        DestIP: IPAddr,
        SrcIP: IPAddr,
        pMacAddr: PVOID,
        PhyAddrLen: PULONG,
    ) -> DWORD;
    pub fn GetRTTAndHopCount(
        DestIpAddress: IPAddr,
        HopCount: PULONG,
        MaxHops: ULONG,
        RTT: PULONG,
    ) -> BOOL;
    pub fn GetFriendlyIfIndex(
        IfIndex: DWORD,
    ) -> DWORD;
    pub fn EnableRouter(
        pHandle: *mut HANDLE,
        pOverlapped: *mut OVERLAPPED,
    ) -> DWORD;
    pub fn UnenableRouter(
        pOverlapped: *mut OVERLAPPED,
        lpdwEnableCount: LPDWORD,
    ) -> DWORD;
    pub fn DisableMediaSense(
        pHandle: *mut HANDLE,
        pOverLapped: *mut OVERLAPPED,
    ) -> DWORD;
    pub fn RestoreMediaSense(
        pOverlapped: *mut OVERLAPPED,
        lpdwEnableCount: LPDWORD,
    ) -> DWORD;
    pub fn GetIpErrorString(
        ErrorCode: IP_STATUS,
        Buffer: PWSTR,
        Size: PDWORD,
    ) -> DWORD;
    pub fn ResolveNeighbor(
        NetworkAddress: *mut SOCKADDR,
        PhysicalAddress: PVOID,
        PhysicalAddressLength: PULONG,
    ) -> ULONG;
    pub fn CreatePersistentTcpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
        Token: PULONG64,
    ) -> ULONG;
    pub fn CreatePersistentUdpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
        Token: PULONG64,
    ) -> ULONG;
    pub fn DeletePersistentTcpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
    ) -> ULONG;
    pub fn DeletePersistentUdpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
    ) -> ULONG;
    pub fn LookupPersistentTcpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
        Token: PULONG64,
    ) -> ULONG;
    pub fn LookupPersistentUdpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
        Token: PULONG64,
    ) -> ULONG;
}
ENUM!{enum NET_ADDRESS_FORMAT {
    NET_ADDRESS_FORMAT_UNSPECIFIED = 0,
    NET_ADDRESS_DNS_NAME = 1,
    NET_ADDRESS_IPV4 = 2,
    NET_ADDRESS_IPV6 = 3,
}}
pub const DNS_MAX_NAME_BUFFER_LENGTH: usize = 256;
STRUCT!{struct NET_ADDRESS_INFO_u_s {
    Address: [WCHAR; DNS_MAX_NAME_BUFFER_LENGTH],
    Port: [WCHAR; 6],
}}
UNION!{union NET_ADDRESS_INFO_u {
    [u32; 131],
    NamedAddress NamedAddress_mut: NET_ADDRESS_INFO_u_s,
    Ipv4Address Ipv4Address_mut: SOCKADDR_IN,
    Ipv6Address Ipv6Address_mut: SOCKADDR_IN6,
    IpAddress IpAddress_mut: SOCKADDR,
}}
STRUCT!{struct NET_ADDRESS_INFO {
    Format: NET_ADDRESS_FORMAT,
    u: NET_ADDRESS_INFO_u,
}}
pub type PNET_ADDRESS_INFO = *mut NET_ADDRESS_INFO;
extern "system" {
    // #if defined (_WS2DEF_) && defined (_WS2IPDEF_) && defined(_WINDNS_INCLUDED_)
    pub fn ParseNetworkString(
        NetworkString: *const *mut WCHAR,
        Types: DWORD,
        AddressInfo: PNET_ADDRESS_INFO,
        PortNumber: *mut USHORT,
        PrefixLength: *mut BYTE,
    ) -> DWORD;
}
