// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module contains the definitions for portable NetBIOS 3.0 support.
use shared::minwindef::{DWORD, PUCHAR, UCHAR, ULONG, USHORT, WORD};
use um::winnt::HANDLE;
pub const NCBNAMSZ: usize = 16;
pub const MAX_LANA: usize = 254;
FN!{stdcall PFPOST(
    *mut NCB,
) -> ()}
#[cfg(target_pointer_width = "64")]
STRUCT!{struct NCB {
    ncb_command: UCHAR,
    ncb_retcode: UCHAR,
    ncb_lsn: UCHAR,
    ncb_num: UCHAR,
    ncb_buffer: PUCHAR,
    ncb_length: WORD,
    ncb_callname: [UCHAR; NCBNAMSZ],
    ncb_name: [UCHAR; NCBNAMSZ],
    ncb_rto: UCHAR,
    ncb_sto: UCHAR,
    ncb_post: PFPOST,
    ncb_lana_num: UCHAR,
    ncb_cmd_cplt: UCHAR,
    ncb_reserve: [UCHAR; 18],
    ncb_event: HANDLE,
}}
#[cfg(target_pointer_width = "32")]
STRUCT!{struct NCB {
    ncb_command: UCHAR,
    ncb_retcode: UCHAR,
    ncb_lsn: UCHAR,
    ncb_num: UCHAR,
    ncb_buffer: PUCHAR,
    ncb_length: WORD,
    ncb_callname: [UCHAR; NCBNAMSZ],
    ncb_name: [UCHAR; NCBNAMSZ],
    ncb_rto: UCHAR,
    ncb_sto: UCHAR,
    ncb_post: PFPOST,
    ncb_lana_num: UCHAR,
    ncb_cmd_cplt: UCHAR,
    ncb_reserve: [UCHAR; 10],
    ncb_event: HANDLE,
}}
pub type PNCB = *mut NCB;
STRUCT!{struct ADAPTER_STATUS {
    adapter_address: [UCHAR; 6],
    rev_major: UCHAR,
    reserved0: UCHAR,
    adapter_type: UCHAR,
    rev_minor: UCHAR,
    duration: WORD,
    frmr_recv: WORD,
    frmr_xmit: WORD,
    iframe_recv_err: WORD,
    xmit_aborts: WORD,
    xmit_success: DWORD,
    recv_success: DWORD,
    iframe_xmit_err: WORD,
    recv_buff_unavail: WORD,
    t1_timeouts: WORD,
    ti_timeouts: WORD,
    reserved1: DWORD,
    free_ncbs: WORD,
    max_cfg_ncbs: WORD,
    max_ncbs: WORD,
    xmit_buf_unavail: WORD,
    max_dgram_size: WORD,
    pending_sess: WORD,
    max_cfg_sess: WORD,
    max_sess: WORD,
    max_sess_pkt_size: WORD,
    name_count: WORD,
}}
pub type PADAPTER_STATUS = *mut ADAPTER_STATUS;
STRUCT!{struct NAME_BUFFER {
    name: [UCHAR; NCBNAMSZ],
    name_num: UCHAR,
    name_flags: UCHAR,
}}
pub type PNAME_BUFFER = *mut NAME_BUFFER;
pub const NAME_FLAGS_MASK: UCHAR = 0x87;
pub const GROUP_NAME: UCHAR = 0x80;
pub const UNIQUE_NAME: UCHAR = 0x00;
pub const REGISTERING: UCHAR = 0x00;
pub const REGISTERED: UCHAR = 0x04;
pub const DEREGISTERED: UCHAR = 0x05;
pub const DUPLICATE: UCHAR = 0x06;
pub const DUPLICATE_DEREG: UCHAR = 0x07;
STRUCT!{struct SESSION_HEADER {
    sess_name: UCHAR,
    num_sess: UCHAR,
    rcv_dg_outstanding: UCHAR,
    rcv_any_outstanding: UCHAR,
}}
pub type PSESSION_HEADER = *mut SESSION_HEADER;
STRUCT!{struct SESSION_BUFFER {
    lsn: UCHAR,
    state: UCHAR,
    local_name: [UCHAR; NCBNAMSZ],
    remote_name: [UCHAR; NCBNAMSZ],
    rcvs_outstanding: UCHAR,
    sends_outstanding: UCHAR,
}}
pub type PSESSION_BUFFER = *mut SESSION_BUFFER;
pub const LISTEN_OUTSTANDING: UCHAR = 0x01;
pub const CALL_PENDING: UCHAR = 0x02;
pub const SESSION_ESTABLISHED: UCHAR = 0x03;
pub const HANGUP_PENDING: UCHAR = 0x04;
pub const HANGUP_COMPLETE: UCHAR = 0x05;
pub const SESSION_ABORTED: UCHAR = 0x06;
STRUCT!{struct LANA_ENUM {
    length: UCHAR,
    lana: [UCHAR; MAX_LANA + 1],
}}
pub type PLANA_ENUM = *mut LANA_ENUM;
STRUCT!{struct FIND_NAME_HEADER {
    node_count: WORD,
    reserved: UCHAR,
    unique_group: UCHAR,
}}
pub type PFIND_NAME_HEADER = *mut FIND_NAME_HEADER;
STRUCT!{struct FIND_NAME_BUFFER {
    length: UCHAR,
    access_control: UCHAR,
    frame_control: UCHAR,
    destination_addr: [UCHAR; 6],
    source_addr: [UCHAR; 6],
    routing_info: [UCHAR; 18],
}}
pub type PFIND_NAME_BUFFER = *mut FIND_NAME_BUFFER;
STRUCT!{struct ACTION_HEADER {
    transport_id: ULONG,
    action_code: USHORT,
    reserved: USHORT,
}}
pub type PACTION_HEADER = *mut ACTION_HEADER;
pub const ALL_TRANSPORTS: ULONG = 0x0000004d;
pub const MS_NBF: ULONG = 0x46424e4d;
pub const NCBCALL: UCHAR = 0x10;
pub const NCBLISTEN: UCHAR = 0x11;
pub const NCBHANGUP: UCHAR = 0x12;
pub const NCBSEND: UCHAR = 0x14;
pub const NCBRECV: UCHAR = 0x15;
pub const NCBRECVANY: UCHAR = 0x16;
pub const NCBCHAINSEND: UCHAR = 0x17;
pub const NCBDGSEND: UCHAR = 0x20;
pub const NCBDGRECV: UCHAR = 0x21;
pub const NCBDGSENDBC: UCHAR = 0x22;
pub const NCBADDNAME: UCHAR = 0x30;
pub const NCBDELNAME: UCHAR = 0x31;
pub const NCBRESET: UCHAR = 0x32;
pub const NCBASTAT: UCHAR = 0x33;
pub const NCBSSTAT: UCHAR = 0x34;
pub const NCBCANCEL: UCHAR = 0x35;
pub const NCBADDGRNAME: UCHAR = 0x36;
pub const NCBENUM: UCHAR = 0x37;
pub const NCBUNLINK: UCHAR = 0x70;
pub const NCBSENDNA: UCHAR = 0x71;
pub const NCBCHAINSENDNA: UCHAR = 0x72;
pub const NCBLANSTALERT: UCHAR = 0x73;
pub const NCBACTION: UCHAR = 0x77;
pub const NCBFINDNAME: UCHAR = 0x78;
pub const NCBTRACE: UCHAR = 0x79;
pub const ASYNCH: UCHAR = 0x80;
pub const NRC_GOODRET: UCHAR = 0x00;
pub const NRC_BUFLEN: UCHAR = 0x01;
pub const NRC_ILLCMD: UCHAR = 0x03;
pub const NRC_CMDTMO: UCHAR = 0x05;
pub const NRC_INCOMP: UCHAR = 0x06;
pub const NRC_BADDR: UCHAR = 0x07;
pub const NRC_SNUMOUT: UCHAR = 0x08;
pub const NRC_NORES: UCHAR = 0x09;
pub const NRC_SCLOSED: UCHAR = 0x0a;
pub const NRC_CMDCAN: UCHAR = 0x0b;
pub const NRC_DUPNAME: UCHAR = 0x0d;
pub const NRC_NAMTFUL: UCHAR = 0x0e;
pub const NRC_ACTSES: UCHAR = 0x0f;
pub const NRC_LOCTFUL: UCHAR = 0x11;
pub const NRC_REMTFUL: UCHAR = 0x12;
pub const NRC_ILLNN: UCHAR = 0x13;
pub const NRC_NOCALL: UCHAR = 0x14;
pub const NRC_NOWILD: UCHAR = 0x15;
pub const NRC_INUSE: UCHAR = 0x16;
pub const NRC_NAMERR: UCHAR = 0x17;
pub const NRC_SABORT: UCHAR = 0x18;
pub const NRC_NAMCONF: UCHAR = 0x19;
pub const NRC_IFBUSY: UCHAR = 0x21;
pub const NRC_TOOMANY: UCHAR = 0x22;
pub const NRC_BRIDGE: UCHAR = 0x23;
pub const NRC_CANOCCR: UCHAR = 0x24;
pub const NRC_CANCEL: UCHAR = 0x26;
pub const NRC_DUPENV: UCHAR = 0x30;
pub const NRC_ENVNOTDEF: UCHAR = 0x34;
pub const NRC_OSRESNOTAV: UCHAR = 0x35;
pub const NRC_MAXAPPS: UCHAR = 0x36;
pub const NRC_NOSAPS: UCHAR = 0x37;
pub const NRC_NORESOURCES: UCHAR = 0x38;
pub const NRC_INVADDRESS: UCHAR = 0x39;
pub const NRC_INVDDID: UCHAR = 0x3B;
pub const NRC_LOCKFAIL: UCHAR = 0x3C;
pub const NRC_OPENERR: UCHAR = 0x3f;
pub const NRC_SYSTEM: UCHAR = 0x40;
pub const NRC_PENDING: UCHAR = 0xff;
extern "system" {
    pub fn Netbios(
        pncb: PNCB,
    ) -> UCHAR;
}
