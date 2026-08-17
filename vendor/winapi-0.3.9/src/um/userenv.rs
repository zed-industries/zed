// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Definitions for the user environment API
use shared::minwindef::{BOOL, DWORD, LPDWORD, LPVOID, PHKEY};
use um::winnt::{
    HANDLE, HRESULT, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PCWSTR, PSID, PSID_AND_ATTRIBUTES, PWSTR
};
use um::winreg::REGSAM;
extern "system" {
    // pub fn LoadUserProfileA(
    //     hToken: HANDLE,
    //     lpProfileInfo: LPPROFILEINFOA,
    // ) -> BOOL;
    // pub fn LoadUserProfileW(
    //     hToken: HANDLE,
    //     lpProfileInfo: LPPROFILEINFOW,
    // ) -> BOOL;
    pub fn UnloadUserProfile(
        hToken: HANDLE,
        hProfile: HANDLE,
    ) -> BOOL;
    pub fn GetProfilesDirectoryA(
        lpProfileDir: LPSTR,
        lpcchSize: LPDWORD,
    ) -> BOOL;
    pub fn GetProfilesDirectoryW(
        lpProfileDir: LPWSTR,
        lpcchSize: LPDWORD,
    ) -> BOOL;
    pub fn GetProfileType(
        dwFlags: *mut DWORD,
    ) -> BOOL;
    pub fn DeleteProfileA(
        lpSidString: LPCSTR,
        lpProfilePath: LPCSTR,
        lpComputerName: LPCSTR,
    ) -> BOOL;
    pub fn DeleteProfileW(
        lpSidString: LPCWSTR,
        lpProfilePath: LPCWSTR,
        lpComputerName: LPCWSTR,
    ) -> BOOL;
    pub fn CreateProfile(
        pszUserSid: LPCWSTR,
        pszUserName: LPCWSTR,
        pszProfilePath: LPWSTR,
        cchProfilePath: DWORD,
    ) -> HRESULT;
    pub fn GetDefaultUserProfileDirectoryA(
        lpProfileDir: LPSTR,
        lpcchSize: LPDWORD,
    ) -> BOOL;
    pub fn GetDefaultUserProfileDirectoryW(
        lpProfileDir: LPWSTR,
        lpcchSize: LPDWORD,
    ) -> BOOL;
    pub fn GetAllUsersProfileDirectoryA(
        lpProfileDir: LPSTR,
        lpcchSize: LPDWORD,
    ) -> BOOL;
    pub fn GetAllUsersProfileDirectoryW(
        lpProfileDir: LPWSTR,
        lpcchSize: LPDWORD,
    ) -> BOOL;
    pub fn GetUserProfileDirectoryA(
        hToken: HANDLE,
        lpProfileDir: LPSTR,
        lpcchSize: LPDWORD,
    ) -> BOOL;
    pub fn GetUserProfileDirectoryW(
        hToken: HANDLE,
        lpProfileDir: LPWSTR,
        lpcchSize: LPDWORD,
    ) -> BOOL;
    pub fn CreateEnvironmentBlock(
        lpEnvironment: *mut LPVOID,
        hToken: HANDLE,
        bInherit: BOOL,
    ) -> BOOL;
    pub fn DestroyEnvironmentBlock(
        lpEnvironment: LPVOID,
    ) -> BOOL;
    pub fn ExpandEnvironmentStringsForUserA(
        hToken: HANDLE,
        lpSrc: LPCSTR,
        lpDest: LPSTR,
        dwSize: DWORD,
    ) -> BOOL;
    pub fn ExpandEnvironmentStringsForUserW(
        hToken: HANDLE,
        lpSrc: LPCWSTR,
        lpDest: LPWSTR,
        dwSize: DWORD,
    ) -> BOOL;
    pub fn RefreshPolicy(
        bMachine: BOOL,
    ) -> BOOL;
    pub fn RefreshPolicyEx(
        bMachine: BOOL,
        dwOptions: DWORD,
    ) -> BOOL;
    pub fn EnterCriticalPolicySection(
        bMachine: BOOL,
    ) -> HANDLE;
    pub fn LeaveCriticalPolicySection(
        hSection: HANDLE,
    ) -> BOOL;
    pub fn RegisterGPNotification(
        hEvent: HANDLE,
        bMachine: BOOL,
    ) -> BOOL;
    pub fn UnregisterGPNotification(
        hEvent: HANDLE,
    ) -> BOOL;
    // pub fn GetGPOListA();
    // pub fn GetGPOListW();
    // pub fn FreeGPOListA();
    // pub fn FreeGPOListW();
    // pub fn GetAppliedGPOListA();
    // pub fn GetAppliedGPOListW();
    // pub fn ProcessGroupPolicyCompleted();
    // pub fn ProcessGroupPolicyCompletedEx();
    // pub fn RsopAccessCheckByType();
    // pub fn RsopFileAccessCheck();
    // pub fn RsopSetPolicySettingStatus();
    // pub fn RsopResetPolicySettingStatus();
    // pub fn GenerateGPNotification();
    pub fn CreateAppContainerProfile(
        pszAppContainerName: PCWSTR,
        pszDisplayName: PCWSTR,
        pszDescription: PCWSTR,
        pCapabilities: PSID_AND_ATTRIBUTES,
        dwCapabilityCount: DWORD,
        ppSidAppContainerSid: *mut PSID,
    ) -> HRESULT;
    pub fn DeleteAppContainerProfile(
        pszAppContainerName: PCWSTR,
    ) -> HRESULT;
    pub fn GetAppContainerRegistryLocation(
        desiredAccess: REGSAM,
        phAppContainerKey: PHKEY,
    ) -> HRESULT;
    pub fn GetAppContainerFolderPath(
        pszAppContainerSid: PCWSTR,
        ppszPath: *mut PWSTR,
    ) -> HRESULT;
    pub fn DeriveAppContainerSidFromAppContainerName(
        pszAppContainerName: PCWSTR,
        ppsidAppContainerSid: *mut PSID,
    ) -> HRESULT;
    pub fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(
        psidAppContainerSid: PSID,
        pszRestrictedAppContainerName: PCWSTR,
        ppsidRestrictedAppContainerSid: *mut PSID,
    ) -> HRESULT;
}
