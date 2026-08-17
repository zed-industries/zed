windows_targets::link!("userenv.dll" "system" fn CreateAppContainerProfile(pszappcontainername : windows_sys::core::PCWSTR, pszdisplayname : windows_sys::core::PCWSTR, pszdescription : windows_sys::core::PCWSTR, pcapabilities : *const super:: SID_AND_ATTRIBUTES, dwcapabilitycount : u32, ppsidappcontainersid : *mut super:: PSID) -> windows_sys::core::HRESULT);
windows_targets::link!("userenv.dll" "system" fn DeleteAppContainerProfile(pszappcontainername : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("userenv.dll" "system" fn DeriveAppContainerSidFromAppContainerName(pszappcontainername : windows_sys::core::PCWSTR, ppsidappcontainersid : *mut super:: PSID) -> windows_sys::core::HRESULT);
windows_targets::link!("userenv.dll" "system" fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid : super:: PSID, pszrestrictedappcontainername : windows_sys::core::PCWSTR, ppsidrestrictedappcontainersid : *mut super:: PSID) -> windows_sys::core::HRESULT);
windows_targets::link!("userenv.dll" "system" fn GetAppContainerFolderPath(pszappcontainersid : windows_sys::core::PCWSTR, ppszpath : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn GetAppContainerNamedObjectPath(token : super::super::Foundation:: HANDLE, appcontainersid : super:: PSID, objectpathlength : u32, objectpath : windows_sys::core::PWSTR, returnlength : *mut u32) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_Registry")]
windows_targets::link!("userenv.dll" "system" fn GetAppContainerRegistryLocation(desiredaccess : u32, phappcontainerkey : *mut super::super::System::Registry:: HKEY) -> windows_sys::core::HRESULT);
windows_targets::link!("isolatedwindowsenvironmentutils.dll" "system" fn IsCrossIsolatedEnvironmentClipboardContent(iscrossisolatedenvironmentclipboardcontent : *mut super::super::Foundation:: BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-security-isolatedcontainer-l1-1-0.dll" "system" fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer : *mut super::super::Foundation:: BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("isolatedwindowsenvironmentutils.dll" "system" fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment : *mut super::super::Foundation:: BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-security-isolatedcontainer-l1-1-1.dll" "system" fn IsProcessInWDAGContainer(reserved : *const core::ffi::c_void, isprocessinwdagcontainer : *mut super::super::Foundation:: BOOL) -> windows_sys::core::HRESULT);
pub const WDAG_CLIPBOARD_TAG: windows_sys::core::PCWSTR = windows_sys::core::w!("CrossIsolatedEnvironmentContent");
pub const IsolatedAppLauncher: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbc812430_e75e_4fd1_9641_1f9f1e2d9a1f);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: super::super::Foundation::BOOL,
    pub CorrelationGUID: windows_sys::core::GUID,
}
