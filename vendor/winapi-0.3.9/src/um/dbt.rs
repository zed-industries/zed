// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_char, wchar_t};
use shared::basetsd::{ULONG32, ULONG64};
use shared::guiddef::GUID;
use shared::minwindef::{BYTE, DWORD, UINT, WORD, WPARAM};
use um::winnt::{HANDLE, LONG};
use um::winuser::HDEVNOTIFY;
pub const WM_DEVICECHANGE: UINT = 0x0219;
pub const BSF_QUERY: DWORD = 0x00000001;
pub const BSF_IGNORECURRENTTASK: DWORD = 0x00000002;
pub const BSF_FLUSHDISK: DWORD = 0x00000004;
pub const BSF_NOHANG: DWORD = 0x00000008;
pub const BSF_POSTMESSAGE: DWORD = 0x00000010;
pub const BSF_FORCEIFHUNG: DWORD = 0x00000020;
pub const BSF_NOTIMEOUTIFNOTHUNG: DWORD = 0x00000040;
pub const BSF_MSGSRV32ISOK: DWORD = 0x80000000;
pub const BSF_MSGSRV32ISOK_BIT: usize = 31;
pub const BSM_ALLCOMPONENTS: DWORD = 0x00000000;
pub const BSM_VXDS: DWORD = 0x00000001;
pub const BSM_NETDRIVER: DWORD = 0x00000002;
pub const BSM_INSTALLABLEDRIVERS: DWORD = 0x00000004;
pub const BSM_APPLICATIONS: DWORD = 0x00000008;
pub const DBT_APPYBEGIN: WPARAM = 0x0000;
pub const DBT_APPYEND: WPARAM = 0x0001;
pub const DBT_DEVNODES_CHANGED: WPARAM = 0x0007;
pub const DBT_QUERYCHANGECONFIG: WPARAM = 0x0017;
pub const DBT_CONFIGCHANGED: WPARAM = 0x0018;
pub const DBT_CONFIGCHANGECANCELED: WPARAM = 0x0019;
pub const DBT_MONITORCHANGE: WPARAM = 0x001B;
pub const DBT_SHELLLOGGEDON: WPARAM = 0x0020;
pub const DBT_CONFIGMGAPI32: WPARAM = 0x0022;
pub const DBT_VXDINITCOMPLETE: WPARAM = 0x0023;
pub const DBT_VOLLOCKQUERYLOCK: WPARAM = 0x8041;
pub const DBT_VOLLOCKLOCKTAKEN: WPARAM = 0x8042;
pub const DBT_VOLLOCKLOCKFAILED: WPARAM = 0x8043;
pub const DBT_VOLLOCKQUERYUNLOCK: WPARAM = 0x8044;
pub const DBT_VOLLOCKLOCKRELEASED: WPARAM = 0x8045;
pub const DBT_VOLLOCKUNLOCKFAILED: WPARAM = 0x8046;
STRUCT!{struct DEV_BROADCAST_HDR {
    dbch_size: DWORD,
    dbch_devicetype: DWORD,
    dbch_reserved: DWORD,
}}
pub type PDEV_BROADCAST_HDR = *mut DEV_BROADCAST_HDR;
STRUCT!{struct VolLockBroadcast {
    vlb_dbh: DEV_BROADCAST_HDR,
    vlb_owner: DWORD,
    vlb_perms: BYTE,
    vlb_lockType: BYTE,
    vlb_drive: BYTE,
    vlb_flags: BYTE,
}}
pub type pVolLockBroadcast = *mut VolLockBroadcast;
pub const LOCKP_ALLOW_WRITES: BYTE = 0x01;
pub const LOCKP_FAIL_WRITES: BYTE = 0x00;
pub const LOCKP_FAIL_MEM_MAPPING: BYTE = 0x02;
pub const LOCKP_ALLOW_MEM_MAPPING: BYTE = 0x00;
pub const LOCKP_USER_MASK: BYTE = 0x03;
pub const LOCKP_LOCK_FOR_FORMAT: BYTE = 0x04;
pub const LOCKF_LOGICAL_LOCK: BYTE = 0x00;
pub const LOCKF_PHYSICAL_LOCK: BYTE = 0x01;
pub const DBT_NO_DISK_SPACE: WPARAM = 0x0047;
pub const DBT_LOW_DISK_SPACE: WPARAM = 0x0048;
pub const DBT_CONFIGMGPRIVATE: WPARAM = 0x7FFF;
pub const DBT_DEVICEARRIVAL: WPARAM = 0x8000;
pub const DBT_DEVICEQUERYREMOVE: WPARAM = 0x8001;
pub const DBT_DEVICEQUERYREMOVEFAILED: WPARAM = 0x8002;
pub const DBT_DEVICEREMOVEPENDING: WPARAM = 0x8003;
pub const DBT_DEVICEREMOVECOMPLETE: WPARAM = 0x8004;
pub const DBT_DEVICETYPESPECIFIC: WPARAM = 0x8005;
pub const DBT_CUSTOMEVENT: WPARAM = 0x8006;
pub const DBT_DEVTYP_OEM: DWORD = 0x00000000;
pub const DBT_DEVTYP_DEVNODE: DWORD = 0x00000001;
pub const DBT_DEVTYP_VOLUME: DWORD = 0x00000002;
pub const DBT_DEVTYP_PORT: DWORD = 0x00000003;
pub const DBT_DEVTYP_NET: DWORD = 0x00000004;
pub const DBT_DEVTYP_DEVICEINTERFACE: DWORD = 0x00000005;
pub const DBT_DEVTYP_HANDLE: DWORD = 0x00000006;
STRUCT!{struct _DEV_BROADCAST_HEADER {
    dbcd_size: DWORD,
    dbcd_devicetype: DWORD,
    dbcd_reserved: DWORD,
}}
STRUCT!{struct DEV_BROADCAST_OEM {
    dbco_size: DWORD,
    dbco_devicetype: DWORD,
    dbco_reserved: DWORD,
    dbco_identifier: DWORD,
    dbco_suppfunc: DWORD,
}}
pub type PDEV_BROADCAST_OEM = *mut DEV_BROADCAST_OEM;
STRUCT!{struct DEV_BROADCAST_DEVNODE {
    dbcd_size: DWORD,
    dbcd_devicetype: DWORD,
    dbcd_reserved: DWORD,
    dbcd_devnode: DWORD,
}}
pub type PDEV_BROADCAST_DEVNODE = *mut DEV_BROADCAST_DEVNODE;
STRUCT!{struct DEV_BROADCAST_VOLUME {
    dbcv_size: DWORD,
    dbcv_devicetype: DWORD,
    dbcv_reserved: DWORD,
    dbcv_unitmask: DWORD,
    dbcv_flags: WORD,
}}
pub type PDEV_BROADCAST_VOLUME = *mut DEV_BROADCAST_VOLUME;
pub const DBTF_MEDIA: WORD = 0x0001;
pub const DBTF_NET: WORD = 0x0002;
STRUCT!{struct DEV_BROADCAST_PORT_A {
    dbcp_size: DWORD,
    dbcp_devicetype: DWORD,
    dbcp_reserved: DWORD,
    dbcp_name: [c_char; 1],
}}
pub type PDEV_BROADCAST_PORT_A = *mut DEV_BROADCAST_PORT_A;
STRUCT!{struct DEV_BROADCAST_PORT_W {
    dbcp_size: DWORD,
    dbcp_devicetype: DWORD,
    dbcp_reserved: DWORD,
    dbcp_name: [wchar_t; 1],
}}
pub type PDEV_BROADCAST_PORT_W = *mut DEV_BROADCAST_PORT_W;
STRUCT!{struct DEV_BROADCAST_NET {
    dbcn_size: DWORD,
    dbcn_devicetype: DWORD,
    dbcn_reserved: DWORD,
    dbcn_resource: DWORD,
    dbcn_flags: DWORD,
}}
pub type PDEV_BROADCAST_NET = *mut DEV_BROADCAST_NET;
STRUCT!{struct DEV_BROADCAST_DEVICEINTERFACE_A {
    dbcc_size: DWORD,
    dbcc_devicetype: DWORD,
    dbcc_reserved: DWORD,
    dbcc_classguid: GUID,
    dbcc_name: [c_char; 1],
}}
pub type PDEV_BROADCAST_DEVICEINTERFACE_A = *mut DEV_BROADCAST_DEVICEINTERFACE_A;
STRUCT!{struct DEV_BROADCAST_DEVICEINTERFACE_W {
    dbcc_size: DWORD,
    dbcc_devicetype: DWORD,
    dbcc_reserved: DWORD,
    dbcc_classguid: GUID,
    dbcc_name: [wchar_t; 1],
}}
pub type PDEV_BROADCAST_DEVICEINTERFACE_W = *mut DEV_BROADCAST_DEVICEINTERFACE_W;
STRUCT!{struct DEV_BROADCAST_HANDLE {
    dbch_size: DWORD,
    dbch_devicetype: DWORD,
    dbch_reserved: DWORD,
    dbch_handle: HANDLE,
    dbch_hdevnotify: HDEVNOTIFY,
    dbch_eventguid: GUID,
    dbch_nameoffset: LONG,
    dbch_data: [BYTE; 1],
}}
pub type PDEV_BROADCAST_HANDLE = *mut DEV_BROADCAST_HANDLE;
STRUCT!{struct DEV_BROADCAST_HANDLE32 {
    dbch_size: DWORD,
    dbch_devicetype: DWORD,
    dbch_reserved: DWORD,
    dbch_handle: ULONG32,
    dbch_hdevnotify: ULONG32,
    dbch_eventguid: GUID,
    dbch_nameoffset: LONG,
    dbch_data: [BYTE; 1],
}}
pub type PDEV_BROADCAST_HANDLE32 = *mut DEV_BROADCAST_HANDLE32;
STRUCT!{struct DEV_BROADCAST_HANDLE64 {
    dbch_size: DWORD,
    dbch_devicetype: DWORD,
    dbch_reserved: DWORD,
    dbch_handle: ULONG64,
    dbch_hdevnotify: ULONG64,
    dbch_eventguid: GUID,
    dbch_nameoffset: LONG,
    dbch_data: [BYTE; 1],
}}
pub type PDEV_BROADCAST_HANDLE64 = *mut DEV_BROADCAST_HANDLE64;
pub const DBTF_RESOURCE: DWORD = 0x00000001;
pub const DBTF_XPORT: DWORD = 0x00000002;
pub const DBTF_SLOWNET: DWORD = 0x00000004;
pub const DBT_VPOWERDAPI: WPARAM = 0x8100;
pub const DBT_USERDEFINED: WPARAM = 0xFFFF;
STRUCT!{struct _DEV_BROADCAST_USERDEFINED {
    dbud_dbh: DEV_BROADCAST_HDR,
    dbud_szName: [c_char; 1],
}}
