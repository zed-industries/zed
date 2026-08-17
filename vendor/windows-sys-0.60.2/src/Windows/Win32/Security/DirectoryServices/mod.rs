#[cfg(feature = "Win32_UI_Controls")]
windows_targets::link!("dssec.dll" "system" fn DSCreateSecurityPage(pwszobjectpath : windows_sys::core::PCWSTR, pwszobjectclass : windows_sys::core::PCWSTR, dwflags : u32, phpage : *mut super::super::UI::Controls:: HPROPSHEETPAGE, pfnreadsd : PFNREADOBJECTSECURITY, pfnwritesd : PFNWRITEOBJECTSECURITY, lpcontext : super::super::Foundation:: LPARAM) -> windows_sys::core::HRESULT);
windows_targets::link!("dssec.dll" "system" fn DSEditSecurity(hwndowner : super::super::Foundation:: HWND, pwszobjectpath : windows_sys::core::PCWSTR, pwszobjectclass : windows_sys::core::PCWSTR, dwflags : u32, pwszcaption : windows_sys::core::PCWSTR, pfnreadsd : PFNREADOBJECTSECURITY, pfnwritesd : PFNWRITEOBJECTSECURITY, lpcontext : super::super::Foundation:: LPARAM) -> windows_sys::core::HRESULT);
pub const DSSI_IS_ROOT: u32 = 16u32;
pub const DSSI_NO_ACCESS_CHECK: u32 = 2u32;
pub const DSSI_NO_EDIT_OWNER: u32 = 8u32;
pub const DSSI_NO_EDIT_SACL: u32 = 4u32;
pub const DSSI_NO_FILTER: u32 = 32u32;
pub const DSSI_NO_READONLY_MESSAGE: u32 = 64u32;
pub const DSSI_READ_ONLY: u32 = 1u32;
#[cfg(feature = "Win32_UI_Controls")]
pub type PFNDSCREATESECPAGE = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: u32, param3: *mut super::super::UI::Controls::HPROPSHEETPAGE, param4: PFNREADOBJECTSECURITY, param5: PFNWRITEOBJECTSECURITY, param6: super::super::Foundation::LPARAM) -> windows_sys::core::HRESULT>;
pub type PFNDSEDITSECURITY = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: u32, param4: windows_sys::core::PCWSTR, param5: PFNREADOBJECTSECURITY, param6: PFNWRITEOBJECTSECURITY, param7: super::super::Foundation::LPARAM) -> windows_sys::core::HRESULT>;
pub type PFNREADOBJECTSECURITY = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: u32, param2: *mut super::PSECURITY_DESCRIPTOR, param3: super::super::Foundation::LPARAM) -> windows_sys::core::HRESULT>;
pub type PFNWRITEOBJECTSECURITY = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: u32, param2: super::PSECURITY_DESCRIPTOR, param3: super::super::Foundation::LPARAM) -> windows_sys::core::HRESULT>;
