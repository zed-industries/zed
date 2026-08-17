// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_char, c_int};
use shared::minwindef::{BOOL, DWORD, INT, LPDWORD, LPINT, LPVOID, ULONG};
use shared::mswsockdef::{PRIORESULT, PRIO_BUF, RIO_BUFFERID, RIO_CQ, RIO_RQ};
use shared::ws2def::{IOC_VENDOR, IOC_WS2, LPWSAMSG, SOCKADDR};
use um::minwinbase::LPOVERLAPPED;
use um::winnt::{CHAR, HANDLE, LARGE_INTEGER, PCHAR, PVOID, WCHAR};
use um::winsock2::{
    LPWSAOVERLAPPED, LPWSAOVERLAPPED_COMPLETION_ROUTINE, LPWSAPOLLFD, SOCKET, WSAESETSERVICEOP,
    WSAPOLLFD,
};
pub const SO_CONNDATA: c_int = 0x7000;
pub const SO_CONNOPT: c_int = 0x7001;
pub const SO_DISCDATA: c_int = 0x7002;
pub const SO_DISCOPT: c_int = 0x7003;
pub const SO_CONNDATALEN: c_int = 0x7004;
pub const SO_CONNOPTLEN: c_int = 0x7005;
pub const SO_DISCDATALEN: c_int = 0x7006;
pub const SO_DISCOPTLEN: c_int = 0x7007;
pub const SO_OPENTYPE: c_int = 0x7008;
pub const SO_SYNCHRONOUS_ALERT: DWORD = 0x10;
pub const SO_SYNCHRONOUS_NONALERT: DWORD = 0x20;
pub const SO_MAXDG: c_int = 0x7009;
pub const SO_MAXPATHDG: c_int = 0x700A;
pub const SO_UPDATE_ACCEPT_CONTEXT: c_int = 0x700B;
pub const SO_CONNECT_TIME: c_int = 0x700C;
pub const SO_UPDATE_CONNECT_CONTEXT: c_int = 0x7010;
pub const TCP_BSDURGENT: c_int = 0x7000;
pub const SIO_UDP_CONNRESET: DWORD = _WSAIOW!(IOC_VENDOR, 12);
pub const SIO_SOCKET_CLOSE_NOTIFY: DWORD = _WSAIOW!(IOC_VENDOR, 13);
pub const SIO_UDP_NETRESET: DWORD = _WSAIOW!(IOC_VENDOR, 15);
extern "system" {
    pub fn WSARecvEx(
        s: SOCKET,
        buf: *mut c_char,
        len: c_int,
        flags: *mut c_int,
    ) -> c_int;
}
STRUCT!{struct TRANSMIT_FILE_BUFFERS {
    Head: LPVOID,
    HeadLength: DWORD,
    Tail: LPVOID,
    TailLength: DWORD,
}}
pub type PTRANSMIT_FILE_BUFFERS = *mut TRANSMIT_FILE_BUFFERS;
pub type LPTRANSMIT_FILE_BUFFERS = *mut TRANSMIT_FILE_BUFFERS;
pub const TF_DISCONNECT: DWORD = 0x01;
pub const TF_REUSE_SOCKET: DWORD = 0x02;
pub const TF_WRITE_BEHIND: DWORD = 0x04;
pub const TF_USE_DEFAULT_WORKER: DWORD = 0x00;
pub const TF_USE_SYSTEM_THREAD: DWORD = 0x10;
pub const TF_USE_KERNEL_APC: DWORD = 0x20;
extern "system" {
    pub fn TransmitFile(
        hSocket: SOCKET,
        hFile: HANDLE,
        nNumberOfBytesToWrite: DWORD,
        nNumberOfBytesPerSend: DWORD,
        lpOverlapped: LPOVERLAPPED,
        lpTransmitBuffers: LPTRANSMIT_FILE_BUFFERS,
        dwReserved: DWORD,
    ) -> BOOL;
    pub fn AcceptEx(
        sListenSocket: SOCKET,
        sAcceptSocket: SOCKET,
        lpOutputBuffer: PVOID,
        dwReceiveDataLength: DWORD,
        dwLocalAddressLength: DWORD,
        dwRemoteAddressLength: DWORD,
        lpdwBytesReceived: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn GetAcceptExSockaddrs(
        lpOutputBuffer: PVOID,
        dwReceiveDataLength: DWORD,
        dwLocalAddressLength: DWORD,
        dwRemoteAddressLength: DWORD,
        LocalSockaddr: *mut *mut SOCKADDR,
        LocalSockaddrLength: LPINT,
        RemoteSockaddr: *mut *mut SOCKADDR,
        RemoteSockaddrLength: LPINT,
    );
}
FN!{stdcall LPFN_TRANSMITFILE(
    hSocket: SOCKET,
    hFile: HANDLE,
    nNumberOfBytesToWrite: DWORD,
    nNumberOfBytesPerSend: DWORD,
    lpOverlapped: LPOVERLAPPED,
    lpTransmitBuffers: LPTRANSMIT_FILE_BUFFERS,
    dwReserved: DWORD,
) -> BOOL}
DEFINE_GUID!{WSAID_TRANSMITFILE,
    0xb5367df0, 0xcbac, 0x11cf, 0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92}
FN!{stdcall LPFN_ACCEPTEX(
    sListenSocket: SOCKET,
    sAcceptSocket: SOCKET,
    lpOutputBuffer: PVOID,
    dwReceiveDataLength: DWORD,
    dwLocalAddressLength: DWORD,
    dwRemoteAddressLength: DWORD,
    lpdwBytesReceived: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL}
DEFINE_GUID!{WSAID_ACCEPTEX,
    0xb5367df1, 0xcbac, 0x11cf, 0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92}
FN!{stdcall LPFN_GETACCEPTEXSOCKADDRS(
    lpOutputBuffer: PVOID,
    dwReceiveDataLength: DWORD,
    dwLocalAddressLength: DWORD,
    dwRemoteAddressLength: DWORD,
    LocalSockaddr: *mut *mut SOCKADDR,
    LocalSockaddrLength: LPINT,
    RemoteSockaddr: *mut *mut SOCKADDR,
    RemoteSockaddrLength: LPINT,
) -> ()}
DEFINE_GUID!{WSAID_GETACCEPTEXSOCKADDRS,
    0xb5367df2, 0xcbac, 0x11cf, 0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92}
pub const TP_ELEMENT_MEMORY: ULONG = 1;
pub const TP_ELEMENT_FILE: ULONG = 2;
pub const TP_ELEMENT_EOP: ULONG = 4;
STRUCT!{struct TRANSMIT_PACKETS_ELEMENT_u_s {
    nFileOffset: LARGE_INTEGER,
    hFile: HANDLE,
}}
UNION!{union TRANSMIT_PACKETS_ELEMENT_u {
    [u64; 2],
    s s_mut: TRANSMIT_PACKETS_ELEMENT_u_s,
    pBuffer pBuffer_mut: PVOID,
}}
STRUCT!{struct TRANSMIT_PACKETS_ELEMENT {
    dwElFlags: ULONG,
    cLength: ULONG,
    u: TRANSMIT_PACKETS_ELEMENT_u,
}}
pub type PTRANSMIT_PACKETS_ELEMENT = *mut TRANSMIT_PACKETS_ELEMENT;
pub type LPTRANSMIT_PACKETS_ELEMENT = *mut TRANSMIT_PACKETS_ELEMENT;
pub const TP_DISCONNECT: DWORD = TF_DISCONNECT;
pub const TP_REUSE_SOCKET: DWORD = TF_REUSE_SOCKET;
pub const TP_USE_DEFAULT_WORKER: DWORD = TF_USE_DEFAULT_WORKER;
pub const TP_USE_SYSTEM_THREAD: DWORD = TF_USE_SYSTEM_THREAD;
pub const TP_USE_KERNEL_APC: DWORD = TF_USE_KERNEL_APC;
FN!{stdcall LPFN_TRANSMITPACKETS(
    hSocket: SOCKET,
    lpPacketArray: LPTRANSMIT_PACKETS_ELEMENT,
    nElementCount: DWORD,
    nSendSize: DWORD,
    lpOverlapped: LPOVERLAPPED,
    dwFlags: DWORD,
) -> BOOL}
DEFINE_GUID!{WSAID_TRANSMITPACKETS,
    0xd9689da0, 0x1f90, 0x11d3, 0x99, 0x71, 0x00, 0xc0, 0x4f, 0x68, 0xc8, 0x76}
FN!{stdcall LPFN_CONNECTEX(
    s: SOCKET,
    name: *const SOCKADDR,
    namelen: c_int,
    lpSendBuffer: PVOID,
    dwSendDataLength: DWORD,
    lpdwBytesSent: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL}
DEFINE_GUID!{WSAID_CONNECTEX,
    0x25a207b9, 0xddf3, 0x4660, 0x8e, 0xe9, 0x76, 0xe5, 0x8c, 0x74, 0x06, 0x3e}
FN!{stdcall LPFN_DISCONNECTEX(
    s: SOCKET,
    lpOverlapped: LPOVERLAPPED,
    dwFlags: DWORD,
    dwReserved: DWORD,
) -> BOOL}
DEFINE_GUID!{WSAID_DISCONNECTEX,
    0x7fda2e11, 0x8630, 0x436f, 0xa0, 0x31, 0xf5, 0x36, 0xa6, 0xee, 0xc1, 0x57}
pub const DE_REUSE_SOCKET: DWORD = TF_REUSE_SOCKET;
DEFINE_GUID!{NLA_NAMESPACE_GUID,
    0x6642243a, 0x3ba8, 0x4aa6, 0xba, 0xa5, 0x2e, 0xb, 0xd7, 0x1f, 0xdd, 0x83}
DEFINE_GUID!{NLA_SERVICE_CLASS_GUID,
    0x37e515, 0xb5c9, 0x4a43, 0xba, 0xda, 0x8b, 0x48, 0xa8, 0x7a, 0xd2, 0x39}
pub const NLA_ALLUSERS_NETWORK: WSAESETSERVICEOP = 0x00000001;
pub const NLA_FRIENDLY_NAME: WSAESETSERVICEOP = 0x00000002;
ENUM!{enum NLA_BLOB_DATA_TYPE {
    NLA_RAW_DATA = 0,
    NLA_INTERFACE = 1,
    NLA_802_1X_LOCATION = 2,
    NLA_CONNECTIVITY = 3,
    NLA_ICS = 4,
}}
pub type PNLA_BLOB_DATA_TYPE = *mut NLA_BLOB_DATA_TYPE;
ENUM!{enum NLA_CONNECTIVITY_TYPE {
    NLA_NETWORK_AD_HOC = 0,
    NLA_NETWORK_MANAGED = 1,
    NLA_NETWORK_UNMANAGED = 2,
    NLA_NETWORK_UNKNOWN = 3,
}}
pub type PNLA_CONNECTIVITY_TYPE = *mut NLA_CONNECTIVITY_TYPE;
ENUM!{enum NLA_INTERNET {
    NLA_INTERNET_UNKNOWN = 0,
    NLA_INTERNET_NO = 1,
    NLA_INTERNET_YES = 2,
}}
pub type PNLA_INTERNET = *mut NLA_INTERNET;
STRUCT!{struct NLA_BLOB_s {
    type_: NLA_BLOB_DATA_TYPE,
    dwSize: DWORD,
    nextOffset: DWORD,
}}
STRUCT!{struct NLA_BLOB_u_s1 {
    dwType: DWORD,
    dwSpeed: DWORD,
    adapterName: [CHAR; 1],
}}
STRUCT!{struct NLA_BLOB_u_s2 {
    information: [CHAR; 1],
}}
STRUCT!{struct NLA_BLOB_u_s3 {
    type_: NLA_CONNECTIVITY_TYPE,
    internet: NLA_INTERNET,
}}
STRUCT!{struct NLA_BLOB_u_s4_s {
    speed: DWORD,
    type_: DWORD,
    state: DWORD,
    machineName: [WCHAR; 256],
    sharedAdapterName: [WCHAR; 256],
}}
STRUCT!{struct NLA_BLOB_u_s4 {
    remote: NLA_BLOB_u_s4_s,
}}
UNION!{union NLA_BLOB_u {
    [u32; 259],
    rawData rawData_mut: [CHAR; 1],
    interfaceData interfaceData_mut: NLA_BLOB_u_s1,
    locationData locationData_mut: NLA_BLOB_u_s2,
    connectivity connectivity_mut: NLA_BLOB_u_s3,
    ICS ICS_mut: NLA_BLOB_u_s4,
}}
STRUCT!{struct NLA_BLOB {
    header: NLA_BLOB_s,
    data: NLA_BLOB_u,
}}
pub type PNLA_BLOB = *mut NLA_BLOB;
pub type LPNLA_BLOB = *mut NLA_BLOB;
FN!{stdcall LPFN_WSARECVMSG(
    s: SOCKET,
    lpMsg: LPWSAMSG,
    lpdwNumberOfBytesRecvd: LPDWORD,
    lpOverlapped: LPWSAOVERLAPPED,
    lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
) -> INT}
DEFINE_GUID!{WSAID_WSARECVMSG,
    0xf689d7c8, 0x6f1f, 0x436b, 0x8a, 0x53, 0xe5, 0x4f, 0xe3, 0x51, 0xc3, 0x22}
pub const SIO_BSP_HANDLE: DWORD = _WSAIOR!(IOC_WS2, 27);
pub const SIO_BSP_HANDLE_SELECT: DWORD = _WSAIOR!(IOC_WS2, 28);
pub const SIO_BSP_HANDLE_POLL: DWORD = _WSAIOR!(IOC_WS2, 29);
pub const SIO_BASE_HANDLE: DWORD = _WSAIOR!(IOC_WS2, 34);
pub const SIO_EXT_SELECT: DWORD = _WSAIORW!(IOC_WS2, 30);
pub const SIO_EXT_POLL: DWORD = _WSAIORW!(IOC_WS2, 31);
pub const SIO_EXT_SENDMSG: DWORD = _WSAIORW!(IOC_WS2, 32);
STRUCT!{struct WSAPOLLDATA {
    result: c_int,
    fds: ULONG,
    timeout: INT,
    fdArray: *mut WSAPOLLFD,
}}
pub type LPWSAPOLLDATA = *mut WSAPOLLDATA;
STRUCT!{struct WSASENDMSG {
    lpMsg: LPWSAMSG,
    dwFlags: DWORD,
    lpNumberOfBytesSent: LPDWORD,
    lpOverlapped: LPWSAOVERLAPPED,
    lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}}
pub type LPWSASENDMSG = *mut WSASENDMSG;
FN!{stdcall LPFN_WSASENDMSG(
    s: SOCKET,
    lpMsg: LPWSAMSG,
    dwFlags: DWORD,
    lpNumberOfBytesSent: LPDWORD,
    lpOverlapped: LPWSAOVERLAPPED,
    lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
) -> INT}
DEFINE_GUID!{WSAID_WSASENDMSG,
    0xa441e712, 0x754f, 0x43ca, 0x84, 0xa7, 0x0d, 0xee, 0x44, 0xcf, 0x60, 0x6d}
FN!{stdcall LPFN_WSAPOLL(
    fdarray: LPWSAPOLLFD,
    nfds: ULONG,
    timeout: INT,
) -> INT}
DEFINE_GUID!{WSAID_WSAPOLL,
    0x18C76F85, 0xDC66, 0x4964, 0x97, 0x2E, 0x23, 0xC2, 0x72, 0x38, 0x31, 0x2B}
FN!{stdcall LPFN_RIORECEIVE(
    SocketQueue: RIO_RQ,
    pData: PRIO_BUF,
    DataBufferCount: ULONG,
    Flags: DWORD,
    RequestContext: PVOID,
) -> BOOL}
FN!{stdcall LPFN_RIORECEIVEEX(
    SocketQueue: RIO_RQ,
    pData: PRIO_BUF,
    DataBufferCount: ULONG,
    pLocalAddress: PRIO_BUF,
    pRemoteAddress: PRIO_BUF,
    pControlContext: PRIO_BUF,
    pFlags: PRIO_BUF,
    Flags: DWORD,
    RequestContext: PVOID,
) -> c_int}
FN!{stdcall LPFN_RIOSEND(
    SocketQueue: RIO_RQ,
    pData: PRIO_BUF,
    DataBufferCount: ULONG,
    Flags: DWORD,
    RequestContext: PVOID,
) -> BOOL}
FN!{stdcall LPFN_RIOSENDEX(
    SocketQueue: RIO_RQ,
    pData: PRIO_BUF,
    DataBufferCount: ULONG,
    pLocalAddress: PRIO_BUF,
    pRemoteAddress: PRIO_BUF,
    pControlContext: PRIO_BUF,
    pFlags: PRIO_BUF,
    Flags: DWORD,
    RequestContext: PVOID,
) -> BOOL}
FN!{stdcall LPFN_RIOCLOSECOMPLETIONQUEUE(
    CQ: RIO_CQ,
) -> ()}
ENUM!{enum RIO_NOTIFICATION_COMPLETION_TYPE {
    RIO_EVENT_COMPLETION = 1,
    RIO_IOCP_COMPLETION = 2,
}}
pub type PRIO_NOTIFICATION_COMPLETION_TYPE = *mut RIO_NOTIFICATION_COMPLETION_TYPE;
STRUCT!{struct RIO_NOTIFICATION_COMPLETION_u_s1 {
    EventHandle: HANDLE,
    NotifyReset: BOOL,
}}
STRUCT!{struct RIO_NOTIFICATION_COMPLETION_u_s2 {
    IocpHandle: HANDLE,
    CompletionKey: PVOID,
    Overlapped: PVOID,
}}
UNION!{union RIO_NOTIFICATION_COMPLETION_u {
    [u32; 3] [u64; 3],
    Event Event_mut: RIO_NOTIFICATION_COMPLETION_u_s1,
    Iocp Iocp_mut: RIO_NOTIFICATION_COMPLETION_u_s2,
}}
STRUCT!{struct RIO_NOTIFICATION_COMPLETION {
    Type: RIO_NOTIFICATION_COMPLETION_TYPE,
    u: RIO_NOTIFICATION_COMPLETION_u,
}}
pub type PRIO_NOTIFICATION_COMPLETION = *mut RIO_NOTIFICATION_COMPLETION;
FN!{stdcall LPFN_RIOCREATECOMPLETIONQUEUE(
    QueueSize: DWORD,
    NotificationCompletion: PRIO_NOTIFICATION_COMPLETION,
) -> RIO_CQ}
FN!{stdcall LPFN_RIOCREATEREQUESTQUEUE(
    Socket: SOCKET,
    MaxOutstandingReceive: ULONG,
    MaxReceiveDataBuffers: ULONG,
    MaxOutstandingSend: ULONG,
    MaxSendDataBuffers: ULONG,
    ReceiveCQ: RIO_CQ,
    SendCQ: RIO_CQ,
    SocketContext: PVOID,
) -> RIO_RQ}
FN!{stdcall LPFN_RIODEQUEUECOMPLETION(
    CQ: RIO_CQ,
    Array: PRIORESULT,
    ArraySize: ULONG,
) -> ULONG}
FN!{stdcall LPFN_RIODEREGISTERBUFFER(
    BufferId: RIO_BUFFERID,
) -> ()}
FN!{stdcall LPFN_RIONOTIFY(
    CQ: RIO_CQ,
) -> INT}
FN!{stdcall LPFN_RIOREGISTERBUFFER(
    DataBuffer: PCHAR,
    DataLength: DWORD,
) -> RIO_BUFFERID}
FN!{stdcall LPFN_RIORESIZECOMPLETIONQUEUE(
    CQ: RIO_CQ,
    QueueSize: DWORD,
) -> BOOL}
FN!{stdcall LPFN_RIORESIZEREQUESTQUEUE(
    RQ: RIO_RQ,
    MaxOutstandingReceive: DWORD,
    MaxOutstandingSend: DWORD,
) -> BOOL}
STRUCT!{struct RIO_EXTENSION_FUNCTION_TABLE {
    cbSize: DWORD,
    RIOReceive: LPFN_RIORECEIVE,
    RIOReceiveEx: LPFN_RIORECEIVEEX,
    RIOSend: LPFN_RIOSEND,
    RIOSendEx: LPFN_RIOSENDEX,
    RIOCloseCompletionQueue: LPFN_RIOCLOSECOMPLETIONQUEUE,
    RIOCreateCompletionQueue: LPFN_RIOCREATECOMPLETIONQUEUE,
    RIOCreateRequestQueue: LPFN_RIOCREATEREQUESTQUEUE,
    RIODequeueCompletion: LPFN_RIODEQUEUECOMPLETION,
    RIODeregisterBuffer: LPFN_RIODEREGISTERBUFFER,
    RIONotify: LPFN_RIONOTIFY,
    RIORegisterBuffer: LPFN_RIOREGISTERBUFFER,
    RIOResizeCompletionQueue: LPFN_RIORESIZECOMPLETIONQUEUE,
    RIOResizeRequestQueue: LPFN_RIORESIZEREQUESTQUEUE,
}}
pub type PRIO_EXTENSION_FUNCTION_TABLE = *mut RIO_EXTENSION_FUNCTION_TABLE;
DEFINE_GUID!{WSAID_MULTIPLE_RIO,
    0x8509e081, 0x96dd, 0x4005, 0xb1, 0x65, 0x9e, 0x2e, 0xe8, 0xc7, 0x9e, 0x3f}
