// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This file contains constants used throughout the LAN Manager API header files.
use shared::minwindef::DWORD;
use um::winnt::{LPCWSTR, LPWSTR};
pub const CNLEN: DWORD = 15;
pub const LM20_CNLEN: DWORD = 15;
pub const DNLEN: DWORD = CNLEN;
pub const LM20_DNLEN: DWORD = LM20_CNLEN;
pub const UNCLEN: DWORD = CNLEN + 2;
pub const LM20_UNCLEN: DWORD = LM20_CNLEN + 2;
pub const NNLEN: DWORD = 80;
pub const LM20_NNLEN: DWORD = 12;
pub const RMLEN: DWORD = UNCLEN + 1 + NNLEN;
pub const LM20_RMLEN: DWORD = LM20_UNCLEN + 1 + LM20_NNLEN;
pub const SNLEN: usize = 80;
pub const LM20_SNLEN: DWORD = 15;
pub const STXTLEN: DWORD = 256;
pub const LM20_STXTLEN: DWORD = 63;
pub const PATHLEN: DWORD = 256;
pub const LM20_PATHLEN: DWORD = 256;
pub const DEVLEN: DWORD = 80;
pub const LM20_DEVLEN: DWORD = 8;
pub const EVLEN: usize = 16;
pub const UNLEN: DWORD = 256;
pub const LM20_UNLEN: DWORD = 20;
pub const GNLEN: DWORD = UNLEN;
pub const LM20_GNLEN: DWORD = LM20_UNLEN;
pub const PWLEN: DWORD = 256;
pub const LM20_PWLEN: DWORD = 14;
pub const SHPWLEN: DWORD = 8;
pub const CLTYPE_LEN: DWORD = 12;
pub const MAXCOMMENTSZ: DWORD = 256;
pub const LM20_MAXCOMMENTSZ: DWORD = 48;
pub const QNLEN: DWORD = NNLEN;
pub const LM20_QNLEN: DWORD = LM20_NNLEN;
pub const ALERTSZ: DWORD = 128;
pub const MAXDEVENTRIES: DWORD = 4 * 8; // FIXME: sizeof(int) instead of 4
pub const NETBIOS_NAME_LEN: DWORD = 16;
pub const MAX_PREFERRED_LENGTH: DWORD = -1i32 as u32;
pub const CRYPT_KEY_LEN: DWORD = 7;
pub const CRYPT_TXT_LEN: DWORD = 8;
pub const ENCRYPTED_PWLEN: usize = 16;
pub const SESSION_PWLEN: DWORD = 24;
pub const SESSION_CRYPT_KLEN: DWORD = 21;
pub const PARM_ERROR_UNKNOWN: DWORD = -1i32 as u32;
pub const PARM_ERROR_NONE: DWORD = 0;
pub const PARMNUM_BASE_INFOLEVEL: DWORD = 1000;
pub type LMSTR = LPWSTR;
pub type LMCSTR = LPCWSTR;
pub type NET_API_STATUS = DWORD;
pub type API_RET_TYPE = NET_API_STATUS;
pub const PLATFORM_ID_DOS: DWORD = 300;
pub const PLATFORM_ID_OS2: DWORD = 400;
pub const PLATFORM_ID_NT: DWORD = 500;
pub const PLATFORM_ID_OSF: DWORD = 600;
pub const PLATFORM_ID_VMS: DWORD = 700;
