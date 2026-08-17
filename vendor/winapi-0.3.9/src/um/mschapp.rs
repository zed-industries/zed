// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::SIZE_T;
use shared::minwindef::{DWORD, UCHAR};
use um::winnt::{BOOLEAN, CHAR, PWSTR};
pub const CYPHER_BLOCK_LENGTH: SIZE_T = 8;
STRUCT!{struct CYPHER_BLOCK {
    data: [CHAR; CYPHER_BLOCK_LENGTH],
}}
STRUCT!{struct LM_OWF_PASSWORD {
    data: [CYPHER_BLOCK; 2],
}}
pub type PLM_OWF_PASSWORD = *mut LM_OWF_PASSWORD;
pub type NT_OWF_PASSWORD = LM_OWF_PASSWORD;
pub type PNT_OWF_PASSWORD = *mut NT_OWF_PASSWORD;
STRUCT!{struct SAMPR_ENCRYPTED_USER_PASSWORD {
    Buffer: [UCHAR; (256 * 2) + 4],
}}
pub type PSAMPR_ENCRYPTED_USER_PASSWORD = *mut SAMPR_ENCRYPTED_USER_PASSWORD;
STRUCT!{struct ENCRYPTED_LM_OWF_PASSWORD {
    data: [CYPHER_BLOCK; 2],
}}
pub type PENCRYPTED_LM_OWF_PASSWORD = *mut ENCRYPTED_LM_OWF_PASSWORD;
pub type ENCRYPTED_NT_OWF_PASSWORD = ENCRYPTED_LM_OWF_PASSWORD;
pub type PENCRYPTED_NT_OWF_PASSWORD = *mut ENCRYPTED_NT_OWF_PASSWORD;
extern "system" {
    pub fn MSChapSrvChangePassword(
        ServerName: PWSTR,
        UserName: PWSTR,
        LmOldPresent: BOOLEAN,
        LmOldOwfPassword: PLM_OWF_PASSWORD,
        LmNewOwfPassword: PLM_OWF_PASSWORD,
        NtOldOwfPassword: PNT_OWF_PASSWORD,
        NtNewOwfPassword: PNT_OWF_PASSWORD,
    ) -> DWORD;
    pub fn MSChapSrvChangePassword2(
        ServerName: PWSTR,
        UserName: PWSTR,
        NewPasswordEncryptedWithOldNt: PSAMPR_ENCRYPTED_USER_PASSWORD,
        OldNtOwfPasswordEncryptedWithNewNt: PENCRYPTED_NT_OWF_PASSWORD,
        LmPresent: BOOLEAN,
        NewPasswordEncryptedWithOldLm: PSAMPR_ENCRYPTED_USER_PASSWORD,
        OldLmOwfPasswordEncryptedWithNewLmOrNt: PENCRYPTED_LM_OWF_PASSWORD,
    ) -> DWORD;
}
