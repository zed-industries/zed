#[inline]
pub unsafe fn GetDeviceID(pbwindowsaik: Option<&mut [u8]>, pcbresult: *mut u32, pfprotectedbytpm: Option<*mut windows_core::BOOL>) -> windows_core::Result<()> {
    windows_core::link!("tbs.dll" "system" fn GetDeviceID(pbwindowsaik : *mut u8, cbwindowsaik : u32, pcbresult : *mut u32, pfprotectedbytpm : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { GetDeviceID(core::mem::transmute(pbwindowsaik.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbwindowsaik.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, pfprotectedbytpm.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GetDeviceIDString(pszwindowsaik: Option<&mut [u16]>, pcchresult: *mut u32, pfprotectedbytpm: Option<*mut windows_core::BOOL>) -> windows_core::Result<()> {
    windows_core::link!("tbs.dll" "system" fn GetDeviceIDString(pszwindowsaik : windows_core::PWSTR, cchwindowsaik : u32, pcchresult : *mut u32, pfprotectedbytpm : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { GetDeviceIDString(core::mem::transmute(pszwindowsaik.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszwindowsaik.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcchresult as _, pfprotectedbytpm.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn Tbsi_Context_Create(pcontextparams: *const TBS_CONTEXT_PARAMS, phcontext: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsi_Context_Create(pcontextparams : *const TBS_CONTEXT_PARAMS, phcontext : *mut *mut core::ffi::c_void) -> u32);
    unsafe { Tbsi_Context_Create(pcontextparams, phcontext as _) }
}
#[inline]
pub unsafe fn Tbsi_Create_Windows_Key(keyhandle: u32) -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsi_Create_Windows_Key(keyhandle : u32) -> u32);
    unsafe { Tbsi_Create_Windows_Key(keyhandle) }
}
#[inline]
pub unsafe fn Tbsi_GetDeviceInfo(size: u32, info: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsi_GetDeviceInfo(size : u32, info : *mut core::ffi::c_void) -> u32);
    unsafe { Tbsi_GetDeviceInfo(size, info as _) }
}
#[inline]
pub unsafe fn Tbsi_Get_OwnerAuth(hcontext: *const core::ffi::c_void, ownerauthtype: u32, poutputbuf: Option<*mut u8>, poutputbuflen: *mut u32) -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsi_Get_OwnerAuth(hcontext : *const core::ffi::c_void, ownerauthtype : u32, poutputbuf : *mut u8, poutputbuflen : *mut u32) -> u32);
    unsafe { Tbsi_Get_OwnerAuth(hcontext, ownerauthtype, poutputbuf.unwrap_or(core::mem::zeroed()) as _, poutputbuflen as _) }
}
#[inline]
pub unsafe fn Tbsi_Get_TCG_Log(hcontext: *const core::ffi::c_void, poutputbuf: Option<*mut u8>, poutputbuflen: *mut u32) -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsi_Get_TCG_Log(hcontext : *const core::ffi::c_void, poutputbuf : *mut u8, poutputbuflen : *mut u32) -> u32);
    unsafe { Tbsi_Get_TCG_Log(hcontext, poutputbuf.unwrap_or(core::mem::zeroed()) as _, poutputbuflen as _) }
}
#[inline]
pub unsafe fn Tbsi_Get_TCG_Log_Ex(logtype: u32, pboutput: Option<*mut u8>, pcboutput: *mut u32) -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsi_Get_TCG_Log_Ex(logtype : u32, pboutput : *mut u8, pcboutput : *mut u32) -> u32);
    unsafe { Tbsi_Get_TCG_Log_Ex(logtype, pboutput.unwrap_or(core::mem::zeroed()) as _, pcboutput as _) }
}
#[inline]
pub unsafe fn Tbsi_Is_Tpm_Present() -> windows_core::BOOL {
    windows_core::link!("tbs.dll" "system" fn Tbsi_Is_Tpm_Present() -> windows_core::BOOL);
    unsafe { Tbsi_Is_Tpm_Present() }
}
#[inline]
pub unsafe fn Tbsi_Physical_Presence_Command(hcontext: *const core::ffi::c_void, pabinput: &[u8], paboutput: *mut u8, pcboutput: *mut u32) -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsi_Physical_Presence_Command(hcontext : *const core::ffi::c_void, pabinput : *const u8, cbinput : u32, paboutput : *mut u8, pcboutput : *mut u32) -> u32);
    unsafe { Tbsi_Physical_Presence_Command(hcontext, core::mem::transmute(pabinput.as_ptr()), pabinput.len().try_into().unwrap(), paboutput as _, pcboutput as _) }
}
#[inline]
pub unsafe fn Tbsi_Revoke_Attestation() -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsi_Revoke_Attestation() -> u32);
    unsafe { Tbsi_Revoke_Attestation() }
}
#[inline]
pub unsafe fn Tbsip_Cancel_Commands(hcontext: *const core::ffi::c_void) -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsip_Cancel_Commands(hcontext : *const core::ffi::c_void) -> u32);
    unsafe { Tbsip_Cancel_Commands(hcontext) }
}
#[inline]
pub unsafe fn Tbsip_Context_Close(hcontext: *const core::ffi::c_void) -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsip_Context_Close(hcontext : *const core::ffi::c_void) -> u32);
    unsafe { Tbsip_Context_Close(hcontext) }
}
#[inline]
pub unsafe fn Tbsip_Submit_Command(hcontext: *const core::ffi::c_void, locality: TBS_COMMAND_LOCALITY, priority: TBS_COMMAND_PRIORITY, pabcommand: &[u8], pabresult: *mut u8, pcbresult: *mut u32) -> u32 {
    windows_core::link!("tbs.dll" "system" fn Tbsip_Submit_Command(hcontext : *const core::ffi::c_void, locality : TBS_COMMAND_LOCALITY, priority : TBS_COMMAND_PRIORITY, pabcommand : *const u8, cbcommand : u32, pabresult : *mut u8, pcbresult : *mut u32) -> u32);
    unsafe { Tbsip_Submit_Command(hcontext, locality, priority, core::mem::transmute(pabcommand.as_ptr()), pabcommand.len().try_into().unwrap(), pabresult as _, pcbresult as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBS_COMMAND_LOCALITY(pub u32);
pub const TBS_COMMAND_LOCALITY_FOUR: TBS_COMMAND_LOCALITY = TBS_COMMAND_LOCALITY(4u32);
pub const TBS_COMMAND_LOCALITY_ONE: TBS_COMMAND_LOCALITY = TBS_COMMAND_LOCALITY(1u32);
pub const TBS_COMMAND_LOCALITY_THREE: TBS_COMMAND_LOCALITY = TBS_COMMAND_LOCALITY(3u32);
pub const TBS_COMMAND_LOCALITY_TWO: TBS_COMMAND_LOCALITY = TBS_COMMAND_LOCALITY(2u32);
pub const TBS_COMMAND_LOCALITY_ZERO: TBS_COMMAND_LOCALITY = TBS_COMMAND_LOCALITY(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TBS_COMMAND_PRIORITY(pub u32);
pub const TBS_COMMAND_PRIORITY_HIGH: TBS_COMMAND_PRIORITY = TBS_COMMAND_PRIORITY(300u32);
pub const TBS_COMMAND_PRIORITY_LOW: TBS_COMMAND_PRIORITY = TBS_COMMAND_PRIORITY(100u32);
pub const TBS_COMMAND_PRIORITY_MAX: TBS_COMMAND_PRIORITY = TBS_COMMAND_PRIORITY(2147483648u32);
pub const TBS_COMMAND_PRIORITY_NORMAL: TBS_COMMAND_PRIORITY = TBS_COMMAND_PRIORITY(200u32);
pub const TBS_COMMAND_PRIORITY_SYSTEM: TBS_COMMAND_PRIORITY = TBS_COMMAND_PRIORITY(400u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBS_CONTEXT_PARAMS {
    pub version: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TBS_CONTEXT_PARAMS2 {
    pub version: u32,
    pub Anonymous: TBS_CONTEXT_PARAMS2_0,
}
impl Default for TBS_CONTEXT_PARAMS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TBS_CONTEXT_PARAMS2_0 {
    pub Anonymous: TBS_CONTEXT_PARAMS2_0_0,
    pub asUINT32: u32,
}
impl Default for TBS_CONTEXT_PARAMS2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBS_CONTEXT_PARAMS2_0_0 {
    pub _bitfield: u32,
}
pub const TBS_CONTEXT_VERSION_ONE: u32 = 1u32;
pub const TBS_CONTEXT_VERSION_TWO: u32 = 2u32;
pub const TBS_OWNERAUTH_TYPE_ADMIN: u32 = 2u32;
pub const TBS_OWNERAUTH_TYPE_ENDORSEMENT: u32 = 4u32;
pub const TBS_OWNERAUTH_TYPE_ENDORSEMENT_20: u32 = 12u32;
pub const TBS_OWNERAUTH_TYPE_FULL: u32 = 1u32;
pub const TBS_OWNERAUTH_TYPE_STORAGE_20: u32 = 13u32;
pub const TBS_OWNERAUTH_TYPE_USER: u32 = 3u32;
pub const TBS_SUCCESS: u32 = 0u32;
pub const TBS_TCGLOG_DRTM_BOOT: u32 = 4u32;
pub const TBS_TCGLOG_DRTM_CURRENT: u32 = 1u32;
pub const TBS_TCGLOG_DRTM_RESUME: u32 = 5u32;
pub const TBS_TCGLOG_SRTM_BOOT: u32 = 2u32;
pub const TBS_TCGLOG_SRTM_CURRENT: u32 = 0u32;
pub const TBS_TCGLOG_SRTM_RESUME: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TPM_DEVICE_INFO {
    pub structVersion: u32,
    pub tpmVersion: u32,
    pub tpmInterfaceType: u32,
    pub tpmImpRevision: u32,
}
pub const TPM_IFTYPE_1: u32 = 1u32;
pub const TPM_IFTYPE_EMULATOR: u32 = 4u32;
pub const TPM_IFTYPE_HW: u32 = 3u32;
pub const TPM_IFTYPE_SPB: u32 = 5u32;
pub const TPM_IFTYPE_TRUSTZONE: u32 = 2u32;
pub const TPM_IFTYPE_UNKNOWN: u32 = 0u32;
pub const TPM_VERSION_12: u32 = 1u32;
pub const TPM_VERSION_20: u32 = 2u32;
pub const TPM_VERSION_UNKNOWN: u32 = 0u32;
pub const TPM_WNF_INFO_CLEAR_SUCCESSFUL: u32 = 1u32;
pub const TPM_WNF_INFO_NO_REBOOT_REQUIRED: u32 = 1u32;
pub const TPM_WNF_INFO_OWNERSHIP_SUCCESSFUL: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TPM_WNF_PROVISIONING {
    pub status: u32,
    pub message: [u8; 28],
}
impl Default for TPM_WNF_PROVISIONING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
