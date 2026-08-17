// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This file contains information about NetServer APIs
use shared::guiddef::GUID;
use shared::lmcons::{LMCSTR, LMSTR, NET_API_STATUS, PARMNUM_BASE_INFOLEVEL, PATHLEN};
use shared::minwindef::{BOOL, BYTE, DWORD, LPBYTE, LPDWORD, ULONG};
use um::winnt::{BOOLEAN, LONG};
use um::winsvc::SERVICE_STATUS_HANDLE;
extern "system" {
    pub fn NetServerEnum(
        servername: LMCSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        servertype: DWORD,
        domain: LMCSTR,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetServerEnumEx(
        servername: LMCSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        servertype: DWORD,
        domain: LMCSTR,
        FirstNameToReturn: LMCSTR,
    ) -> NET_API_STATUS;
    pub fn NetServerGetInfo(
        servername: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerSetInfo(
        servername: LMSTR,
        level: DWORD,
        buf: LPBYTE,
        ParmError: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetServerDiskEnum(
        servername: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetServerComputerNameAdd(
        ServerName: LMSTR,
        EmulatedDomainName: LMSTR,
        EmulatedServerName: LMSTR,
    ) -> NET_API_STATUS;
    pub fn NetServerComputerNameDel(
        ServerName: LMSTR,
        EmulatedServerName: LMSTR,
    ) -> NET_API_STATUS;
    pub fn NetServerTransportAdd(
        servername: LMSTR,
        level: DWORD,
        bufptr: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerTransportAddEx(
        servername: LMSTR,
        level: DWORD,
        bufptr: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerTransportDel(
        servername: LMSTR,
        level: DWORD,
        bufptr: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerTransportEnum(
        servername: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn SetServiceBits(
        hServiceStatus: SERVICE_STATUS_HANDLE,
        dwServiceBits: DWORD,
        bSetBitsOn: BOOL,
        bUpdateImmediately: BOOL,
    ) -> BOOL;
}
STRUCT!{struct SERVER_INFO_100 {
    sv100_platform_id: DWORD,
    sv100_name: LMSTR,
}}
pub type PSERVER_INFO_100 = *mut SERVER_INFO_100;
pub type LPSERVER_INFO_100 = *mut SERVER_INFO_100;
STRUCT!{struct SERVER_INFO_101 {
    sv101_platform_id: DWORD,
    sv101_name: LMSTR,
    sv101_version_major: DWORD,
    sv101_version_minor: DWORD,
    sv101_type: DWORD,
    sv101_comment: LMSTR,
}}
pub type PSERVER_INFO_101 = *mut SERVER_INFO_101;
pub type LPSERVER_INFO_101 = *mut SERVER_INFO_101;
STRUCT!{struct SERVER_INFO_102 {
    sv102_platform_id: DWORD,
    sv102_name: LMSTR,
    sv102_version_major: DWORD,
    sv102_version_minor: DWORD,
    sv102_type: DWORD,
    sv102_comment: LMSTR,
    sv102_users: DWORD,
    sv102_disc: LONG,
    sv102_hidden: BOOL,
    sv102_announce: DWORD,
    sv102_anndelta: DWORD,
    sv102_licenses: DWORD,
    sv102_userpath: LMSTR,
}}
pub type PSERVER_INFO_102 = *mut SERVER_INFO_102;
pub type LPSERVER_INFO_102 = *mut SERVER_INFO_102;
STRUCT!{struct SERVER_INFO_103 {
    sv103_platform_id: DWORD,
    sv103_name: LMSTR,
    sv103_version_major: DWORD,
    sv103_version_minor: DWORD,
    sv103_type: DWORD,
    sv103_comment: LMSTR,
    sv103_users: DWORD,
    sv103_disc: LONG,
    sv103_hidden: BOOL,
    sv103_announce: DWORD,
    sv103_anndelta: DWORD,
    sv103_licenses: DWORD,
    sv103_userpath: LMSTR,
    sv103_capabilities: DWORD,
}}
pub type PSERVER_INFO_103 = *mut SERVER_INFO_103;
pub type LPSERVER_INFO_103 = *mut SERVER_INFO_103;
STRUCT!{struct SERVER_INFO_402 {
    sv402_ulist_mtime: DWORD,
    sv402_glist_mtime: DWORD,
    sv402_alist_mtime: DWORD,
    sv402_alerts: LMSTR,
    sv402_security: DWORD,
    sv402_numadmin: DWORD,
    sv402_lanmask: DWORD,
    sv402_guestacct: LMSTR,
    sv402_chdevs: DWORD,
    sv402_chdevq: DWORD,
    sv402_chdevjobs: DWORD,
    sv402_connections: DWORD,
    sv402_shares: DWORD,
    sv402_openfiles: DWORD,
    sv402_sessopens: DWORD,
    sv402_sessvcs: DWORD,
    sv402_sessreqs: DWORD,
    sv402_opensearch: DWORD,
    sv402_activelocks: DWORD,
    sv402_numreqbuf: DWORD,
    sv402_sizreqbuf: DWORD,
    sv402_numbigbuf: DWORD,
    sv402_numfiletasks: DWORD,
    sv402_alertsched: DWORD,
    sv402_erroralert: DWORD,
    sv402_logonalert: DWORD,
    sv402_accessalert: DWORD,
    sv402_diskalert: DWORD,
    sv402_netioalert: DWORD,
    sv402_maxauditsz: DWORD,
    sv402_srvheuristics: LMSTR,
}}
pub type PSERVER_INFO_402 = *mut SERVER_INFO_402;
pub type LPSERVER_INFO_402 = *mut SERVER_INFO_402;
STRUCT!{struct SERVER_INFO_403 {
    sv403_ulist_mtime: DWORD,
    sv403_glist_mtime: DWORD,
    sv403_alist_mtime: DWORD,
    sv403_alerts: LMSTR,
    sv403_security: DWORD,
    sv403_numadmin: DWORD,
    sv403_lanmask: DWORD,
    sv403_guestacct: LMSTR,
    sv403_chdevs: DWORD,
    sv403_chdevq: DWORD,
    sv403_chdevjobs: DWORD,
    sv403_connections: DWORD,
    sv403_shares: DWORD,
    sv403_openfiles: DWORD,
    sv403_sessopens: DWORD,
    sv403_sessvcs: DWORD,
    sv403_sessreqs: DWORD,
    sv403_opensearch: DWORD,
    sv403_activelocks: DWORD,
    sv403_numreqbuf: DWORD,
    sv403_sizreqbuf: DWORD,
    sv403_numbigbuf: DWORD,
    sv403_numfiletasks: DWORD,
    sv403_alertsched: DWORD,
    sv403_erroralert: DWORD,
    sv403_logonalert: DWORD,
    sv403_accessalert: DWORD,
    sv403_diskalert: DWORD,
    sv403_netioalert: DWORD,
    sv403_maxauditsz: DWORD,
    sv403_srvheuristics: LMSTR,
    sv403_auditedevents: DWORD,
    sv403_autoprofile: DWORD,
    sv403_autopath: LMSTR,
}}
pub type PSERVER_INFO_403 = *mut SERVER_INFO_403;
pub type LPSERVER_INFO_403 = *mut SERVER_INFO_403;
STRUCT!{struct SERVER_INFO_502 {
    sv502_sessopens: DWORD,
    sv502_sessvcs: DWORD,
    sv502_opensearch: DWORD,
    sv502_sizreqbuf: DWORD,
    sv502_initworkitems: DWORD,
    sv502_maxworkitems: DWORD,
    sv502_rawworkitems: DWORD,
    sv502_irpstacksize: DWORD,
    sv502_maxrawbuflen: DWORD,
    sv502_sessusers: DWORD,
    sv502_sessconns: DWORD,
    sv502_maxpagedmemoryusage: DWORD,
    sv502_maxnonpagedmemoryusage: DWORD,
    sv502_enablesoftcompat: BOOL,
    sv502_enableforcedlogoff: BOOL,
    sv502_timesource: BOOL,
    sv502_acceptdownlevelapis: BOOL,
    sv502_lmannounce: BOOL,
}}
pub type PSERVER_INFO_502 = *mut SERVER_INFO_502;
pub type LPSERVER_INFO_502 = *mut SERVER_INFO_502;
STRUCT!{struct SERVER_INFO_503 {
    sv503_sessopens : DWORD,
    sv503_sessvcs: DWORD,
    sv503_opensearch: DWORD,
    sv503_sizreqbuf: DWORD,
    sv503_initworkitems: DWORD,
    sv503_maxworkitems: DWORD,
    sv503_rawworkitems: DWORD,
    sv503_irpstacksize: DWORD,
    sv503_maxrawbuflen: DWORD,
    sv503_sessusers: DWORD,
    sv503_sessconns: DWORD,
    sv503_maxpagedmemoryusage: DWORD,
    sv503_maxnonpagedmemoryusage: DWORD,
    sv503_enablesoftcompat: BOOL,
    sv503_enableforcedlogoff: BOOL,
    sv503_timesource: BOOL,
    sv503_acceptdownlevelapis: BOOL,
    sv503_lmannounce: BOOL,
    sv503_domain: LMSTR,
    sv503_maxcopyreadlen: DWORD,
    sv503_maxcopywritelen: DWORD,
    sv503_minkeepsearch: DWORD,
    sv503_maxkeepsearch: DWORD,
    sv503_minkeepcomplsearch: DWORD,
    sv503_maxkeepcomplsearch: DWORD,
    sv503_threadcountadd: DWORD,
    sv503_numblockthreads: DWORD,
    sv503_scavtimeout: DWORD,
    sv503_minrcvqueue: DWORD,
    sv503_minfreeworkitems: DWORD,
    sv503_xactmemsize: DWORD,
    sv503_threadpriority: DWORD,
    sv503_maxmpxct: DWORD,
    sv503_oplockbreakwait: DWORD,
    sv503_oplockbreakresponsewait: DWORD,
    sv503_enableoplocks: BOOL,
    sv503_enableoplockforceclose: BOOL,
    sv503_enablefcbopens: BOOL,
    sv503_enableraw: BOOL,
    sv503_enablesharednetdrives: BOOL,
    sv503_minfreeconnections: DWORD,
    sv503_maxfreeconnections: DWORD,
}}
pub type PSERVER_INFO_503 = *mut SERVER_INFO_503;
pub type LPSERVER_INFO_503 = *mut SERVER_INFO_503;
STRUCT!{struct SERVER_INFO_599 {
    sv599_sessopens: DWORD,
    sv599_sessvcs: DWORD,
    sv599_opensearch: DWORD,
    sv599_sizreqbuf: DWORD,
    sv599_initworkitems: DWORD,
    sv599_maxworkitems: DWORD,
    sv599_rawworkitems: DWORD,
    sv599_irpstacksize: DWORD,
    sv599_maxrawbuflen: DWORD,
    sv599_sessusers: DWORD,
    sv599_sessconns: DWORD,
    sv599_maxpagedmemoryusage: DWORD,
    sv599_maxnonpagedmemoryusage: DWORD,
    sv599_enablesoftcompat: BOOL,
    sv599_enableforcedlogoff: BOOL,
    sv599_timesource: BOOL,
    sv599_acceptdownlevelapis: BOOL,
    sv599_lmannounce: BOOL,
    sv599_domain: LMSTR,
    sv599_maxcopyreadlen: DWORD,
    sv599_maxcopywritelen: DWORD,
    sv599_minkeepsearch: DWORD,
    sv599_maxkeepsearch: DWORD,
    sv599_minkeepcomplsearch: DWORD,
    sv599_maxkeepcomplsearch: DWORD,
    sv599_threadcountadd: DWORD,
    sv599_numblockthreads: DWORD,
    sv599_scavtimeout: DWORD,
    sv599_minrcvqueue: DWORD,
    sv599_minfreeworkitems: DWORD,
    sv599_xactmemsize: DWORD,
    sv599_threadpriority: DWORD,
    sv599_maxmpxct: DWORD,
    sv599_oplockbreakwait: DWORD,
    sv599_oplockbreakresponsewait: DWORD,
    sv599_enableoplocks: BOOL,
    sv599_enableoplockforceclose: BOOL,
    sv599_enablefcbopens: BOOL,
    sv599_enableraw: BOOL,
    sv599_enablesharednetdrives: BOOL,
    sv599_minfreeconnections: DWORD,
    sv599_maxfreeconnections: DWORD,
    sv599_initsesstable: DWORD,
    sv599_initconntable: DWORD,
    sv599_initfiletable: DWORD,
    sv599_initsearchtable: DWORD,
    sv599_alertschedule: DWORD,
    sv599_errorthreshold: DWORD,
    sv599_networkerrorthreshold: DWORD,
    sv599_diskspacethreshold: DWORD,
    sv599_reserved: DWORD,
    sv599_maxlinkdelay: DWORD,
    sv599_minlinkthroughput: DWORD,
    sv599_linkinfovalidtime: DWORD,
    sv599_scavqosinfoupdatetime: DWORD,
    sv599_maxworkitemidletime: DWORD,
}}
pub type PSERVER_INFO_599 = *mut SERVER_INFO_599;
pub type LPSERVER_INFO_599 = *mut SERVER_INFO_599;
STRUCT!{struct SERVER_INFO_598 {
    sv598_maxrawworkitems: DWORD,
    sv598_maxthreadsperqueue: DWORD,
    sv598_producttype: DWORD,
    sv598_serversize: DWORD,
    sv598_connectionlessautodisc: DWORD,
    sv598_sharingviolationretries: DWORD,
    sv598_sharingviolationdelay: DWORD,
    sv598_maxglobalopensearch: DWORD,
    sv598_removeduplicatesearches: DWORD,
    sv598_lockviolationoffset: DWORD,
    sv598_lockviolationdelay: DWORD,
    sv598_mdlreadswitchover: DWORD,
    sv598_cachedopenlimit: DWORD,
    sv598_otherqueueaffinity: DWORD,
    sv598_restrictnullsessaccess: BOOL,
    sv598_enablewfw311directipx: BOOL,
    sv598_queuesamplesecs: DWORD,
    sv598_balancecount: DWORD,
    sv598_preferredaffinity: DWORD,
    sv598_maxfreerfcbs: DWORD,
    sv598_maxfreemfcbs: DWORD,
    sv598_maxfreelfcbs: DWORD,
    sv598_maxfreepagedpoolchunks: DWORD,
    sv598_minpagedpoolchunksize: DWORD,
    sv598_maxpagedpoolchunksize: DWORD,
    sv598_sendsfrompreferredprocessor: BOOL,
    sv598_cacheddirectorylimit: DWORD,
    sv598_maxcopylength: DWORD,
    sv598_enablecompression: BOOL,
    sv598_autosharewks: BOOL,
    sv598_autoshareserver: BOOL,
    sv598_enablesecuritysignature: BOOL,
    sv598_requiresecuritysignature: BOOL,
    sv598_minclientbuffersize: DWORD,
    sv598_serverguid: GUID,
    sv598_ConnectionNoSessionsTimeout: DWORD,
    sv598_IdleThreadTimeOut: DWORD,
    sv598_enableW9xsecuritysignature: BOOL,
    sv598_enforcekerberosreauthentication: BOOL,
    sv598_disabledos: BOOL,
    sv598_lowdiskspaceminimum: DWORD,
    sv598_disablestrictnamechecking: BOOL,
    sv598_enableauthenticateusersharing: BOOL,
}}
pub type PSERVER_INFO_598 = *mut SERVER_INFO_598;
pub type LPSERVER_INFO_598 = *mut SERVER_INFO_598;
STRUCT!{struct SERVER_INFO_1005 {
    sv1005_comment: LMSTR,
}}
pub type PSERVER_INFO_1005 = *mut SERVER_INFO_1005;
pub type LPSERVER_INFO_1005 = *mut SERVER_INFO_1005;
STRUCT!{struct SERVER_INFO_1107 {
    sv1107_users: DWORD,
}}
pub type PSERVER_INFO_1107 = *mut SERVER_INFO_1107;
pub type LPSERVER_INFO_1107 = *mut SERVER_INFO_1107;
STRUCT!{struct SERVER_INFO_1010 {
    sv1010_disc: LONG,
}}
pub type PSERVER_INFO_1010 = *mut SERVER_INFO_1010;
pub type LPSERVER_INFO_1010 = *mut SERVER_INFO_1010;
STRUCT!{struct SERVER_INFO_1016 {
    sv1016_hidden: BOOL,
}}
pub type PSERVER_INFO_1016 = *mut SERVER_INFO_1016;
pub type LPSERVER_INFO_1016 = *mut SERVER_INFO_1016;
STRUCT!{struct SERVER_INFO_1017 {
    sv1017_announce: DWORD,
}}
pub type PSERVER_INFO_1017 = *mut SERVER_INFO_1017;
pub type LPSERVER_INFO_1017 = *mut SERVER_INFO_1017;
STRUCT!{struct SERVER_INFO_1018 {
    sv1018_anndelta: DWORD,
}}
pub type PSERVER_INFO_1018 = *mut SERVER_INFO_1018;
pub type LPSERVER_INFO_1018 = *mut SERVER_INFO_1018;
STRUCT!{struct SERVER_INFO_1501 {
    sv1501_sessopens: DWORD,
}}
pub type PSERVER_INFO_1501 = *mut SERVER_INFO_1501;
pub type LPSERVER_INFO_1501 = *mut SERVER_INFO_1501;
STRUCT!{struct SERVER_INFO_1502 {
    sv1502_sessvcs: DWORD,
}}
pub type PSERVER_INFO_1502 = *mut SERVER_INFO_1502;
pub type LPSERVER_INFO_1502 = *mut SERVER_INFO_1502;
STRUCT!{struct SERVER_INFO_1503 {
    sv1503_opensearch: DWORD,
}}
pub type PSERVER_INFO_1503 = *mut SERVER_INFO_1503;
pub type LPSERVER_INFO_1503 = *mut SERVER_INFO_1503;
STRUCT!{struct SERVER_INFO_1506 {
    sv1506_maxworkitems: DWORD,
}}
pub type PSERVER_INFO_1506 = *mut SERVER_INFO_1506;
pub type LPSERVER_INFO_1506 = *mut SERVER_INFO_1506;
STRUCT!{struct SERVER_INFO_1509 {
    sv1509_maxrawbuflen: DWORD,
}}
pub type PSERVER_INFO_1509 = *mut SERVER_INFO_1509;
pub type LPSERVER_INFO_1509 = *mut SERVER_INFO_1509;
STRUCT!{struct SERVER_INFO_1510 {
    sv1510_sessusers: DWORD,
}}
pub type PSERVER_INFO_1510 = *mut SERVER_INFO_1510;
pub type LPSERVER_INFO_1510 = *mut SERVER_INFO_1510;
STRUCT!{struct SERVER_INFO_1511 {
    sv1511_sessconns: DWORD,
}}
pub type PSERVER_INFO_1511 = *mut SERVER_INFO_1511;
pub type LPSERVER_INFO_1511 = *mut SERVER_INFO_1511;
STRUCT!{struct SERVER_INFO_1512 {
    sv1512_maxnonpagedmemoryusage: DWORD,
}}
pub type PSERVER_INFO_1512 = *mut SERVER_INFO_1512;
pub type LPSERVER_INFO_1512 = *mut SERVER_INFO_1512;
STRUCT!{struct SERVER_INFO_1513 {
    sv1513_maxpagedmemoryusage: DWORD,
}}
pub type PSERVER_INFO_1513 = *mut SERVER_INFO_1513;
pub type LPSERVER_INFO_1513 = *mut SERVER_INFO_1513;
STRUCT!{struct SERVER_INFO_1514 {
    sv1514_enablesoftcompat: BOOL,
}}
pub type PSERVER_INFO_1514 = *mut SERVER_INFO_1514;
pub type LPSERVER_INFO_1514 = *mut SERVER_INFO_1514;
STRUCT!{struct SERVER_INFO_1515 {
    sv1515_enableforcedlogoff: BOOL,
}}
pub type PSERVER_INFO_1515 = *mut SERVER_INFO_1515;
pub type LPSERVER_INFO_1515 = *mut SERVER_INFO_1515;
STRUCT!{struct SERVER_INFO_1516 {
    sv1516_timesource: BOOL,
}}
pub type PSERVER_INFO_1516 = *mut SERVER_INFO_1516;
pub type LPSERVER_INFO_1516 = *mut SERVER_INFO_1516;
STRUCT!{struct SERVER_INFO_1518 {
    sv1518_lmannounce: BOOL,
}}
pub type PSERVER_INFO_1518 = *mut SERVER_INFO_1518;
pub type LPSERVER_INFO_1518 = *mut SERVER_INFO_1518;
STRUCT!{struct SERVER_INFO_1520 {
    sv1520_maxcopyreadlen: DWORD,
}}
pub type PSERVER_INFO_1520 = *mut SERVER_INFO_1520;
pub type LPSERVER_INFO_1520 = *mut SERVER_INFO_1520;
STRUCT!{struct SERVER_INFO_1521 {
    sv1521_maxcopywritelen: DWORD,
}}
pub type PSERVER_INFO_1521 = *mut SERVER_INFO_1521;
pub type LPSERVER_INFO_1521 = *mut SERVER_INFO_1521;
STRUCT!{struct SERVER_INFO_1522 {
    sv1522_minkeepsearch: DWORD,
}}
pub type PSERVER_INFO_1522 = *mut SERVER_INFO_1522;
pub type LPSERVER_INFO_1522 = *mut SERVER_INFO_1522;
STRUCT!{struct SERVER_INFO_1523 {
    sv1523_maxkeepsearch: DWORD,
}}
pub type PSERVER_INFO_1523 = *mut SERVER_INFO_1523;
pub type LPSERVER_INFO_1523 = *mut SERVER_INFO_1523;
STRUCT!{struct SERVER_INFO_1524 {
    sv1524_minkeepcomplsearch: DWORD,
}}
pub type PSERVER_INFO_1524 = *mut SERVER_INFO_1524;
pub type LPSERVER_INFO_1524 = *mut SERVER_INFO_1524;
STRUCT!{struct SERVER_INFO_1525 {
    sv1525_maxkeepcomplsearch: DWORD,
}}
pub type PSERVER_INFO_1525 = *mut SERVER_INFO_1525;
pub type LPSERVER_INFO_1525 = *mut SERVER_INFO_1525;
STRUCT!{struct SERVER_INFO_1528 {
    sv1528_scavtimeout: DWORD,
}}
pub type PSERVER_INFO_1528 = *mut SERVER_INFO_1528;
pub type LPSERVER_INFO_1528 = *mut SERVER_INFO_1528;
STRUCT!{struct SERVER_INFO_1529 {
    sv1529_minrcvqueue: DWORD,
}}
pub type PSERVER_INFO_1529 = *mut SERVER_INFO_1529;
pub type LPSERVER_INFO_1529 = *mut SERVER_INFO_1529;
STRUCT!{struct SERVER_INFO_1530 {
    sv1530_minfreeworkitems: DWORD,
}}
pub type PSERVER_INFO_1530 = *mut SERVER_INFO_1530;
pub type LPSERVER_INFO_1530 = *mut SERVER_INFO_1530;
STRUCT!{struct SERVER_INFO_1533 {
    sv1533_maxmpxct: DWORD,
}}
pub type PSERVER_INFO_1533 = *mut SERVER_INFO_1533;
pub type LPSERVER_INFO_1533 = *mut SERVER_INFO_1533;
STRUCT!{struct SERVER_INFO_1534 {
    sv1534_oplockbreakwait: DWORD,
}}
pub type PSERVER_INFO_1534 = *mut SERVER_INFO_1534;
pub type LPSERVER_INFO_1534 = *mut SERVER_INFO_1534;
STRUCT!{struct SERVER_INFO_1535 {
    sv1535_oplockbreakresponsewait: DWORD,
}}
pub type PSERVER_INFO_1535 = *mut SERVER_INFO_1535;
pub type LPSERVER_INFO_1535 = *mut SERVER_INFO_1535;
STRUCT!{struct SERVER_INFO_1536 {
    sv1536_enableoplocks: BOOL,
}}
pub type PSERVER_INFO_1536 = *mut SERVER_INFO_1536;
pub type LPSERVER_INFO_1536 = *mut SERVER_INFO_1536;
STRUCT!{struct SERVER_INFO_1537 {
    sv1537_enableoplockforceclose: BOOL,
}}
pub type PSERVER_INFO_1537 = *mut SERVER_INFO_1537;
pub type LPSERVER_INFO_1537 = *mut SERVER_INFO_1537;
STRUCT!{struct SERVER_INFO_1538 {
    sv1538_enablefcbopens: BOOL,
}}
pub type PSERVER_INFO_1538 = *mut SERVER_INFO_1538;
pub type LPSERVER_INFO_1538 = *mut SERVER_INFO_1538;
STRUCT!{struct SERVER_INFO_1539 {
    sv1539_enableraw: BOOL,
}}
pub type PSERVER_INFO_1539 = *mut SERVER_INFO_1539;
pub type LPSERVER_INFO_1539 = *mut SERVER_INFO_1539;
STRUCT!{struct SERVER_INFO_1540 {
    sv1540_enablesharednetdrives: BOOL,
}}
pub type PSERVER_INFO_1540 = *mut SERVER_INFO_1540;
pub type LPSERVER_INFO_1540 = *mut SERVER_INFO_1540;
STRUCT!{struct SERVER_INFO_1541 {
    sv1541_minfreeconnections: BOOL,
}}
pub type PSERVER_INFO_1541 = *mut SERVER_INFO_1541;
pub type LPSERVER_INFO_1541 = *mut SERVER_INFO_1541;
STRUCT!{struct SERVER_INFO_1542 {
    sv1542_maxfreeconnections: BOOL,
}}
pub type PSERVER_INFO_1542 = *mut SERVER_INFO_1542;
pub type LPSERVER_INFO_1542 = *mut SERVER_INFO_1542;
STRUCT!{struct SERVER_INFO_1543 {
    sv1543_initsesstable: DWORD,
}}
pub type PSERVER_INFO_1543 = *mut SERVER_INFO_1543;
pub type LPSERVER_INFO_1543 = *mut SERVER_INFO_1543;
STRUCT!{struct SERVER_INFO_1544 {
    sv1544_initconntable: DWORD,
}}
pub type PSERVER_INFO_1544 = *mut SERVER_INFO_1544;
pub type LPSERVER_INFO_1544 = *mut SERVER_INFO_1544;
STRUCT!{struct SERVER_INFO_1545 {
    sv1545_initfiletable: DWORD,
}}
pub type PSERVER_INFO_1545 = *mut SERVER_INFO_1545;
pub type LPSERVER_INFO_1545 = *mut SERVER_INFO_1545;
STRUCT!{struct SERVER_INFO_1546 {
    sv1546_initsearchtable: DWORD,
}}
pub type PSERVER_INFO_1546 = *mut SERVER_INFO_1546;
pub type LPSERVER_INFO_1546 = *mut SERVER_INFO_1546;
STRUCT!{struct SERVER_INFO_1547 {
    sv1547_alertschedule: DWORD,
}}
pub type PSERVER_INFO_1547 = *mut SERVER_INFO_1547;
pub type LPSERVER_INFO_1547 = *mut SERVER_INFO_1547;
STRUCT!{struct SERVER_INFO_1548 {
    sv1548_errorthreshold: DWORD,
}}
pub type PSERVER_INFO_1548 = *mut SERVER_INFO_1548;
pub type LPSERVER_INFO_1548 = *mut SERVER_INFO_1548;
STRUCT!{struct SERVER_INFO_1549 {
    sv1549_networkerrorthreshold: DWORD,
}}
pub type PSERVER_INFO_1549 = *mut SERVER_INFO_1549;
pub type LPSERVER_INFO_1549 = *mut SERVER_INFO_1549;
STRUCT!{struct SERVER_INFO_1550 {
    sv1550_diskspacethreshold: DWORD,
}}
pub type PSERVER_INFO_1550 = *mut SERVER_INFO_1550;
pub type LPSERVER_INFO_1550 = *mut SERVER_INFO_1550;
STRUCT!{struct SERVER_INFO_1552 {
    sv1552_maxlinkdelay: DWORD,
}}
pub type PSERVER_INFO_1552 = *mut SERVER_INFO_1552;
pub type LPSERVER_INFO_1552 = *mut SERVER_INFO_1552;
STRUCT!{struct SERVER_INFO_1553 {
    sv1553_minlinkthroughput: DWORD,
}}
pub type PSERVER_INFO_1553 = *mut SERVER_INFO_1553;
pub type LPSERVER_INFO_1553 = *mut SERVER_INFO_1553;
STRUCT!{struct SERVER_INFO_1554 {
    sv1554_linkinfovalidtime: DWORD,
}}
pub type PSERVER_INFO_1554 = *mut SERVER_INFO_1554;
pub type LPSERVER_INFO_1554 = *mut SERVER_INFO_1554;
STRUCT!{struct SERVER_INFO_1555 {
    sv1555_scavqosinfoupdatetime: DWORD,
}}
pub type PSERVER_INFO_1555 = *mut SERVER_INFO_1555;
pub type LPSERVER_INFO_1555 = *mut SERVER_INFO_1555;
STRUCT!{struct SERVER_INFO_1556 {
    sv1556_maxworkitemidletime: DWORD,
}}
pub type PSERVER_INFO_1556 = *mut SERVER_INFO_1556;
pub type LPSERVER_INFO_1556 = *mut SERVER_INFO_1556;
STRUCT!{struct SERVER_INFO_1557 {
    sv1557_maxrawworkitems: DWORD,
}}
pub type PSERVER_INFO_1557 = *mut SERVER_INFO_1557;
pub type LPSERVER_INFO_1557 = *mut SERVER_INFO_1557;
STRUCT!{struct SERVER_INFO_1560 {
    sv1560_producttype: DWORD,
}}
pub type PSERVER_INFO_1560 = *mut SERVER_INFO_1560;
pub type LPSERVER_INFO_1560 = *mut SERVER_INFO_1560;
STRUCT!{struct SERVER_INFO_1561 {
    sv1561_serversize: DWORD,
}}
pub type PSERVER_INFO_1561 = *mut SERVER_INFO_1561;
pub type LPSERVER_INFO_1561 = *mut SERVER_INFO_1561;
STRUCT!{struct SERVER_INFO_1562 {
    sv1562_connectionlessautodisc: DWORD,
}}
pub type PSERVER_INFO_1562 = *mut SERVER_INFO_1562;
pub type LPSERVER_INFO_1562 = *mut SERVER_INFO_1562;
STRUCT!{struct SERVER_INFO_1563 {
    sv1563_sharingviolationretries: DWORD,
}}
pub type PSERVER_INFO_1563 = *mut SERVER_INFO_1563;
pub type LPSERVER_INFO_1563 = *mut SERVER_INFO_1563;
STRUCT!{struct SERVER_INFO_1564 {
    sv1564_sharingviolationdelay: DWORD,
}}
pub type PSERVER_INFO_1564 = *mut SERVER_INFO_1564;
pub type LPSERVER_INFO_1564 = *mut SERVER_INFO_1564;
STRUCT!{struct SERVER_INFO_1565 {
    sv1565_maxglobalopensearch: DWORD,
}}
pub type PSERVER_INFO_1565 = *mut SERVER_INFO_1565;
pub type LPSERVER_INFO_1565 = *mut SERVER_INFO_1565;
STRUCT!{struct SERVER_INFO_1566 {
    sv1566_removeduplicatesearches: BOOL,
}}
pub type PSERVER_INFO_1566 = *mut SERVER_INFO_1566;
pub type LPSERVER_INFO_1566 = *mut SERVER_INFO_1566;
STRUCT!{struct SERVER_INFO_1567 {
    sv1567_lockviolationretries: DWORD,
}}
pub type PSERVER_INFO_1567 = *mut SERVER_INFO_1567;
pub type LPSERVER_INFO_1567 = *mut SERVER_INFO_1567;
STRUCT!{struct SERVER_INFO_1568 {
    sv1568_lockviolationoffset: DWORD,
}}
pub type PSERVER_INFO_1568 = *mut SERVER_INFO_1568;
pub type LPSERVER_INFO_1568 = *mut SERVER_INFO_1568;
STRUCT!{struct SERVER_INFO_1569 {
    sv1569_lockviolationdelay: DWORD,
}}
pub type PSERVER_INFO_1569 = *mut SERVER_INFO_1569;
pub type LPSERVER_INFO_1569 = *mut SERVER_INFO_1569;
STRUCT!{struct SERVER_INFO_1570 {
    sv1570_mdlreadswitchover: DWORD,
}}
pub type PSERVER_INFO_1570 = *mut SERVER_INFO_1570;
pub type LPSERVER_INFO_1570 = *mut SERVER_INFO_1570;
STRUCT!{struct SERVER_INFO_1571 {
    sv1571_cachedopenlimit: DWORD,
}}
pub type PSERVER_INFO_1571 = *mut SERVER_INFO_1571;
pub type LPSERVER_INFO_1571 = *mut SERVER_INFO_1571;
STRUCT!{struct SERVER_INFO_1572 {
    sv1572_criticalthreads: DWORD,
}}
pub type PSERVER_INFO_1572 = *mut SERVER_INFO_1572;
pub type LPSERVER_INFO_1572 = *mut SERVER_INFO_1572;
STRUCT!{struct SERVER_INFO_1573 {
    sv1573_restrictnullsessaccess: DWORD,
}}
pub type PSERVER_INFO_1573 = *mut SERVER_INFO_1573;
pub type LPSERVER_INFO_1573 = *mut SERVER_INFO_1573;
STRUCT!{struct SERVER_INFO_1574 {
    sv1574_enablewfw311directipx: DWORD,
}}
pub type PSERVER_INFO_1574 = *mut SERVER_INFO_1574;
pub type LPSERVER_INFO_1574 = *mut SERVER_INFO_1574;
STRUCT!{struct SERVER_INFO_1575 {
    sv1575_otherqueueaffinity: DWORD,
}}
pub type PSERVER_INFO_1575 = *mut SERVER_INFO_1575;
pub type LPSERVER_INFO_1575 = *mut SERVER_INFO_1575;
STRUCT!{struct SERVER_INFO_1576 {
    sv1576_queuesamplesecs: DWORD,
}}
pub type PSERVER_INFO_1576 = *mut SERVER_INFO_1576;
pub type LPSERVER_INFO_1576 = *mut SERVER_INFO_1576;
STRUCT!{struct SERVER_INFO_1577 {
    sv1577_balancecount: DWORD,
}}
pub type PSERVER_INFO_1577 = *mut SERVER_INFO_1577;
pub type LPSERVER_INFO_1577 = *mut SERVER_INFO_1577;
STRUCT!{struct SERVER_INFO_1578 {
    sv1578_preferredaffinity: DWORD,
}}
pub type PSERVER_INFO_1578 = *mut SERVER_INFO_1578;
pub type LPSERVER_INFO_1578 = *mut SERVER_INFO_1578;
STRUCT!{struct SERVER_INFO_1579 {
    sv1579_maxfreerfcbs: DWORD,
}}
pub type PSERVER_INFO_1579 = *mut SERVER_INFO_1579;
pub type LPSERVER_INFO_1579 = *mut SERVER_INFO_1579;
STRUCT!{struct SERVER_INFO_1580 {
    sv1580_maxfreemfcbs: DWORD,
}}
pub type PSERVER_INFO_1580 = *mut SERVER_INFO_1580;
pub type LPSERVER_INFO_1580 = *mut SERVER_INFO_1580;
STRUCT!{struct SERVER_INFO_1581 {
    sv1581_maxfreemlcbs: DWORD,
}}
pub type PSERVER_INFO_1581 = *mut SERVER_INFO_1581;
pub type LPSERVER_INFO_1581 = *mut SERVER_INFO_1581;
STRUCT!{struct SERVER_INFO_1582 {
    sv1582_maxfreepagedpoolchunks: DWORD,
}}
pub type PSERVER_INFO_1582 = *mut SERVER_INFO_1582;
pub type LPSERVER_INFO_1582 = *mut SERVER_INFO_1582;
STRUCT!{struct SERVER_INFO_1583 {
    sv1583_minpagedpoolchunksize: DWORD,
}}
pub type PSERVER_INFO_1583 = *mut SERVER_INFO_1583;
pub type LPSERVER_INFO_1583 = *mut SERVER_INFO_1583;
STRUCT!{struct SERVER_INFO_1584 {
    sv1584_maxpagedpoolchunksize: DWORD,
}}
pub type PSERVER_INFO_1584 = *mut SERVER_INFO_1584;
pub type LPSERVER_INFO_1584 = *mut SERVER_INFO_1584;
STRUCT!{struct SERVER_INFO_1585 {
    sv1585_sendsfrompreferredprocessor: BOOL,
}}
pub type PSERVER_INFO_1585 = *mut SERVER_INFO_1585;
pub type LPSERVER_INFO_1585 = *mut SERVER_INFO_1585;
STRUCT!{struct SERVER_INFO_1586 {
    sv1586_maxthreadsperqueue: DWORD,
}}
pub type PSERVER_INFO_1586 = *mut SERVER_INFO_1586;
pub type LPSERVER_INFO_1586 = *mut SERVER_INFO_1586;
STRUCT!{struct SERVER_INFO_1587 {
    sv1587_cacheddirectorylimit: DWORD,
}}
pub type PSERVER_INFO_1587 = *mut SERVER_INFO_1587;
pub type LPSERVER_INFO_1587 = *mut SERVER_INFO_1587;
STRUCT!{struct SERVER_INFO_1588 {
    sv1588_maxcopylength: DWORD,
}}
pub type PSERVER_INFO_1588 = *mut SERVER_INFO_1588;
pub type LPSERVER_INFO_1588 = *mut SERVER_INFO_1588;
STRUCT!{struct SERVER_INFO_1590 {
    sv1590_enablecompression: DWORD,
}}
pub type PSERVER_INFO_1590 = *mut SERVER_INFO_1590;
pub type LPSERVER_INFO_1590 = *mut SERVER_INFO_1590;
STRUCT!{struct SERVER_INFO_1591 {
    sv1591_autosharewks: DWORD,
}}
pub type PSERVER_INFO_1591 = *mut SERVER_INFO_1591;
pub type LPSERVER_INFO_1591 = *mut SERVER_INFO_1591;
STRUCT!{struct SERVER_INFO_1592 {
    sv1592_autosharewks: DWORD,
}}
pub type PSERVER_INFO_1592 = *mut SERVER_INFO_1592;
pub type LPSERVER_INFO_1592 = *mut SERVER_INFO_1592;
STRUCT!{struct SERVER_INFO_1593 {
    sv1593_enablesecuritysignature: DWORD,
}}
pub type PSERVER_INFO_1593 = *mut SERVER_INFO_1593;
pub type LPSERVER_INFO_1593 = *mut SERVER_INFO_1593;
STRUCT!{struct SERVER_INFO_1594 {
    sv1594_requiresecuritysignature: DWORD,
}}
pub type PSERVER_INFO_1594 = *mut SERVER_INFO_1594;
pub type LPSERVER_INFO_1594 = *mut SERVER_INFO_1594;
STRUCT!{struct SERVER_INFO_1595 {
    sv1595_minclientbuffersize: DWORD,
}}
pub type PSERVER_INFO_1595 = *mut SERVER_INFO_1595;
pub type LPSERVER_INFO_1595 = *mut SERVER_INFO_1595;
STRUCT!{struct SERVER_INFO_1596 {
    sv1596_ConnectionNoSessionsTimeout: DWORD,
}}
pub type PSERVER_INFO_1596 = *mut SERVER_INFO_1596;
pub type LPSERVER_INFO_1596 = *mut SERVER_INFO_1596;
STRUCT!{struct SERVER_INFO_1597 {
    sv1597_IdleThreadTimeOut: DWORD,
}}
pub type PSERVER_INFO_1597 = *mut SERVER_INFO_1597;
pub type LPSERVER_INFO_1597 = *mut SERVER_INFO_1597;
STRUCT!{struct SERVER_INFO_1598 {
    sv1598_enableW9xsecuritysignature: DWORD,
}}
pub type PSERVER_INFO_1598 = *mut SERVER_INFO_1598;
pub type LPSERVER_INFO_1598 = *mut SERVER_INFO_1598;
STRUCT!{struct SERVER_INFO_1599 {
    sv1598_enforcekerberosreauthentication: BOOLEAN,
}}
pub type PSERVER_INFO_1599 = *mut SERVER_INFO_1599;
pub type LPSERVER_INFO_1599 = *mut SERVER_INFO_1599;
STRUCT!{struct SERVER_INFO_1600 {
    sv1598_disabledos: BOOLEAN,
}}
pub type PSERVER_INFO_1600 = *mut SERVER_INFO_1600;
pub type LPSERVER_INFO_1600 = *mut SERVER_INFO_1600;
STRUCT!{struct SERVER_INFO_1601 {
    sv1598_lowdiskspaceminimum: DWORD,
}}
pub type PSERVER_INFO_1601 = *mut SERVER_INFO_1601;
pub type LPSERVER_INFO_1601 = *mut SERVER_INFO_1601;
STRUCT!{struct SERVER_INFO_1602 {
    sv_1598_disablestrictnamechecking: BOOL,
}}
pub type PSERVER_INFO_1602 = *mut SERVER_INFO_1602;
pub type LPSERVER_INFO_1602 = *mut SERVER_INFO_1602;
STRUCT!{struct SERVER_TRANSPORT_INFO_0 {
    svti0_numberofvcs: DWORD,
    svti0_transportname: LMSTR,
    svti0_transportaddress: LPBYTE,
    svti0_transportaddresslength: DWORD,
    svti0_networkaddress: LMSTR,
}}
pub type PSERVER_TRANSPORT_INFO_0 = *mut SERVER_TRANSPORT_INFO_0;
pub type LPSERVER_TRANSPORT_INFO_0 = *mut SERVER_TRANSPORT_INFO_0;
STRUCT!{struct SERVER_TRANSPORT_INFO_1 {
    svti1_numberofvcs: DWORD,
    svti1_transportname: LMSTR,
    svti1_transportaddress: LPBYTE,
    svti1_transportaddresslength: DWORD,
    svti1_networkaddress: LMSTR,
    svti1_domain: LMSTR,
}}
pub type PSERVER_TRANSPORT_INFO_1 = *mut SERVER_TRANSPORT_INFO_1;
pub type LPSERVER_TRANSPORT_INFO_1 = *mut SERVER_TRANSPORT_INFO_1;
STRUCT!{struct SERVER_TRANSPORT_INFO_2 {
    svti2_numberofvcs: DWORD,
    svti2_transportname: LMSTR,
    svti2_transportaddress: LPBYTE,
    svti2_transportaddresslength: DWORD,
    svti2_networkaddress: LMSTR,
    svti2_domain: LMSTR,
    svti2_flags: ULONG,
}}
pub type PSERVER_TRANSPORT_INFO_2 = *mut SERVER_TRANSPORT_INFO_2;
pub type LPSERVER_TRANSPORT_INFO_2 = *mut SERVER_TRANSPORT_INFO_2;
STRUCT!{struct SERVER_TRANSPORT_INFO_3 {
    svti3_numberofvcs: DWORD,
    svti3_transportname: LMSTR,
    svti3_transportaddress: LPBYTE,
    svti3_transportaddresslength: DWORD,
    svti3_networkaddress: LMSTR,
    svti3_domain: LMSTR,
    svti3_flags: ULONG,
    svti3_passwordlength: DWORD,
    svti3_password: [BYTE; 256],
}}
pub type PSERVER_TRANSPORT_INFO_3 = *mut SERVER_TRANSPORT_INFO_3;
pub type LPSERVER_TRANSPORT_INFO_3 = *mut SERVER_TRANSPORT_INFO_3;
pub const SV_PLATFORM_ID_OS2: DWORD = 400;
pub const SV_PLATFORM_ID_NT: DWORD = 500;
pub const MAJOR_VERSION_MASK: DWORD = 0x0F;
pub const SV_TYPE_WORKSTATION: DWORD = 0x00000001;
pub const SV_TYPE_SERVER: DWORD = 0x00000002;
pub const SV_TYPE_SQLSERVER: DWORD = 0x00000004;
pub const SV_TYPE_DOMAIN_CTRL: DWORD = 0x00000008;
pub const SV_TYPE_DOMAIN_BAKCTRL: DWORD = 0x00000010;
pub const SV_TYPE_TIME_SOURCE: DWORD = 0x00000020;
pub const SV_TYPE_AFP: DWORD = 0x00000040;
pub const SV_TYPE_NOVELL: DWORD = 0x00000080;
pub const SV_TYPE_DOMAIN_MEMBER: DWORD = 0x00000100;
pub const SV_TYPE_PRINTQ_SERVER: DWORD = 0x00000200;
pub const SV_TYPE_DIALIN_SERVER: DWORD = 0x00000400;
pub const SV_TYPE_XENIX_SERVER: DWORD = 0x00000800;
pub const SV_TYPE_SERVER_UNIX: DWORD = SV_TYPE_XENIX_SERVER;
pub const SV_TYPE_NT: DWORD = 0x00001000;
pub const SV_TYPE_WFW: DWORD = 0x00002000;
pub const SV_TYPE_SERVER_MFPN: DWORD = 0x00004000;
pub const SV_TYPE_SERVER_NT: DWORD = 0x00008000;
pub const SV_TYPE_POTENTIAL_BROWSER: DWORD = 0x00010000;
pub const SV_TYPE_BACKUP_BROWSER: DWORD = 0x00020000;
pub const SV_TYPE_MASTER_BROWSER: DWORD = 0x00040000;
pub const SV_TYPE_DOMAIN_MASTER: DWORD = 0x00080000;
pub const SV_TYPE_SERVER_OSF: DWORD = 0x00100000;
pub const SV_TYPE_SERVER_VMS: DWORD = 0x00200000;
pub const SV_TYPE_WINDOWS: DWORD = 0x00400000;
pub const SV_TYPE_DFS: DWORD = 0x00800000;
pub const SV_TYPE_CLUSTER_NT: DWORD = 0x01000000;
pub const SV_TYPE_TERMINALSERVER: DWORD = 0x02000000;
pub const SV_TYPE_CLUSTER_VS_NT: DWORD = 0x04000000;
pub const SV_TYPE_DCE: DWORD = 0x10000000;
pub const SV_TYPE_ALTERNATE_XPORT: DWORD = 0x20000000;
pub const SV_TYPE_LOCAL_LIST_ONLY: DWORD = 0x40000000;
pub const SV_TYPE_DOMAIN_ENUM: DWORD = 0x80000000;
pub const SV_TYPE_ALL: DWORD = 0xFFFFFFFF;
pub const SV_NODISC: DWORD = -1i32 as u32;
pub const SV_USERSECURITY: DWORD = 1;
pub const SV_SHARESECURITY: DWORD = 0;
pub const SV_HIDDEN: DWORD = 1;
pub const SV_VISIBLE: DWORD = 0;
pub const SV_PLATFORM_ID_PARMNUM: DWORD = 101;
pub const SV_NAME_PARMNUM: DWORD = 102;
pub const SV_VERSION_MAJOR_PARMNUM: DWORD = 103;
pub const SV_VERSION_MINOR_PARMNUM: DWORD = 104;
pub const SV_TYPE_PARMNUM: DWORD = 105;
pub const SV_COMMENT_PARMNUM: DWORD = 5;
pub const SV_USERS_PARMNUM: DWORD = 107;
pub const SV_DISC_PARMNUM: DWORD = 10;
pub const SV_HIDDEN_PARMNUM: DWORD = 16;
pub const SV_ANNOUNCE_PARMNUM: DWORD = 17;
pub const SV_ANNDELTA_PARMNUM: DWORD = 18;
pub const SV_USERPATH_PARMNUM: DWORD = 112;
pub const SV_ULIST_MTIME_PARMNUM: DWORD = 401;
pub const SV_GLIST_MTIME_PARMNUM: DWORD = 402;
pub const SV_ALIST_MTIME_PARMNUM: DWORD = 403;
pub const SV_ALERTS_PARMNUM: DWORD = 11;
pub const SV_SECURITY_PARMNUM: DWORD = 405;
pub const SV_NUMADMIN_PARMNUM: DWORD = 406;
pub const SV_LANMASK_PARMNUM: DWORD = 407;
pub const SV_GUESTACC_PARMNUM: DWORD = 408;
pub const SV_CHDEVQ_PARMNUM: DWORD = 410;
pub const SV_CHDEVJOBS_PARMNUM: DWORD = 411;
pub const SV_CONNECTIONS_PARMNUM: DWORD = 412;
pub const SV_SHARES_PARMNUM: DWORD = 413;
pub const SV_OPENFILES_PARMNUM: DWORD = 414;
pub const SV_SESSREQS_PARMNUM: DWORD = 417;
pub const SV_ACTIVELOCKS_PARMNUM: DWORD = 419;
pub const SV_NUMREQBUF_PARMNUM: DWORD = 420;
pub const SV_NUMBIGBUF_PARMNUM: DWORD = 422;
pub const SV_NUMFILETASKS_PARMNUM: DWORD = 423;
pub const SV_ALERTSCHED_PARMNUM: DWORD = 37;
pub const SV_ERRORALERT_PARMNUM: DWORD = 38;
pub const SV_LOGONALERT_PARMNUM: DWORD = 39;
pub const SV_ACCESSALERT_PARMNUM: DWORD = 40;
pub const SV_DISKALERT_PARMNUM: DWORD = 41;
pub const SV_NETIOALERT_PARMNUM: DWORD = 42;
pub const SV_MAXAUDITSZ_PARMNUM: DWORD = 43;
pub const SV_SRVHEURISTICS_PARMNUM: DWORD = 431;
pub const SV_SESSOPENS_PARMNUM: DWORD = 501;
pub const SV_SESSVCS_PARMNUM: DWORD = 502;
pub const SV_OPENSEARCH_PARMNUM: DWORD = 503;
pub const SV_SIZREQBUF_PARMNUM: DWORD = 504;
pub const SV_INITWORKITEMS_PARMNUM: DWORD = 505;
pub const SV_MAXWORKITEMS_PARMNUM: DWORD = 506;
pub const SV_RAWWORKITEMS_PARMNUM: DWORD = 507;
pub const SV_IRPSTACKSIZE_PARMNUM: DWORD = 508;
pub const SV_MAXRAWBUFLEN_PARMNUM: DWORD = 509;
pub const SV_SESSUSERS_PARMNUM: DWORD = 510;
pub const SV_SESSCONNS_PARMNUM: DWORD = 511;
pub const SV_MAXNONPAGEDMEMORYUSAGE_PARMNUM: DWORD = 512;
pub const SV_MAXPAGEDMEMORYUSAGE_PARMNUM: DWORD = 513;
pub const SV_ENABLESOFTCOMPAT_PARMNUM: DWORD = 514;
pub const SV_ENABLEFORCEDLOGOFF_PARMNUM: DWORD = 515;
pub const SV_TIMESOURCE_PARMNUM: DWORD = 516;
pub const SV_ACCEPTDOWNLEVELAPIS_PARMNUM: DWORD = 517;
pub const SV_LMANNOUNCE_PARMNUM: DWORD = 518;
pub const SV_DOMAIN_PARMNUM: DWORD = 519;
pub const SV_MAXCOPYREADLEN_PARMNUM: DWORD = 520;
pub const SV_MAXCOPYWRITELEN_PARMNUM: DWORD = 521;
pub const SV_MINKEEPSEARCH_PARMNUM: DWORD = 522;
pub const SV_MAXKEEPSEARCH_PARMNUM: DWORD = 523;
pub const SV_MINKEEPCOMPLSEARCH_PARMNUM: DWORD = 524;
pub const SV_MAXKEEPCOMPLSEARCH_PARMNUM: DWORD = 525;
pub const SV_THREADCOUNTADD_PARMNUM: DWORD = 526;
pub const SV_NUMBLOCKTHREADS_PARMNUM: DWORD = 527;
pub const SV_SCAVTIMEOUT_PARMNUM: DWORD = 528;
pub const SV_MINRCVQUEUE_PARMNUM: DWORD = 529;
pub const SV_MINFREEWORKITEMS_PARMNUM: DWORD = 530;
pub const SV_XACTMEMSIZE_PARMNUM: DWORD = 531;
pub const SV_THREADPRIORITY_PARMNUM: DWORD = 532;
pub const SV_MAXMPXCT_PARMNUM: DWORD = 533;
pub const SV_OPLOCKBREAKWAIT_PARMNUM: DWORD = 534;
pub const SV_OPLOCKBREAKRESPONSEWAIT_PARMNUM: DWORD = 535;
pub const SV_ENABLEOPLOCKS_PARMNUM: DWORD = 536;
pub const SV_ENABLEOPLOCKFORCECLOSE_PARMNUM: DWORD = 537;
pub const SV_ENABLEFCBOPENS_PARMNUM: DWORD = 538;
pub const SV_ENABLERAW_PARMNUM: DWORD = 539;
pub const SV_ENABLESHAREDNETDRIVES_PARMNUM: DWORD = 540;
pub const SV_MINFREECONNECTIONS_PARMNUM: DWORD = 541;
pub const SV_MAXFREECONNECTIONS_PARMNUM: DWORD = 542;
pub const SV_INITSESSTABLE_PARMNUM: DWORD = 543;
pub const SV_INITCONNTABLE_PARMNUM: DWORD = 544;
pub const SV_INITFILETABLE_PARMNUM: DWORD = 545;
pub const SV_INITSEARCHTABLE_PARMNUM: DWORD = 546;
pub const SV_ALERTSCHEDULE_PARMNUM: DWORD = 547;
pub const SV_ERRORTHRESHOLD_PARMNUM: DWORD = 548;
pub const SV_NETWORKERRORTHRESHOLD_PARMNUM: DWORD = 549;
pub const SV_DISKSPACETHRESHOLD_PARMNUM: DWORD = 550;
pub const SV_MAXLINKDELAY_PARMNUM: DWORD = 552;
pub const SV_MINLINKTHROUGHPUT_PARMNUM: DWORD = 553;
pub const SV_LINKINFOVALIDTIME_PARMNUM: DWORD = 554;
pub const SV_SCAVQOSINFOUPDATETIME_PARMNUM: DWORD = 555;
pub const SV_MAXWORKITEMIDLETIME_PARMNUM: DWORD = 556;
pub const SV_MAXRAWWORKITEMS_PARMNUM: DWORD = 557;
pub const SV_PRODUCTTYPE_PARMNUM: DWORD = 560;
pub const SV_SERVERSIZE_PARMNUM: DWORD = 561;
pub const SV_CONNECTIONLESSAUTODISC_PARMNUM: DWORD = 562;
pub const SV_SHARINGVIOLATIONRETRIES_PARMNUM: DWORD = 563;
pub const SV_SHARINGVIOLATIONDELAY_PARMNUM: DWORD = 564;
pub const SV_MAXGLOBALOPENSEARCH_PARMNUM: DWORD = 565;
pub const SV_REMOVEDUPLICATESEARCHES_PARMNUM: DWORD = 566;
pub const SV_LOCKVIOLATIONRETRIES_PARMNUM: DWORD = 567;
pub const SV_LOCKVIOLATIONOFFSET_PARMNUM: DWORD = 568;
pub const SV_LOCKVIOLATIONDELAY_PARMNUM: DWORD = 569;
pub const SV_MDLREADSWITCHOVER_PARMNUM: DWORD = 570;
pub const SV_CACHEDOPENLIMIT_PARMNUM: DWORD = 571;
pub const SV_CRITICALTHREADS_PARMNUM: DWORD = 572;
pub const SV_RESTRICTNULLSESSACCESS_PARMNUM: DWORD = 573;
pub const SV_ENABLEWFW311DIRECTIPX_PARMNUM: DWORD = 574;
pub const SV_OTHERQUEUEAFFINITY_PARMNUM: DWORD = 575;
pub const SV_QUEUESAMPLESECS_PARMNUM: DWORD = 576;
pub const SV_BALANCECOUNT_PARMNUM: DWORD = 577;
pub const SV_PREFERREDAFFINITY_PARMNUM: DWORD = 578;
pub const SV_MAXFREERFCBS_PARMNUM: DWORD = 579;
pub const SV_MAXFREEMFCBS_PARMNUM: DWORD = 580;
pub const SV_MAXFREELFCBS_PARMNUM: DWORD = 581;
pub const SV_MAXFREEPAGEDPOOLCHUNKS_PARMNUM: DWORD = 582;
pub const SV_MINPAGEDPOOLCHUNKSIZE_PARMNUM: DWORD = 583;
pub const SV_MAXPAGEDPOOLCHUNKSIZE_PARMNUM: DWORD = 584;
pub const SV_SENDSFROMPREFERREDPROCESSOR_PARMNUM: DWORD = 585;
pub const SV_MAXTHREADSPERQUEUE_PARMNUM: DWORD = 586;
pub const SV_CACHEDDIRECTORYLIMIT_PARMNUM: DWORD = 587;
pub const SV_MAXCOPYLENGTH_PARMNUM: DWORD = 588;
pub const SV_ENABLECOMPRESSION_PARMNUM: DWORD = 590;
pub const SV_AUTOSHAREWKS_PARMNUM: DWORD = 591;
pub const SV_AUTOSHARESERVER_PARMNUM: DWORD = 592;
pub const SV_ENABLESECURITYSIGNATURE_PARMNUM: DWORD = 593;
pub const SV_REQUIRESECURITYSIGNATURE_PARMNUM: DWORD = 594;
pub const SV_MINCLIENTBUFFERSIZE_PARMNUM: DWORD = 595;
pub const SV_CONNECTIONNOSESSIONSTIMEOUT_PARMNUM: DWORD = 596;
pub const SV_IDLETHREADTIMEOUT_PARMNUM: DWORD = 597;
pub const SV_ENABLEW9XSECURITYSIGNATURE_PARMNUM: DWORD = 598;
pub const SV_ENFORCEKERBEROSREAUTHENTICATION_PARMNUM: DWORD = 599;
pub const SV_DISABLEDOS_PARMNUM: DWORD = 600;
pub const SV_LOWDISKSPACEMINIMUM_PARMNUM: DWORD = 601;
pub const SV_DISABLESTRICTNAMECHECKING_PARMNUM: DWORD = 602;
pub const SV_ENABLEAUTHENTICATEUSERSHARING_PARMNUM: DWORD = 603;
pub const SV_COMMENT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_COMMENT_PARMNUM;
pub const SV_USERS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_USERS_PARMNUM;
pub const SV_DISC_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_DISC_PARMNUM;
pub const SV_HIDDEN_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_HIDDEN_PARMNUM;
pub const SV_ANNOUNCE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_ANNOUNCE_PARMNUM;
pub const SV_ANNDELTA_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_ANNDELTA_PARMNUM;
pub const SV_SESSOPENS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_SESSOPENS_PARMNUM;
pub const SV_SESSVCS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_SESSVCS_PARMNUM;
pub const SV_OPENSEARCH_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_OPENSEARCH_PARMNUM;
pub const SV_MAXWORKITEMS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MAXWORKITEMS_PARMNUM;
pub const SV_MAXRAWBUFLEN_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MAXRAWBUFLEN_PARMNUM;
pub const SV_SESSUSERS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_SESSUSERS_PARMNUM;
pub const SV_SESSCONNS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_SESSCONNS_PARMNUM;
pub const SV_MAXNONPAGEDMEMORYUSAGE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXNONPAGEDMEMORYUSAGE_PARMNUM;
pub const SV_MAXPAGEDMEMORYUSAGE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXPAGEDMEMORYUSAGE_PARMNUM;
pub const SV_ENABLESOFTCOMPAT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_ENABLESOFTCOMPAT_PARMNUM;
pub const SV_ENABLEFORCEDLOGOFF_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_ENABLEFORCEDLOGOFF_PARMNUM;
pub const SV_TIMESOURCE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_TIMESOURCE_PARMNUM;
pub const SV_LMANNOUNCE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_LMANNOUNCE_PARMNUM;
pub const SV_MAXCOPYREADLEN_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MAXCOPYREADLEN_PARMNUM;
pub const SV_MAXCOPYWRITELEN_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXCOPYWRITELEN_PARMNUM;
pub const SV_MINKEEPSEARCH_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MINKEEPSEARCH_PARMNUM;
pub const SV_MAXKEEPSEARCH_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MAXKEEPSEARCH_PARMNUM;
pub const SV_MINKEEPCOMPLSEARCH_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MINKEEPCOMPLSEARCH_PARMNUM;
pub const SV_MAXKEEPCOMPLSEARCH_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXKEEPCOMPLSEARCH_PARMNUM;
pub const SV_SCAVTIMEOUT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_SCAVTIMEOUT_PARMNUM;
pub const SV_MINRCVQUEUE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MINRCVQUEUE_PARMNUM;
pub const SV_MINFREEWORKITEMS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MINFREEWORKITEMS_PARMNUM;
pub const SV_MAXMPXCT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MAXMPXCT_PARMNUM;
pub const SV_OPLOCKBREAKWAIT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_OPLOCKBREAKWAIT_PARMNUM;
pub const SV_OPLOCKBREAKRESPONSEWAIT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_OPLOCKBREAKRESPONSEWAIT_PARMNUM;
pub const SV_ENABLEOPLOCKS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_ENABLEOPLOCKS_PARMNUM;
pub const SV_ENABLEOPLOCKFORCECLOSE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_ENABLEOPLOCKFORCECLOSE_PARMNUM;
pub const SV_ENABLEFCBOPENS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_ENABLEFCBOPENS_PARMNUM;
pub const SV_ENABLERAW_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_ENABLERAW_PARMNUM;
pub const SV_ENABLESHAREDNETDRIVES_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_ENABLESHAREDNETDRIVES_PARMNUM;
pub const SV_MINFREECONNECTIONS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MINFREECONNECTIONS_PARMNUM;
pub const SV_MAXFREECONNECTIONS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXFREECONNECTIONS_PARMNUM;
pub const SV_INITSESSTABLE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_INITSESSTABLE_PARMNUM;
pub const SV_INITCONNTABLE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_INITCONNTABLE_PARMNUM;
pub const SV_INITFILETABLE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_INITFILETABLE_PARMNUM;
pub const SV_INITSEARCHTABLE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_INITSEARCHTABLE_PARMNUM;
pub const SV_ALERTSCHEDULE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_ALERTSCHEDULE_PARMNUM;
pub const SV_ERRORTHRESHOLD_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_ERRORTHRESHOLD_PARMNUM;
pub const SV_NETWORKERRORTHRESHOLD_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_NETWORKERRORTHRESHOLD_PARMNUM;
pub const SV_DISKSPACETHRESHOLD_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_DISKSPACETHRESHOLD_PARMNUM;
pub const SV_MAXLINKDELAY_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MAXLINKDELAY_PARMNUM;
pub const SV_MINLINKTHROUGHPUT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MINLINKTHROUGHPUT_PARMNUM;
pub const SV_LINKINFOVALIDTIME_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_LINKINFOVALIDTIME_PARMNUM;
pub const SV_SCAVQOSINFOUPDATETIME_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_SCAVQOSINFOUPDATETIME_PARMNUM;
pub const SV_MAXWORKITEMIDLETIME_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXWORKITEMIDLETIME_PARMNUM;
pub const SV_MAXRAWWORKITEMS_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXRAWWORKITEMS_PARMNUM;
pub const SV_PRODUCTTYPE_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_PRODUCTTYPE_PARMNUM;
pub const SV_SERVERSIZE_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_SERVERSIZE_PARMNUM;
pub const SV_CONNECTIONLESSAUTODISC_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_CONNECTIONLESSAUTODISC_PARMNUM;
pub const SV_SHARINGVIOLATIONRETRIES_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_SHARINGVIOLATIONRETRIES_PARMNUM;
pub const SV_SHARINGVIOLATIONDELAY_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_SHARINGVIOLATIONDELAY_PARMNUM;
pub const SV_MAXGLOBALOPENSEARCH_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXGLOBALOPENSEARCH_PARMNUM;
pub const SV_REMOVEDUPLICATESEARCHES_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_REMOVEDUPLICATESEARCHES_PARMNUM;
pub const SV_LOCKVIOLATIONRETRIES_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_LOCKVIOLATIONRETRIES_PARMNUM;
pub const SV_LOCKVIOLATIONOFFSET_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_LOCKVIOLATIONOFFSET_PARMNUM;
pub const SV_LOCKVIOLATIONDELAY_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_LOCKVIOLATIONDELAY_PARMNUM;
pub const SV_MDLREADSWITCHOVER_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MDLREADSWITCHOVER_PARMNUM;
pub const SV_CACHEDOPENLIMIT_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_CACHEDOPENLIMIT_PARMNUM;
pub const SV_CRITICALTHREADS_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_CRITICALTHREADS_PARMNUM;
pub const SV_RESTRICTNULLSESSACCESS_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_RESTRICTNULLSESSACCESS_PARMNUM;
pub const SV_ENABLEWFW311DIRECTIPX_INFOLOEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_ENABLEWFW311DIRECTIPX_PARMNUM;
pub const SV_OTHERQUEUEAFFINITY_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_OTHERQUEUEAFFINITY_PARMNUM;
pub const SV_QUEUESAMPLESECS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_QUEUESAMPLESECS_PARMNUM;
pub const SV_BALANCECOUNT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_BALANCECOUNT_PARMNUM;
pub const SV_PREFERREDAFFINITY_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_PREFERREDAFFINITY_PARMNUM;
pub const SV_MAXFREERFCBS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MAXFREERFCBS_PARMNUM;
pub const SV_MAXFREEMFCBS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MAXFREEMFCBS_PARMNUM;
pub const SV_MAXFREELFCBS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MAXFREELFCBS_PARMNUM;
pub const SV_MAXFREEPAGEDPOOLCHUNKS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXFREEPAGEDPOOLCHUNKS_PARMNUM;
pub const SV_MINPAGEDPOOLCHUNKSIZE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MINPAGEDPOOLCHUNKSIZE_PARMNUM;
pub const SV_MAXPAGEDPOOLCHUNKSIZE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXPAGEDPOOLCHUNKSIZE_PARMNUM;
pub const SV_SENDSFROMPREFERREDPROCESSOR_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_SENDSFROMPREFERREDPROCESSOR_PARMNUM;
pub const SV_MAXTHREADSPERQUEUE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MAXTHREADSPERQUEUE_PARMNUM;
pub const SV_CACHEDDIRECTORYLIMIT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_CACHEDDIRECTORYLIMIT_PARMNUM;
pub const SV_MAXCOPYLENGTH_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_MAXCOPYLENGTH_PARMNUM;
pub const SV_ENABLECOMPRESSION_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_ENABLECOMPRESSION_PARMNUM;
pub const SV_AUTOSHAREWKS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_AUTOSHAREWKS_PARMNUM;
pub const SV_AUTOSHARESERVER_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_AUTOSHARESERVER_PARMNUM;
pub const SV_ENABLESECURITYSIGNATURE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_ENABLESECURITYSIGNATURE_PARMNUM;
pub const SV_REQUIRESECURITYSIGNATURE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_REQUIRESECURITYSIGNATURE_PARMNUM;
pub const SV_MINCLIENTBUFFERSIZE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_MINCLIENTBUFFERSIZE_PARMNUM;
pub const SV_CONNECTIONNOSESSIONSTIMEOUT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_CONNECTIONNOSESSIONSTIMEOUT_PARMNUM;
pub const SV_IDLETHREADTIMEOUT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_IDLETHREADTIMEOUT_PARMNUM;
pub const SV_ENABLEW9XSECURITYSIGNATURE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_ENABLEW9XSECURITYSIGNATURE_PARMNUM;
pub const SV_ENFORCEKERBEROSREAUTHENTICATION_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_ENFORCEKERBEROSREAUTHENTICATION_PARMNUM;
pub const SV_DISABLEDOS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SV_DISABLEDOS_PARMNUM;
pub const SV_LOWDISKSPACEMINIMUM_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_LOWDISKSPACEMINIMUM_PARMNUM;
pub const SV_DISABLESTRICTNAMECHECKING_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_DISABLESTRICTNAMECHECKING_PARMNUM;
pub const SV_ENABLEAUTHENTICATEUSERSHARING_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL
    + SV_ENABLEAUTHENTICATEUSERSHARING_PARMNUM;
pub const SVI1_NUM_ELEMENTS: DWORD = 5;
pub const SVI2_NUM_ELEMENTS: DWORD = 40;
pub const SVI3_NUM_ELEMENTS: DWORD = 44;
pub const SV_MAX_CMD_LEN: DWORD = PATHLEN;
pub const SW_AUTOPROF_LOAD_MASK: DWORD = 0x1;
pub const SW_AUTOPROF_SAVE_MASK: DWORD = 0x2;
pub const SV_MAX_SRV_HEUR_LEN: DWORD = 32;
pub const SV_USERS_PER_LICENSE: DWORD = 5;
pub const SVTI2_REMAP_PIPE_NAMES: DWORD = 0x02;
pub const SVTI2_SCOPED_NAME: DWORD = 0x04;
pub const SVTI2_CLUSTER_NAME: DWORD = 0x08;
pub const SVTI2_CLUSTER_DNN_NAME: DWORD = 0x10;
pub const SVTI2_UNICODE_TRANSPORT_ADDRESS: DWORD = 0x20;
pub const SVTI2_RESERVED1: DWORD = 0x1000;
pub const SVTI2_RESERVED2: DWORD = 0x2000;
pub const SVTI2_RESERVED3: DWORD = 0x4000;
pub const SVTI2_VALID_FLAGS: DWORD = SVTI2_REMAP_PIPE_NAMES | SVTI2_SCOPED_NAME
    | SVTI2_CLUSTER_NAME | SVTI2_CLUSTER_DNN_NAME | SVTI2_UNICODE_TRANSPORT_ADDRESS;
pub const SRV_SUPPORT_HASH_GENERATION: DWORD = 0x0001;
pub const SRV_HASH_GENERATION_ACTIVE: DWORD = 0x0002;
