// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD, LPDWORD};
use um::winnt::{LANGID, LCID, LPWSTR, WCHAR};
ENUM!{enum INSTALLSPECTYPE {
    APPNAME = 1,
    FILEEXT,
    PROGID,
    COMCLASS,
}}
STRUCT!{struct INSTALLSPEC_APPNAME {
    Name: *mut WCHAR,
    GPOId: GUID,
}}
STRUCT!{struct INSTALLSPEC_COMCLASS {
    Clsid: GUID,
    ClsCtx: DWORD,
}}
UNION!{union INSTALLSPEC {
    [u32; 5] [u64; 3],
    AppName AppName_mut: INSTALLSPEC_APPNAME,
    FileExt FileExt_mut: *mut WCHAR,
    ProgId ProgId_mut: *mut WCHAR,
    COMClass COMClass_mut: INSTALLSPEC_COMCLASS,
}}
STRUCT!{struct INSTALLDATA {
    Type: INSTALLSPECTYPE,
    Spec: INSTALLSPEC,
}}
pub type PINSTALLDATA = *mut INSTALLDATA;
ENUM!{enum APPSTATE {
    ABSENT,
    ASSIGNED,
    PUBLISHED,
}}
pub const LOCALSTATE_ASSIGNED: DWORD = 0x1;
pub const LOCALSTATE_PUBLISHED: DWORD = 0x2;
pub const LOCALSTATE_UNINSTALL_UNMANAGED: DWORD = 0x4;
pub const LOCALSTATE_POLICYREMOVE_ORPHAN: DWORD = 0x8;
pub const LOCALSTATE_POLICYREMOVE_UNINSTALL: DWORD = 0x10;
pub const LOCALSTATE_ORPHANED: DWORD = 0x20;
pub const LOCALSTATE_UNINSTALLED: DWORD = 0x40;
STRUCT!{struct LOCALMANAGEDAPPLICATION {
    pszDeploymentName: LPWSTR,
    pszPolicyName: LPWSTR,
    pszProductId: LPWSTR,
    dwState: DWORD,
}}
pub type PLOCALMANAGEDAPPLICATION = *mut LOCALMANAGEDAPPLICATION;
pub const MANAGED_APPS_USERAPPLICATIONS: DWORD = 0x1;
pub const MANAGED_APPS_FROMCATEGORY: DWORD = 0x2;
pub const MANAGED_APPS_INFOLEVEL_DEFAULT: DWORD = 0x10000;
pub const MANAGED_APPTYPE_WINDOWSINSTALLER: DWORD = 0x1;
pub const MANAGED_APPTYPE_SETUPEXE: DWORD = 0x2;
pub const MANAGED_APPTYPE_UNSUPPORTED: DWORD = 0x3;
STRUCT!{struct MANAGEDAPPLICATION {
    pszPackageName: LPWSTR,
    pszPublisher: LPWSTR,
    dwVersionHi: DWORD,
    dwVersionLo: DWORD,
    dwRevision: DWORD,
    GpoId: GUID,
    pszPolicyName: LPWSTR,
    ProductId: GUID,
    Language: LANGID,
    pszOwner: LPWSTR,
    pszCompany: LPWSTR,
    pszComments: LPWSTR,
    pszContact: LPWSTR,
    pszSupportUrl: LPWSTR,
    dwPathType: DWORD,
    bInstalled: BOOL,
}}
pub type PMANAGEDAPPLICATION = *mut MANAGEDAPPLICATION;
STRUCT!{struct APPCATEGORYINFO {
    Locale: LCID,
    pszDescription: LPWSTR,
    AppCategoryId: GUID,
}}
STRUCT!{struct APPCATEGORYINFOLIST {
    cCategory: DWORD,
    pCategoryInfo: *mut APPCATEGORYINFO,
}}
extern "system" {
    pub fn InstallApplication(
        pInstallInfo: PINSTALLDATA,
    ) -> DWORD;
    pub fn UninstallApplication(
        ProductCode: LPWSTR,
        dwStatus: DWORD,
    ) -> DWORD;
    pub fn CommandLineFromMsiDescriptor(
        Descriptor: LPWSTR,
        CommandLine: LPWSTR,
        CommandLineLength: *mut DWORD,
    ) -> DWORD;
    pub fn GetManagedApplications(
        pCategory: *mut GUID,
        dwQueryFlags: DWORD,
        dwInfoLevel: DWORD,
        pdwApps: LPDWORD,
        prgManagedApps: *mut PMANAGEDAPPLICATION,
    ) -> DWORD;
    pub fn GetLocalManagedApplications(
        bUserApps: BOOL,
        pdwApps: LPDWORD,
        prgManagedApps: *mut PMANAGEDAPPLICATION,
    ) -> DWORD;
    pub fn GetLocalManagedApplicationData(
        ProductCode: LPWSTR,
        DisplayName: *mut LPWSTR,
        SupportUrl: *mut LPWSTR,
    );
    pub fn GetManagedApplicationCategories(
        dwReserved: DWORD,
        pAppCategory: *mut APPCATEGORYINFOLIST,
    ) -> DWORD;
}
