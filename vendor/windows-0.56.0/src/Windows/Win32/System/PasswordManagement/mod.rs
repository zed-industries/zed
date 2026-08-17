#[inline]
pub unsafe fn MSChapSrvChangePassword<P0, P1, P2>(servername: P0, username: P1, lmoldpresent: P2, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("advapi32.dll" "system" fn MSChapSrvChangePassword(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, lmoldpresent : super::super::Foundation:: BOOLEAN, lmoldowfpassword : *const LM_OWF_PASSWORD, lmnewowfpassword : *const LM_OWF_PASSWORD, ntoldowfpassword : *const LM_OWF_PASSWORD, ntnewowfpassword : *const LM_OWF_PASSWORD) -> u32);
    MSChapSrvChangePassword(servername.param().abi(), username.param().abi(), lmoldpresent.param().abi(), lmoldowfpassword, lmnewowfpassword, ntoldowfpassword, ntnewowfpassword)
}
#[inline]
pub unsafe fn MSChapSrvChangePassword2<P0, P1, P2>(servername: P0, username: P1, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: P2, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("advapi32.dll" "system" fn MSChapSrvChangePassword2(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, newpasswordencryptedwitholdnt : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt : *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent : super::super::Foundation:: BOOLEAN, newpasswordencryptedwitholdlm : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt : *const ENCRYPTED_LM_OWF_PASSWORD) -> u32);
    MSChapSrvChangePassword2(servername.param().abi(), username.param().abi(), newpasswordencryptedwitholdnt, oldntowfpasswordencryptedwithnewnt, lmpresent.param().abi(), newpasswordencryptedwitholdlm, oldlmowfpasswordencryptedwithnewlmornt)
}
#[repr(C)]
pub struct CYPHER_BLOCK {
    pub data: [i8; 8],
}
impl Copy for CYPHER_BLOCK {}
impl Clone for CYPHER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for CYPHER_BLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CYPHER_BLOCK").field("data", &self.data).finish()
    }
}
impl windows_core::TypeKind for CYPHER_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for CYPHER_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Eq for CYPHER_BLOCK {}
impl Default for CYPHER_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ENCRYPTED_LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
impl Copy for ENCRYPTED_LM_OWF_PASSWORD {}
impl Clone for ENCRYPTED_LM_OWF_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for ENCRYPTED_LM_OWF_PASSWORD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ENCRYPTED_LM_OWF_PASSWORD").field("data", &self.data).finish()
    }
}
impl windows_core::TypeKind for ENCRYPTED_LM_OWF_PASSWORD {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for ENCRYPTED_LM_OWF_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Eq for ENCRYPTED_LM_OWF_PASSWORD {}
impl Default for ENCRYPTED_LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
impl Copy for LM_OWF_PASSWORD {}
impl Clone for LM_OWF_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for LM_OWF_PASSWORD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LM_OWF_PASSWORD").field("data", &self.data).finish()
    }
}
impl windows_core::TypeKind for LM_OWF_PASSWORD {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for LM_OWF_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Eq for LM_OWF_PASSWORD {}
impl Default for LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SAMPR_ENCRYPTED_USER_PASSWORD {
    pub Buffer: [u8; 516],
}
impl Copy for SAMPR_ENCRYPTED_USER_PASSWORD {}
impl Clone for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SAMPR_ENCRYPTED_USER_PASSWORD").field("Buffer", &self.Buffer).finish()
    }
}
impl windows_core::TypeKind for SAMPR_ENCRYPTED_USER_PASSWORD {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer
    }
}
impl Eq for SAMPR_ENCRYPTED_USER_PASSWORD {}
impl Default for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
