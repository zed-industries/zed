use shared::minwindef::BOOL;
use shared::ntdef::{PHANDLE, ULONG};
//1286
extern "system" {
    pub fn WTSQueryUserToken(SessionId: ULONG, phToken: PHANDLE) -> BOOL;
}
