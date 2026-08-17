#[inline]
pub unsafe fn MSChapSrvChangePassword<P0, P1>(servername: P0, username: P1, lmoldpresent: bool, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn MSChapSrvChangePassword(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, lmoldpresent : bool, lmoldowfpassword : *const LM_OWF_PASSWORD, lmnewowfpassword : *const LM_OWF_PASSWORD, ntoldowfpassword : *const LM_OWF_PASSWORD, ntnewowfpassword : *const LM_OWF_PASSWORD) -> u32);
    unsafe { MSChapSrvChangePassword(servername.param().abi(), username.param().abi(), lmoldpresent, lmoldowfpassword, lmnewowfpassword, ntoldowfpassword, ntnewowfpassword) }
}
#[inline]
pub unsafe fn MSChapSrvChangePassword2<P0, P1>(servername: P0, username: P1, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: bool, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn MSChapSrvChangePassword2(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, newpasswordencryptedwitholdnt : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt : *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent : bool, newpasswordencryptedwitholdlm : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt : *const ENCRYPTED_LM_OWF_PASSWORD) -> u32);
    unsafe { MSChapSrvChangePassword2(servername.param().abi(), username.param().abi(), newpasswordencryptedwitholdnt, oldntowfpasswordencryptedwithnewnt, lmpresent, newpasswordencryptedwitholdlm, oldlmowfpasswordencryptedwithnewlmornt) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CYPHER_BLOCK {
    pub data: [i8; 8],
}
impl Default for CYPHER_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCRYPTED_LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
impl Default for ENCRYPTED_LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
impl Default for LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAMPR_ENCRYPTED_USER_PASSWORD {
    pub Buffer: [u8; 516],
}
impl Default for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
